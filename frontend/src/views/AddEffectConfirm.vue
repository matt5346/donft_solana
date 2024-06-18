<template>
  <div class="page">
    <nav-bar :navigation="getNav"/>
    <div v-if="!getNFTsLoadStatus" class="loading-container">
      <spinner :size="92" color="#000" />
      <h2>{{ statusText }}</h2>
    </div>
    <main v-else>
      <div class="effect-confirm">
        <div class="effect-confirm__selected">
          <h2>Selected NFT</h2>
          <div
            class="nft-cards-box"
            v-if="NFTComputedData"
          >
            <token-card
              :get-info="true"
              :metadata="NFTComputedData"
              :edit-available="false"
            />
          </div>
        </div>
        <div class="effect-confirm__selected">
          <h2>NFT effects</h2>

          <token-card
            v-if="getEffect"
            :metadata="getEffect"
            :edit-available="false"
          />
        </div>
        <div class="form-nft-send__inputs form-nft-send__inputs--effects">
          <h2>Bundle metadata</h2>
          <div class="effect-form-wrap">
            <span class="form-nft-send__inputs-title">Title</span>
            <input
              type="text"
              placeholder="NFT title"
              class="input form-nft__input"
              v-model="nftObj.name"
            >
            <div class="form-nft__bottom">
              <button
                class="main-btn"
                @click="handleMint"
              >Confirm</button>
            </div>
          </div>
        </div>
      </div>

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
              StatusType.Applying,
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
import { actions } from "@metaplex/js";
import { computed, onMounted, reactive, ref } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import Spinner from "@/components/Spinner";
import TokenCard from "@/components/TokenCard/TokenCard";
import NavBar from "@/components/NavBar/NavBar";
import ModalTemplate from "@/components/ModalTemplate/ModalTemplate";

import { notify } from "@kyvg/vue3-notification";
import statusMixin from "@/mixins/StatusMixin";

