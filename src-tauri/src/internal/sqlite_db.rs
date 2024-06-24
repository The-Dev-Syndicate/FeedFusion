// use tauri::command;
use std::time::Duration;
use rusqlite::{params, Connection, Result};
// use crate::internal::dbo::article::Article;
use crate::internal::dbo::feed::{Feed, FeedType, FeedItem, AtomEntry};
use crate::internal::feed_config::Feed as FeedFE; // for displaying on FE
// use std::collections::HashMap;

// use tauri::{AppHandle, Runtime, Manager};

pub fn create_db() {
    create_atom_entry_table().expect("Error creating AtomEntry table");
    create_rss_feed_table().expect("Error creating RSSFeed table");
    // create_articles_table().expect("Error creating articles table");
}

pub fn create_rss_feed_table() -> Result<Connection> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "CREATE TABLE IF NOT EXISTS RSSFeeds (
        feed_id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        poll_interval INTEGER NOT NULL,
        category TEXT,
        alias TEXT
        )",
        [],
    )?;

    Ok(conn)
}

pub fn create_atom_entry_table() -> Result<Connection> {
    //TODO: make this persistant
    // let conn = Connection::open_in_memory()?;
    
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    // TODO: Only create DB & tables the first time its opened
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

// pub fn create_articles_table() -> Result<Connection> {
//     let path = "./local_db.db3";
//     let conn = Connection::open(path)?;
//     print!("{:?}\n", conn.is_autocommit());

//     // TODO: Create other tables (feeds, categories, etc.)
//     conn.execute(
//         "CREATE TABLE  IF NOT EXISTS articles (
//             id  INTEGER PRIMARY KEY AUTOINCREMENT,
//             title   TEXT NOT NULL,
//             description TEXT,
//             author TEXT,
//             datetime TEXT
//         )",
//         [], // list of parameters
//     )?;

//     Ok(conn)
// }

pub fn create_fake_data() {
    create_fake_feed_items().expect("Feed Item creation error.");
    create_fake_feeds().expect("Feed creation error.");
    // create_fake_articles().expect("Article creation error.");
}

pub fn create_fake_feeds() -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());
    
    // example to use: https://mastodon.social/@lunar_vagabond.rss
    // misalignement between Feed fields here and those the frontend is expecting...
    let starting_feed = Feed {
        url: "https://mastodon.social/@lunar_vagabond.rss".to_string(),
        feed_type: FeedType::RSS,
        poll_interval: Duration::from_secs(1 * 60 * 60), // every hour,
    };
    
    let feed_vec = vec![starting_feed]; // not necesary, but my brain wanted to do it

    // TODO, add ToSql trait to interpret feed fields as what sql expects, hard coding for now
    for f in feed_vec { // PK auto incremented in SQLite/rusqulite
        conn.execute( // leave "alias" null
            "INSERT OR IGNORE INTO RSSFeeds
                (feed_id, url, poll_interval, category)
            VALUES
                (?1, ?2, ?3, ?4)
                ",
                // params![1, f.url, f.feed_type, f.poll_interval, "test_category", None] // The trait bound 'FeedType: ToSql' is not satisfied 
                params![1, f.url.to_string(), 60*60, "test_category"]
        )?;
    }
    
    Ok(())
}

pub fn create_fake_feed_items() -> Result<()> {
    // let conn = Connection::open_in_memory()?;

    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());
    
    // TODO: Actually read these articles from the feed, for now, hard code to test
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

// pub fn create_fake_articles() -> Result<()> {
//     // let conn = Connection::open_in_memory()?;

//     let path = "./local_db.db3";
//     let conn = Connection::open(path)?;
//     print!("{:?}\n", conn.is_autocommit());
    
//     // TODO: Actually read these articles from the feed, for now, hard code to test
//     let orig_articles: Vec<Article> = vec![
//         Article::new(
//             "First Article",
//             "This is the description of the first article.",
//             "John Doe",
//             "2024-05-30T12:00:00",
//         ),
//         Article::new(
//             "Second Article",
//             "This is the description of the second article.",
//             "Jane Smith",
//             "2024-05-31T09:30:00",
//         ),
//     ];

//     let mut i: i32 = 1;
    
//     for a in orig_articles { // PK auto incremented in SQLite/rusqulite
//         conn.execute(
//             "INSERT OR IGNORE INTO articles
//                 (id, title, description, author, datetime)
//                 VALUES
//                 (?1, ?2, ?3, ?4, ?5)
//                 ",
//                 params![i, a.title, a.description, a.author, a.datetime]
//         )?;
//         i += 1;
//     }
    
//     Ok(())
// }

// pub fn insert_to_db(stmt: &str, params: Vec<()>) -> Result<Connection> {
//     let path = "./local_db.db3";
//     let conn = Connection::open(path)?;
//     print!("{:?}\n", conn.is_autocommit());

