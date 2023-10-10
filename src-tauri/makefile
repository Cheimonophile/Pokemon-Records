


db:
	rm dev/test-db.sqlite
	diesel database reset
	BUILD_DB=true time cargo run

redo:
	diesel migration redo
	diesel migration redo