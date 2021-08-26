default:
	cargo build

clean:
	cargo clean

test:
	cargo test --all-features --no-fail-fast

migrate:
	@-rm mainframe.db
	sqlx database create
	sqlx migrate run
	@cargo sqlx prepare -- --bin mainframe --all-features
