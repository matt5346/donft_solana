<template>
  <div class="home-text">
    <div class="home-text__inner">
      <div class="home-text__inner__header">
        <h2>Welcome to DoNFT demo on Solana</h2>
        <img src="@/assets/solana-sol-logo.png" />
      </div>
      <p class="home-text__inner__subtext">
        Use power of neural network algorithms to create new unique combinations
        of digital art
      </p>
      <p
        style="text-align: center; margin-top: 2.5em"
      >
        <wallet-multi-button dark></wallet-multi-button>
      </p>
    </div>
    <img class="home-text__logo" src="@/assets/home_img.jpg" style="height: 30%; border-radius: 20%" />
  </div>
</template>

<script setup>
import { WalletMultiButton } from "solana-wallets-vue";
</script>

<style lang="scss" scoped>
.home-text {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  min-height: 60vh;
}

.home-text__inner {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  width: 50%;
}

.home-text__inner__header {
  display: flex;
  align-items: center;
  justify-content: space-between;

  img {
    height: 130px;
    margin-left: 40px;
  }
}

.home-text__logo {
  width: 30%;
  margin-left: 20px;
}
</style>
