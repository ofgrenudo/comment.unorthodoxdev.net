use std::vec;

use uuid::Uuid;
use sqlite::{self, State};
pub mod new;

/// # Comment
/// 
/// Comment is the data type that we store in our sqlite3 database. It consists of the following
/// 
/// - id
/// - ip
/// - username
/// - comment
/// - timestamp
/// - visible
/// 
/// The ID and Timestamp are automatically generated. The ID is generated using a UUID V3 format (essentially completly random). The timestamp is in UTC time, cuz we are cool
/// The username, and comment and IP are user supplied. Typically though, the IP is sourced from the Actix Web Server.
///  
#[derive(Debug)]
pub struct Comment {
    pub id: Uuid,
    pub ip: String,
    pub username: String,
    pub comment: String,
    pub timestamp: String,
    pub visible: i64,
}

#[derive(Debug)]
pub struct CommentError {
    error: String,
    comment: Comment,
}

pub enum CommentResult<Comment, CommentError> {
    Ok(Comment),
    Err(CommentError),
}

/// Depreciated, please use cmanager::new::comment;
/// 
/// Takes 3 inputs, IP (String), Username (String), and Comment (String). It will return to you a Result, Ok(Comment) or Err(CommentError).
pub fn new(ip: String, username: String, comment: String) -> Result<Comment, CommentError> {
    new::comment(ip, username, comment) // Look at that sexy refactoring, where I keep the origional API alive :) 
}

/// Soon to be Depreciated.
/// 
/// This function will return all contents up to 50 rows (not really all, which is why we are migrating to a new api endpoint). 
pub fn get_all() -> Vec<Comment> {
    let mut comments: Vec<Comment> = vec![];
    let connection = sqlite::open("comments.db").unwrap();
    
    // Note we had to split up the blow two statements. For some reason, the statement.next() function later down the program would not pull comments when we ran the CREATE TABLE command.
    // Maybe its because I didnt do the format!() like i did in the new comment function???
    // The compiler is angry here, i know. Ill fix it all later, but for now it looks aesthetically pleasing uwu.
    
    let query = "SELECT * FROM comments ORDER BY timestamp DESC LIMIT 50;";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let temp_id = statement.read::<String, _>("id").unwrap();
        let id: Uuid = Uuid::parse_str(&temp_id).unwrap();

        comments.push(Comment {
            id: id,
            ip: statement.read::<String, _>("ip").unwrap().to_string(),
            username: statement.read::<String, _>("username").unwrap().to_string(),
            comment: statement.read::<String, _>("comment").unwrap().to_string(),
            timestamp: statement.read::<String, _>("timestamp").unwrap(),
            visible: statement.read::<i64, _>("visible").unwrap(),            
        });
    }

    comments
}