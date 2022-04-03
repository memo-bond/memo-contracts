net-work ?= testnet

console:
	@terrain console

new-contract:
	@terrain code:new $(contract-name)

store-code:
	@terrain code:store $(contract-name) --signer test_account --network $(net-work)

init:
	@terrain contract:instantiate $(contract-name) --signer test_account --network $(net-work)

.PHONY: store-code new-contract init