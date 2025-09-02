
lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::must-use-candidate \
      -A clippy::missing-errors-doc \
      -A clippy::missing-panics-doc \
      -A clippy::module-name-repetitions \
      -A clippy::return-self-not-must-use \
      \
      -D warnings

test:
	cargo test --all
	cargo test --all --release
	make test-wasm

test-wasm:
	cargo install wasm-pack
	cd vents && wasm-pack test --firefox --headless

test-browser:
	cargo install wasm-pack
	cd vents && wasm-pack test --firefox
