CREATE TABLE users
(
  id SERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  encrypted_pass VARCHAR(255) NOT NULL 
)
