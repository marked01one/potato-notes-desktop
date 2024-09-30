<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const message = ref("");
const morningMsg = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function morning() {
  morningMsg.value = await invoke("morning", { message: message.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>
  <form class="row" @submit.prevent="morning">
    <input id="morning-input" v-model="message" placeholder="Enter a message" />
    <button type="submit">Morning</button>
  </form>

  <p>{{ greetMsg }}</p>
  <p>{{ morningMsg }}</p>
</template>
