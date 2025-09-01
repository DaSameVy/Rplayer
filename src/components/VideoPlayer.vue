<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { 
  PlayIcon, 
  PauseIcon, 
  SpeakerWaveIcon, 
  SpeakerXMarkIcon,
  BackwardIcon,
  ForwardIcon,
  StopIcon,
  ArrowsPointingOutIcon,
  ArrowsPointingInIcon
} from '@heroicons/vue/24/solid';

// 播放状态
const isPlaying = ref(false);
const isPaused = ref(false);
const isMuted = ref(false);
const isFullscreen = ref(false);

// 控制显示
const showControls = ref(true);
const hideControlsTimer = ref<number | null>(null);

// 音量和进度
const volume = ref(50);
const currentTime = ref(0);
const duration = ref(100);
const isDragging = ref(false);

// Props
const props = defineProps<{
  videoUrl?: string;
}>();

// 播放控制函数
async function togglePlayPause() {
  try {
    if (isPlaying.value) {
      await invoke('pause_video');
      isPlaying.value = false;
      isPaused.value = true;
    } else {
      await invoke('resume_video');
      isPlaying.value = true;
      isPaused.value = false;
    }
    showControlsTemporarily();
  } catch (error) {
    console.error('播放控制失败:', error);
  }
}

async function stopVideo() {
  try {
    await invoke('stop_video');
    isPlaying.value = false;
    isPaused.value = false;
    currentTime.value = 0;
    showControlsTemporarily();
  } catch (error) {
    console.error('停止播放失败:', error);
  }
}

async function seek(seconds: number) {
  try {
    await invoke('seek_relative', { seconds });
    showControlsTemporarily();
  } catch (error) {
    console.error('快进/快退失败:', error);
  }
}

async function seekToPosition(event: MouseEvent) {
  if (isDragging.value) return;
  
  const progressBar = event.currentTarget as HTMLElement;
  const rect = progressBar.getBoundingClientRect();
  const clickX = event.clientX - rect.left;
  const percentage = clickX / rect.width;
  const newTime = percentage * duration.value;
  
  try {
    await invoke('seek_to_time', { time: newTime });
    currentTime.value = newTime;
    showControlsTemporarily();
  } catch (error) {
    console.error('进度跳转失败:', error);
  }
}

async function toggleMute() {
  try {
    isMuted.value = !isMuted.value;
    await invoke('set_muted', { muted: isMuted.value });
    showControlsTemporarily();
  } catch (error) {
    console.error('静音控制失败:', error);
  }
}

async function setVolume(event: Event) {
  const target = event.target as HTMLInputElement;
  volume.value = parseInt(target.value);
  
  try {
    await invoke('set_volume', { volume: volume.value / 100 });
    if (volume.value === 0) {
      isMuted.value = true;
    } else {
      isMuted.value = false;
    }
    showControlsTemporarily();
  } catch (error) {
    console.error('音量控制失败:', error);
  }
}

function toggleFullscreen() {
  try {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
      isFullscreen.value = true;
    } else {
      document.exitFullscreen();
      isFullscreen.value = false;
    }
    showControlsTemporarily();
  } catch (error) {
    console.error('全屏切换失败:', error);
  }
}

function handleMouseMove() {
  showControlsTemporarily();
}

function showControlsTemporarily() {
  showControls.value = true;
  
  if (hideControlsTimer.value) {
    clearTimeout(hideControlsTimer.value);
  }
  
  if (isPlaying.value) {
    hideControlsTimer.value = window.setTimeout(() => {
      showControls.value = false;
    }, 3000);
  }
}

