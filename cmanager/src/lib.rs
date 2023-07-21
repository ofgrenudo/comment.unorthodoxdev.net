use uuid::Uuid;

#[derive(Debug)]
pub struct Comment {
    id: Uuid,
    ip: String,
    username: String,
    comment: String,
    visible: bool,
}

#[derive(Debug)]
pub struct CommentError {
    error: String,
    comment: Comment,
}

enum CommentResult<Comment, CommentError> {
    Ok(Comment),
    Err(CommentError),
}

pub fn new(ip: String, username: String, comment: String) -> Result<Comment, CommentError> {
    // Filter Comment String Lengths
    if ip.len() > 500 {
        let problem_comment = Comment {
            id: Uuid::new_v4(),
            ip: "error".to_string(),
            username: username,
            comment: comment,
            visible: false,        
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
            visible: false,        
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
            visible: false,        
        };

        let comment_too_long = CommentError {
            error: "Error, your comment was over 10,000 characters, this attempt has been logged, and will be reviewed later. Pleas try again :)".to_string(),
            comment: problem_comment
        };

        return Err(comment_too_long);
    }

    let incoming_comment = Comment {
        id: Uuid::new_v4(),
        ip: ip,
        username: username,
        comment: comment,
        visible: true,        
    };
    
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

// todo
pub fn get() -> Comment {
    let example_comment = Comment {
        id: Uuid::new_v4(),
        ip: "terst".to_string(),
        username: "terst".to_string(),
        comment: "terst".to_string(),
        visible: true,    
    };

    return example_comment
}