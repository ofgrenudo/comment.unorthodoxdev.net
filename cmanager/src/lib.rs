use uuid::Uuid;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
pub mod new;
pub mod get;

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
#[derive(Debug)]  // Added Serde derives
pub struct Comment {
    pub id: Uuid,
    pub ip: String,
    pub username: String,
    pub comment: String,
    pub timestamp: String,
    pub visible: i64,
    pub post_url: String,
}

// Manual Serialize implementation for Comment
impl Serialize for Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Comment", 7)?;
        
        // Manually serialize each field
        state.serialize_field("id", &self.id.to_string())?;  // Convert UUID to string
        state.serialize_field("ip", &self.ip)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("visible", &self.visible)?;
        state.serialize_field("post_url", &self.post_url)?;
        
        // Finalize the serialization
        state.end()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
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
    get::get_latest()
}