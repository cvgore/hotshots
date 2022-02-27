use rusqlite::{params, Connection};
use std::error::Error;

const MAX_MIGRATIONS: usize = 1;

pub fn migrate(conn: &Connection) -> Result<(), Box<dyn Error>> {
    lvl0(conn)?;

    let mut level = conn
        .query_row("SELECT MAX(id) FROM migrations", [], |row| row.get(0))
        .unwrap_or(0);

    while level < MAX_MIGRATIONS {
        let migrator = match level {
            1 => lvl1,
            _ => lvl_max,
        };

        migrator(conn)?;

        level += 1;

        conn.execute("UPDATE migrations SET id=?1", params![level])?;
    }

    Ok(())
}

fn lvl_max(_conn: &Connection) -> rusqlite::Result<()> {
    println!("no migration required, max level reached");

    Ok(())
}

fn lvl0(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations ( id INTEGER PRIMARY KEY )",
        [],
    )
    .map(|_| ())
}

fn lvl1(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE hs_xkom (
                id TEXT PRIMARY KEY,
                price REAL NOT NULL,
                old_price REAL NOT NULL,
                promotion_total_count INTEGER NOT NULL,
                sale_count INTEGER NOT NULL,
                max_buy_count INTEGER NOT NULL,
                promotion_name TEXT NOT NULL,
                promotion_end INTEGER NOT NULL,
                product_id TEXT NOT NULL,
                product_name TEXT NOT NULL,
                product_category_id TEXT NOT NULL,
                product_category_name_singular TEXT NOT NULL,
                product_web_url TEXT NOT NULL
            )
        ",
        [],
    )
    .map(|_| ())
}
