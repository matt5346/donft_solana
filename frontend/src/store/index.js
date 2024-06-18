import { createStore } from "vuex";

import {
  deployNFTtoIPFS,
  loadAllNFTs,
  getImageForTokenByURI,
} from "@/solana_utilities";

import {
  StatusType,
  getIPFS,
  SystemErrors
} from "@/utilities";

export default createStore({
  state: {
    ipfs: null,
    deployedNFT: null,
    solanaDialogOpenStatus: false,
    solanaWalletConnected: false,
    solanaWalletAddress: null,
    solanaInstance: null,
    solanaWalletInstance: null,
    loadingNFTs: false,
    status: StatusType.ChoosingParameters,
    allNFTs: [],
    currentNFT: {},
    walletError: false,
    nftsAreLoaded: false,
    choice: false,
    // preuploaded images for random NFTs
    randomNFTArr: [
      "https://bafkreibsgijmp53nwssszihmmq25q4ippdfqm67hplgvnjajyymsngksey.ipfs.nftstorage.link/",
      "https://bafkreieflpqauetjd52ywpcyqggp66vtnfqhkmthr6ng3yqagk45yltp6q.ipfs.nftstorage.link/",
      "https://bafkreig3suewzhzunlimmtfhycefv3szqux5qbsrllvovrk7ftv5aifmhu.ipfs.nftstorage.link/",
      "https://bafkreih6hd4ysjce443cmiuagli5bnog4g6uz2r66xqmyg4d6ufdh3ttmm.ipfs.nftstorage.link/",
      "https://bafkreibb4ukdluctf3d377i4r627xcn2ydqtl35yulawwa2ty65cysv6vq.ipfs.nftstorage.link/",
    ],
    randomEffectsArr: [
      "https://bafkreif42p2e7ep5q3kpuoz2dlpxmctwr7unajswe3ibq6fm6rcinvjwk4.ipfs.nftstorage.link/",
      "https://bafkreieqmictg2ujcnsrudnq72mmijb3nelwlpzb7gr7xhn77j2aihzlcu.ipfs.nftstorage.link/",
      "https://bafybeifdsj6pzie4vos42oyiou3lyesyefs5uj7uyxlgjqbdmg7msdvvpi.ipfs.nftstorage.link/",
      "https://bafybeigfhupxt6zjfqykf54whf7pabtugo5l57n6t5k5or66e4sovsdaiu.ipfs.nftstorage.link/",
      "https://bafybeibe2px7qbklyk4r7d4mxttb3cqwjx46cdjteg6pd55oajbmxxcvfm.ipfs.nftstorage.link/",
      "https://bafybeidctmnh3y4g4i57b5pltsvslkjjooa64avfyyoj5de25mouf3ct5e.ipfs.nftstorage.link/",
      "https://bafybeidctmnh3y4g4i57b5pltsvslkjjooa64avfyyoj5de25mouf3ct5e.ipfs.nftstorage.link/",
    ]
  },
  mutations: {
    setStatus (state, status) {
      state.status = status;
    },
    setIpfs (state, ipfsInstance) {
      state.ipfs = ipfsInstance;
    },
    // func for updating recently created NFT
    ADD_MINTED_NFT(state, payload) {
      state.allNFTs.push(payload);
    },
    REMOVE_FROM_NFT_LIST(state, id) {
      const index = state.allNFTs.map((item) => item.mint).indexOf(id);

      if (index > -1) {
        state.allNFTs.splice(index, 1);
      }
    },
    SET_EFFECT_CHOICE(state, choice) {
      state.effectChoice = choice;
    },
    SET_NFTS_LOADED(state, payload) {
      state.nftsAreLoaded = payload;
    },
    SET_BUNDLE_NFTS (state, payload) {
      localStorage.setItem("tokens_id", payload);
    },
    SET_LOADING_NFTS (state, payload) {
      state.loadingNFTs = payload;
    },
    SET_ALL_NFTS(state, payload) {
      state.allNFTs = payload;
    },
    // for single NFT pages, extra info about NFT
    SET_CURRENT_NFT(state, payload) {
      state.currentNFT = payload;
    },
    SET_CONNECTED(state, address) {
      state.solanaWalletConnected = true;
      state.solanaWalletAddress = address;
      localStorage.setItem("solana_wallet_address", address);
    },
    SET_DEPLOYED_NFT(state, payload) {
      state.deployedNFT = payload;
    },
    SET_WALLET_DIALOG(state, data) {
      state.solanaDialogOpenStatus = data;
    },
    SET_WALLET_DISCONNECTED(state) {
      state.solanaWalletConnected = false;
      state.solanaWalletAddress = null;
      state.solanaInstance = null;
      state.solanaWalletInstance = null;
      state.allNFTs = [];
      localStorage.removeItem("solana_wallet_address", null);
    },
    SET_SOLANA_INSTANCE(state, payload) {
      state.solanaInstance = payload;
    },
    SET_CURRENT_WALLET(state, payload) {
      state.solanaWalletInstance = payload;
    },
    SET_WALLET_ERROR(state, payload) {
      state.walletError = payload;
    },
  },
  actions: {
    async setIpfs ({commit}) {
      const ipfs = await getIPFS();
      commit("setIpfs", await ipfs.create());
    },
    async setDeployToIPFS ({commit, getters}, { isImageDeployed = false, meta }) {
      try {
        console.log(meta, "META");
        commit("SET_DEPLOYED_NFT", await deployNFTtoIPFS(getters.getIpfs, meta, isImageDeployed));
      } catch(err) {
        console.log(err, "ERROR setDeployToIPFS");
        throw SystemErrors.IPFS_SAVE;
      }
    },
    // solana storage a little different with NEAR
    // data of NFT storing link to IPFS with extra data, where are METAPLEX fields stored with image
    // after loading METAPLEX data, we can get real image getImageForTokenByURI
    async setTokenImage ({getters}, { token, getIPFSurl }) {
      let url = token.uri;
      let data = null;

      if (getters.getIpfs) {
        data = await getImageForTokenByURI(getters.getIpfs, url, getIPFSurl);
      }

      return data;
    },
    setStatus ({commit}, status) {
      commit("setStatus", status);
    },
    async setAllSolanaNFts ({commit, getters}) {
      console.log("setAllSolanaNFts");
      commit("SET_LOADING_NFTS", true);
      commit("SET_ALL_NFTS", await loadAllNFTs(getters.getSolanaInstance, getters.getSolanaWalletInstance, commit));
      commit("SET_LOADING_NFTS", false);
    },
  },
  getters: {
    getIpfs: state => state.ipfs,
    getNFTdeployResult: state => state.deployedNFT,
    getAllNFTs: state => state.allNFTs,
    getNFTchoice: () => (localStorage.getItem("tokens_id") || "").split(","),
    getDialogWalletStatus: (state) => state.solanaDialogOpenStatus,
    getWalletConnection: (state) => state.solanaWalletConnected,
    getWalletAddress: (state) => state.solanaWalletAddress,
    getSolanaInstance: (state) => state.solanaInstance,
    getSolanaWalletInstance: (state) => state.solanaWalletInstance,
    getStatus: state => state.status,
    getCurrentNFT: state => state.currentNFT,
    getLoadingNFTsStatus: state => state.loadingNFTs,
    getWalletError: state => state.walletError,
    getNFTsLoadStatus: state => state.nftsAreLoaded,
    getEffectChoice: state => state.effectChoice,
    getEffect: state => state.allNFTs.find(x => x.mint === state.effectChoice),
    filteredNFTbyContract: (state) => {
      if (state.allNFTs && state.allNFTs.length) {
        const nftContract = [];
        const effectContract = [];
        const bundleContract = [];
        const otherContract = [];

        state.allNFTs.forEach((item) => {
          if (item.data.symbol === "nft") {
            return nftContract.push(item);
          }
          if (item.data.symbol === "effect") {
            effectContract.push(item);
            return;
          }
          if (item.data.symbol === "bundle") {
            bundleContract.push(item);
            return;
          }

          otherContract.push(item);
        });

        return [
          { name: "nft",  items: nftContract },
          { name: "effect", items: effectContract },
          { name: "bundle", items: bundleContract },
          { name: "other", items: otherContract },
        ];
      }

      return [];
    }
  },
});
