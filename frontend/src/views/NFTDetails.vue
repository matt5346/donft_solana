<template>
  <div class="page">
    <nav-bar :navigation="getNav"/>
    <div
      v-if="!NFTComputedData" class="loading-container"
    >
      <spinner :size="92" color="#000" />
    </div>
    <main v-else>
      <div>
        <h1>Details of NFT</h1>
        <div
          class="form-nft-send form-nft__detail-page"
          v-if="NFTComputedData"
        >
          <div class="nft-cards">
            <token-card
              :get-info="true"
              :metadata="NFTComputedData"
              :edit-available="false"
            />
          </div>
          <div
            class="form-nft-send__inputs"
          >
            <span class="form-nft-send__inputs-title">Title</span>
            <input
              type="text"
              placeholder="NFT title"
              class="input form-nft__input"
              :value="NFTComputedData.data.name"
              readonly
            >
            <div class="form-nft__bottom">
              <button
                class="main-btn"
                @click="burnNFTHandler"
              >Burn NFT</button>
              <button
                class="main-btn"
                :disabled="true"
                @click="changeFormat"
              >Change Format</button>
              <button
                class="main-btn"
                :disabled="true"
                @click="approveNFTHandler"
              >Approve NFT</button>
              <button
                class="main-btn"
                :disabled="bundleNFTs && !bundleNFTs.length"
                @click="unbundleNFT"
              >Unbundle NFT</button>
              <router-link
                class="main-btn"
                :to="{ name: 'SendNFT', params: { id: NFTComputedData.mint }}"
              >Send NFT</router-link>
            </div>
          </div>
        </div>
      </div>

      <div class="bundle-data" v-if="bundleNFTs && bundleNFTs.length">
        <h2>Bundle contain {{ bundleNFTs.length }} NFT</h2>
        <div class="nft-cards__contract-inner">
          <div
            class="nft-cards__contract__item nft-cards__contract__item--bundle-data"
            v-for="item in bundleNFTs"
            :key="item.mint"
          >
            <h5>Token ID: <br> {{ item.mint.slice(0, 8) }} ... {{ item.mint.slice(item.mint.length - 6) }}</h5>
            <token-card
              class="bundle-data__token"
              :is-bundle="true"
              :metadata="item"
            />
          </div>
        </div>
      </div>

      <div
        v-if="[
          StatusType.Approving,
          StatusType.Sending,
        ].includes(getStatus)" class="loading-container"
      >
        <spinner :size="92" color="#000" />
        <h2>{{ getStatusText(getStatus) }}</h2>
      </div>

    </main>
  </div>
</template>

