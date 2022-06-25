use rusqlite::Connection;
use serde::Deserialize;
use std::io;
use ureq::{Agent, Request, Response};

pub mod xkom;

pub(crate) trait HotShotSource<'h> {
    type Output: Deserialize<'h>;

    fn check_configuration(&self) -> Result<(), String>;
    fn configure_scraper(&self, agent: &Agent) -> Request;
    fn store(&self, db_conn: &Connection, response: Self::Output) -> io::Result<()>;
    fn transform_response(&self, raw_response: Response) -> std::io::Result<Self::Output>;
    fn new() -> Self;
}
