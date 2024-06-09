use tauri::command;
use rusqlite::{params, Connection, Result};
use crate::internal::article::Article;
use crate::internal::api; //::get_articles; // is this correct?

//#[derive(Debug)]

// #[command]
pub fn create_and_retrieve_fake_data() ->Result<Vec<Article>> { //Result<()> {
    //TODO: make this persistant
    let conn = Connection::open_in_memory()?;

    // TODO: Only create DB & tables the first time its opened
    conn.execute(
        // id  INTEGER PRIMARY KEY,
        "CREATE TABLE articles (
            id  INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            description TEXT,
            author TEXT,
            datetime TEXT
        )",
        [], // list of parameters
    )?;

    // TODO: Actually read these articles from the feed
    // For now, hard code to test
    let orig_articles: Vec<Article> = api::get_articles(); // correct way to use internal classes?

    for a in orig_articles { // PK auto incremented in SQLite/rusqulite
        conn.execute(
            "INSERT INTO articles
                (title, description, author, datetime)
                VALUES
                (?1, ?2, ?3, ?4)
                ",
                params![a.title, a.description, a.author, a.datetime]
        )?;
    }
// }

// #[command] // If I need to call this command from the front end -- for tauri wrappers to use
// pub fn retrieve_articles() -> Result<Vec<Article>> { // Vec<Article> {
    // let conn = Connection::open_in_memory()?;

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        // SELECT id, title, description, author, datetime
        "
        SELECT title, description, author, datetime
        FROM articles
        "
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let article_iter = stmt.query_map([], |row| {
        Ok(Article {
            // id: row.get(0)?, // need way of having ID be an Article field, but auto incr when inserted
            title: row.get(0)?,
            description: row.get(1)?,
            author: row.get(2)?,
            datetime: row.get(3)?,
        })
    })?;

    let mut db_articles: Vec<Article> = vec![];

    for article in article_iter { // this is innefficient, can I do something directily from article_iter?
        // This is erroring
        let a = match article {
            Ok(article) => article,
            Err(_) => {
                println!("Error converting article result to vector");
                println!("Article: {:?}", article.unwrap());
                continue;
            }
        };
        db_articles.push(a);
    }

    //return db_articles;
    Ok(db_articles)
}