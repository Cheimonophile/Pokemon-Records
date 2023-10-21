

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
	cp pokemon.sqlite src-tauri/dev/test-db.sqlite

shell:
	cd src && python manage.py shell