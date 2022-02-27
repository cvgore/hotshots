use rusqlite::Connection;
use std::io;
use ureq::{Agent, Request, Response};

pub mod xkom;

pub(crate) trait HotShotSource {
    type Output;

    fn check_configuration(&self) -> Result<(), String>;
    fn configure_scraper(&self, agent: &Agent) -> Request;
    fn store(&self, db_conn: &Connection, response: Self::Output) -> io::Result<()>;
    fn transform_response(&self, raw_response: Response) -> std::io::Result<Self::Output>;
    fn new() -> Self;
}
