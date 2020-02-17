-- Your SQL goes here
CREATE TABLE tweets (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    content VARCHAR(120) NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);