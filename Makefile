#
#
#
.PHONY: commit release debug

release:
	@cargo build --release --bin mu-rover

debug:
	@cargo build --bin mu-rover

commit:
	cargo fmt
	cargo test
	cargo clippy
