CREATE TABLE users (
                       id INT AUTO_INCREMENT PRIMARY KEY,
                       username VARCHAR(255) NOT NULL,
                       password VARCHAR(255) NOT NULL,
                       status INT NOT NULL DEFAULT 1,
                       created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                       updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                       last_login TIMESTAMP,
                       UNIQUE (username)
);

CREATE TABLE posts (
                       id INT AUTO_INCREMENT PRIMARY KEY,
                       author_id INT NOT NULL REFERENCES users(id),
                       slug VARCHAR(255) NOT NULL,
                       title VARCHAR(255) NOT NULL,
                       content TEXT NOT NULL,
                       status integer NOT NULL DEFAULT 1,
                       created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                       updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                       UNIQUE (slug)
);
