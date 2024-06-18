<template>
  <div :class="$options.name" :style="containerStyle">
    {{size}}
    <div v-for="i in 3" :key="i" :class="`bounce${i}`" :style="blobStyle" />
  </div>
</template>

<script>
export default {
  name: "spinner-dots",
  inheritAttrs: true,
  props: {
    size: {
      type: Number,
      default: 48,
    },
    color: {
      type: String,
    },
  },
  computed: {
    containerStyle() {
      console.log(this.size, "containerStyle");
      const size = this.size && `${this.size}px`;
      return { width: size, height: size };
    },
    blobStyle() {
      const size = this.size && `${this.size / 3}px`;
      return { color: this.color, width: size, height: size };
    },
  },
};
</script>

<style lang="scss" scoped>
$target-scale: .55;
$min-scale: .37;

$animationDuration: 1s;
.spinner-dots {
  font-size: 0;
  text-align: center;
  white-space: nowrap;

  display: flex;
  align-items: center;

  color: inherit;
}

.spinner-dots > div {
  background-color: currentColor;
  border-radius: 100%;
  display: inline-block;
  animation: sk-bouncedelay $animationDuration infinite ease-in-out both;
}

.spinner-dots .bounce1 {
  animation-delay: -(calc($animationDuration / 3));
}

.spinner-dots .bounce2 {
  animation-delay: -(calc($animationDuration / 6));
}

@-webkit-keyframes sk-bouncedelay {
  0%,
  80%,
  100% {
    transform: scale($min-scale);
  }

  40% {
    transform: scale($target-scale);
  }
}

@keyframes sk-bouncedelay {
  0%,
  80%,
  100% {
    transform: scale($min-scale);
  }

  40% {
    transform: scale($target-scale);
  }
}
</style>
