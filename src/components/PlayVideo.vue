<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { PlayIcon } from '@heroicons/vue/24/solid';

// 定义事件
const emit = defineEmits<{
  videoSelected: [videoUrl: string]
}>();

async function play_video() {
  const uri = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: 'Video Files',
      extensions: ['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm', 'm4v', '3gp']
    }]
  });
  
  console.log(uri);
  if (uri) {
    // 发出事件，让父组件处理视频显示
    emit('videoSelected', uri);
    
    // 调用后端播放视频
    try {
      await invoke('play_video', { uri: uri });
    } catch (error) {
      console.error('播放视频失败:', error);
    }
  }
}
</script>

<template>
  <div class="card">
    <button type="button" @click="play_video" class="play-button">
      <PlayIcon class="play-icon" />
    </button>
  </div>
</template>

<style scoped>
.play-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1.5em;
  background: transparent;
  color: white;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.3s ease;
  outline: none;
}

.play-button:focus {
  outline: none;
}

.play-button:active {
  outline: none;
}

.play-button:hover {
  color: #42b883;
  transform: scale(1.1);
  filter: drop-shadow(0 0 10px #42b883aa);
}

.play-icon {
  width: 5em;
  height: 5em;
}

.read-the-docs {
  color: #888;
}
</style>
