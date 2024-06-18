# DoNFT MVP for Solana Frontend

Functions:
- connect Solana wallet
- showing NFTs of this wallet
- Minting new NFT
- Sending NFT
- Bundling NFTs
- Bundling NFT with Effect
- Unbundling NFTs

## Frontend starting guide:

### Node:
Version while dev 15.14.0

### Start frontend
1. Change or copy .env.development.local to .env
2. yarn install
3. yarn serve
4. finally check localhost:8080


## Deploying NFT smart-contract to near guide:

### Case: testing on nft-example.near_testing.testnet

1. near login (if unavailable -> npm install -g near-cli)
2. Check your .env -> VUE_APP_NFT_STORAGE_API_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDA5QWEwMEQxNjNFZTk0MDM1RGIwM0I3ODg2NmMyRWUzZTBjMjEzQkIiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY1MDYzMzQ3MDIzNCwibmFtZSI6IkRvTkZUIn0.3uUl6Ia6KuBpRjAa3-vVKikvc0BRcEZK6Ize5Uu98QU
3. yarn dev:deploy:contract

### Case: new Smart-contract project

1. near login
2. near create-account nft-example.your-account.testnet --masterAccount your-account.testnet --initialBalance 10
3. Create vars:
- NFT_CONTRACT_ID=nft-example.your-account.testnet
- MAIN_ACCOUNT=your-account.testnet
4. near deploy --accountId $NFT_CONTRACT_ID --wasmFile out/main.wasm
5. near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID

## docker

### build

    docker build --rm --no-cache -t donft_frontend:latest .

### run

    docker run --rm  -p 8080:80  --name donft_frontend donft_frontend:latest