# DoNFT &mdash; Solana program
The program provides a facility to create bundles.
Bundle is an account which remembers a SuperNFT's mint, and a list of accounts to store
"bundled" nfts.

Key NFT should:
- Belong to a tx signer
- Should have a mint of 1 item and have no mint authority (so that the owner of super NFT can not print another one after selling the original)

## user\_data parameters
- [1 byte] {0 = lock, 1 = unlock}
- [1 byte] bump

## accounts
### Lock
1. Signer's account, writable, signer
2. SuperNFT's mint account, readable
3. SuperNFT's user's account, readable (as it won't be trasferred). Should be owned by TOKEN\_PROGRAM, be of SuperNFT, and contain 1 item.
4. Storage for this bundle - should be owned by this program
5. Token Program
6-7, 8-9, ...: pairs of accounts with NFTs: first is source NFT account, second is destination NFT account. Destination NFT account should belong to PDA (PDA=SuperNFT mint)

### Unlock
1. Signer's account, writable, signer
2. Storage for this bundle - should be owned by this program
3. SuperNFT's user's account, readable (as it won't be trasferred). Should be owned by TOKEN\_PROGRAM, be of SuperNFT, and contain 1 item.
4. Token Program
5-6, 7-8, ...: pairs of accounts with NFTs: forst is source NFT account, which should also be the same as in storage. Destination is a client's account for this NFT
