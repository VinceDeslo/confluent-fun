run:
	RUST_LOG=debug cargo run

run-server:
	cargo run --bin server

rds-connect:
	psql --host=$(host) --port=5432 --username=eda --password --dbname=edafun