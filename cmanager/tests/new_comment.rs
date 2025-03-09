use cmanager::new::comment;
use cmanager::{Comment, CommentError};
use std::fs;

#[test]
fn test_valid_comment() {
    // Cleanup database before the test (optional)
    let _ = fs::remove_file("comments.db");

    let result = comment(
        "192.168.1.1".to_string(),
        "testuser".to_string(),
        "This is a test comment".to_string()
    );

    assert!(result.is_ok());

    let comment = result.unwrap();
    assert_eq!(comment.username, "testuser");
    assert_eq!(comment.comment, "This is a test comment");
    assert_eq!(comment.visible, 1);

    // Open the database and ensure the record exists
    let connection = sqlite::open("comments.db").unwrap();
    let mut statement = connection.prepare("SELECT * FROM comments").unwrap();
    let mut found = false;

    while let Ok(sqlite::State::Row) = statement.next() {
        let username: String = statement.read(2).unwrap();
        let comment: String = statement.read(3).unwrap();

        if username == "testuser" && comment == "This is a test comment" {
            found = true;
        }
    }

    assert!(found, "Comment was not found in the database");
}
