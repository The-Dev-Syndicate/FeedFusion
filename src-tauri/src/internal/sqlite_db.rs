use std::collections::hash_set;

use crate::internal::dbo::feed::{AtomEntry, Feed, FeedItem, FeedType, RssEntry};
use rusqlite::{params, Connection, Result};

//-----------//
// Functions //
//-----------//

//----------//
// DB Setup //
//----------//

pub fn create_db() {
    create_rss_feed_table().expect("Error creating RSSFeed table");
    create_atom_feed_table().expect("Error creating AtomFeed table");
    create_rss_entry_table().expect("Error creating RSSEntry table");
    create_atom_entry_table().expect("Error creating AtomEntry table");
}

//------------------------------------------------------------------------------------------

pub fn create_rss_feed_table() -> Result<Connection> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "CREATE TABLE IF NOT EXISTS RSSFeeds (
        feed_id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        poll_interval INTEGER NOT NULL,
        alias TEXT
        )",
        [],
    )?;

    Ok(conn)
}

//------------------------------------------------------------------------------------------

pub fn create_atom_feed_table() -> Result<Connection> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "CREATE TABLE IF NOT EXISTS AtomFeeds (
        feed_id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        poll_interval INTEGER NOT NULL,
        alias TEXT
        )",
        [],
    )?;

    Ok(conn)
}

//------------------------------------------------------------------------------------------

pub fn create_rss_entry_table() -> Result<Connection> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;

    //print!("{:?}\n", conn.is_autocommit());

    // TODO: Only create DB & tables the first time its opened
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS RSSEntry (
            item_id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            link        TEXT NOT NULL,    
            description TEXT NOT NULL,
            pub_date    TEXT,
            author      TEXT,
            category    TEXT,
            comments    TEXT,
            enclosure   TEXT,
            guid        TEXT,
            hash        INTEGER NOT NULL
        )",
        [], // list of parameters
    )?;

    Ok(conn)
}

//------------------------------------------------------------------------------------------

pub fn create_atom_entry_table() -> Result<Connection> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: Only create DB & tables the first time its opened
    conn.execute(
        "CREATE TABLE  IF NOT EXISTS AtomEntry (
            item_id  INTEGER PRIMARY KEY AUTOINCREMENT,
            title   TEXT NOT NULL,
            link    TEXT NOT NULL,    
            summary TEXT NOT NULL,
            id TEXT,
            updated TEXT,
            author TEXT,
            category TEXT,
            content TEXT,
            contributor TEXT,
            pub_date TEXT,
            rights TEXT,
            hash   INTEGER NOT NULL
        )",
        [], // list of parameters
    )?;

    Ok(conn)
}

//------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------

//----------------//
// Fake Test Data //
//----------------//

pub fn create_fake_data() {
    create_fake_feeds().expect("Feed creation error.");
    create_fake_feed_items().expect("Feed Item creation error.");
}

//------------------------------------------------------------------------------------------

pub fn create_fake_feeds() -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: create ToSql trait for FeedType, Duration
    let rss_feed = Feed {
        url: "https://mastodon.social/@lunar_vagabond.rss".to_string(),
        feed_type: FeedType::RSS,
        poll_interval: 10, // every 10 sec,
        alias: Some("Lunar Vagabound".to_string()),
    };

    let rss_feed_vec = vec![rss_feed];

    // TODO, add ToSql trait to interpret feed fields as what sql expects, hard coding for now
    for f in rss_feed_vec {
        // PK auto incremented in SQLite/rusqulite
        conn.execute(
            "INSERT OR IGNORE INTO RSSFeeds
                (feed_id, url, poll_interval, alias)
            VALUES
                (?1, ?2, ?3, ?4)
                ",
            params![1, f.url, f.poll_interval, f.alias],
        )?;
    }

    // // TODO: Add atom Feed
    // let atom_feed = Feed {
    //     url: "https://run.mocky.io/v3/d3d616ed-4780-41f9-915f-bce277ae0afe".to_string(), // this url may need to be regenerated every so often
    //     feed_type: FeedType::ATOM,
    //     poll_interval: Duration::from_secs(1* 1* 10), // every 10 sec
    // };

    // let atom_feed_vec = vec![atom_feed]; // not necesary, but my brain wanted to do it

    // // TODO add ToSql trait to interpret feed fields as what sql expects, hard coding for now
    // for f in atom_feed_vec {
    //     conn.execute(
    //         "INSERT OR IGNORE INTO AtomFeeds
    //             (feed_id, url, poll_interval, alias)
    //         VALUES
    //             (?1, ?2, ?3, ?4)
    //             ",
    //             params![1, f.url, 10, "Mocky"]
    //     )?;
    // }

    Ok(())
}

