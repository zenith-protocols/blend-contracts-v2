default: build

test: build
	cargo test --all --tests

build:
	stellar contract build --manifest-path=pool-factory/Cargo.toml
	stellar contract build --manifest-path=backstop/Cargo.toml
	stellar contract build --manifest-path=pool/Cargo.toml
	
	mkdir -p target/wasm32v1-none/optimized
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/pool_factory.wasm \
		--wasm-out target/wasm32v1-none/optimized/pool_factory.wasm
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/backstop.wasm \
		--wasm-out target/wasm32v1-none/optimized/backstop.wasm
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/pool.wasm \
		--wasm-out target/wasm32v1-none/optimized/pool.wasm
	cd target/wasm32v1-none/optimized/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

fmt:
	cargo fmt --all

clean:
	cargo clean

generate-js:
	stellar contract bindings typescript --overwrite \
		--contract-id CBWH54OKUK6U2J2A4J2REJEYB625NEFCHISWXLOPR2D2D6FTN63TJTWN \
		--wasm ./target/wasm32v1-none/optimized/backstop.wasm --output-dir ./js/js-backstop/ \
		--rpc-url http://localhost:8000 --network-passphrase "Standalone Network ; February 2017" --network Standalone
	stellar contract bindings typescript --overwrite \
		--contract-id CBWH54OKUK6U2J2A4J2REJEYB625NEFCHISWXLOPR2D2D6FTN63TJTWN \
		--wasm ./target/wasm32v1-none/optimized/pool_factory.wasm --output-dir ./js/js-pool-factory/ \
		--rpc-url http://localhost:8000 --network-passphrase "Standalone Network ; February 2017" --network Standalone
	stellar contract bindings typescript --overwrite \
		--contract-id CBWH54OKUK6U2J2A4J2REJEYB625NEFCHISWXLOPR2D2D6FTN63TJTWN \
		--wasm ./target/wasm32v1-none/optimized/pool.wasm --output-dir ./js/js-pool/ \
		--rpc-url http://localhost:8000 --network-passphrase "Standalone Network ; February 2017" --network Standalone
