<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

async function openfile() {
  const file = await open({
    multiple: false,
    directory: false,
  });
  console.log(file);
  if (file) {
    await invoke('play', { filePath: file });
  }
}
</script>

<template>
  <div class="card">
    <button type="button" @click="openfile">Open a file</button>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
