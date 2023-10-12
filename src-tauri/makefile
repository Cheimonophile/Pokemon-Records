


db:
	rm dev/test-db.sqlite
	diesel database reset
	BUILD_DB=true DATABASE_URL="sqlite://dev/test-db.sqlite" time cargo run

redo:
	diesel migration redo
	diesel migration redo