import { AppError, SystemErrors } from "@/utilities";
import { applyNFTsEffect } from "@/api";
import { Account, PublicKey, SystemProgram, Keypair, TransactionInstruction, Transaction } from "@solana/web3.js";
import { AccountLayout, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";

const store = useStore();
const router = useRouter();
const { StatusType } = statusMixin();
let showApproveModal = ref(false);

const CONTRACT_PROGRAM_ID = new PublicKey(
  "DyPhsdovhyrTktsJS6nrghGvkRoo2FK3ztH1sKKPBDU4",
);

const uintSeed = Uint8Array.from([138,133,11,131,247,141,131,185,159,96,109,107,180,236,20,176,63,41,69,76,179,63,201,132,193,76,220,28,143,52,254,215,31,128,60,52,52,212,51,196,74,36,28,61,13,2,210,174,164,102,234,182,74,120,227,153,67,193,173,126,14,38,102,210]);

let nftObj = reactive({
  name: "",
  symbol: "bundle",
  seller_fee_basis_points: 0,
  description: "NFT token 2 description",
  image: "",
  isMutable: 0,
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

const getNav = [
  {
    text: "Back to Gallery",
    name: "ChooseNFT",
    params: null,
  },
];

let blobImg = null;

const getStatus = computed(() => store.getters.getStatus);
const getNFTsLoadStatus = computed(() => store.getters.getNFTsLoadStatus);
const getEffect = computed(() => store.getters.getEffect);

const NFTComputedData = computed({
  get() {
    if (store.getters.getAllNFTs && store.getters.getAllNFTs.length) {
      return store.getters.getAllNFTs.find((item) => item.mint === router.currentRoute.value.params.id);
    }

    return null;
  },
});

onMounted(() => {
  store.commit("SET_EFFECT_CHOICE", router.currentRoute.value.params.effectId);
  // creating nft, require wallet key of creator
  const defaultCreator = {
    "address": store.getters.getSolanaWalletInstance.publicKey.toString(),
    "share": 100
  };
  nftObj.properties.creators.push(defaultCreator);
});

const handleMint = async () => {
  if (!nftObj.name) {
    alert("Title field is empty");
  } else {
    showApproveModal.value = true;
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    const tokenData1 = await store.dispatch("setTokenImage", { token: NFTComputedData.value.data, getIPFSurl: true });
    const tokenData2 = await store.dispatch("setTokenImage", { token: getEffect.value.data, getIPFSurl: true });
    console.log(tokenData1, "tokenData1 CID");
    console.log(tokenData2, "tokenData2 CID");

    const effectObj = {
      original: {
        contract: fromWallet.publicKey,
        tokenId: NFTComputedData.value.mint,
        contentUrl: tokenData1.image,
      },
      modificator: {
        contract: fromWallet.publicKey,
        tokenId: getEffect.value.mint,
        contentUrl: tokenData2.image,
      },
      sender: fromWallet.publicKey,
    };

    // creating new Effect
    try {
      store.dispatch("setStatus", StatusType.Applying);
      const cidData = await applyNFTsEffect(effectObj);
      console.log(cidData, "CID");
      nftObj.image = cidData.cid;
      blobImg = cidData.hashBlob;
    } catch(err) {
      console.log(err);
      if (err instanceof AppError) {
        throw err; 
      } else {
        throw SystemErrors.NFT_EFFECT_CONFIRM;
      }
    }

    try {
      // is Random, for skipping image ipfs deploy, cause we already have img
      await store.dispatch("setDeployToIPFS", { isImageDeployed: true, meta: nftObj });
    } catch(err) {
      if (err instanceof AppError) {
        throw err; 
      } else {
        throw SystemErrors.IPFS_SAVE;
      }
    }

    try {
      console.log("CREATING");

      const keyWallet = new PublicKey(fromWallet.publicKey.toString());

      const mainToken = new PublicKey(NFTComputedData.value.mint);
      const effectToken = new PublicKey(getEffect.value.mint);
      console.log(keyWallet, "keyWallet");

      // creating Token Account for mainToken
      const mainNFTtokenAccount = await PublicKey.findProgramAddress(
        [
          fromWallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          mainToken.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      // creating Token Account for effectToken
      const effectNFTtokenAccount = await PublicKey.findProgramAddress(
        [
          fromWallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          effectToken.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      console.log(mainNFTtokenAccount[0].toString(), "BundleTokenAccount1");

      // getting token Account mint addres for both tokens
      const tokenMintAccountForMainNFT = new PublicKey((await connection.getParsedAccountInfo(mainNFTtokenAccount[0], "devnet")).value.data.parsed.info.mint);
      const tokenMintAccountForEffect = new PublicKey((await connection.getParsedAccountInfo(effectNFTtokenAccount[0], "devnet")).value.data.parsed.info.mint);

      // just an empty account instance
      const tokenAccountInstanceForMainNFT = new Account();
      const tokenAccountInstanceForEffect = new Account();

      // TOKENS STORAGE
      // here is accounts, to which we gonna send bundled NFT tokens
      // for first token
      const tempTokenAccForMainNFT = SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID,
        space: AccountLayout.span,
        lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span, "singleGossip"),
        fromPubkey: keyWallet,
        newAccountPubkey: tokenAccountInstanceForMainNFT.publicKey
      });

      // for second token
      const tempTokenAccForEffectNFT = SystemProgram.createAccount({
        programId: TOKEN_PROGRAM_ID,
        space: AccountLayout.span,
        lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span, "singleGossip"),
        fromPubkey: keyWallet,
        newAccountPubkey: tokenAccountInstanceForEffect.publicKey
      });


      store.dispatch("setStatus", StatusType.DeployingToIPFS);
      await store.dispatch("setDeployToIPFS", { isImageDeployed: true, meta: nftObj });
      
      // if need some test data
      //"https://ipfs.io/ipfs/Qmb8yTr9CRhFzTwasPCgXAtLHrfhmNw6i4raJZHr82hzUs"

      console.log(store.getters.getNFTdeployResult, "CREATING");

      // minting MAIN Bundle NFT
      const signature = await actions.mintNFT({
        connection,
        wallet: fromWallet,
        uri: store.getters.getNFTdeployResult,
        maxSupply: 1,
      });

      store.dispatch("setStatus", StatusType.Minting);
      await connection.confirmTransaction(signature.txId, "finalized");


      store.dispatch("setStatus", StatusType.Approving);

      // getting Token Account for MAIN Bundle NFT, with owner of CONTRACT_PROGRAM_ID
      const bundleStorageTokenAccountProgram = await PublicKey.findProgramAddress(
        [
          signature.mint.toBuffer(),
        ],
        CONTRACT_PROGRAM_ID
      );

      console.log(uintSeed, "uint");
      const bundleStorageAccount = Keypair.fromSecretKey(uintSeed);

      const seed = signature.mint.toString().substr(0, 20);
      console.log(seed, "----seed----");
      // creating Account with seed, for getting INNER NFT of Bundle
      const transferAcc = await PublicKey.createWithSeed(
        bundleStorageAccount.publicKey,
        seed,
        CONTRACT_PROGRAM_ID,
      );

      // BUNDLE STORAGE
      // creating transaction for account with seed
      const bundleStorageTokenAccountIx = SystemProgram.createAccountWithSeed({
        basePubkey: bundleStorageAccount.publicKey,
        fromPubkey: keyWallet,
        lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span, "singleGossip"),
        newAccountPubkey: transferAcc,
        programId: CONTRACT_PROGRAM_ID,
        seed,
        space: 101,
      });

      // creating instruction for 1st NFT Token Account
      const programAccForMainNFT = Token.createInitAccountInstruction(
        TOKEN_PROGRAM_ID,
        tokenMintAccountForMainNFT,
        tokenAccountInstanceForMainNFT.publicKey,
        bundleStorageTokenAccountProgram[0]
      );

      // creating instruction for 2nd NFT Token Account
      const programAccForEffect = Token.createInitAccountInstruction(
        TOKEN_PROGRAM_ID,
        tokenMintAccountForEffect,
        tokenAccountInstanceForEffect.publicKey,
        bundleStorageTokenAccountProgram[0]
      );
      console.log(signature, "signature mint");
      console.log(signature.mint.toString(), "signature mint");

      // getting Token Account for MAIN Bundle NFT, with owner of ASSOCIATED_TOKEN_PROGRAM_ID
      const superNFTTokenAccount = await PublicKey.findProgramAddress(
        [
          fromWallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          signature.mint.toBuffer()
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      // finally, collecting all keys, for Solana program (smart-contract)
      const keys =  [
        { pubkey: keyWallet, isSigner: true, isWritable: false },
        { pubkey: signature.mint, isSigner: false, isWritable: false },
        { pubkey: superNFTTokenAccount[0], isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: bundleStorageTokenAccountIx.keys[1].pubkey, isSigner: false, isWritable: true },
        { pubkey: mainNFTtokenAccount[0], isSigner: false, isWritable: true },
        { pubkey: programAccForMainNFT.keys[0].pubkey, isSigner: false, isWritable: true },
        { pubkey: effectNFTtokenAccount[0], isSigner: false, isWritable: true },
        { pubkey: programAccForEffect.keys[0].pubkey, isSigner: false, isWritable: true },
      ];

      // instruction data define WHAT WE DO
      // bundle_instruction_data = 0 (BUNDLE NFT)
      // bundle_instruction_data = 1 (UNBUNDLE NFT)
      // bundleStorageTokenAccountProgram fetched from FindProgramAddress
      // data: [0, 254]
      const bundle_instruction_data = [0, bundleStorageTokenAccountProgram[1]];

      console.log(bundle_instruction_data, "bundle_instruction_data.mint");

      const initEscrowIx = new TransactionInstruction({
        programId: CONTRACT_PROGRAM_ID,
        keys,
        data: Buffer.from(bundle_instruction_data)
      });

      const latestBlockHash = await connection.getLatestBlockhash();

      // every transaction which creating smth
      // require full account with public key and private key
      // as createTempTokenAccount1/programs_account_for_mint1 require tempTokenAccount1
      const tx = new Transaction({
        feePayer: keyWallet,
        recentBlockhash: latestBlockHash.blockhash,
      });

      tx.add(
        tempTokenAccForMainNFT,
        tempTokenAccForEffectNFT,
        programAccForMainNFT,
        programAccForEffect,
        bundleStorageTokenAccountIx,
        initEscrowIx,
      );
      console.log("tx 1", tx);

      // signing tempTokenAccountInstruction, programsAccountForMint and bundleStorageTokenAccountIx
      tx.sign(tokenAccountInstanceForMainNFT, tokenAccountInstanceForEffect, bundleStorageAccount);
      // also we should sign with Phantom wallet, it gonna ask user to approve
      await fromWallet.signTransaction(tx);
      // sending all of it as bytes, and confirm then
      const accountsCreateTx = await connection.sendRawTransaction(tx.serialize());
      const accountsCreateTxResponse = await connection.confirmTransaction(accountsCreateTx, "processed");
      console.log("signature 1", accountsCreateTx);
      console.log("response", accountsCreateTxResponse);
      store.dispatch("setStatus", StatusType.Minting);

      // if response contain EMPTY ERROR, its Successed
      if (accountsCreateTxResponse.value && accountsCreateTxResponse.value.err === null) {
        store.dispatch("setStatus", StatusType.ChoosingParameters);
        showApproveModal.value = false;

        // adding bundle obj, to display it without waiting ipfs
        const bundleObj = {
          mint: signature.mint.toString(),
          data: {
            ...nftObj,
            image: blobImg
          }
        };

        // updating NFTS list in store
        store.commit("ADD_MINTED_NFT", bundleObj);
        store.commit("REMOVE_FROM_NFT_LIST", mainToken.toString());
        store.commit("REMOVE_FROM_NFT_LIST", effectToken.toString());


        router.push({ name: "ChooseNFT"});
        notify({
          title: "Transaction status",
          type: "success",
          text: "NFT successfully Minted!",
          duration: 6000,
        });
      }

    } catch(err) {
      console.log(err, "error");
      console.log(err, "ERROR BUNDLE");
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      showApproveModal.value = false;

      if(err instanceof AppError) {
        notify({
          title: "Error",
          type: "error",
          text: err,
          duration: 6000,
        });
      } else {
        notify({
          title: "Error",
          type: "error",
          text: "Undefined error",
          duration: 6000,
        });
      }
    }
  }
};

const getStatusText = (status) => {
  const statusData = statusMixin(status);
  console.log(statusData, "statusData");

  return statusData.statusText;
};
</script>
