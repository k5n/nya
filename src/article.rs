use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::{ Path, PathBuf };
use github::GitHub;
use article_id::ArticleId;
use git::GitRepository;
use super::{ Result, Failable };

const META_FILE_NAME: &'static str = "meta.json";

fn is_draft_path(path: &Path) -> Option<bool> {
    let is_draft = path.starts_with("draft");
    if !is_draft && !path.starts_with("post") {
        None
    } else {
        Some(is_draft)
    }
}

fn copy_all_files<P: AsRef<Path>, Q: AsRef<Path>>(from_dir: P, to_dir: Q)
        -> Failable {
    let from_dir = from_dir.as_ref();
    let to_dir = to_dir.as_ref();
    if !from_dir.is_dir() || !to_dir.is_dir() {
        return Err(From::from("Both 'from_dir' and 'to_dir' must be directories."))
    }

    for entry in fs::read_dir(&from_dir)? {
        let entry: ::std::fs::DirEntry = entry?;
        if entry.file_type()?.is_file() {
            let from: PathBuf = entry.path();
            let filename = from.file_name()
                .ok_or(format!("Failed to get filename from path: {}",
                               from.display()))?;
            let to = to_dir.join(&filename);
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}

struct ArticleFile {
    path: PathBuf
}

impl ArticleFile {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        ArticleFile { path: path.as_ref().to_owned() }
    }

    fn generate_empty_file(&self) -> Failable {
        File::create(&self.path)?;
        Ok(())
    }

    fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    fn get_filename(&self) -> &str {
        self.get_path()
            .file_name().expect("ArticleFile has no filename.")
            .to_str().expect("ArticleFile's filename is not UTF-8.")
    }

    fn get_content(&self) -> Result<String> {
        let mut f = File::open(&self.path)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }

    fn get_title(&self) -> Result<String> {
        let content = self.get_content()?;
        let first_line = content.as_str().lines().nth(0)
            .ok_or("Failed to find the title of this article.")?;
        Ok(first_line.to_string())
    }
}

pub struct MetaFile {
    path: PathBuf
}

impl MetaFile {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        MetaFile { path: path.as_ref().to_owned() }
    }

    fn generate_empty_file(&self) -> Failable {
        File::create(&self.path)?
            .write_all(b"{\n  \"tags\": []\n}\n")?;
        Ok(())
    }
}

pub struct Article {
    id: ArticleId,
    is_draft: bool,
    article_file: ArticleFile,
    meta_file: MetaFile,
}

impl Article {
    /// generates a new draft article.
    pub fn generate() -> Failable {
        let id = ArticleId::generate_draft_id();
        let article = Article::new(Path::new("draft").join(&id.id))?;
        GitRepository::init(article.get_path())?;
        article.get_article_file().generate_empty_file()?;
        article.get_meta_file().generate_empty_file()?;
        article.commit("initial commit")?;
        Ok(())
    }

    /// lists draft or post articles. dir must be "draft" or "post".
    pub fn list(dir: &str) -> Failable {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            println!("{}", path.display());
            let article = Article::new(&path)?;
            match article.get_title() {
                Ok(title) => println!("  {}", title),
                Err(error) => println!("!!Error: {}", error)
            }
        }
        Ok(())
    }

    pub fn new<P: AsRef<Path>>(article_path: P) -> Result<Self> {
        let path = article_path.as_ref();
        let is_draft = is_draft_path(&path)
            .ok_or("The specified path is not either draft or post.")?;
        let id = if is_draft {
                ArticleId::from_draft_dir(&path)
            } else {
                ArticleId::from_post_dir(&path)
            }.ok_or("Invalid article ID.")?;

        let article_file = ArticleFile::new(path.join("article.md"));
        let meta_file = MetaFile::new(path.join(META_FILE_NAME));

        Ok(Article {
            id: id,
            is_draft: is_draft,
            article_file: article_file,
            meta_file: meta_file
            })
    }

    pub fn get_id(&self) -> &str {
        &self.id.id
    }

    pub fn is_draft(&self) -> bool {
        self.is_draft
    }

    pub fn get_title(&self) -> Result<String> {
        self.get_article_file().get_title()
    }

    pub fn get_path(&self) -> PathBuf {
        Path::new(if self.is_draft { "draft" } else { "post" })
            .join(&self.get_id())
    }

    fn get_article_file(&self) -> &ArticleFile {
        &self.article_file
    }

    fn get_meta_file(&self) -> &MetaFile {
        &self.meta_file
    }

    /// commits to the local git repository.
    pub fn commit(&self, message: &str) -> Failable {
        let git_repo = GitRepository::open(self.get_path())?;
        let files = ["."];
        git_repo.add_files_to_index(files.iter())?;
        git_repo.commit(message)?;
        Ok(())
    }

    // TODO Rollback if any statements fail.
    /// posts a draft ariticle and returns the posted article.
    pub fn post(&self, github: &GitHub) -> Result<Self> {
        if !self.is_draft() {
            return Err(From::from("This is not a draft article."))
        }

        let filename = self.get_article_file().get_filename();
        let content = self.get_article_file().get_content()?;
        let (id, push_url) = github.create_gist(filename, &content)?;

        let post_dir = Path::new("post").join(id);
        GitRepository::clone(&push_url, &post_dir)?;

        copy_all_files(self.get_path(), &post_dir)?;
        let post_article = Article::new(&post_dir)?;
        post_article.commit("post")?;

        let git_repo = GitRepository::open(&post_dir)?;
        git_repo.push(github.get_access_token(), "")?;

        Ok(post_article)
    }

    pub fn update(&self, github: &GitHub) -> Failable {
        if self.is_draft() {
            return Err(From::from("This is not a posted article."))
        }
        let git_repo = GitRepository::open(self.get_path())?;
        git_repo.push(github.get_access_token(), "")?;
        Ok(())
    }

    /// removes the directory of this article
    pub fn remove(&self) -> Failable {
        fs::remove_dir_all(self.get_path()).map_err(From::from)
    }
}
