-- Your SQL goes here

CREATE TABLE comments (
    id INTEGER PRIMARY KEY NOT NULL,
    post_time DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    ip_address BLOB,
    author VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    from_engineer BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO comments (content, author, from_engineer) VALUES ("Tell us how you use your Cum Engine!", "Dr. Zaun Mundo, PhD", true);
