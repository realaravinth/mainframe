default:
	cargo build

clean:
	cargo clean

test:
	cargo test --all-features --no-fail-fast

migrate:
	@-rm -rf db
	@-mkdir db
	@cargo run --bin migrate
