

BEFORE_MIGRATION_DB_URL = ./dev/before_migration.sqlite
AFTER_MIGRATION_DB_URL = ./dev/after_migration.sqlite
PROD_DB_URL = /Users/ben/Dropbox/Benjamin/Games/Pokemon/pokemon.sqlite



run:
	diesel migration run --database-url $(AFTER_MIGRATION_DB_URL)

redo:
	diesel migration redo --database-url $(AFTER_MIGRATION_DB_URL)
	diesel migration redo --database-url $(AFTER_MIGRATION_DB_URL)

db:
	rm $(BEFORE_MIGRATION_DB_URL) || true
	rm $(AFTER_MIGRATION_DB_URL) || true
	mkdir -p dev
	cp $(PROD_DB_URL) $(BEFORE_MIGRATION_DB_URL)
	cp $(PROD_DB_URL) $(AFTER_MIGRATION_DB_URL)
	diesel migration run --database-url $(AFTER_MIGRATION_DB_URL)

down: db
	diesel migration revert --database-url $(AFTER_MIGRATION_DB_URL)

diff: db
	@du -sh $(BEFORE_MIGRATION_DB_URL)
	@du -sh $(AFTER_MIGRATION_DB_URL)