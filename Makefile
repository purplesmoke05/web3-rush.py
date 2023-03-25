.PHONY: default install build run

default: test

install:
	poetry install

build:
	poetry run maturin develop

build-prod:
	poetry run maturin develop --release

test: build
	poetry run pytest -v --benchmark-skip -s tests/*

b: build
	poetry run pytest -v --benchmark-only --benchmark-json output.json --benchmark-verbose --benchmark-group-by=func --benchmark-sort=fullname

format:
	cargo fmt
	poetry run black tests web3_rush