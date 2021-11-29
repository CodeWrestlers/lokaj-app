CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    is_bot BOOLEAN NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NULL,
    username VARCHAR NULL,
    language_code VARCHAR NULL
)
