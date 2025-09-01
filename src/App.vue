<script setup lang="ts">
import { ref } from 'vue';
import PlayVideo from './components/PlayVideo.vue';
import VideoPlayer from './components/VideoPlayer.vue';

const selectedVideoUrl = ref('');
const showVideoPlayer = ref(false);

function onVideoSelected(videoUrl: string) {
  selectedVideoUrl.value = videoUrl;
  showVideoPlayer.value = true;
}

function closeVideoPlayer() {
  showVideoPlayer.value = false;
  selectedVideoUrl.value = '';
}

function openVideoPlayer() {
  showVideoPlayer.value = true;
  selectedVideoUrl.value = ''; // 空URL表示只是打开播放器界面
}
</script>

<template>
  <div v-if="!showVideoPlayer" class="main-view">
    <div class="header">
      <span class="title">Rplayer</span>
    </div>
    
    <div class="main-content">
      <div class="action-section">
        <PlayVideo @videoSelected="onVideoSelected" />
      </div>
      
      <div class="divider">或</div>
      
      <div class="action-section">
        <button @click="openVideoPlayer" class="open-player-btn">
          <span class="btn-text">打开播放器</span>
        </button>
      </div>
    </div>
  </div>

  <!-- 全屏视频播放器 -->
  <div v-else class="fullscreen-player">
    <VideoPlayer :videoUrl="selectedVideoUrl" />
    <button @click="closeVideoPlayer" class="close-btn">
      ✕
    </button>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 3em;
}

.title {
  font-size: 2.5em;
  font-weight: 600;
  color: white;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
}

.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2em;
  padding: 0 2em;
}

.action-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5em;
}

.section-title {
  color: white;
  font-size: 1.3em;
  font-weight: 500;
  margin: 0;
  text-align: center;
  opacity: 0.9;
}

.divider {
  color: rgba(255, 255, 255, 0.5);
  font-size: 1.2em;
  font-weight: 300;
  margin: 1em 0;
}

.open-player-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 50px;
  padding: 1em 2.5em;
  font-size: 1.1em;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 200px;
}

.open-player-btn:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 25px rgba(102, 126, 234, 0.5);
  background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
}

.open-player-btn:active {
  transform: translateY(-1px);
}

.btn-text {
  font-size: inherit;
}

.fullscreen-player {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: #000;
  z-index: 1000;
  display: flex;
  flex-direction: column;
}

.close-btn {
  position: absolute;
  top: 20px;
  right: 20px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  border: none;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  font-size: 20px;
  cursor: pointer;
  z-index: 1001;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.close-btn:hover {
  background: rgba(255, 0, 0, 0.7);
  transform: scale(1.1);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .main-content {
    padding: 0 1em;
  }
  
  .title {
    font-size: 2em;
  }
  
  .open-player-btn {
    padding: 0.8em 2em;
    font-size: 1em;
    min-width: 180px;
  }
}
</style>
