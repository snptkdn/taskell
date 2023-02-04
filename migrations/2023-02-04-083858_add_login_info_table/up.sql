CREATE TABLE login_info
(
  id SERIAL PRIMARY KEY,
  mac_address VARCHAR(64) NOT NULL,
  user_id int
    REFERENCES users(id)
    ON DELETE SET NULL 
    ON UPDATE CASCADE
)
