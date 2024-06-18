<template>
  <div class="nft-cards__media-wrap">
    <img
      class="nft-cards__media"
      :src="nftImage || placeholder()"
    >
    <p class="nft-cards__title">{{props.metadata.data.name}}</p>
    <!-- <template v-if="isApprovedContract && !isNFTApproved">
      <button
        class="main-btn"
        @click="approveNFTHandler"
      >Approve NFT</button>
      <p
        class="nft-cards__approve"
      >Please approve NFT first</p>
    </template> -->
    <router-link
      v-if="editAvailable"
      class="nft-cards__info"
      :to="{ name: 'NFTDetails', params: { id: props.metadata.mint }}"
    >
      <icon-component icon-type="pencil" width="16" height="16" class="pencil-icon" />
    </router-link>
  </div>
</template>

<script setup>
import { defineProps, ref, computed, onMounted, watch } from "vue";
import { useStore } from "vuex";
import { placeholder } from "@/utilities";

const props = defineProps({
  metadata: Object,
  editAvailable: Boolean,
  isBundle: Boolean,
  getInfo: Boolean,
  isApprovedContract: String,
});

const store = useStore();
let nftImage = ref(null);

const getIpfs = computed({
  get() {
    return store.getters["getIpfs"];
  },
});

watch(() => getIpfs.value, () => {
  if (getIpfs.value) {
    loadContent();
  }
});

onMounted(() => {
  if (getIpfs.value) {
    loadContent();
  }
});

const loadContent = async () => {
  if (props.metadata) {
    let tokenData = null;

    // case for recently created image with BLOB
    if (props.metadata.data && props.metadata.data.image)  {
      nftImage.value = props.metadata.data.image;
    }

    if (!nftImage.value || props.isBundle) {
      // here is all data about NFT in METAPLEX Version
      tokenData = await store.dispatch("setTokenImage", { token: props.metadata.data, getIPFSurl: false });

      // if need extra info for main BUNDLE NFT, on single page, commit
      if (props.getInfo) {
        store.commit("SET_CURRENT_NFT", tokenData);
      }

      nftImage.value = tokenData ? tokenData.image : null;
    }

    

  }
};
</script>

<style lang="scss" scope>
.nft-cards__media-wrap {
  position: relative;

  .effect-cards-box & {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  &:hover {
    .nft-cards__info {
      opacity: 1;
    }
  }
}

.nft-cards__info {
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  right: 10px;
  top: 10px;
  border-radius: 4px;
  background: #5ce9bc;
  padding: 8px;
  opacity: .4;
  transition: background .15s ease-in-out, transform .1s ease-in, opacity .15s ease;
  
  &:hover {
    background: #2d0949;
    fill: #fff;
    transform: scale(1.2);
  }
}

.nft-cards__approve {
  font-size: 16px;
  background: red;
  color: #fff;
  padding: 5px 8px;
  border-radius: 4px;
  cursor: text;
}

.nft-cards__title {
  .effect-cards-box & {
    padding: 0 5px;
    font-size: 16px;
  }
}
</style>