function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  
  if (hours > 0) {
    return `${hours}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

// 进度条拖拽
function startDrag(event: MouseEvent) {
  isDragging.value = true;
  document.addEventListener('mousemove', onDrag);
  document.addEventListener('mouseup', stopDrag);
  onDrag(event);
}

function onDrag(event: MouseEvent) {
  const progressContainer = document.querySelector('.progress-container') as HTMLElement;
  if (!progressContainer) return;
  
  const rect = progressContainer.getBoundingClientRect();
  const clickX = Math.max(0, Math.min(event.clientX - rect.left, rect.width));
  const percentage = clickX / rect.width;
  currentTime.value = percentage * duration.value;
}

async function stopDrag() {
  if (isDragging.value) {
    try {
      await invoke('seek_to_time', { time: currentTime.value });
    } catch (error) {
      console.error('拖拽跳转失败:', error);
    }
  }
  
  isDragging.value = false;
  document.removeEventListener('mousemove', onDrag);
  document.removeEventListener('mouseup', stopDrag);
}

// 键盘快捷键
function handleKeydown(event: KeyboardEvent) {
  switch (event.code) {
    case 'Space':
      event.preventDefault();
      togglePlayPause();
      break;
    case 'ArrowLeft':
      event.preventDefault();
      seek(-10);
      break;
    case 'ArrowRight':
      event.preventDefault();
      seek(10);
      break;
    case 'ArrowUp':
      event.preventDefault();
      volume.value = Math.min(100, volume.value + 5);
      setVolume({ target: { value: volume.value.toString() } } as any);
      break;
    case 'ArrowDown':
      event.preventDefault();
      volume.value = Math.max(0, volume.value - 5);
      setVolume({ target: { value: volume.value.toString() } } as any);
      break;
    case 'KeyM':
      event.preventDefault();
      toggleMute();
      break;
    case 'KeyF':
      event.preventDefault();
      toggleFullscreen();
      break;
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
  
  // 模拟一些初始数据
  duration.value = 120; // 2分钟的视频
  
  // 模拟播放进度更新
  const updateProgress = () => {
    if (isPlaying.value && !isDragging.value) {
      currentTime.value += 0.1;
      if (currentTime.value >= duration.value) {
        currentTime.value = duration.value;
        isPlaying.value = false;
        isPaused.value = false;
      }
    }
  };
  
  setInterval(updateProgress, 100);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
  if (hideControlsTimer.value) {
    clearTimeout(hideControlsTimer.value);
  }
});
</script>

<template>
  <div class="video-player" @mousemove="handleMouseMove">
    <!-- 视频显示区域 -->
    <div class="video-area">
      <div class="video-placeholder">
        <div class="video-info">
          <h3>视频播放区域</h3>
          <p>{{ props.videoUrl || '未选择视频文件' }}</p>
        </div>
      </div>
    </div>

    <!-- 控制层遮罩 -->
    <div class="controls-overlay" :class="{ 'visible': showControls }">
      <!-- 中央播放按钮 -->
      <!-- <div v-if="!isPlaying" class="center-play" @click="togglePlayPause">
        <PlayIcon class="center-play-icon" />
      </div> -->

      <!-- 底部控制栏 -->
      <div class="bottom-controls">
        <!-- 进度条 -->
        <div class="progress-container" @click="seekToPosition">
          <div class="progress-track">
            <div 
              class="progress-fill" 
              :style="{ width: duration > 0 ? (currentTime / duration) * 100 + '%' : '0%' }"
            ></div>
            <div 
              class="progress-thumb"
              :style="{ left: duration > 0 ? (currentTime / duration) * 100 + '%' : '0%' }"
              @mousedown="startDrag"
            ></div>
          </div>
        </div>

        <!-- 控制按钮栏 -->
        <div class="controls-bar">
          <!-- 左侧控制组 -->
          <div class="controls-left">
            <button @click="togglePlayPause" class="control-btn primary" title="播放/暂停">
              <PlayIcon v-if="!isPlaying" class="control-icon" />
              <PauseIcon v-else class="control-icon" />
            </button>

            <button @click="stopVideo" class="control-btn" title="停止">
              <StopIcon class="control-icon" />
            </button>

            <button @click="seek(-10)" class="control-btn" title="后退10秒">
              <BackwardIcon class="control-icon" />
            </button>

            <button @click="seek(10)" class="control-btn" title="前进10秒">
              <ForwardIcon class="control-icon" />
            </button>

            <div class="volume-controls">
              <button @click="toggleMute" class="control-btn" title="静音">
                <SpeakerWaveIcon v-if="!isMuted && volume > 0" class="control-icon" />
                <SpeakerXMarkIcon v-else class="control-icon" />
              </button>
              
              <div class="volume-slider-container">
                <input 
                  type="range" 
                  min="0" 
                  max="100" 
                  :value="volume"
                  @input="setVolume"
                  class="volume-slider"
                  :class="{ 'muted': isMuted }"
                />
              </div>
            </div>
          </div>

          <!-- 中间时间显示 -->
          <div class="time-display">
            <span class="current-time">{{ formatTime(currentTime) }}</span>
            <span class="time-separator">/</span>
            <span class="total-time">{{ formatTime(duration) }}</span>
          </div>

          <!-- 右侧控制组 -->
          <div class="controls-right">
            <button @click="toggleFullscreen" class="control-btn" title="全屏">
              <ArrowsPointingOutIcon v-if="!isFullscreen" class="control-icon" />
              <ArrowsPointingInIcon v-else class="control-icon" />
            </button>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<style scoped>
.video-player {
  position: relative;
  width: 100%;
  height: 100vh;
  background: #000;
  border-radius: 0;
  overflow: hidden;
  box-shadow: none;
}

.video-area {
  width: 100%;
  height: 100%;
  position: relative;
}

.video-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #000;
}

.video-info {
  text-align: center;
  color: white;
}

.video-info h3 {
  margin: 0 0 10px 0;
  font-size: 1.5em;
  font-weight: 600;
}

.video-info p {
  margin: 0;
  opacity: 0.8;
  font-size: 0.9em;
}

.controls-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  transition: all 0.3s ease;
  opacity: 0;
  pointer-events: none;
}

.controls-overlay.visible {
  opacity: 1;
  pointer-events: all;
}

.center-play {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(66, 184, 131, 0.9);
  border: none;
  border-radius: 50%;
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.center-play:hover {
  background: rgba(66, 184, 131, 1);
  transform: translate(-50%, -50%) scale(1.1);
}

.center-play-icon {
  width: 32px;
  height: 32px;
  color: white;
  margin-left: 4px;
}

.bottom-controls {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  padding: 0 0 16px 0;
  background: rgba(0,0,0,0.95);
  box-shadow: 0 -2px 16px rgba(0,0,0,0.5);
  z-index: 20;
}

.progress-container {
  margin-bottom: 8px;
  cursor: pointer;
  padding: 0 32px;
}

.progress-track {
  position: relative;
  height: 6px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
}

.progress-fill {
  height: 100%;
  background: #42b883;
  border-radius: 3px;
  transition: width 0.1s ease;
}

.progress-thumb {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 16px;
  height: 16px;
  background: #42b883;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
  opacity: 0;
}

.progress-container:hover .progress-thumb {
  opacity: 1;
}

.controls-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 32px;
}

.controls-left,
.controls-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.control-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  border-radius: 8px;
  color: white;
  cursor: pointer;
  padding: 8px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(10px);
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: scale(1.05);
}

.control-btn.primary {
  background: rgba(66, 184, 131, 0.8);
}

.control-btn.primary:hover {
  background: rgba(66, 184, 131, 1);
}

.control-icon {
  width: 20px;
  height: 20px;
}

.volume-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 10px;
}

.volume-slider-container {
  width: 80px;
}

.volume-slider {
  width: 100%;
  height: 4px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 2px;
  outline: none;
  cursor: pointer;
  appearance: none;
}

.volume-slider::-webkit-slider-thumb {
  appearance: none;
  width: 14px;
  height: 14px;
  background: #42b883;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
}

.volume-slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.volume-slider.muted {
  opacity: 0.5;
}

.time-display {
  display: flex;
  align-items: center;
  gap: 5px;
  color: white;
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 14px;
  font-weight: 500;
  background: rgba(0, 0, 0, 0.4);
  padding: 6px 12px;
  border-radius: 6px;
  backdrop-filter: blur(10px);
}

.time-separator {
  opacity: 0.6;
}

.shortcuts-hint {
  position: absolute;
  top: 20px;
  right: 20px;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  padding: 12px;
  color: white;
  font-size: 12px;
  transition: all 0.3s ease;
  opacity: 0;
  pointer-events: none;
}

.shortcuts-hint.visible {
  opacity: 1;
}

.shortcut-item {
  margin-bottom: 4px;
  opacity: 0.8;
}

.shortcut-item:last-child {
  margin-bottom: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .controls-bar {
    flex-direction: column;
    gap: 15px;
  }
  
  .volume-controls {
    margin-left: 0;
  }
  
  .shortcuts-hint {
    display: none;
  }
}
</style>
