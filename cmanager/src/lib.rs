use std::vec;

use uuid::Uuid;
use sqlite::{self, State};

#[derive(Debug)]
pub struct Comment {
    pub id: Uuid,
    pub ip: String,
    pub username: String,
    pub comment: String,
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

pub fn new(ip: String, username: String, comment: String) -> Result<Comment, CommentError> {
    // Filter comment string length
    if ip.len() > 500 {
        let problem_comment = Comment {
            id: Uuid::new_v4(),
            ip: "error".to_string(),
            username: username,
            comment: comment,
            visible: 0,        
        };

        let ip_too_long = CommentError {
            error: "Error, your ip address was over 500 characters, this attempt has been logged, and will be reviewed later. Pleas try again :)".to_string(),
            comment: problem_comment,
        };

        return Err(ip_too_long);
    }

    if username.len() > 500 { 
        let problem_comment = Comment {
            id: Uuid::new_v4(),
            ip: ip,
            username: "error".to_string(),
            comment: comment,
            visible: 0,        
        };

        let username_too_long = CommentError {
            error: "Error, your user name was over 500 characters, this attempt has been logged, and will be reviewed later. Pleas try again :)".to_string(),
            comment: problem_comment,
        };

        return Err(username_too_long);
    }

    if comment.len() > 10000 {
        let problem_comment = Comment {
            id: Uuid::new_v4(),
            ip: ip,
            username: username,
            comment: "error".to_string(),
            visible: 0,        
        };

        let comment_too_long = CommentError {
            error: "Error, your comment was over 10,000 characters, this attempt has been logged, and will be reviewed later. Pleas try again :)".to_string(),
            comment: problem_comment
        };

        return Err(comment_too_long);
    }


    // Everything looks good, lets move forward with commiting the information to the database.
    let incoming_comment = Comment {
        id: Uuid::new_v4(),
        ip: ip,
        username: username,
        comment: comment,
        visible: 1,        
    };
    
    let connection = sqlite::open("comments.db").unwrap();

    let query = format!("
        CREATE TABLE IF NOT EXISTs comments (id TEXT NOT NULL PRIMARY KEY, ip TEXT, username TEXT NOT NULL, comment TEXT NOT NULL, visible INT NOT NULL);
        INSERT INTO comments VALUES ('{}', '{}', '{}', '{}', {})", incoming_comment.id, incoming_comment.ip, incoming_comment.username, incoming_comment.comment, incoming_comment.visible );

    connection.execute(query).unwrap();

    // All looks good, lets return it.
    Ok(incoming_comment)
}

#[test]
fn test_new_ip_limit(){
    let myComment = new("1.2.3.4".to_string().repeat(5000), "uname".to_string(), "test".to_string());
    assert!(myComment.is_err());
}

#[test]
fn test_new_username_limit(){
    let myComment = new("1.2.3.4".to_string(), "uname".to_string().repeat(5000), "test".to_string());
    assert!(myComment.is_err());    
}

#[test]
fn test_new_comment_limit(){
    let myComment = new("1.2.3.4".to_string(), "uname".to_string(), "test".to_string().repeat(5000));
    assert!(myComment.is_err());    
}

#[test]
fn test_new_comment(){
    let myComment = new("1.2.3.4".to_string(), "uname".to_string(), "test".to_string());
    assert!(myComment.is_ok());        
}

pub fn get_all() -> Vec<Comment> {
    let mut comments: Vec<Comment> = vec![];
    let connection = sqlite::open("comments.db").unwrap();
    let query = "SELECT * FROM comments;";
    let mut statement = connection.prepare(query).unwrap(); // note this statement will break if I use anything other than a UUID as the primary key in ID. Ive spent hours troubleshooting this damn issue.
    
    while let Ok(State::Row) = statement.next() {
        let temp_id = statement.read::<String, _>("id").unwrap();
        let id: Uuid = Uuid::parse_str(&temp_id).unwrap();

        comments.push(Comment {
            id: id,
            ip: statement.read::<String, _>("ip").unwrap().to_string(),
            username: statement.read::<String, _>("username").unwrap().to_string(),
            comment: statement.read::<String, _>("comment").unwrap().to_string(),
            visible: statement.read::<i64, _>("visible").unwrap(),            
        });
    }

    comments
}