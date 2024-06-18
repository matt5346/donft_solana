<template>
  <div class="take-photo">
    <video ref="video" playsinline>Video stream not available.</video>
    <span
      class="main-btn main-btn--reverse main-btn--photo"
      @click.prevent="clickHandle"
    >{{ openCamera? 'Take shot' : 'Init...' }}</span>
    <canvas ref="canvas"></canvas>
    <img v-show="lastShot" :src="lastShot" alt="">
    <span v-show="lastShot" class="button take-photo__shot" @click.prevent="clearShot">Clear</span>
  </div>
</template>

<script setup>

import {ref, onMounted, onBeforeUnmount, watch, nextTick, defineEmits} from "vue";
const emit = defineEmits(["fireError", "setFile"]);
const lastShot = ref(null);
const clearShot = () => {
  lastShot.value = null;
  emit("setFile", null);
};
watch(lastShot, (_, prevURL) => {
  if(prevURL) URL.revokeObjectURL(prevURL);
});

const openCamera = ref(false);

const video = ref(null);
const canvas = ref(null);

let mediaSize = 0;
let longerSide = null;

const startRecord = async () => {
  console.log("startRecord");
  try{
    lastShot.value = null;
    video.value.srcObject = await navigator.mediaDevices.getUserMedia({ video: true, audio: false });
    video.value.play();
    openCamera.value = true;
  }
  catch (e){
    console.log(e);
    emit("fireError", "MEDIA_NOT_ALLOWED");
  }
};

const takePhoto = () => {
  console.log("takePhoto");
  if(canvas.value && mediaSize){
    canvas.value.width = mediaSize;
    canvas.value.height = mediaSize;

    const context = canvas.value.getContext("2d");
    if(longerSide === "same"){
      context.drawImage(video.value, 0, 0, mediaSize, mediaSize, 0, 0, mediaSize, mediaSize);
    }
    else if(longerSide === "width"){
      context.drawImage(video.value, Math.floor((video.value.videoWidth - mediaSize) / 2), 0, mediaSize, mediaSize, 0, 0, mediaSize, mediaSize);
    }
    else {
      context.drawImage(video.value, 0, Math.floor((video.value.videoHeight - mediaSize) / 2), mediaSize, mediaSize, 0, 0, mediaSize, mediaSize);
    }

    canvas.value.toBlob(blob => {
      const photoLink = URL.createObjectURL(blob);
      lastShot.value = photoLink;
      emit("setFile", blob);
    });
  }
};

const canPlayListener = (e) => {
  console.log("video can play", e, video.value);
  if(video.value.videoHeight < video.value.videoWidth){
    longerSide = "width";
    mediaSize = video.value.videoHeight;
  }
  else{
    longerSide = "height";
    mediaSize = video.value.videoWidth;
  }
  if(video.value.videoHeight === video.value.videoWidth) longerSide = "same";
  console.log("Media size:", mediaSize);
};

onMounted(() => {
  video.value.addEventListener("canplay", canPlayListener);
});
onBeforeUnmount(() => {
  video.value.removeEventListener("canplay", canPlayListener);
});

const clickHandle = () => {
  if(openCamera.value) takePhoto();
  else startRecord();
};

nextTick().then(() => startRecord());

</script>