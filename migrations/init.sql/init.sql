CREATE TABLE IF NOT EXISTS url_mapping (
    id VARCHAR(6) PRIMARY KEY UNIQUE NOT NULL,
    long_url VARCHAR(255) NOT NULL,
    short_url VARCHAR(50) NOT NULL
);

-- insert some data
INSERT INTO url_mapping (id, long_url, short_url) VALUES ('yt55fd', 'https://www.google.com', 'google');