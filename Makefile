default:
	cargo build

clean:
	cargo clean

doc:
	cargo doc --no-deps --workspace --all-features

test:
	cargo test --all-features --no-fail-fast

migrate:
	@-rm -rf db
	@-mkdir db
	@cargo run --bin migrate
