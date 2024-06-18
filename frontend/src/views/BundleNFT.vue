<template>
  <div class="page">
    <nav-bar :navigation="getNavigation"/>
    <!-- <div
      v-if="[
        StatusType.Applying,
        StatusType.DeployingToIPFS,
        StatusType.DeployedToIPFS,
        StatusType.Minting
      ].includes(getStatus)"
      class="loading-container"
    >
      <spinner :size="92" color="#000" />
      <h1>{{ statusText }}</h1>
    </div> -->
    <main>
      <h1>Selected NFTs</h1>
      <div
        class="nft-cards"
        v-if="getNFTsData && getNFTsData.length"
      >
        <div
          v-for="item in getNFTsData"
          :key="item.mint"
          class="nft-cards__contract__item nft-cards__contract__item--bundle-data"
        >
          <token-card
            :metadata="item"
            :edit-available="false"
          />
        </div>
      </div>

      <form class="form-nft">
        <uploader-comp
          @selected="setUploadedImg"
        />
        <div class="form-ntf__inputs">
          <input
            type="text"
            placeholder="NFT title"
            class="input form-nft__input"
            v-model="bundleObj.name"
          >
          <button
            class="main-btn"
            type="submit"
            @click.prevent="bundleNFTs"
          >Bundle NFTs!</button>
        </div>
      </form>


      <modal-template
        v-if="showApproveModal"
        :is-blocked="true"
        @close="closeModal"
      >
        <template #header>
          <h3>Status of transaction</h3>
        </template>
        <template #content>
          <div
            v-if="[
              StatusType.Approving,
              StatusType.Sending,
              StatusType.Minting,
              StatusType.DeployingToIPFS,
            ].includes(getStatus)" class="loading-container"
          >
            <spinner :size="92" color="#000" />
            <h2>{{ getStatusText(getStatus) }}</h2>
          </div>
        </template>
      </modal-template>
    </main>
  </div>
</template>

