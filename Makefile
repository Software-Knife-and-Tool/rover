#
#
#
.PHONY: commit

commit:
	cargo fmt
	cargo test
	cargo clippy
