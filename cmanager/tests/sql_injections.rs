use cmanager::new::comment;
use cmanager::{Comment, CommentError};
use std::fs;
use sqlite::Connection;
use sqlite::Value;



#[test]
fn test_sql_injection_attempt() {
    // Cleanup database before the test
    let _ = fs::remove_file("comments.db");

    // Attempt a malicious SQL injection attack
    let result = comment(
        "192.168.1.1".to_string(),
        "testuser'); DROP TABLE comments; --".to_string(),
        "This is a malicious comment".to_string()
    );

    // Assert the function still returned Ok (even with malicious input)
    assert!(result.is_ok());

    // Connect to the database and ensure the comments table still exists
    let connection = Connection::open("comments.db").unwrap();
    let mut statement = connection.prepare("
        SELECT COUNT(*) FROM comments
    ").unwrap();

    let mut count = 0;
    while let Ok(sqlite::State::Row) = statement.next() {
        count = statement.read::<i64, usize>(0).unwrap();
    }

    // Assert the database was NOT dropped
    assert!(count == 1, "SQL Injection succeeded! TABLE DROPPED!");

    // Assert the username was sanitized
    let mut statement = connection.prepare("
        SELECT username FROM comments
    ").unwrap();

    let mut found_username = String::new();
    while let Ok(sqlite::State::Row) = statement.next() {
        found_username = statement.read::<String, usize>(0).unwrap();
    }

    assert_eq!(found_username, "testuser'); DROP TABLE comments; --", "Username was altered!");
}
