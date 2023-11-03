install:
	cd ./frontend && yarn
	cd ./backend && cargo build

start:
	docker compose up

build:
	docker compose up --build