<script setup>
import { programs, actions } from "@metaplex/js";
import { computed, watch, ref, onMounted } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { PublicKey, Keypair, TransactionInstruction, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { notify } from "@kyvg/vue3-notification";
import statusMixin from "@/mixins/StatusMixin";
import { AppError } from "@/utilities";

import NavBar from "@/components/NavBar/NavBar";
import TokenCard from "@/components/TokenCard/TokenCard";
import Spinner from "@/components/Spinner";

let vaultPubkey = null;

const bundleNFTs = ref([]);

const CONTRACT_PROGRAM_ID = new PublicKey(
  "DyPhsdovhyrTktsJS6nrghGvkRoo2FK3ztH1sKKPBDU4",
);

const bundleStorageAccount = Keypair.fromSecretKey(Uint8Array.from([138,133,11,131,247,141,131,185,159,96,109,107,180,236,20,176,63,41,69,76,179,63,201,132,193,76,220,28,143,52,254,215,31,128,60,52,52,212,51,196,74,36,28,61,13,2,210,174,164,102,234,182,74,120,227,153,67,193,173,126,14,38,102,210]));

const router = useRouter();
const store = useStore();
const { StatusType } = statusMixin();

// getters
const getStatus = computed(() => store.getters.getStatus);

const getNav = [
  {
    text: "Back to Gallery",
    name: "ChooseNFT",
    params: null,
  },
];

const NFTComputedData = computed({
  get() {
    if (store.getters.getAllNFTs && store.getters.getAllNFTs.length) {
      return store.getters.getAllNFTs.find((item) => item.mint === router.currentRoute.value.params.id);
    }
    return null;
  },
});

onMounted(() => {
  console.log(NFTComputedData.value, "ONMOUNTED");
  if (NFTComputedData.value) {
    loadBundleNFTs();
  }
});

watch(() => NFTComputedData.value, () => {
  if (NFTComputedData.value) {
    loadBundleNFTs();
  }
});

const burnNFTHandler = async () => {
  console.log(actions, "burnNFTHandler");
  try {
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    const mint = new PublicKey(NFTComputedData.value.mint);

    const fromTokenAccount = await PublicKey.findProgramAddress(
      [
        fromWallet.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    console.log(fromTokenAccount[0].toString(), "fromTokenAccount");

    store.dispatch("setStatus", StatusType.Approving);
    const signature = await actions.burnToken({
      connection: connection,
      wallet: fromWallet,
      token: fromTokenAccount[0],
      mint: mint,
      amount: 1,
      owner: fromWallet.publicKey,
    });

    console.log(signature, "signature");
    store.dispatch("setStatus", StatusType.Sending);
    const response = await connection.confirmTransaction(signature.txId, "processed");
    console.log(response, "response");

    if (response.value && response.value.err === null) {
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      store.dispatch("setAllSolanaNFts");
      router.push({ name: "ChooseNFT"});
      notify({
        title: "Transaction status",
        type: "success",
        text: "NFT successfully burned!",
        duration: 6000,
      });
    }

  } catch(err) {
    console.log(err, "ERRROR burnNFTHandler");
    store.dispatch("setStatus", StatusType.ChoosingParameters);
    notify({
      title: "Transaction status",
      type: "error",
      text: `Something wrong, Error: ${err}`,
      duration: 6000,
    });
  }
};

const loadBundleNFTs = async () => {
  const connection = store.getters.getSolanaInstance;
  console.log(bundleStorageAccount, "bundle acc");
  const seed = NFTComputedData.value.mint.substr(0, 20);
  const transferAcc = await PublicKey.createWithSeed(
    bundleStorageAccount.publicKey,
    seed,
    CONTRACT_PROGRAM_ID,
  );
  vaultPubkey = transferAcc;

  const TokenMintAccountPubkey1 = (await connection.getAccountInfo(transferAcc, "devnet"));
  if (!TokenMintAccountPubkey1) return;

  const bufferTest = Buffer.from(TokenMintAccountPubkey1.data);
  console.log(bufferTest, "BUFFER TEST");

  // counting nfts in bundle data
  // 37 for bundle, 32 per 1 NFT
  const totalNFTsInBundle = Math.floor(((bufferTest.length - 37) / 32));
  const bundleItems = [];
  console.log(totalNFTsInBundle, "totalNFTsInBundle TEST");

  for (let i = 0;  i < totalNFTsInBundle; i++) {
    const bundleItem = bufferTest.subarray(37 + (32 * i), 37 + (32 * (i + 1)));
    console.log(bundleItem, "bundleItem");
    bundleItems.push(new PublicKey(bundleItem));
  }

  const dataForEveryNFT = await Promise.all(bundleItems.map(async (item) => {
    let realMint = (await connection.getParsedAccountInfo(item, "devnet"))?.value?.data?.parsed?.info?.mint;
    
    if (realMint === undefined || realMint === "null") return {};
    else new PublicKey(realMint);

    const data = await programs.metadata.Metadata.findByMint(connection, realMint);

    return {
      ...data.data,
      pdaMint: item,
    };
  }));

  console.log(dataForEveryNFT, "dataForEveryNFT");

  dataForEveryNFT.forEach((item) => {
    bundleNFTs.value.push(item);
  });
  
  console.log(bundleNFTs.value, "bundleNFTs.value 2");
};

const approveNFTHandler = () => {
  console.log("approveNFTHandler");
};

const unbundleNFT = async () => {
  console.log("unbundleNFT");
  try {
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    const keyWallet = fromWallet.publicKey;

    const tokensMintKeys = [];

    bundleNFTs.value.forEach((item) => {
      tokensMintKeys.push(new PublicKey(item.mint));
    });

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

    const bundleStorageTokenAccountProgram = await PublicKey.findProgramAddress(
      [
        new PublicKey(NFTComputedData.value.mint).toBuffer(),
      ],
      CONTRACT_PROGRAM_ID
    );

    const superNFTTokenAccount = await PublicKey.findProgramAddress(
      [
        fromWallet.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        new PublicKey(NFTComputedData.value.mint).toBuffer()
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    console.log(superNFTTokenAccount[0].toString(), "superNFTTokenAccount[0] mint");

    const burnTx = Token.createBurnInstruction(
      TOKEN_PROGRAM_ID,
      new PublicKey(NFTComputedData.value.mint),
      superNFTTokenAccount[0],
      fromWallet.publicKey,
      [],
      1,
    );

    const bundleTokensSorted = [];

    bundleTokenAccountForMints.forEach((tokenAcc, index) => {
      console.log(bundleNFTs.value, "bundleTokenAccountForMints");
      const pdaForMint = {
        pubkey: bundleNFTs.value[index].pdaMint,
        isSigner: false,
        isWritable: true,
      };

      const nftTokenAcc = {
        pubkey: tokenAcc[0],
        isSigner: false,
        isWritable: true,
      };
      bundleTokensSorted.push(pdaForMint);
      bundleTokensSorted.push(nftTokenAcc);
    });

    const keys =  [
      { pubkey: keyWallet, isSigner: true, isWritable: false },
      { pubkey: vaultPubkey, isSigner: false, isWritable: true },
      { pubkey: superNFTTokenAccount[0], isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: bundleStorageTokenAccountProgram[0], isSigner: false, isWritable: false },
      ...bundleTokensSorted,
    ];

    const bundle_instruction_data = [1, bundleStorageTokenAccountProgram[1]];

    const latestBlockHash = await connection.getLatestBlockhash();
    const initEscrowIx = new TransactionInstruction({
      programId: CONTRACT_PROGRAM_ID,
      keys,
      data: Buffer.from(bundle_instruction_data)
    });
    console.log(initEscrowIx, "initEscrowIx.mint");

    store.dispatch("setStatus", StatusType.Sending);
    const tx = new Transaction({
      feePayer: keyWallet,
      recentBlockhash: latestBlockHash.blockhash,
    });

    tx.add(
      initEscrowIx,
      burnTx,
    );

    const signed = await fromWallet.signTransaction(tx);
    const sendTx = await connection.sendRawTransaction(signed.serialize());
    const response = await connection.confirmTransaction(sendTx, "processed");

    if (response.value && response.value.err === null) {
      store.dispatch("setAllSolanaNFts");
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      router.push({ name: "ChooseNFT"});
      notify({
        title: "Transaction status",
        type: "success",
        text: "NFT successfully Minted!",
        duration: 6000,
      });
    }
  } catch(err) {
    console.log(err, "ERROR BUNDLE");
    store.dispatch("setStatus", StatusType.ChoosingParameters);

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
};

const changeFormat = () => {
  console.log("changeFormat");
};

const getStatusText = (status) => {
  const statusData = statusMixin(status);
  console.log(statusData, "statusData");

  return statusData.statusText;
};
</script>

<style scoped lang="scss">
.bundle-data {
  margin-top: 50px;

  h2 {
    margin-bottom: 40px;
  }
}

.bundle-data__token {
  margin-right: 15px;
}
</style>