//------------------------------------------------------------------------------------------

pub fn create_fake_feed_items() -> Result<()> {
    // let conn = Connection::open_in_memory()?;

    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: Actually read these articles from the feed, for now, hard code to test
    let fake_atom_entry: Vec<AtomEntry> = vec![AtomEntry::new(
        "Fake First Article".to_string(),
        Some("url.com".to_string()),
        Some("This is the description of the first article.".to_string()),
        Some("1".to_string()),
        Some("Yesterday".to_string()),
        Some("John Doe".to_string()),
        Some("News".to_string()),
        Some("John Doe has lots to say about the news.".to_string()),
        Some("Jane Doe".to_string()),
        Some("2024-05-30T12:00:00".to_string()),
        None, // rights
    )];
    for e in fake_atom_entry {
        // PK auto incremented in SQLite/rusqulite
        conn.execute(
            "INSERT OR IGNORE INTO AtomEntry
                (item_id, title, link, summary, id, updated, author, category, content, contributor, pub_date, rights, hash)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                ",
                params![1, e.title, e.link, e.summary, e.id, e.updated, e.author,
                        e.category, e.content, e.contributor, e.pub_date, e.rights, e.hash]
        )?;
        // i += 1;
    }

    Ok(())
}

//------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------

//-------------//
// App Actions //
//-------------//

// TO DO make feed_category, and feed_alias Option<> to accept None
pub fn put_rss_feed_db(feed_url: String, poll_timer: i32, feed_alias: String) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "INSERT OR IGNORE INTO RSSFeeds
            (url, poll_interval, alias)
        VALUES
            (?1, ?2, ?3)
        ",
        params![feed_url, poll_timer, feed_alias], // send "test_category" for feed_category
    )?;

    println!("RSS Feed uploaded, but not the correct way to test this, use match, or something");

    Ok(())
}

//------------------------------------------------------------------------------------------

pub fn put_atom_feed_db(feed_url: String, poll_timer: i32, feed_alias: String) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "INSERT OR IGNORE INTO AtomFeeds
            (url, poll_interval, alias)
        VALUES
            (?1, ?2, ?3)
        ",
        params![feed_url, poll_timer, feed_alias],
    )?;

    println!("Atom Feed uploaded, but not the correct way to test this, use match, or something");

    Ok(())
}

//------------------------------------------------------------------------------------------

// let _e =

pub fn put_feed_items_db(items: Vec<FeedItem>) -> Result<()> {
    for item in items {
        let _e = match item {
            FeedItem::Rss(e) => {
                put_rss_entry_db(e).expect("Thing1");
            }
            FeedItem::Atom(e) => {
                put_atom_entry_db(e).expect("Thing2");
            }
        };
    }

    Ok(())
}

//------------------------------------------------------------------------------------------

pub fn put_atom_entry_db(e: AtomEntry) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "INSERT OR IGNORE INTO AtomEntry
            (title, link, summary, id, updated, author, category, content, contributor, pub_date, rights, hash)
        VALUES
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ",
            params![e.title, e.link, e.summary, e.id, e.updated, e.author,
                    e.category, e.content, e.contributor, e.pub_date, e.rights, e.hash]
    )?;

    Ok(())
}
//------------------------------------------------------------------------------------------

pub fn put_rss_entry_db(e: RssEntry) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    conn.execute(
        "INSERT OR IGNORE INTO RSSEntry
            (title, link, description, pub_date, author, category, comments, enclosure, guid, hash)
        VALUES
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ",
        params![
            e.title,
            e.link,
            e.description,
            e.pub_date,
            e.author,
            e.category,
            e.comments,
            e.enclosure,
            e.guid,
            e.hash
        ],
    )?;

    Ok(())
}
//------------------------------------------------------------------------------------------

