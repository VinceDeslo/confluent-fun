run:
	RUST_LOG=debug cargo run

rds-connect:
	psql --host=$(host) --port=5432 --username=eda --password --dbname=edafun