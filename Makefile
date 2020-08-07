clippy:
	@cargo clippy

check:
	@cargo check
	
fmt:
	@cargo +nightly fmt

test:
	@cargo test -- --nocapture
