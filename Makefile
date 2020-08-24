clippy:
	@cargo clippy

check:
	@cargo check
	
fmt:
	@cargo +nightly fmt

test:
	@cargo test -- --nocapture

cover:
	@export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" && export CARGO_INCREMENTAL=0 && export RUSTDOCFLAGS="-Cpanic=abort" && cargo +nightly build && cargo +nightly test && grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/ && firefox ./target/debug/coverage/index.html
