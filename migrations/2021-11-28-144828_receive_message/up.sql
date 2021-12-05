CREATE TABLE messages (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL NOT NULL,
    text TEXT NOT NULL,
    utc_timestamp TIMESTAMPTZ NOT NULL,
    unix_timestamp INT NOT NULL
);
