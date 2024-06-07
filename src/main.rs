use diesel::SqliteConnection;
use log::{error, info, warn, trace};
use diesel::Connection;
use log4rs;
use clap::Parser;
use dotenv::dotenv;
use std::env;

// Import Modules
mod api;

#[derive(Parser, Debug)]
#[command(name = "comments.unorthodoxdev.net")]
#[command(version = "0.1")]
#[command(about="This application is what manages the comments on unorthodoxdev.net!")]
#[command(version, about, long_about = None)]
struct Args {
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    //Configure log4rs
    log4rs::init_file("configurations/log4rs.yaml", Default::default()).unwrap();

    trace!("[MAIN.rs] Initializing environment variables.");
    dotenv().ok(); // load .env

    info!("[MAIN.rs] Parsing runtime args.");
    let cli = Args::parse();


    // check if database exists.
        // create if does not exists.

    // check mode running in, default webserver mode
        // if webserver mode start webserver.
            // register routes.
}