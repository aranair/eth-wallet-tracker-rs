# ETH Wallet Tracker

Tracks transactions for a watchlist of wallets

### How to use?

Create a `watchlist.json` with the following format

```
{
  "watchlist": [
    {
      "name": "some identifier of address",
      "address": "0x9c5111dd4838e120dbbbbbbccc2179692aaaaaaa"
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
