# illustrate

This class mainly implements a simulated oracle contract for testing.
The implementation method is: store a price mapping table in the contract, update the price through the `update_price_feed` method, and query the price through the `price_feed` method.

# compile

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

# deploy

```bash
seid tx wasm store artifacts/mock_oracle.wasm -y --from=admin \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ \
--gas=1225925 --gas-prices=0.01usei --broadcast-mode=block
```

# Instantiate

```bash
seid tx wasm instantiate 750 '{}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block --label "test" \
--admin sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz
```

Return contract address: `sei1f6ctrws0eqha3nn7gxetj7h95wlmjrslwqgs9ecumf4cs00wktfstel0m9`

# Configuration and query

Set price

ps:`id` must be a hexadecimal string

```bash
seid tx wasm execute sei1f6ctrws0eqha3nn7gxetj7h95wlmjrslwqgs9ecumf4cs00wktfstel0m9 \
'{"update_price_feed":{"id":"5bc91f13e412c07599167bae86f07543f076a638962b8d6017ec19dab4a82814","price":189000000000}}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block
```

price checking

```bash
seid query wasm contract-state smart sei1f6ctrws0eqha3nn7gxetj7h95wlmjrslwqgs9ecumf4cs00wktfstel0m9 \
	'{"price_feed":{"id":"5bc91f13e412c07599167bae86f07543f076a638962b8d6017ec19dab4a82814"}}' \
	--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --output json
```
