## Delete a Comment

`DELETE FROM comments WHERE username = "<badcomment_username>"`

This can also be done with ip

`DELETE FROM comments WHERE ip = "somesha256sum"`

Or with specific workds

`DELETE FROM comments WHERE comment LIKE "<problem_word>%"`;