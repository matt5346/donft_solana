target/deploy/donft.so: src/lib.rs
	cargo build-bpf

.PHONY: test
test: target/deploy/donft.so tests/lib.rs
	cargo test-bpf

.PHONY: build
build: target/deploy/donft.so

.PHONY: deploy
deploy: target/deploy/donft.so
	solana program deploy target/deploy/donft.so
