use chrono::Utc;
use sha256::digest;
use uuid::Uuid;

use crate::{Comment, CommentError, DatabaseError};


/// to test comment
/// ```bash
/// curl -X POST http://localhost:8080/comment/new/ \                           
/// -H "Content-Type: application/x-www-form-urlencoded" \
/// -d "id=some-unique-id&ip=127.0.0.1&username=JohnDoe&comment=This+is+a+test+comment.&timestamp=2025-03-02T12%3A00%3A00Z&visible=1&post_url=http%3A%2F%2Fexample.com%2Fpost%2F123"
/// ```
pub fn comment(ip: String, username: String, comment: String) -> Result<Comment, CommentError> {
    let sha_ip = digest(&ip.replace(":", ""));

    let max_ip_length = 10_000;
    let max_username_length = 500;
    let max_comment_length = 10_000;


    let mut new_comment = Comment {
        id: Uuid::new_v4(),
        ip: if sha_ip.len() > max_ip_length { "ip_too_long".to_string() } else { sha_ip.to_string() },
        username: if username.len() > max_username_length { "username_too_long".to_string() } else { username },
        comment: if comment.len() > max_comment_length { "comment_too_long".to_string() } else { comment },
        timestamp: Utc::now().to_string(),
        visible: 1,
        post_url: "null".to_string(),        
    };

    if new_comment.username == "username_too_long" || new_comment.comment == "comment_too_long" || new_comment.ip == "ip_too_long" {
        new_comment.visible = 0;
    }
    

     // Open a connection to the database
    let connection = sqlite::open("comments.db").map_err(|_| DatabaseError::DatabaseError).unwrap();

    // Ensure table exists before inserting data (use prepared statements)
    if let Err(e) = connection.execute("
        CREATE TABLE IF NOT EXISTS comments (
            id TEXT NOT NULL PRIMARY KEY,
            ip TEXT,
            username TEXT NOT NULL,
            comment TEXT NOT NULL,
            timestamp TEXT,
            visible INT NOT NULL,
            post_url TEXT
        );
    ") {
        eprintln!("Failed to create table: {}", e);
    }


    // Use a prepared statement to avoid SQL injection
    let mut statement = connection.prepare("
        INSERT INTO comments (id, ip, username, comment, timestamp, visible)
        VALUES (?, ?, ?, ?, ?, ?);
    ").map_err(|_| DatabaseError::DatabaseError).unwrap();

    statement.bind((1, new_comment.id.to_string().as_str())).unwrap();
    statement.bind((2, new_comment.ip.as_str())).unwrap();
    statement.bind((3, new_comment.username.as_str())).unwrap();
    statement.bind((4, new_comment.comment.as_str())).unwrap();
    statement.bind((5, new_comment.timestamp.as_str())).unwrap();
    statement.bind((6, new_comment.visible)).unwrap();

    statement.next().map_err(|_| DatabaseError::DatabaseError).unwrap();

    Ok(new_comment)
}

/// to test comment on post
/// ```bash
/// curl -X POST http://localhost:8080/comment/new/post \                           
/// -H "Content-Type: application/x-www-form-urlencoded" \
/// -d "id=some-unique-id&ip=127.0.0.1&username=JohnDoe&comment=This+is+a+test+comment.&timestamp=2025-03-02T12%3A00%3A00Z&visible=1&post_url=http%3A%2F%2Fexample.com%2Fpost%2F123"
/// ```
pub fn comment_on_post(ip: String, username: String, comment: String, url: String) -> Result<Comment, CommentError> {
    let sha_ip = digest(&ip.replace(":", ""));

    let max_ip_length = 10_000;
    let max_username_length = 500;
    let max_comment_length = 10_000;
    let max_url_length = 2_048;

    let mut new_comment = Comment {
        id: Uuid::new_v4(),
        ip: if sha_ip.len() > max_ip_length { "ip_too_long".to_string() } else { sha_ip.to_string() },
        username: if username.len() > max_username_length { "username_too_long".to_string() } else { username },
        comment: if comment.len() > max_comment_length { "comment_too_long".to_string() } else { comment },
        timestamp: Utc::now().to_string(),
        visible: 1,
        post_url: if url.len() > max_url_length { "url_too_long".to_string() } else { url.to_string() },
    };

    // Hide comment if any field exceeded the limit
    if new_comment.username == "username_too_long" 
        || new_comment.comment == "comment_too_long"
        || new_comment.ip == "ip_too_long"
        || new_comment.post_url == "url_too_long" {
        new_comment.visible = 0;
    }

    // Open a connection to the database
    let connection = sqlite::open("comments.db").map_err(|_| DatabaseError::DatabaseError).unwrap();

    // Ensure table exists before inserting data (use prepared statements)
    if let Err(e) = connection.execute("
        CREATE TABLE IF NOT EXISTS comments (
            id TEXT NOT NULL PRIMARY KEY,
            ip TEXT,
            username TEXT NOT NULL,
            comment TEXT NOT NULL,
            timestamp TEXT,
            visible INT NOT NULL,
            post_url TEXT
        );
    ") {
        eprintln!("Failed to create table: {}", e);
    }

    // Use a prepared statement to avoid SQL injection
    let mut statement = connection.prepare("
        INSERT INTO comments (id, ip, username, comment, timestamp, visible, post_url)
        VALUES (?, ?, ?, ?, ?, ?, ?);
    ").map_err(|_| DatabaseError::DatabaseError).unwrap();

    statement.bind((1, new_comment.id.to_string().as_str())).unwrap();
    statement.bind((2, new_comment.ip.as_str())).unwrap();
    statement.bind((3, new_comment.username.as_str())).unwrap();
    statement.bind((4, new_comment.comment.as_str())).unwrap();
    statement.bind((5, new_comment.timestamp.as_str())).unwrap();
    statement.bind((6, new_comment.visible)).unwrap();
    statement.bind((7, new_comment.post_url.as_str())).unwrap();

    statement.next().map_err(|_| DatabaseError::DatabaseError).unwrap();

    Ok(new_comment)
}
