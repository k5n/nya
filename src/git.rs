extern crate git2;

use self::git2::{ Repository, IntoCString, PushOptions, RemoteCallbacks, Cred };
use std::path::Path;
use super::Result;
use super::Failable;

pub struct GitRepository {
    repository: Repository,
}

impl GitRepository {
    pub fn clone<P: AsRef<Path>>(url: &str, into: P) -> Failable {
        Repository::clone(url, into)?;
        Ok(())
    }

    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let repository = Repository::init(&path)?;
        Ok(Self::new(repository))
    }

    fn new(repository: Repository) -> GitRepository {
        GitRepository { repository: repository }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repository = Repository::open(path)?;
        Ok(Self::new(repository))
    }

    pub fn commit(&self, message: &str) -> Failable {
        let signature = self.repository.signature()?;
        let tree_oid = self.repository.index()?.write_tree()?;
        let tree = self.repository.find_tree(tree_oid)?;

        let parents =
            if let Ok(head_reference) = self.repository.head() {
                let head_oid = head_reference.target()
                    .ok_or("Failed to get OID of HEAD.")?;
                let parent_commit = self.repository.find_commit(head_oid)?;
                vec![ parent_commit ]
            } else {
                vec![]
            };
        let parents: Vec<&git2::Commit> = parents.iter().collect();

        self.repository.commit(Some("HEAD"), &signature, &signature, message,
                               &tree, &parents)?;
        Ok(())
    }

    pub fn add_files_to_index<T, I>(&self, files: I) -> Failable
            where T: IntoCString, I: IntoIterator<Item=T> {
        let mut index = self.repository.index()?;
        index.add_all(files, git2::ADD_DEFAULT, None)?;
        index.write()?;
        Ok(())
    }

    pub fn push(&self, username: &str, password: &str) -> Failable {
        let mut remote = self.repository.find_remote("origin")?;
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_, _, _|
            Cred::userpass_plaintext(username, password).map_err(From::from)
        );
        callbacks.update_tips(|_, _, _| true);
        callbacks.certificate_check(|_, _| true);
        let mut options = PushOptions::new();
        options.remote_callbacks(callbacks);
        remote.push(&[ "refs/heads/master" ], Some(&mut options))?;
        Ok(())
    }
}

