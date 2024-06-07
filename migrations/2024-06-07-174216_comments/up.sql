CREATE TABLE comments (
  id INT PRIMARY KEY,
  ip_address TEXT NOT NULL,
  related_post_id TEXT,
  -- Comment Information
  username TEXT NOT NULL,
  comment TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  visible INT NOT NULL DEFAULT FALSE,
)