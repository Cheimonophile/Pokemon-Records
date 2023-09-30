

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

database:
	rm pokemon.sqlite3 || true
	python migrate.py

shell:
	cd src && python manage.py shell

rust:
	python master.py > src-tauri/src/master2.rs