//     //my_string.parse::<i32>().unwrap(); // convert String to i32
//     // ISSUES:
//     // 1) Can't pass generic tuple of parameters for generalized INSERT wrapper (have to specify eact makeup of tuple)
//     // 2) Can't iterate over Vec/Hashmap and then push/add to a tuple or params!
//     // 3) Annoying solution might be to have giant tuple of all possible VALUE fields for all tables in DB -- no that prob won't work
    
//     Ok(conn)
// }

pub fn add_feed_db(feed_url: String, feed_alias: String, poll_timer: i32) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "INSERT OR IGNORE INTO RSSFeeds
            (url, poll_interval, category, alias)
        VALUES
            (?1, ?2, ?3, ?4)
        ",
        params![feed_url, poll_timer, "test_category", feed_alias],
    )?;

    println!("Feed uploaded, but not the correct way to test this, use match, or something");

    Ok(())
}

pub fn db_fetch_feed_for_front_end() -> Result<Vec<FeedFE>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT url, alias, category, poll_interval
        FROM RSSFeeds
        "
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let feed_iter = stmt.query_map([], |row| {
        // let interval: u8 = row.get(3)?;
        Ok(FeedFE {
            url: row.get(0)?,
            alias: row.get(1)?,
            category: row.get(2)?,
            poll_timer: row.get(3)?,
        })
    })?;

    //let mut db_atom_entry: Vec<AtomEntry> = vec![];
    let mut db_rss_feed: Vec<FeedFE> = vec![];

    for entry in feed_iter { // this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting RSSFeed query to vector");
                println!("FeedQuery: {:?}", entry.unwrap());
                continue;
            }
        };
        db_rss_feed.push(e);
    }

    Ok(db_rss_feed)
}

pub fn db_fetch_feeds_for_pull() -> Result<Vec<Feed>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT url, poll_interval
        FROM RSSFeeds
        "
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed {
            url: row.get(0)?,
            feed_type: FeedType::RSS,
            poll_interval: Duration::from_secs(row.get(1)?),
        })
    })?;

    //let mut db_atom_entry: Vec<AtomEntry> = vec![];
    let mut db_rss_feed: Vec<Feed> = vec![];

    for entry in feed_iter { // this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting RSSFeed query to vector");
                // println!("AtomEntry: {:?}", entry.unwrap());
                continue;
            }
        };
        db_rss_feed.push(e);
    }

    Ok(db_rss_feed)

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

    Ok(db_atom_entry)
}

pub fn db_fetch_feed_items() -> Vec<FeedItem> {
    let mut atom_feed: Vec<FeedItem> = fetch_atom_entry().expect("Panic query fake AtomEntry");
    let mut rss_feed: Vec<FeedItem> = Vec::new(); // hard code as empty for testing

    let mut full_feeds: Vec<FeedItem> = Vec::new();
    full_feeds.append(&mut atom_feed); // put em together
    full_feeds.append(&mut rss_feed); // put em together
    
    return full_feeds
}

// pub fn fetch_and_emit_feed_db<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
//     let items = db_fetch_feed_items();
//     app.emit_all("new-rss-items", &items)?;
//     Ok(())
// }

// #[command] // If I need to call this command from the front end -- for tauri wrappers to use
// pub fn retrieve_articles() -> Result<Vec<Article>> { // Vec<Article> {
//     // let conn = Connection::open_in_memory()?;
    
//     let path = "./local_db.db3";
//     let conn = Connection::open(path)?;
//     print!("{:?}\n", conn.is_autocommit());

//     // TODO: read in select statement, hard code for now
//     let mut stmt = conn.prepare(
//         // SELECT id, title, description, author, datetime
//         "
//         SELECT title, description, author, datetime
//         FROM articles
//         "
//     )?;

//     // TODO: Control logic that this is a valid SELECT statement
//     let article_iter = stmt.query_map([], |row| {
//         Ok(Article {
//             // id: row.get(0)?, // need way of having ID be an Article field, but auto incr when inserted
//             title: row.get(0)?,
//             description: row.get(1)?,
//             author: row.get(2)?,
//             datetime: row.get(3)?,
//         })
//     })?;

//     let mut db_articles: Vec<Article> = vec![];

//     for article in article_iter { // this is innefficient, can I do something directily from article_iter?
//         // This is erroring
//         let a = match article {
//             Ok(article) => article,
//             Err(_) => {
//                 println!("Error converting article result to vector");
//                 println!("Article: {:?}", article.unwrap());
//                 continue;
//             }
//         };
//         db_articles.push(a);
//     }

//     //return db_articles;
//     Ok(db_articles)
// }