import { h } from "vue";
import SpinnerDots from "./SpinnerDots.vue";
import SpinnerFadingCircle from "./SpinnerFadingCircle.vue";

export default {
  name: "spinner",
  inheritAttrs: false,
  props: {
    size: Number,
    color: String,
    type: {
      type: String,
      default: "dots",
      validator: (prop) => ["dots", "fading-circle"].includes(prop),
    },
  },
  render() {
    const props = { size: this.size, color: this.color };
    switch (this.type) {
    case "fading-circle":
      return h(SpinnerFadingCircle, props);

    case "dots":
    default:
      return h(SpinnerDots, props );
    }
  },
};
