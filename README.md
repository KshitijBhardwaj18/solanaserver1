# solana-rust-axum-backend
This is a webserver written in Rust to provide APIs to communicate with the Solana Program (Smart contract).

# Prerequisites
1. This server uses the [Axum](https://docs.rs/axum/latest/axum/) framework to handle HTTP requests. A basic foundation is necessary to understand and modify them.

2. Some basic knowledge of the Solana blockchain and how to interact with it. I have created a basic [example project](https://github.com/bunnyBites/learn-solana-with-rust) in Rust which can help you practically understand them.

3. You may need to create environment variables for your server to run. This is the sample env configuration I have used in this project. You have to create your environment variables.

```sh
MY_PUB_KEY=JBFs2brb5KrFH6D9HqX2YF5JigySowHPN79gBSaoxKrR

OTHER_PUB_KEY=4uyGibipBg7E1Svsai4fg1BTaBWBMRTtxJKX7agFAdLe

MY_SECRET_KEY=28rUCoiLSWGdH6ToJU7tzrjS6oU5u9MbeRJhBh7HN4h26E1w8uZCigJVArNssFEnH3gzak2DSiYQfBWaHyN5FzGu
```

Here MY_PUB_KEY and MY_SECRET_KEY are the public key and secret key of the sender account. OTHER_PUB_KEY is not used within our server but it's just the public of the receiver account (to which will be sending SOLS).

You can create a new account or wallet as shown in the official [docs](https://solanacookbook.com/references/keypairs-and-wallets.html). You can also refer to my example to create a local wallet and get familiar with the functionality [here](https://github.com/bunnyBites/learn-solana-with-rust/blob/patch-1/README.md).

# Available API endpoints:
You can find the API collection(JSON) for thunder client in this project within thunder-collection_solana-blockchain.json

1. Get 5 SOLS - /getSols
2. Transfer SOLS - /transferSols
3. Get available SOLS from our account - /getBalance

# Running the server

```bash
cargo run
```