use std::env;

use database::comment;
use log::{debug, info, trace};
use dotenv::dotenv as dotnet;
use clap::{command, Parser};
use log4rs;

// Import Modules
mod database;

#[derive(Parser, Debug)]
#[command(name = "comments.unorthodoxdev.net")]
#[command(version = "0.1")]
#[command(about="This application is what manages the comments on unorthodoxdev.net!")]
#[command(version, about, long_about = None)]
struct Args {
}

fn main() {
    //Configure log4rs
    log4rs::init_file("configurations/log4rs.yaml", Default::default()).unwrap();

    trace!("[MAIN.rs] Initializing environment variables.");
    dotnet().ok(); // load .env

    info!("[MAIN.rs] Parsing runtime args.");
    let _cli = Args::parse();

    info!("[MAIN.rs] Connecting to the database at sqlite://{}", env::var("DATABASE_URL").unwrap());
    // check if database exists.
    comment::check_database();

    // TEST

    let _ = crate::database::comment::create::create(crate::database::comment::Comment { id: uuid::Uuid::new_v4(), ip_address: "169.120.1.1".to_string(), related_post_id: "None".to_string(), username: "Joshua Winters-Brown".to_string(), comment: "This is a comment :)".to_string(), time_stamp: "time".to_string(), visible: true });
    let _database_read = crate::database::comment::read::read();
    debug!("{:#?}", _database_read);
}