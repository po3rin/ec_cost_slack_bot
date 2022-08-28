run:
	cargo build --release
	./target/release/ec_cost_slack_bot

check:
	cargo fmt -- --check && cargo clippy -- -D warnings

test:
	cargo test

doc:
	cargo doc --open
