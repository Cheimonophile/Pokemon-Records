

EMPTY_DB_URL = ./dev/empty_db.sqlite
BEFORE_MIGRATION_DB_URL = ./dev/before_migration_db.sqlite
AFTER_MIGRATION_DB_URL = ./dev/after_migration_db.sqlite
PROD_DB_URL = /Users/ben/Dropbox/Benjamin/Games/Pokemon/pokemon.sqlite



db-build:
	mkdir -p dev
	sqlx database setup --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate run --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate run --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate revert --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate run --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate revert --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate run --database-url sqlite://$(EMPTY_DB_URL)



db:
	rm $(EMPTY_DB_URL) $(BEFORE_MIGRATION_DB_URL) $(AFTER_MIGRATION_DB_URL) || true

	mkdir -p dev
	cp $(PROD_DB_URL) $(BEFORE_MIGRATION_DB_URL)
	cp $(PROD_DB_URL) $(AFTER_MIGRATION_DB_URL)
	sqlx database setup --database-url sqlite://$(EMPTY_DB_URL)
	sqlx migrate run --database-url sqlite://$(AFTER_MIGRATION_DB_URL)

redo: db
	sqlx migrate run --database-url sqlite://$(AFTER_MIGRATION_DB_URL)
	sqlx migrate revert --database-url sqlite://$(AFTER_MIGRATION_DB_URL)
	sqlx migrate run --database-url sqlite://$(AFTER_MIGRATION_DB_URL)
	sqlx migrate revert --database-url sqlite://$(AFTER_MIGRATION_DB_URL)
	sqlx migrate run --database-url sqlite://$(AFTER_MIGRATION_DB_URL)


down: db
	sqlx migrate revert --database-url sqlite://$(AFTER_MIGRATION_DB_URL)

diff: db
	@du -sh $(BEFORE_MIGRATION_DB_URL)
	@du -sh $(AFTER_MIGRATION_DB_URL)