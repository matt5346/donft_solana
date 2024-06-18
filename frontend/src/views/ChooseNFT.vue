<template>
  <div class="page">
    <nav-bar
      :navigation="getNav"
      :show-generate-nft="true"
      @generate-random-nft="generateRandomNFT"
    />

    <div
      v-if="getLoadingNFTsStatus"
      class="loading-container loading-container--side"
    >
      <spinner :size="92" color="#000" />
      <h2>Loading NFTs</h2>
    </div>
    <main v-else>
      <h1>Choose NFT and apply effect</h1>
      <div
        class="nft-cards__contract-inner"
        v-if="getAllNFTs"
      >
        <div
          v-for="contractData in filteredNFTbyContract"
          :key="contractData.name"
          class="nft-cards__contract"
        >
          <template v-if="contractData.items && contractData.items.length">
            <h3>Contract: {{contractData.name}}</h3>
            <div class="nft-cards__contract-inner">
              <div
                v-for="nft in contractData.items"
                :key="`key-${nft.mint}`"
                class="nft-cards__contract__item"
                :class="{ 'chosen-card': cardClass(nft.mint)}"
              >
                <token-card
                  :metadata="nft"
                  :edit-available="true"
                  @click="chooseNFT(nft)"
                />
              </div>
            </div>
          </template>
        </div>
      </div>
    </main>

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
          ].includes(getStatus)"
          class="loading-container"
        >
          <spinner :size="92" color="#000" />
          <h2>{{ getStatusText(getStatus) }}</h2>
        </div>
      </template>
    </modal-template>

  </div>
</template>
<script setup>
import { actions } from "@metaplex/js";
import { computed, ref, onMounted } from "vue";
import { useStore } from "vuex";
import NavBar from "@/components/NavBar/NavBar";
import TokenCard from "@/components/TokenCard/TokenCard";
import Spinner from "@/components/Spinner";
import { AppError, SystemErrors } from "@/utilities";
import { notify } from "@kyvg/vue3-notification";
import statusMixin from "@/mixins/StatusMixin";
import ModalTemplate from "@/components/ModalTemplate/ModalTemplate";

const store = useStore();
const { StatusType } = statusMixin();
let token_id = ref([]);

let showApproveModal = ref(false);

let nftObj = {
  name: "",
  symbol: "test",
  seller_fee_basis_points: 0,
  description: "",
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
};

const randomNFTArr = store.state.randomNFTArr;
const randomEffectsArr = store.state.randomEffectsArr;

const getAllNFTs = computed(() => store.getters.getAllNFTs);
const filteredNFTbyContract = computed(() => store.getters.filteredNFTbyContract);
const getLoadingNFTsStatus = computed(() => store.getters.getLoadingNFTsStatus);
const getStatus = computed(() => store.getters.getStatus);


const cardClass = computed({
  get() {
    return(idx) => token_id.value.indexOf(idx) !== -1;
  },
});

const getNav = computed({
  get() {
    return [
      {
        text: "Create New",
        name: "CreateNFT",
        params: null,
      },
      {
        text: "Send",
        name: "SendNFT",
        params: {
          id: token_id.value && token_id.value.length === 1 ? token_id.value[0] : ""
        },
      },
      {
        text: "Bundle",
        name: "BundleNFT",
        params: {
          id: token_id.value && token_id.value.length > 1 ? token_id.value : null
        },
      },
      {
        text: "Effect",
        name: "AddEffect",
        params: {
          id: token_id.value && token_id.value.length === 1 ? token_id.value[0] : ""
        },
      },
    ];
  },
});

onMounted(() => {
  // creating nft, require wallet key of creator
  const defaultCreator = {
    "address": store.getters.getSolanaWalletInstance.publicKey.toString(),
    "share": 100
  };
  nftObj.properties.creators.push(defaultCreator);
});