pub fn get_feeds_db() -> Result<Vec<Feed>> {
    let mut rss_feeds: Vec<Feed> = get_rss_feeds_db().expect("Error getting FE RSS feeds from DB");
    let mut atom_feeds: Vec<Feed> =
        get_atom_feeds_db().expect("Error getting FE Atom feeds from DB");

    let mut full_feeds: Vec<Feed> = Vec::new();
    full_feeds.append(&mut rss_feeds); // put em together
    full_feeds.append(&mut atom_feeds); // put em together

    Ok(full_feeds)
}

//------------------------------------------------------------------------------------------

pub fn get_rss_feeds_db() -> Result<Vec<Feed>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT url, alias, poll_interval
        FROM RSSFeeds
        ",
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed {
            url: row.get(0)?,
            alias: row.get(1)?,
            poll_interval: row.get(2)?,
            feed_type: FeedType::RSS,
        })
    })?;

    let mut db_rss_feed: Vec<Feed> = vec![];

    for entry in feed_iter {
        // FIXME: this is innefficient, can I do something directily from article_iter?
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

//------------------------------------------------------------------------------------------

pub fn get_atom_feeds_db() -> Result<Vec<Feed>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT url, alias, poll_interval
        FROM AtomFeeds
        ",
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed {
            url: row.get(0)?,
            alias: row.get(1)?,
            poll_interval: row.get(2)?,
            feed_type: FeedType::ATOM,
        })
    })?;

    let mut db_atom_feed: Vec<Feed> = vec![];

    for entry in feed_iter {
        // FIXME this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting AtomFeed query to vector");
                println!("FeedQuery: {:?}", entry.unwrap());
                continue;
            }
        };
        db_atom_feed.push(e);
    }

    Ok(db_atom_feed)
}

//------------------------------------------------------------------------------------------

fn get_atom_entry_db() -> Result<Vec<FeedItem>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT title, link, summary, id, updated, author, category, content, contributor, pub_date, rights, hash
        FROM AtomEntry
        "
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let atom_iter = stmt.query_map([], |row| {
        let hash_str: String = row.get(9)?;
        let hash: i64 = hash_str.parse().map_or(0, |x| x);
        Ok(AtomEntry {
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
            hash,
        })
    })?;

    let mut db_atom_entry: Vec<FeedItem> = vec![];

    for entry in atom_iter {
        // FIXME this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting AtomEntry result to vector");
                continue;
            }
        };
        db_atom_entry.push(FeedItem::Atom(e));
    }

    Ok(db_atom_entry)
}

//------------------------------------------------------------------------------------------

fn get_rss_entry_db() -> Result<Vec<FeedItem>> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: read in select statement, hard code for now
    let mut stmt = conn.prepare(
        "
        SELECT title, link, description, pub_date, author, category, comments, enclosure, guid, hash
        FROM RSSEntry
        ",
    )?;

    // TODO: Control logic that this is a valid SELECT statement
    let rss_iter = stmt.query_map([], |row| {
        let hash_str: String = row.get(9)?;
        let hash: i64 = hash_str.parse().map_or(0, |x| x);
        Ok(RssEntry {
            title: row.get(0)?,
            link: row.get(1)?,
            description: row.get(2)?,
            pub_date: row.get(3)?,
            author: row.get(4)?,
            category: row.get(5)?,
            comments: row.get(6)?,
            enclosure: row.get(7)?,
            guid: row.get(8)?,
            hash: hash,
        })
    })?;

    let mut db_rss_entry: Vec<FeedItem> = vec![];

    for entry in rss_iter {
        // this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting AtomEntry result to vector");
                continue;
            }
        };
        db_rss_entry.push(FeedItem::Rss(e));
    }

    Ok(db_rss_entry)
}

//------------------------------------------------------------------------------------------

pub fn get_feed_items_db() -> Vec<FeedItem> {
    let mut atom_feed: Vec<FeedItem> = get_atom_entry_db().expect("Panic query fake AtomEntry");
    let mut rss_feed: Vec<FeedItem> = get_rss_entry_db().expect("Panic query fake RSSEntry");

    let mut full_feeds: Vec<FeedItem> = Vec::new();
    full_feeds.append(&mut atom_feed);
    full_feeds.append(&mut rss_feed);

    println!("{:?}", full_feeds.len());

    return full_feeds;
}
