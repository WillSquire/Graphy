CREATE TABLE users (
  id uuid PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL
  -- password VARCHAR(255) NOT NULL
)