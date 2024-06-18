<template>
  <div :style="iconStyle" class="icon-box">
    <component :is="iconType"></component>
  </div>
</template>

<script>
import icons from "@/assets/svg";

export default {
  name: "SvgIcon",
  components: {
    ...icons
  },
  props: {
    iconType: {
      type: String
    },
    iconColor: {
      type: String,
      default: "currentColor"
    },

    width: {
      type: [Number, String],
      default: 18
    },
    height: {
      type: [Number, String],
      default: 18
    }
  },
  computed: {
    iconStyle() {
      const { width, height } = this;
      return {
        width: `${width}px`,
        height: `${height}px`
      };
    }
  }
};
</script>

<style lang="scss" scoped>
.icon-box {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>