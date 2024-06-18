import { createRouter, createWebHistory } from "vue-router";
import { initWallet, useWallet } from "solana-wallets-vue";
import store from "@/store";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { notify } from "@kyvg/vue3-notification";
import statusMixin from "@/mixins/StatusMixin";

import CreateNFT from "@/views/CreateNFT";
import SendNFT from "@/views/SendNFT";
import ChooseNFT from "@/views/ChooseNFT";
import NFTDetails from "@/views/NFTDetails";
import BundleNFT from "@/views/BundleNFT";
import AddEffect from "@/views/AddEffect";
import AddEffectConfirm from "@/views/AddEffectConfirm";

const { StatusType } = statusMixin();

const routes = [
  {
    path: "/",
    name: "home",
    redirect: { name: "ChooseNFT" },
  },
  {
    path: "/login",
    name: "LoginView",
    component: () => import(/* webpackChunkName: "LoginView" */ "@/views/LoginView.vue"),
    meta: { title: "Do[NFT]" },
  },
  {
    path: "/create_nft",
    name: "CreateNFT",
    component: CreateNFT,
    meta: { title: "Do[NFT]", requiresAuth: true }
  },
  {
    path: "/send_nft/:id",
    name: "SendNFT",
    component: SendNFT,
    meta: { title: "Do[NFT]", requiresAuth: true }
  },
  {
    path: "/choose_nft",
    name: "ChooseNFT",
    component: ChooseNFT,
    meta: { title: "Do[NFT]", requiresAuth: true },
  },
  {
    path: "/nft_details/:id",
    name: "NFTDetails",
    component: NFTDetails,
    meta: { title: "Do[NFT]", requiresAuth: true }
  },
  {
    path: "/bundle_nft",
    name: "BundleNFT",
    component: BundleNFT,
    meta: { title: "Do[NFT]", requiresAuth: true }
  },
  {
    path: "/add_effect/:id",
    name: "AddEffect",
    component: AddEffect,
    meta: { title: "Do[NFT]", requiresAuth: true },
  },
  {
    path: "/add_effect/:id/confirm/:effectId",
    name: "AddEffectConfirm",
    component: AddEffectConfirm,
    meta: { title: "Do[NFT]", requiresAuth: true },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// its here for using router push
const walletOptions = {
  wallets: [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter({ network: "devnet" }),
  ],
  autoConnect: true,
  onError: (errorData) => {
    // Error of rejecting on auth
    console.log(router, "router");
    if (errorData.error && errorData.error.code === 4001) {
      console.log(errorData, "Errordata2");
      notify({
        type: "error",
        title: "Wallet error",
        text: errorData.error.message,
        duration: 6000,
      });
      store.commit("SET_WALLET_ERROR", true);
      store.dispatch("setStatus", StatusType.ChoosingParameters);
    } 
  }
};

initWallet(walletOptions);

const { connecting, connected, disconnecting, wallet } = useWallet();

router.beforeEach((to) => {
  // wallet.value._events.onReadyStateChange(() => console.log("READY STATE"));
  console.log(wallet.value, "wallet.value");

  const requiresAuth = to.matched.some((record) => record.meta.requiresAuth);
  console.log(connected.value, "connected");
  console.log(connecting.value, "connecting");
  console.log(disconnecting.value, "disconnecting");
  // todo: try to change to a better checking? currently, this is most stable way
  // to get info about auth of user, other things do not work, as it too early for plugin
  // https://github.com/lorisleiva/solana-wallets-vue?ref=vuejsexamples.com
  const user = connecting.value || connected.value;

  if (requiresAuth && !user) {
    return { name: "LoginView"};
  } else {
    return true;
  }
});

export default router;
