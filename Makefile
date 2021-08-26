default:
	cargo run

clean:
	cargo clean

migrate:
	@-rm mainframe.db
	sqlx database create
	sqlx migrate run
	@cargo sqlx prepare -- --bin mainframe --all-features