<script setup>
import {
  actions,
} from "@metaplex/js";
import { reactive, computed, onMounted, ref } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { notify } from "@kyvg/vue3-notification";
import { Account, PublicKey, SystemProgram, Keypair, TransactionInstruction, Transaction } from "@solana/web3.js";
import { AccountLayout, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import statusMixin from "@/mixins/StatusMixin";

import NavBar from "@/components/NavBar/NavBar";
import TokenCard from "@/components/TokenCard/TokenCard";
import UploaderComp from "@/components/Uploader/UploaderComp";
import Spinner from "@/components/Spinner";
import ModalTemplate from "@/components/ModalTemplate/ModalTemplate";

const CONTRACT_PROGRAM_ID = new PublicKey(
  "DyPhsdovhyrTktsJS6nrghGvkRoo2FK3ztH1sKKPBDU4",
);

const bundleStorageAccount = Keypair.fromSecretKey(Uint8Array.from([138,133,11,131,247,141,131,185,159,96,109,107,180,236,20,176,63,41,69,76,179,63,201,132,193,76,220,28,143,52,254,215,31,128,60,52,52,212,51,196,74,36,28,61,13,2,210,174,164,102,234,182,74,120,227,153,67,193,173,126,14,38,102,210]));

const { StatusType } = statusMixin();
const store = useStore();
const router = useRouter();
let showApproveModal = ref(false);

const bundleObj = reactive({
  name: "NFT token 2 title",
  symbol: "bundle",
  seller_fee_basis_points: 0,
  external_url: "",
  description: "NFT token 2 description",
  image: "",
  properties: {
    files: [
      {
        uri: "",
        type: "image/*"
      }
    ],
    category: "image",
    creators: []
  },
  collection: null,
  use: null
});

const getStatus = computed(() => store.getters.getStatus);

const getNavigation = [{
  text: "Back to Gallery",
  name: "ChooseNFT",
}];

const getNFTsData = computed({
  get() {
    if (store.getters.getAllNFTs && store.getters.getAllNFTs.length) {
      return store.getters.getNFTchoice.map((urlToken) => {
        const item = store.getters.getAllNFTs.find((nftObjData) => nftObjData.mint === urlToken);
        return item;
      }).filter(Boolean);
    }
    return [];
  },
});

onMounted(async ()=> {
  const test = await PublicKey.findProgramAddress(
    [
      store.getters.getSolanaWalletInstance.publicKey.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      new PublicKey("4syogkhaM4kvLEe2TjwbXRSdhw43YC2aGTACWQdfajWj").toBuffer(),
    ],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  // creating nft, require wallet key of creator
  const defaultCreator = {
    "address": store.getters.getSolanaWalletInstance.publicKey.toString(),
    "share": 100
  };
  bundleObj.properties.creators.push(defaultCreator);
  console.log(test[0].toString(), "mounted");
});

// "https://ipfs.io/ipfs/QmVUguweEjtDThd8ztRY4B2twczFZ7Q7DCXrTypJDuz44g"

const bundleNFTs = async () => {
  console.log("BUNDLE");
  try {
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    const keyWallet = fromWallet.publicKey;
    const tokensMintKeys = [];
    showApproveModal.value = true;

    store.getters.getNFTchoice.forEach((item) => {
      tokensMintKeys.push(new PublicKey(item));
    });

    // BundleTokenAccounts (1, 2, 3 ...)
    const bundleTokenAccountForMints = await Promise.all(tokensMintKeys.map(async (item) => {
      const token = await PublicKey.findProgramAddress(
        [
          fromWallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          item.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      return token;
    }));

    // requestin mint ID for every Token Account of existing NFT, which gonna be bundled later
    const TokenAccountParsedMint = await Promise.all(bundleTokenAccountForMints.map(async (item) => {
      const token = new PublicKey((await store.getters.getSolanaInstance.getParsedAccountInfo(item[0], "devnet")).value.data.parsed.info.mint);
      return token;
    }));
    
    const tempTokenAccounts = [];

    for (let i = 0; i < TokenAccountParsedMint.length; i++) {
      tempTokenAccounts.push(new Account);
    }

    // TOKENS STORAGE
    // here are accounts, to which we gonna send bundled NFT tokens
    const tempTokenAccountInstruction = await Promise.all(tempTokenAccounts.map(async (item, index) => {
      const instruction = SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID,
        space: AccountLayout.span,
        lamports: await store.getters.getSolanaInstance.getMinimumBalanceForRentExemption(AccountLayout.span, "singleGossip"),
        fromPubkey: keyWallet,
        newAccountPubkey: tempTokenAccounts[index].publicKey
      });

      return instruction;
    }));

    // TOKENS STORAGE
    store.dispatch("setStatus", StatusType.DeployingToIPFS);

    await store.dispatch("setDeployToIPFS", { isImageDeployed: false, meta: bundleObj });
    console.log(store.getters.getNFTdeployResult, "CREATING");

    store.dispatch("setStatus", StatusType.Approving);

    // Block of minting BUNDLE NFT
    const signature = await actions.mintNFT({
      connection,
      wallet: store.getters.getSolanaWalletInstance,
      uri: store.getters.getNFTdeployResult,
      maxSupply: 1,
    });

    store.dispatch("setStatus", StatusType.Minting);

    await connection.confirmTransaction(signature.txId, "finalized");

    // token account of bundle nft
    const bundleStorageTokenAccountProgram = await PublicKey.findProgramAddress(
      [
        signature.mint.toBuffer(),
      ],
      CONTRACT_PROGRAM_ID
    );

    // creating account with seed
    // seed gonna help us to find bundle inner NFT later
    const seed = signature.mint.toString().substr(0, 20);

    const transferAcc = await PublicKey.createWithSeed(
      bundleStorageAccount.publicKey,
      seed,
      CONTRACT_PROGRAM_ID,
    );

    store.dispatch("setStatus", StatusType.Approving);

    // todo: SET AUTHORITY for BUNDLE NFT, isMutable property in metadata of token
    // its need for making BUNDLE NFT unmutable, or user will have a chance to change BUNDLE NFT

    // const bundleMintAuthority = new PublicKey((await getSolanaInstance.value.getParsedAccountInfo(signature.mint, "devnet")).value.data.parsed.info.mintAuthority);
    // const setBundleAuthorityTransaction = Token.createSetAuthorityInstruction(
    //   TOKEN_PROGRAM_ID,
    //   bundleStorageTokenAccountProgram[0],
    //   null,
    //   "MintTokens",
    //   bundleMintAuthority,
    //   [],
    // );
    // console.log(setBundleAuthorityTransaction, "setBundleAuthority");

    // BUNDLE STORAGE

    const bundleStorageTokenAccountIx = SystemProgram.createAccountWithSeed({
      basePubkey: bundleStorageAccount.publicKey,
      fromPubkey: keyWallet,
      lamports: await store.getters.getSolanaInstance.getMinimumBalanceForRentExemption(AccountLayout.span, "singleGossip"),
      newAccountPubkey: transferAcc,
      programId: CONTRACT_PROGRAM_ID,
      seed,
      space: 1 + 32 + 4 + (32 * TokenAccountParsedMint.length),
    });


    // creating program accounts for every inner NFT of bundle
    const programsAccountForMint = await Promise.all(tempTokenAccounts.map(async (item, index) => {
      const instruction = Token.createInitAccountInstruction(
        TOKEN_PROGRAM_ID,
        TokenAccountParsedMint[index],
        tempTokenAccounts[index].publicKey,
        bundleStorageTokenAccountProgram[0]
      );

      return instruction;
    }));


    // here is Token Account for Bundle Front NFT, which gonna contain NFTs
    const superNFTTokenAccount = await PublicKey.findProgramAddress(
      [
        fromWallet.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        signature.mint.toBuffer()
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const bundleTokensSorted = [];

    // sorting tokens which gonna be BUNDLED
    // order is very important, transaction will fail if its wrong
    bundleTokenAccountForMints.forEach((tokenAcc, index) => {
      const nftTokenAcc = {
        pubkey: tokenAcc[0],
        isSigner: false,
        isWritable: true,
      };

      const pdaForMint = {
        pubkey: programsAccountForMint[index].keys[0].pubkey,
        isSigner: false,
        isWritable: true,
      };

      bundleTokensSorted.push(nftTokenAcc);
      bundleTokensSorted.push(pdaForMint);
    });

    console.log(bundleTokensSorted, "bundleTokensSorted------");


    const latestBlockHash1 = await connection.getLatestBlockhash();

    // Init of main transactions, which gonna prepare for us empty accounts on Solana chain
    // after initing this account, we can init main Transaction, which gonna bundle NFTs
    // also there is 1295 bytes size Transaction restriction, because of this
    // we had to separate initBundleTx Transaction from others, or size gonna be larger than 1295 bytes
    const tx = new Transaction({
      feePayer: keyWallet,
      recentBlockhash: latestBlockHash1.blockhash,
    });

    tx.add(
      ...tempTokenAccountInstruction,
      ...programsAccountForMint,
      bundleStorageTokenAccountIx,
    );
    
    // signing tempTokenAccountInstruction, programsAccountForMint and bundleStorageTokenAccountIx
    tx.sign(...tempTokenAccounts, bundleStorageAccount);
    
    // also we should sign with Phantom wallet, it gonna ask user to approve
    await store.getters.getSolanaWalletInstance.signTransaction(tx);
  
    // sending all of it as bytes, and confirm then
    const accountsCreateTx = await connection.sendRawTransaction(tx.serialize());
    await connection.confirmTransaction(accountsCreateTx, "processed");

    // creating Transaction keys for initBundleTx
    const keys =  [
      { pubkey: keyWallet, isSigner: true, isWritable: false },
      { pubkey: signature.mint, isSigner: false, isWritable: false },
      { pubkey: superNFTTokenAccount[0], isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: bundleStorageTokenAccountIx.keys[1].pubkey, isSigner: false, isWritable: true },
      ...bundleTokensSorted
    ];

    console.log(keys, "keys------");
    const bundle_instruction_data = [0, bundleStorageTokenAccountProgram[1]];

    // data: [0, 254]
    console.log(bundle_instruction_data, "bundle_instruction_data.mint");

    // passing all data to Instruction, which gonna bundle all NFTs
    const initBundleTx = new TransactionInstruction({
      programId: CONTRACT_PROGRAM_ID,
      keys,
      data: Buffer.from(bundle_instruction_data)
    });

    const latestBlockHash2 = await connection.getLatestBlockhash();
    const tx2 = new Transaction({
      feePayer: keyWallet,
      recentBlockhash: latestBlockHash2.blockhash,
    }).add(
      initBundleTx,
    );

    await store.getters.getSolanaWalletInstance.signTransaction(tx2);
    store.dispatch("setStatus", StatusType.Sending);

    const sendBundleTx = await connection.sendRawTransaction(tx2.serialize());

    // awaiting result, if no error, its completed
    const sendBundleTxResponse = await connection.confirmTransaction(sendBundleTx, "processed");
    console.log("sendBundleTxResponse", sendBundleTxResponse);

    if (sendBundleTxResponse.value && sendBundleTxResponse.value.err === null) {
      store.dispatch("setAllSolanaNFts");
      router.push({ name: "ChooseNFT"});
      showApproveModal.value = false;

      notify({
        title: "Transaction status",
        type: "success",
        text: "NFT successfully Minted!",
        duration: 6000,
      });
    }
    store.dispatch("setStatus", StatusType.ChoosingParameters);
  } catch(err) {
    console.log(err, "ERROR BUNDLE");
    showApproveModal.value = false;
    store.dispatch("setStatus", StatusType.ChoosingParameters);
    notify({
      title: "Transaction status",
      type: "error",
      text: `Something wrong, Error: ${err}`,
      duration: 6000,
    });
  }
};

const setUploadedImg = (src) => {
  bundleObj.image = src; 
};

const getStatusText = (status) => {
  const statusData = statusMixin(status);
  console.log(statusData, "statusData");

  return statusData.statusText;
};
</script>