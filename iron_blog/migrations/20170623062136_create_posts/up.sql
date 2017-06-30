CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    filename TEXT NOT NULL,
    author TEXT NOT NULL DEFAULT 'Anonymous',
    time BIGINT NOT NULL
)
