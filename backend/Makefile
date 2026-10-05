SQLX ?= sqlx
MIGRATIONS ?= src/migrations

.PHONY: help db\:info db\:up db\:down db\:down\:all

help:
	@echo "Migration commands:"
	@echo "  make db:info                     Show migration status"
	@echo "  make db:up                       Apply pending up migrations"
	@echo "  make db:down                     Revert the latest migration"
	@echo "  make db:down:2                   Revert down to version 2"
	@echo "  make db:down:all                 Revert every migration"
	@echo "  make db:add:add_post_summary     Create reversible up/down migration files"
	@echo ""
	@echo "DATABASE_URL must be available in the environment or .env."

db\:info:
	$(SQLX) migrate info --source $(MIGRATIONS)

db\:up:
	$(SQLX) migrate run --source $(MIGRATIONS)

db\:down:
	$(SQLX) migrate revert --source $(MIGRATIONS)

db\:down\:all:
	$(SQLX) migrate revert --target-version 0 --source $(MIGRATIONS)

db\:down\:%:
	$(SQLX) migrate revert --target-version $* --source $(MIGRATIONS)

db\:add\:%:
	$(SQLX) migrate add -r --sequential --source $(MIGRATIONS) $*
