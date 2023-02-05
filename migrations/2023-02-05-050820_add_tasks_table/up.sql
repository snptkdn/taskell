CREATE TABLE tasks
(
  id SERIAL PRIMARY KEY,
  point int,
  user_id int
    REFERENCES users(id)
    ON DELETE SET NULL 
    ON UPDATE CASCADE
)
