<script setup lang="ts">
import { ref, onMounted } from 'vue';
import VideoPlayer from '../components/VideoPlayer.vue';

const videoUrl = ref('');

onMounted(() => {
  // 从URL参数获取视频文件路径
  const urlParams = new URLSearchParams(window.location.search);
  const fileParam = urlParams.get('file');
  if (fileParam) {
    videoUrl.value = decodeURIComponent(fileParam);
  }
});
</script>

<template>
  <div class="video-player-view">
    <VideoPlayer :videoUrl="videoUrl" />
  </div>
</template>

<style scoped>
.video-player-view {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 全局样式重置 */
:global(body) {
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>
