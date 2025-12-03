<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const videoUrl = ref('');
const windowReady = ref(false);

onMounted(async () => {
  // 从URL参数获取视频文件路径
  const urlParams = new URLSearchParams(window.location.search);
  const videoParam = urlParams.get('video');
  if (videoParam) {
    videoUrl.value = decodeURIComponent(videoParam);
  }
  
  windowReady.value = true;
  
  // 通知主窗口这个视频窗口已准备好
  const currentWindow = getCurrentWindow();
  console.log('视频窗口已准备好，视频URL:', videoUrl.value);
});
</script>

<template>
  <div class="video-window" id="video-render-area">
    <!-- 调试：添加更明显的内容 -->
    <div class="debug-content">
      <h1 style="color: red; font-size: 3em; text-align: center;">视频窗口调试</h1>
      <p style="color: yellow; font-size: 1.5em; text-align: center;">
        窗口就绪: {{ windowReady }}
      </p>
      <p style="color: white; font-size: 1.2em; text-align: center; word-break: break-all;">
        视频URL: {{ videoUrl || '无视频URL' }}
      </p>
    </div>
    
    <!-- 这个div将作为GStreamer视频渲染的目标区域 -->
    <div class="video-content" v-if="windowReady">
      <div class="video-info">
        <h3>视频播放窗口</h3>
        <p v-if="videoUrl">{{ videoUrl }}</p>
        <p v-else>等待视频加载...</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.video-window {
  width: 100%;
  height: 100%;
  background: #000;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin: 0;
  padding: 0;
  overflow: hidden;
  position: relative;
}

.debug-content {
  position: absolute;
  top: 20px;
  left: 20px;
  right: 20px;
  z-index: 1000;
  background: rgba(0, 0, 0, 0.8);
  border: 2px solid red;
  border-radius: 10px;
  padding: 20px;
}

.video-content {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.video-info {
  text-align: center;
  color: white;
  background: rgba(0, 0, 0, 0.7);
  padding: 20px;
  border-radius: 10px;
  max-width: 80%;
}

.video-info h3 {
  margin: 0 0 10px 0;
  color: #42b883;
}

.video-info p {
  margin: 5px 0;
  font-size: 14px;
  opacity: 0.8;
  word-break: break-all;
}

/* 全局样式重置 - 更强制性的重写 */
:global(html) {
  margin: 0 !important;
  padding: 0 !important;
  height: 100% !important;
  background: #000 !important;
  overflow: hidden !important;
}

:global(body) {
  margin: 0 !important;
  padding: 0 !important;
  overflow: hidden !important;
  background: #000 !important;
  display: block !important; /* 覆盖 style.css 中的 flex */
  place-items: unset !important; /* 移除居中对齐 */
  min-width: unset !important;
  min-height: unset !important;
  height: 100vh !important;
  width: 100vw !important;
}

:global(#app) {
  background: #000 !important;
  max-width: none !important; /* 覆盖 style.css 中的 max-width */
  margin: 0 !important; /* 覆盖居中对齐 */
  padding: 0 !important; /* 移除 padding */
  text-align: left !important; /* 移除居中文本对齐 */
  height: 100vh !important;
  width: 100vw !important;
  display: block !important;
}
</style>
