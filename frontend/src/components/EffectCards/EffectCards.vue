<template>
  <div class="effect-cards">
    <div v-for="card in props.cards" class="effect-cards__item" :key="card.mint" :class="cardClass(card.mint)">
      <div @click="cardClicked(card)" class="effect-cards__item__inner">
        <token-card
          :metadata="card"
        />
        <div class="effect-cards__item__footer">
          <p :class="{'card-title': props.cardSize === 'standard', 'card-text': props.cardSize === 'smaller'}">
            <span v-if="showId && card.mint">{{ shorten(card.mint, 5) }}: </span>{{ shorten(card.name, 30) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { defineProps, defineEmits } from "vue";
import { shorten } from "@/utilities";
import TokenCard from "@/components/TokenCard/TokenCard";

const props = defineProps({
  cards: Array,
  choice: Array,
  cardSize: String,
  showId: Boolean,
  contentType: String,
});

const emitList = defineEmits(["card-clicked"]);

const cardClicked = (card) => {
  emitList("card-clicked", card);
};

const cardClass = (idx) => {
  return {
    "chosen-card": props.choice.indexOf(idx) !== -1,
    "choosable-card": props.choice.indexOf(idx) === -1,
    "standard-card-size": props.cardSize === "standard",
    "smaller-card-size": props.cardSize === "smaller"
  };
};

</script>

<style lang="scss" scoped>
.standard-card-size {
    height: 242px;
    width: 168px;
}

.smaller-card-size {
    height: 200px;
    width: 137px;
}

.rounded-16 {
    border-radius: 16px !important;
}

.card-body {
  max-height: 56px;
}

.effect-cards {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
}

.effect-cards__item {
  width: 14%;
  margin-bottom: 20px;
  background: linear-gradient(0deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.05)), #121212;
  box-shadow: 0px 1px 1px rgba(0, 0, 0, 0.14), 0px 2px 1px rgba(0, 0, 0, 0.12), 0px 1px 3px rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  color: #dedede;
  cursor: pointer;
  transition: transform .1s ease-in-out, box-shadow .1s ease;
}

.chosen-card {
    box-shadow: 0px 0px 7px 6px rgba(192, 127, 255, 0.7);
    transform: scale(0.92);
}

.card-img {
  width: 100%;
  min-height: 200px;
  object-fit: cover;
}

.effect-cards__item__inner {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.effect-cards__item__footer {
  text-align: center;
  
  p {
    margin: 5px 0;
  }
}
</style>