<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

async function play_video() {
  const uri = await open({
    multiple: false,
    directory: false,
  });
  console.log(uri);
  if (uri) {
    await invoke('play_video', { uri: uri });
  }
}
</script>

<template>
  <div class="card">
    <button type="button" @click="play_video">Open a file</button>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
