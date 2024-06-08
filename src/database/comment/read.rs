use log::{debug, info};
use sanitizer::Sanitize;
use uuid::Uuid;
use std::{env, time::SystemTime};
use sqlite::{self, State};

use super::Comment;

pub fn read() -> Vec<Comment> {
    let mut comments: Vec<Comment> = vec![];
    let connection = sqlite::open(env::var("DATABASE_URL").unwrap()).unwrap();

    let query = "SELECT * FROM comments ORDER BY time_stamp DESC LIMIT 50;"; 
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let temp_id = statement.read::<String, _>("id").unwrap();
        let id: Uuid = Uuid::parse_str(&temp_id).unwrap();
        
        
        let visible_raw = statement.read::<i64, _>("visible").unwrap();
        let mut visible_processed = false;
        if visible_raw > 0 { visible_processed = true; }


        comments.push(Comment {
            id,
            ip_address: statement.read::<String, _>("ip_address").unwrap().to_string(),
            related_post_id: statement.read::<String, _>("related_post_id").unwrap().to_string(),
            username: statement.read::<String, _>("username").unwrap().to_string(),
            comment: statement.read::<String, _>("comment").unwrap().to_string(),
            time_stamp: statement.read::<String, _>("time_stamp").unwrap(),
            visible: visible_processed,
        });
    }

    comments
}

// #[cfg(test)]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;

//     #[test]
//     fn test_read() {
//         let _ = crate::database::comment::READ(crate::database::comment::Comment { id: uuid::Uuid::new_v4(), ip_address: "169.120.1.1".to_string(), related_post_id: "None".to_string(), username: "Joshua Winters-Brown".to_string(), comment: "This is a comment :)".to_string(), time_stamp: "time".to_string(), visible: true });
//         assert_eq!(1, 1);
//     }
// }