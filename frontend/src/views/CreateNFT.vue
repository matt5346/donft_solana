<template>
  <div class="page">
    <nav-bar :navigation="getNavigation"/>
    <main>
      <h1>Create new NFT</h1>
      <div class="form-nft">
        <div class="form-nft__photo">

          <TakePhoto  v-if="mode === 'selfie'"  @setFile="setFile"/>
          <uploader-comp v-else @selected="setUploadedImg"/>

          <button
            v-if="mode === 'photo'"
            class="main-btn"
            @click="mode = 'selfie'"
          >Take a selfie</button>
          <div
            v-else
            @click="mode = 'photo'"
          >
            <icon-component
              icon-type="cross"
              width="32"
              height="32"
              class="form-nft__photo-close"
            />
          </div>
        </div>
        <div class="form-ntf__inputs">
          <span class="form-nft-send__inputs-title">Title</span>
          <input
            type="text"
            placeholder="NFT title"
            class="input form-nft__input"
            v-model="nftObj.name"
          >
          <span class="form-nft-send__inputs-title">NFT symbol</span>
          <textarea
            type="text"
            placeholder="NFT symbol"
            class="input form-nft__input form-nft__textarea"
            v-model="nftObj.symbol"
          />
          <button
            class="main-btn"
            @click.prevent="createNewNFT"
          >Submit</button>
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
              StatusType.Sending,
              StatusType.Minting,
              StatusType.DeployingToIPFS
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
import { reactive, computed, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "vuex";
import statusMixin from "@/mixins/StatusMixin";
import { notify } from "@kyvg/vue3-notification";

import NavBar from "@/components/NavBar/NavBar";
import UploaderComp from "@/components/Uploader/UploaderComp";
import ModalTemplate from "@/components/ModalTemplate/ModalTemplate";
import TakePhoto from "@/components/TakePhoto/TakePhoto";
import Spinner from "@/components/Spinner";
import { AppError } from "@/utilities";

const store = useStore();
const router = useRouter();
const { StatusType } = statusMixin();

const nftObj = reactive({
  name: "NFT token 2 title",
  symbol: "nft",
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

let showApproveModal = ref(false);
let mode = ref("photo");

const getNavigation = [{
  text: "Back to Gallery",
  name: "ChooseNFT",
}];

const getStatus = computed(() => store.getters.getStatus);

onMounted(() => {
  // creating nft, require wallet key of creator
  const defaultCreator = {
    "address": store.getters.getSolanaWalletInstance.publicKey.toString(),
    "share": 100
  };
  nftObj.properties.creators.push(defaultCreator);
});

const setFile = (blob) => {
  const reader = new FileReader();
  reader.readAsDataURL(blob); 
  reader.onloadend = function() {
    const base64data = reader.result;                
    nftObj.image = base64data; 
  };
};

const setUploadedImg = (img) => {
  nftObj.image = img; 
};

const closeModal = () => {
  showApproveModal.value = false;
};

//"https://ipfs.io/ipfs/QmX4rTazAq9gmJJJBP3a9FFyQZnoYQfqUUCCizRyhopcAt"

const createNewNFT = async () => {
  console.log(nftObj, "NFT OBJ");
  try {
    const connection = store.getters.getSolanaInstance;
    const fromWallet = store.getters.getSolanaWalletInstance;
    showApproveModal.value = true;

    store.dispatch("setStatus", StatusType.DeployingToIPFS);
    await store.dispatch("setDeployToIPFS", { isImageDeployed: false, meta: nftObj });

    store.dispatch("setStatus", StatusType.Approving);
    console.log(store.getters.getNFTdeployResult, "CREATING");
    const signature = await actions.mintNFT({
      connection,
      wallet: fromWallet,
      uri: store.getters.getNFTdeployResult,
      maxSupply: 1
    });
    store.dispatch("setStatus", StatusType.Minting);
    const response = await connection.confirmTransaction(signature.txId, "finalized");
    console.log(signature.mint.toString(), "signature mint");

    // console.log(setBundleAuthority, "FINAL RESPONSE");
    if (response.value && response.value.err === null) {
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      store.dispatch("setAllSolanaNFts");
      showApproveModal.value = false;
      router.push({ name: "ChooseNFT"});
      notify({
        title: "Transaction status",
        type: "success",
        text: "NFT successfully Minted!",
        duration: 6000,
      });
    }

  } catch(err) {
    console.log(err, "ERRROR createNewNFT");
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
};

const getStatusText = (status) => {
  const statusData = statusMixin(status);

  return statusData.statusText;
};
</script>
