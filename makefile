


setup: db
	npm install

db:
	rm src-tauri/dev/test-db.sqlite || true
	mkdir -p src-tauri/dev
	cp /Users/ben/Dropbox/Benjamin/Games/Pokemon/pokemon.sqlite src-tauri/dev/test-db.sqlite

shell:
	cd src && python manage.py shell