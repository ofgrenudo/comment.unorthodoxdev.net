use std::env;

use log::debug;
use sanitizer::prelude::*;
use uuid::Uuid;
pub mod create;
pub mod read;
pub mod update;
pub mod delete;

#[derive(Debug, Sanitize)]
pub struct Comment {
    pub id: Uuid,

    #[sanitize(trim, lower_case, alphanumeric)]
    pub ip_address: String,

    #[sanitize(trim, lower_case, alphanumeric)]
    pub related_post_id: String,
    // Comment Information

    #[sanitize(trim, lower_case, alphanumeric)]
    pub username: String,

    #[sanitize(trim, lower_case, alphanumeric)]
    pub comment: String,

    #[sanitize(trim, lower_case, alphanumeric)]
    pub time_stamp: String,

    pub visible: bool,
}

pub fn check_database() {
    let connection = sqlite::open(env::var("DATABASE_URL").unwrap()).unwrap();
    debug!("[COMMENTS.rs] Checking that the table 'comments' exists.");
    let query = format!("SELECT 1 FROM comments;");
    let result = connection.execute(query).expect("\n\nERROR: No table 'comments', exists. Please run, `diesel migration run` to generate a new comments table.\n\n");
}