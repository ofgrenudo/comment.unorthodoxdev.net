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
}