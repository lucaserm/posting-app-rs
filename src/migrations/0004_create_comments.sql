CREATE TABLE comments (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL
        CHECK (length(trim(content)) BETWEEN 1 AND 2000),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX comments_post_created_idx
    ON comments (post_id, created_at DESC, id DESC);

CREATE INDEX comments_user_idx
    ON comments (user_id);
