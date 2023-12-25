


setup: db
	npm install

db:
	cd src-tauri && make db

shell:
	cd src && python manage.py shell