use sanitizer::Sanitize;
use uuid::Uuid;
use std::env;
use sqlite;

pub fn create(mut comment: crate::database::comment::Comment) -> Result<i32, i32> {
    let connection = sqlite::open(env::var("DATABASE_URL").unwrap()).unwrap();

    comment.id = Uuid::new_v4();

    comment.sanitize();

    let query = format!("
        CREATE TABLE IF NOT EXISTs comments (id TEXT NOT NULL PRIMARY KEY, ip_address TEXT, related_post_id TEXT, username TEXT NOT NULL, comment TEXT NOT NULL, time_stamp TEXT, visible INT NOT NULL);
        INSERT INTO comments VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {})", 
        comment.id, comment.ip_address, comment.related_post_id, comment.username, comment.comment, comment.time_stamp, comment.visible);

    connection.execute(query).unwrap();

    Ok(0)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_create() {
        let _ = crate::database::comment::create::create(crate::database::comment::Comment { id: uuid::Uuid::new_v4(), ip_address: "169.120.1.1".to_string(), related_post_id: "None".to_string(), username: "Joshua Winters-Brown".to_string(), comment: "This is a comment :)".to_string(), time_stamp: "time".to_string(), visible: true });
        assert_eq!(1, 1);
    }
}