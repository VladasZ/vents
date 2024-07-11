
lint:
	./scripts/lint.sh

test:
	cargo test --all
	cargo test --all --release
