.PHONY: default install build run

default: test

install:
	poetry install

build:
	poetry run maturin develop

build-prod:
	poetry run maturin develop --release

test: build
	docker-compose up -d
	poetry run pytest --benchmark-skip -s tests/*

b: build
	poetry run pytest -v --benchmark-only
