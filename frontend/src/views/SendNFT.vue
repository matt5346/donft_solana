<template>
  <div class="page">
    <nav-bar :navigation="getNav"/>
    <!-- {{
      [
        statusText.Applying,
        statusText.DeployingToIPFS,
        statusText.DeployedToIPFS,
        statusText.Minting
      ].includes(getStatus)
    }} -->
    <div
      v-if="!NFTComputedData" class="loading-container"
    >
      <spinner :size="92" color="#000" />
    </div>
    <main v-else>
      <div>
        <h1 >Send NFTs</h1>
        <div
          class="form-nft-send form-nft__detail-page"
        >
          <div
            class="nft-cards"
            v-if="NFTComputedData"
          >
            <token-card
              :get-info="true"
              :metadata="NFTComputedData"
              :edit-available="false"
            />
          </div>
          <div class="form-nft-send__inputs">
            <div>
              <span class="form-nft-send__inputs-title">Receipt ID</span>
              <textarea
                v-model="receiver_id"
                type="text"
                placeholder="Receipt ID"
                class="input form-nft__input"
                :resize="false"
              />
            </div>
            <div class="form-nft__bottom">
              <button
                class="main-btn"
                @click="sendNFTHandler"
              >Send</button>
            </div>
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
import { computed, ref } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { actions } from "@metaplex/js/";
import { PublicKey } from "@solana/web3.js";
import statusMixin from "@/mixins/StatusMixin";
import { notify } from "@kyvg/vue3-notification";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

import NavBar from "@/components/NavBar/NavBar";
import TokenCard from "@/components/TokenCard/TokenCard";
import Spinner from "@/components/Spinner";

const router = useRouter();
const store = useStore();
const { StatusType } = statusMixin();

// default wallet id, for testing
let receiver_id = ref("8T8zhN7AAR3UBfYhiBvKkzS39ii3AZMARZZz2KjA5UnV");

const getNav = [{
  text: "Back to Gallery",
  name: "ChooseNFT",
}];

const NFTComputedData = computed({
  get() {
    if (store.getters.getAllNFTs && store.getters.getAllNFTs.length) {
      return store.getters.getAllNFTs.find((item) => item.mint === router.currentRoute.value.params.id);
    }
    return null;
  },
});

const getStatus = computed(() => store.getters.getStatus);

const sendNFTHandler = async () => {
  try {
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    const mint = new PublicKey(NFTComputedData.value.mint);
    const toWallet = new PublicKey(receiver_id.value);

    const fromTokenAccount = await PublicKey.findProgramAddress(
      [
        fromWallet.publicKey.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log(StatusType, "StatusType");
    store.dispatch("setStatus", StatusType.Approving);
    const signature = await actions.sendToken({
      connection,
      wallet: fromWallet,
      source: fromTokenAccount[0],
      destination: toWallet,
      mint,
      amount: 1,
    });

    console.log(signature, "dest ATA");
    store.dispatch("setStatus", StatusType.Sending);
    const response = await connection.confirmTransaction(signature.txId, "processed");
    console.log("response signature", response);

    if (response.value && response.value.err === null) {
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      store.dispatch("setAllSolanaNFts");
      router.push({ name: "ChooseNFT"});
      notify({
        title: "Transaction status",
        type: "success",
        text: "NFT successfully sent!",
        duration: 6000,
      });
    }
    console.log(actions, "actions ATA");
  } catch (err) {
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

const getStatusText = (status) => {
  const statusData = statusMixin(status);
  console.log(statusData, "statusData");

  return statusData.statusText;
};
</script>