CREATE TABLE url_mapping (
    id VARCHAR(6) PRIMARY KEY,
    long_url VARCHAR(255) NOT NULL,
    short_url VARCHAR(50) NOT NULL
);

-- insert some data
INSERT INTO url_mapping (long_url, short_url) VALUES ('https://www.google.com', 'google');