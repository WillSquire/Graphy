CREATE TABLE users_groups
(
  id uuid PRIMARY KEY,
  added_at TIMESTAMP WITH TIME ZONE NOT NULL,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  group_id uuid NOT NULL REFERENCES groups (id) ON DELETE CASCADE
)
