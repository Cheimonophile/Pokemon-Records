

makemigrations:
	cd src && python manage.py makemigrations


migrate: makemigrations
	cd src && python manage.py migrate

user:
	cd src && python manage.py createsuperuser

clean:
	rm src/db.sqlite3

run: database
	# rm src/db.sqlite3 || true
	# make migrate
	# cd src && python manage.py shell <master.py
	time python master.py

db:
	rm src-tauri/dev/test-db.sqlite || true
	cd src-tauri && diesel database reset --database-url dev/test-db.sqlite
	cd src-tauri && BUILD_DB=true cargo run

shell:
	cd src && python manage.py shell