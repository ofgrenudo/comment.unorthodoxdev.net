/// Returns all comments with the newest on top.
pub fn get_all_comments() {
    let mut comments: Vec<Comment> = vec![];
    let connection = sqlite::open("comments.db").unwrap();
    
    // Note we had to split up the blow two statements. For some reason, the statement.next() function later down the program would not pull comments when we ran the CREATE TABLE command.
    // Maybe its because I didnt do the format!() like i did in the new comment function???
    // The compiler is angry here, i know. Ill fix it all later, but for now it looks aesthetically pleasing uwu.
    
    let query = "SELECT * FROM comments ORDER BY timestamp DESC;";
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

/// Returns top 50 comments with the newest on top (DESC order).
pub fn get_latest() -> Vec<Comment> {
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

/// Returns one comment with the ID that you searched for.
pub fn get_one_by_id() {

}

/// Returns all comments with a matching IP address.
pub fn get_all_by_ip() {

}

/// Returns all comments with a username.
pub fn get_all_by_username() {

}