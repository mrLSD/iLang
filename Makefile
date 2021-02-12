clippy:
	@cargo clippy

check:
	@cargo check

build:
	@cargo build --release
	
fmt:
	@cargo +nightly fmt

test:
	@cargo test --tests -- --nocapture

atest:
	@cargo test --all

cover:
	@export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
		&& export CARGO_INCREMENTAL=0 \
		&& export RUSTDOCFLAGS="-Cpanic=abort" \
		&& rm -rf ./target \
		&& cargo +nightly build --all \
		&& cargo +nightly test --all \
		&& grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/ \
		&& firefox ./target/debug/coverage/index.html \
		&& export RUSTFLAGS=""  && cargo clippy && cargo test 

tst:
	@cargo test test_codegen_global_let_binding -- --nocapture
	