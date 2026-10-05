DROP INDEX posts_author_id_idx;

ALTER TABLE posts
DROP COLUMN author_id;
