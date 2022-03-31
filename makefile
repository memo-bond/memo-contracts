build:
	@cargo wasm

build-optimize:
	@cargo run-script optimize

generate-schema:
	@cargo schema

upload:
	@make build-optimize
	@terrad tx wasm store artifacts/memo_contract.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block

.PHONY: build build-optimize