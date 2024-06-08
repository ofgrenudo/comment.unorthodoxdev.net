use log::debug;
use std::env;
use sqlite;

// Reads the latest 50 comments
#[allow(dead_code)]
pub fn read() {
    debug!("{}", env::var("DATABASE_URL").unwrap());

    let mut _comments: Vec<crate::database::comment::Comment> = vec![];
    let _connection = sqlite::open(env::var("DATABASE_URL").unwrap()).unwrap();
}