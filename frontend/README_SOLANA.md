# DoNFT MVP for SOLANA

### Research about NFTs on Solana ###

Solana, nft/ft program calling spt-token, which do not contain any metadata (uri, title...)
https://spl.solana.com/token

Metaplex - program build on top of Solana, which extending functionality of spt-token program
https://docs.metaplex.com/

At 28.03.2022, some of Frontend Metaplex libraries still unstable, and in development mode
so, some of functions may not work

### Helpful Docs/API links ###

### FRONTEND:

METAPLEX/JS ---> frontend library for interacting with metaplex smart contracts (mint, burn, transfer, retrieving data)
https://metaplex-foundation.github.io/js/

SOLANA/WEB3.JS ---> same purpose as with metaplex/js, although more complicated, as metaplex/js build on top of solana/web3js
https://solana-labs.github.io/solana-web3.js/

SOLANA-WALLETS-VUE && @solana/wallet-adapter-wallets ---> helpful plugins for interacting with different Solana wallets, support 10+ wallets,
possible, to replace it with @solana/web3.js plugin, for optimizing packages
https://solana-labs.github.io/wallet-adapter/
https://github.com/lorisleiva/solana-wallets-vue

### BLOCKCHAIN:
METAPLEX Smart Contracts --> different contracts on top of Solana (minting, transfer, burning, metadata for NFT (image, title)...)
https://github.com/metaplex-foundation/metaplex-program-library

ANCHOR ---> Solana framework, which gonna help with developing of smart contract on Solana,
and serializing/deserializing of data in frontend, and some other stuff
https://project-serum.github.io/anchor/getting-started/introduction.html


### ALL INFO ABOUT SOLANA
https://wiki.nikitavoloboev.xyz/databases/blockchain/solana

### Website with tutorials on Solana
### This one clearly comparing with Ethereum NFTs tech stack, and show differences
https://app.buildspace.so/projects/CO77556be5-25e9-49dd-a799-91a2fc29520e/lessons/LE0f8a7abb-1a70-434d-a0d2-b7452d7638e3

NFT Lootbox on Solana, here is result of this tutorial:
https://solana-nft-lootbox.vercel.app/

### Deep explanation of main Metaplex contracts
https://medium.com/metaplex/metaplex-architecture-b94a64c37130

### Solana Transactions in depth
https://medium.com/@asmiller1989/solana-transactions-in-depth-1f7f7fe06ac2
