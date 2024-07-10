use crate::internal::dbo::entry::{AtomEntry, FeedEntryType, RssEntry};
use crate::internal::dbo::feed::{Feed, FeedType};
use rusqlite::{params, Connection, OptionalExtension, Result};

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
    //create_fake_feed_items().expect("Feed Item creation error.");
}

//------------------------------------------------------------------------------------------

pub fn create_fake_feeds() -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;
    //print!("{:?}\n", conn.is_autocommit());

    // TODO: create ToSql trait for FeedType, Duration
    let rss_feed = Feed::new(
        "https://mastodon.social/@lunar_vagabond.rss".to_string(),
        "Lunar Vagabound".to_string(),
        10,
        FeedType::RSS,
    );

    let atom_feed = Feed::new(
        "https://dcorps.dev/feed.xml".to_string(),
        "DCrops".to_string(),
        30,
        FeedType::ATOM,
    );

    let rss_feed_vec = vec![rss_feed];

    // TODO, add ToSql trait to interpret feed fields as what sql expects, hard coding for now
    for (idx, f) in rss_feed_vec.iter().enumerate() {
        // PK auto incremented in SQLite/rusqulite
        conn.execute(
            "INSERT OR IGNORE INTO RSSFeeds
                (feed_id, url, poll_interval, alias)
            VALUES
                (?1, ?2, ?3, ?4)
                ",
            params![idx, f.url, f.poll_interval, f.alias],
        )?;
    }

    // // TODO: Add atom Feed

    let atom_feed_vec = vec![atom_feed]; // not necesary, but my brain wanted to do it

    // TODO add ToSql trait to interpret feed fields as what sql expects, hard coding for now
    for (_, f) in atom_feed_vec.iter().enumerate() {
        conn.execute(
            "INSERT OR IGNORE INTO AtomFeeds
                 (feed_id, url, poll_interval, alias)
             VALUES
                 (?1, ?2, ?3, ?4)
                 ",
            params![1, f.url, f.poll_interval, f.alias],
        )?;
    }

    Ok(())
}

//------------------------------------------------------------------------------------------
//
//pub fn create_fake_feed_items() -> Result<()> {
//    // let conn = Connection::open_in_memory()?;
//
//    let path = "./local_db.db3";
//    let conn = Connection::open(path)?;
//    //print!("{:?}\n", conn.is_autocommit());
//
//    // TODO: Actually read these articles from the feed, for now, hard code to test
//    let fake_atom_entry: Vec<AtomEntry> = vec![AtomEntry::new(
//        "Fake First Article".to_string(),
//        Some("url.com".to_string()),
//        Some("<h1>This is the description of the first article.</h1>".to_string()),
//        Some("1".to_string()),
//        Some("Yesterday".to_string()),
//        Some("John Doe".to_string()),
//        Some("News".to_string()),
//        Some("John Doe has lots to say about the news.".to_string()),
//        Some("Jane Doe".to_string()),
//        Some("2024-05-30T12:00:00".to_string()),
//        None, // rights
//    )];
//
//    for e in fake_atom_entry {
//        // PK auto incremented in SQLite/rusqulite
//        conn.execute(
//            "INSERT OR IGNORE INTO AtomEntry
//                (item_id, title, link, summary, id, updated, author, category, content, contributor, pub_date, rights, hash)
//            VALUES
//                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
//                ",
//                params![1, e.title, e.link, e.summary, e.id, e.updated, e.author,
//                        e.category, e.content, e.contributor, e.pub_date, e.rights, e.hash]
//        )?;
//    }
//
//    Ok(())
//}

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

// FIXME: This code is dead code but leaving it here so a developer can use it to consolidate the
// other put_rss_feed function into a single func to handle both
//pub fn put_atom_feed_db(feed_url: String, poll_timer: i32, feed_alias: String) -> Result<()> {
//    let path = "./local_db.db3";
//    let conn = Connection::open(path)?;
//    //print!("{:?}\n", conn.is_autocommit());
//
//    conn.execute(
//        "INSERT OR IGNORE INTO AtomFeeds
//            (url, poll_interval, alias)
//        VALUES
//            (?1, ?2, ?3)
//        ",
//        params![feed_url, poll_timer, feed_alias],
//    )?;
//
//    println!("Atom Feed uploaded, but not the correct way to test this, use match, or something");
//
//    Ok(())
//}

//------------------------------------------------------------------------------------------

