# Backend migrations

Run these commands from the backend directory after installing sqlx-cli:

```sh
make db:up
make db:down
make db:down:2
make db:down:all
make db:add:add_post_summary
```

DATABASE_URL must be defined in your shell or in .env.
