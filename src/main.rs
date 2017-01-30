extern crate rustc_serialize;

mod article_id;
mod article;
mod git;
mod config;
mod github;

use std::env;
use article::Article;
use config::Config;
use github::GitHub;

const NAME: &'static str = "nya";

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Failable = Result<()>;

fn print_usage() {
    println!("Usage:\n\
             create a config file        : {0} init\n\
             create a draft article      : {0} new\n\
             post a draft article        : {0} post draft/ARTICLE_DIRECTORY\n\
             update a posted article     : {0} update post/ARTICLE_DIRECTORY\n\
             save an article to local git: {0} save draft/ARTICLE_DIRECTORY\n\
             or                          : {0} save post/ARTICLE_DIRECTORY\n\
             list draft articles         : {0} drafts\n\
             list posted articles        : {0} posts\n",
             NAME)
}

fn update_article(article_path: &str) -> Failable {
    let config = Config::load()?;
    let github = GitHub::new(&config.github.access_token);
    let article = Article::new(article_path)?;
    article.update(&github)?;
    Ok(())
}

fn post_article(article_path: &str) -> Failable {
    let config = Config::load()?;
    let github = GitHub::new(&config.github.access_token);
    let article = Article::new(article_path)?;
    article.post(&github)?;
    article.remove()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    let command = &args[1];

    let result = match &command as &str {
        "init" => Config::generate(),
        "new" => Article::generate(),
        "post" => post_article(&args[2]),
        "update" => update_article(&args[2]),
        "save" => Article::new(&args[2])
            .and_then(|article| article.commit("save")),
        "drafts" => Article::list("draft"),
        "posts" => Article::list("post"),
        _ => Err(From::from("Unknown command."))
    };

    if let Err(error) = result {
        println!("{}", error);
    }
}