pub fn put_feed_items_db(items: Vec<FeedEntryType>) -> Result<()> {
    for item in items {
        let _e = match item {
            FeedEntryType::RSS(e) => {
                put_rss_entry_db(e).expect("Thing1");
            }
            FeedEntryType::ATOM(e) => {
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

    let mut stmt = conn.prepare(
        "
        SELECT hash
        FROM AtomEntry
        WHERE link = ?1
        AND title = ?2
        AND pub_date = ?3
    ",
    )?;

    let existing_hash: Option<i64> = stmt
        .query_row(params![e.link, e.title, e.pub_date], |row| row.get(0))
        .optional()?;

    match existing_hash {
        Some(_) => {
            if existing_hash.unwrap() != e.hash {
                // Update the existing entry if the hash does not match
                let _ = conn.execute(
                    "UPDATE AtomEntry
                    SET title       = ?1,
                        link        = ?2,
                        summary     = ?3,
                        id          = ?4,
                        updated     = ?5,
                        author      = ?6,
                        category    = ?7,
                        content     = ?8,
                        contributor = ?9,
                        pub_date    = ?10
                        rights      = ?11
                        hash        = ?12
                    WHERE hash      = ?13",
                    params![
                        e.title,
                        e.link,
                        e.summary,
                        e.id,
                        e.updated,
                        e.author,
                        e.category,
                        e.content,
                        e.contributor,
                        e.pub_date,
                        e.rights,
                        e.hash,
                        existing_hash
                    ],
                );
            }
        }
        None => {
            // Insert a new entry if no match is found
            conn.execute(
                "INSERT OR IGNORE INTO AtomEntry
                    (title, link, summary, id, updated, author, category, content, contributor, pub_date, rights, hash)
                VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                    ",
                    params![e.title, e.link, e.summary, e.id, e.updated, e.author,
                            e.category, e.content, e.contributor, e.pub_date, e.rights, e.hash]
            )?;
        }
    }

    Ok(())
}
//------------------------------------------------------------------------------------------

pub fn put_rss_entry_db(e: RssEntry) -> Result<()> {
    let path = "./local_db.db3";
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare(
        "
        SELECT hash
        FROM RSSEntry
        WHERE link = ?1
          AND pub_date = ?2 
    ",
    )?;

    let existing_hash: Option<i64> = stmt
        .query_row(params![e.link, e.pub_date], |row| row.get(0))
        .optional()?;

    match existing_hash {
        Some(_) => {
            if existing_hash.unwrap() != e.hash {
                // Update the existing entry if the hash does not match
                let _ = conn.execute(
                    "UPDATE RSSEntry
                    SET description = ?1,
                        category    = ?2,
                        comments    = ?3,
                        enclosure   = ?4,
                        guid        = ?5,
                        hash        = ?6,
                        title       = ?7,
                        link        = ?8,
                        pub_date    = ?9,
                        author      = ?10
                    WHERE hash      = ?11
                      ",
                    params![
                        e.description,
                        e.category,
                        e.comments,
                        e.enclosure,
                        e.guid,
                        e.hash,
                        e.title,
                        e.link,
                        e.pub_date,
                        e.author,
                        existing_hash
                    ],
                );
            }
        }
        None => {
            // Insert a new entry if no match is found
            conn.execute(
                "INSERT INTO RSSEntry
                    (title, link, description, pub_date, author, category, comments, enclosure, guid, hash)
                VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
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
                    e.hash,
                ],
            )?;
        }
    }

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
        Ok(Feed::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            FeedType::RSS,
        ))
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

    // TODO: We should get rid of all the ? and ensure proper uncrashable handling
    let feed_iter = stmt.query_map([], |row| {
        Ok(Feed::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            FeedType::ATOM,
        ))
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

fn get_atom_entry_db() -> Result<Vec<FeedEntryType>> {
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
        let hash: i64 = match row.get(11) {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Error loading atom hash: {e}");
                0
            }
        };
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

    let mut db_atom_entry: Vec<FeedEntryType> = vec![];

    for entry in atom_iter {
        // FIXME this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting AtomEntry result to vector");
                continue;
            }
        };
        db_atom_entry.push(FeedEntryType::ATOM(e));
    }

    Ok(db_atom_entry)
}

//------------------------------------------------------------------------------------------

fn get_rss_entry_db() -> Result<Vec<FeedEntryType>> {
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
        let hash: i64 = match row.get(9) {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Error Loading Hash: {e}");
                0
            }
        };
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

    let mut db_rss_entry: Vec<FeedEntryType> = vec![];

    for entry in rss_iter {
        // this is innefficient, can I do something directily from article_iter?
        let e = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("Error converting RssEntry result to vector");
                continue;
            }
        };
        db_rss_entry.push(FeedEntryType::RSS(e));
    }

    Ok(db_rss_entry)
}

//------------------------------------------------------------------------------------------

pub fn get_feed_items_db() -> Vec<FeedEntryType> {
    let mut atom_feed: Vec<FeedEntryType> =
        get_atom_entry_db().expect("Panic query fake AtomEntry");
    let mut rss_feed: Vec<FeedEntryType> = get_rss_entry_db().expect("Panic query fake RSSEntry");
    let mut full_feeds: Vec<FeedEntryType> = Vec::new();
    full_feeds.append(&mut atom_feed);
    full_feeds.append(&mut rss_feed);

    println!("Number of rows loaded from DB: {:?}", full_feeds.len());

    return full_feeds;
}
