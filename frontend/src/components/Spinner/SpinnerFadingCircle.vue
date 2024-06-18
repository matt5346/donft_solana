<template>
  <div :class="$options.name" :style="style" class="sk-fading-circle">
    <div v-for="i in 8" :key="i" :class="'sk-circle sk-circle' + i" />
  </div>
</template>

<script>
export default {
  name: "spinner-fading-circle",
  inheritAttrs: false,
  props: {
    size: {
      type: Number,
      default: 48,
    },
  },
  computed: {
    style() {
      const size = this.size && `${this.size}px`;
      return { width: size, height: size };
    },
  },
};
</script>

<style lang="scss" scoped>
$circle-count: 8;
$animation-duration: .1s * $circle-count;
$spinkit-size: 36px;

.sk-fading-circle {
  position: relative;

  width: $spinkit-size;
  height: $spinkit-size;
  color: currentColor;

  .sk-circle {
    width: 100%;
    height: 100%;
    position: absolute;
    left: 0;
    top: 0;
  }

  .sk-circle::before {
    $width: round(.1 * $spinkit-size);
    content: '';
    display: block;
    margin: 0 auto;
    width: $width;
    height: $width;
    overflow: hidden;
    background-color: currentColor;
    border-radius: calc($width / 2);
    animation: sk-circleFadeDelay $animation-duration infinite ease-in-out both;
  }

  @for $i from 1 through $circle-count {
    .sk-circle#{$i} {
      transform: rotate(360deg / calc($circle-count * ($i - 1)));

      &::before {
        $a: -$animation-duration + calc(($i - 1) * $animation-duration/$circle-count);
        animation-delay: $a;
      }
    }
  }
}

@keyframes sk-circleFadeDelay {
  0%,
  39%,
  100% {
    opacity: 0;
  }

  40% {
    opacity: 1;
  }
}
</style>