// Creating Random NFT, depend on Math random
// currently only for VUE_APP_NFTS_CONTRACT and VUE_APP_NFTS_EFFECTS_CONTRACT
const generateRandomNFT = async (nftType) => {
  const connection = store.getters.getSolanaInstance;

  try {
    let randomNumber = Math.floor(Math.random() * 5);
    let randomImage =  randomNFTArr[randomNumber];

    showApproveModal.value = true;

    if (nftType === "effectNFT") {
      randomNumber = Math.floor(Math.random() * 6);
      randomImage =  randomEffectsArr[randomNumber];
    }

    console.log(randomNumber, "randomNumber");

    nftObj = {
      ...nftObj,
      name: `Test NFT ${randomNumber}`,
      symbol: "nft",
      image: randomImage,
    };

    if (nftType === "effectNFT") {
      nftObj = {
        ...nftObj,
        name: `Test Effect NFT ${randomNumber}`,
        symbol: "effect",
        image: randomImage,
      };
    }

    try {
      store.dispatch("setStatus", StatusType.DeployingToIPFS);
      // is Random, for skipping image ipfs deploy
      await store.dispatch("setDeployToIPFS", { isImageDeployed: true, meta: nftObj });
    } catch(err) {
      if (err instanceof AppError) {
        throw err; 
      } else {
        throw SystemErrors.IPFS_SAVE;
      }
    }

    try {
      console.log(store.getters.getNFTdeployResult, "CREATING");
      store.dispatch("setStatus", StatusType.Approving);

      const signature = await actions.mintNFT({
        connection,
        wallet: store.getters.getSolanaWalletInstance,
        uri: store.getters.getNFTdeployResult,
        maxSupply: 1
      });

      store.dispatch("setStatus", StatusType.Minting);
      const response = await connection.confirmTransaction(signature.txId, "finalized");

      console.log(signature, "CREATING");
      if (response.value && response.value.err === null) {
        store.dispatch("setStatus", StatusType.ChoosingParameters);
        store.dispatch("setAllSolanaNFts");
        showApproveModal.value = false;

        notify({
          title: "Transaction status",
          type: "success",
          text: "NFT successfully Minted!",
          duration: 6000,
        });
      }

    } catch(err) {
      console.log(err, "error");
      store.dispatch("setStatus", StatusType.ChoosingParameters);
      showApproveModal.value = false;

      if (err instanceof AppError) {
        throw err; 
      } else {
        throw SystemErrors.MINT_NFT;
      }
    }
  } catch(err) {
    store.dispatch("setStatus", StatusType.ChoosingParameters);
    console.log(err, "MAIN ERROR");

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

const chooseNFT = (item) => {
  const index = token_id.value.findIndex((_) => _ === item.mint);

  // Currently approving multiple NFTs is problem,
  // for this need smart contract, bundle approve + bundle sending
  if (index > -1) {
    token_id.value.splice(index, 1);
  } else {
    token_id.value.push(item.mint);
  }

  // this one for single actions, send or effects page
  token_id.value && token_id.value.length === 1 ? store.commit("SET_CURRENT_NFT", item) : store.commit("SET_CURRENT_NFT", {});

  // this one for bundle page
  store.commit("SET_BUNDLE_NFTS", token_id.value);
};

const getStatusText = (status) => {
  const statusData = statusMixin(status);

  return statusData.statusText;
};
</script>

<style lang="scss">
.nft-cards {
  display: flex;
  flex-wrap: wrap;
}

.nft-cards__contract {
  width: 100%;

  h3 {
    margin-bottom: 10px;
  }
}

.nft-cards__contract-inner {
  display: flex;
  flex-wrap: wrap;
}

.nft-cards__contract__item {
  width: 19%;
  min-width: 200px;
  margin-bottom: 30px;
  margin-right: 5px;
  cursor: pointer;
  transition: transform .1s ease-in-out, box-shadow .1s ease;

  &:last-child {
    margin-right: 0;
  }
}

.nft-cards__contract__item--bundle-data {
  width: 24%;
  cursor: initial;

  img {
    border: 1px solid #2d094970;
    margin-top: 15px;
    border-radius: 4px;
  }
}

.nft-cards__contract__item.chosen-card {
  box-shadow: -2px -2px 12px 11px rgba(127, 251, 255, 0.7);
  transform: scale(0.9);
  .nft-cards__info {
    opacity: 1;
  }
}

.nft-cards__media {
  display: block;
  width: 100%;
  height: 200px;
  object-fit: cover;

  .form-nft__detail-page & {
    width: 300px;
    height: 300px;
  }
}

.nft-cards__title {
  margin-top: 20px;
}

h1 {
  margin-bottom: 30px;
}

</style>