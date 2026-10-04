ALTER TABLE posts
ADD COLUMN author_id BIGINT REFERENCES users(id) ON DELETE CASCADE;

CREATE INDEX posts_author_id_idx ON posts(author_id);
