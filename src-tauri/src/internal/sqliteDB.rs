use rusqlite::{params, Connection, Result};
use crate::internal::article::Article;
use crate::internal::api::get_articles; // is this correct?

#[derive(Debug)]

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // TODO: Only create DB & tables the first time its opened

    conn.execute(
        "CREATE TABLE articles (
            id  INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            description TEXT,
            author TEXT,
            datetime TEXT
        )",
        (), // list of parameters
    )?;

    // TODO: Actually read these articles from the feed
    // For now, hard code to test

    let orig_articles: Vec<Article> = api::get_articles(); // correct way to use internal classes?

    // TODO: id must be controlled from DB here not from loop
    for (i, article) in orig_articles.enumerate() { // is this correct for getting index as looping?
        conn.execute(
            "INSERT INTO articles
                (id, title, description, author, datetime)
                VALUES
                ({i+1}, {article.title}, {article.description}, {article.author}, {article.datetime})"
        )
    }

    fn retrieve_articles(select_statement: String) -> Vec<Article> {
        let mut db_articles: Vec<Article>;

        // TODO: Control logic that this is a valid SELECT statement
        let article_iter = select_statement.query_map([], |row| {
            OK(Article {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                author: row.get(3),
                datetime: row.get(4),
            })
        })?;

        for article in article_iter { // this is innefficient, can I do something directily from article_iter?
            db_articles.push(article);
        }

        return db_articles;
    }
}