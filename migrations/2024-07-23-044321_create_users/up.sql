-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    id_user_discord BIGINT NOT NULL,
    id_server_discord BIGINT NOT NULL
)