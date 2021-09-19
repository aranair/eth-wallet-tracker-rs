# ETH Wallet Tracker

Tracks transactions for a watchlist of wallets

### How to use?

Create a `watchlist.json` with the following format

```
{
  "watchlist": [
    {
      "name": "some identifier of address",
      "address": "0x0000000000000000000000000000"
    },
    ...
  ]
}
```

Get a infura address

```
cargo run -- \
  --url="wss://mainnet.infura.io/ws/v3/xxxxxxxxxxxxxxxxxx"
```
