#![feature(result_option_inspect)]

extern crate core;

use crate::hot_shots::HotShotSource;
use rusqlite::Connection;
use std::env::{args, current_exe};
use std::process::exit;
use std::time::Duration;

mod db;
mod hot_shots;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let (hs_name, hs) = args()
        .nth(1)
        .and_then(|s| match s.as_str() {
            "xkom" => Some((s, hot_shots::xkom::HotShot::new())),
            _ => None,
        })
        .or_else(|| {
            println!(
                "usage: ./{} <hot shot provider name>",
                current_exe()
                    .ok()?
                    .to_str()?
                    .rsplit(std::path::MAIN_SEPARATOR)
                    .next()
                    .unwrap()
            );

            exit(1);
        })
        .unwrap();

    println!("checking configuration for {hs_name} trait");

    hs.check_configuration()?;

    println!("configuration valid for {hs_name} trait");

    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .user_agent(&format!("hotshots/{}", env!("CARGO_PKG_VERSION")))
        .build();

    print!("db connection...");
    let conn = Connection::open("./db.sqlite").unwrap();
    println!("works");
    
    db::migrate(&conn)?;

    print!("configuring scraper...");
    let req = hs.configure_scraper(&agent);
    println!("done");

    print!("request perform...");
    let rsp = req.call()?;
    println!("done");

    print!("transform response...");

    hs.transform_response(rsp)
        .inspect(|_| println!("and then store"))
        .and_then(|data| hs.store(&conn, data))?;

    println!("bye!");

    Ok(())
}
