// use tauri::command;
use rusqlite::{params, Connection, Result};
use crate::internal::dbo::article::Article;
use crate::internal::dbo::feed::{FeedItem, AtomEntry};

//#[derive(Debug)]

// fetch_and_emit
//  -> fetch_feed (vec<FeedItems>)
//      assume that I know it is Atom/Rss
//      I will fetch the indi items as X, then wrap them in an FeedItem

// fn db_fetch_feed() {
//     RSS_feed_items = fetch_rss_table(path) // vec<FeedItem>
//     Atom_feed_items = fetch_rss_table(path) // nothing
//     full_feeds = Rss_feed_items + Atom_feed_items // put em together
//     return full_feeds
// }

// fn fetch_rss_table() {
//     let resp = getDB();

//     for row in resp {
//         FeedItem:Rss(RssEntry {
//             row.title...
//         })
//     }
// }

// #[command]
// pub fn create_and_retrieve_fake_data() ->Result<Vec<Article>> { //Result<()> {
pub fn create_db() -> Result<Connection> {
    //TODO: make this persistant
    // let conn = Connection::open_in_memory()?;
    
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    // TODO: Only create DB & tables the first time its opened
    // TODO: Create other tables (feeds, categories, etc.)
    // conn.execute(
    //     "CREATE TABLE  IF NOT EXISTS articles (
    //         id  INTEGER PRIMARY KEY AUTOINCREMENT,
    //         title   TEXT NOT NULL,
    //         description TEXT,
    //         author TEXT,
    //         datetime TEXT
    //     )",
    //     [], // list of parameters
    // )?;
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS AtomEntry (
            item_id  INTEGER PRIMARY KEY AUTOINCREMENT,
            title   TEXT NOT NULL,
            link    TEXT NOT NULL,    
            summary TEXT NOT NULL,
            id, TEXT,
            updated TEXT,
            author TEXT,
            category TEXT,
            content TEXT,
            contributor TEXT,
            pub_date TEXT,
            rights TEXT
        )",
        [], // list of parameters
    )?;

    Ok(conn)
}

pub fn create_fake_data() -> Result<()> {
    // let conn = Connection::open_in_memory()?;

    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());
    
    // TODO: Actually read these articles from the feed, for now, hard code to test
    // let orig_articles: Vec<Article> = api::get_articles(); // correct way to use internal classes?

    // let a: Vec<FeedItem> = vec![

    // ]

    let fake_atom_entry: Vec<AtomEntry> = vec![
        AtomEntry {
            title: "First Article".to_string(),
            link: Some("url.com".to_string()),
            summary: Some("This is the description of the first article.".to_string()),
            // Optional fields
            id: Some("1".to_string()),
            updated: Some("Yesterday".to_string()),
            author: Some("John Doe".to_string()),
            category: Some("News".to_string()),
            content: Some("John Doe has lots to say about the news.".to_string()),
            contributor: Some("Jane Doe".to_string()),
            pub_date: Some("2024-05-30T12:00:00".to_string()),
            rights: None,
        }
    ];

    // let orig_articles: Vec<Article> = vec![
    //     Article::new(
    //         "First Article",
    //         "This is the description of the first article.",
    //         "John Doe",
    //         "2024-05-30T12:00:00",
    //     ),
    //     Article::new(
    //         "Second Article",
    //         "This is the description of the second article.",
    //         "Jane Smith",
    //         "2024-05-31T09:30:00",
    //     ),
    // ];

    // let mut i: i32 = 1;
    
    // for a in orig_articles { // PK auto incremented in SQLite/rusqulite
    //     conn.execute(
    //         "INSERT OR IGNORE INTO articles
    //             (id, title, description, author, datetime)
    //             VALUES
    //             (?1, ?2, ?3, ?4, ?5)
    //             ",
    //             params![i, a.title, a.description, a.author, a.datetime]
    //     )?;
    //     i += 1;
    // }
    for e in fake_atom_entry { // PK auto incremented in SQLite/rusqulite
        conn.execute(
            "INSERT OR IGNORE INTO AtomEntry
                (item_id, title, link, summary, id, updated, author, category, content, contributor, pub_date, rights)
                VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                ",
                params![1, e.title, e.link, e.summary, e.id, e.updated, e.author,
                        e.category, e.content, e.contributor, e.pub_date, e.rights]
        )?;
        // i += 1;
    }
    Ok(())
}

// Not exposed to public use, used internally then wrapped
fn fetch_atom_entry() -> Result<Vec<FeedItem>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT title, link, summary, id, updated, author, category, content, contributor, pub_date, rights
        FROM AtomEntry
        "
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let atom_iter = stmt.query_map([], |row| {
        Ok(AtomEntry {
            // id: row.get(0)?, // need way of having ID be an Article field, but auto incr when inserted
            title: row.get(0)?,
            link: row.get(1)?,
            summary: row.get(2)?,
            id: row.get(3)?,
            updated: row.get(4)?,
            author: row.get(5)?,
            category: row.get(6)?,
            content: row.get(7)?,
            contributor: row.get(8)?,
            pub_date: row.get(9)?,
            rights: row.get(10)?,
        })
    })?;

    //let mut db_atom_entry: Vec<AtomEntry> = vec![];
    let mut db_atom_entry: Vec<FeedItem> = vec![];

    for entry in atom_iter { // this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting AtomEntry result to vector");
                // println!("AtomEntry: {:?}", entry.unwrap());
                continue;
            }
        };
        db_atom_entry.push(FeedItem::Atom(e));
    }

    // FeedItem::Atom(e)

    //return db_articles;
    Ok(db_atom_entry)

}

pub fn db_fetch_feed() -> Vec<FeedItem> {
    let mut atom_feed: Vec<FeedItem> = fetch_atom_entry().expect("Panic query fake AtomEntry");
    let mut rss_feed: Vec<FeedItem> = Vec::new(); // hard code as empty for testing

    let mut full_feeds: Vec<FeedItem> = Vec::new();
    full_feeds.append(&mut atom_feed); // put em together
    full_feeds.append(&mut rss_feed); // put em together
    
    return full_feeds
}

// #[command] // If I need to call this command from the front end -- for tauri wrappers to use
pub fn retrieve_articles() -> Result<Vec<Article>> { // Vec<Article> {
    // let conn = Connection::open_in_memory()?;
    
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

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