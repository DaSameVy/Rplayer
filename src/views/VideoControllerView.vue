<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { 
  PlayIcon, 
  PauseIcon, 
  SpeakerWaveIcon, 
  SpeakerXMarkIcon,
  BackwardIcon,
  ForwardIcon,
  StopIcon,
  XMarkIcon
} from '@heroicons/vue/24/solid';

// 播放状态
const isPlaying = ref(false);
const isPaused = ref(false);
const isMuted = ref(false);
const volume = ref(50);
const currentTime = ref(0);
const duration = ref(100);
const isDragging = ref(false);

// 视频和窗口相关
const videoUrl = ref('');
const videoWindow = ref<WebviewWindow | null>(null);

onMounted(async () => {
  // 从URL参数获取视频文件路径
  const urlParams = new URLSearchParams(window.location.search);
  const videoParam = urlParams.get('video');
  const videoWindowId = urlParams.get('videoWindow');
  
  if (videoParam) {
    videoUrl.value = decodeURIComponent(videoParam);
    console.log('📹 从URL获取视频路径:', videoUrl.value);
  }
  
  if (videoWindowId) {
    // 获取对应的视频窗口引用
    console.log('🔍 尝试获取视频窗口引用:', videoWindowId);
    try {
      videoWindow.value = await WebviewWindow.getByLabel(videoWindowId);
      if (videoWindow.value) {
        console.log('🎬 成功获取到视频窗口引用:', videoWindowId);
        console.log('🎬 视频窗口标签:', videoWindow.value.label);
      } else {
        console.warn('⚠️ 未能获取到视频窗口引用');
      }
    } catch (error) {
      console.error('❌ 获取视频窗口引用失败:', error);
    }
  }
  
  // 如果有视频URL，自动开始播放
  if (videoUrl.value && videoWindow.value) {
    console.log('🚀 准备自动开始播放视频...');
    console.log('📹 视频路径:', videoUrl.value);
    console.log('🎬 视频窗口:', videoWindow.value.label);
    
    // 延迟更长时间确保窗口完全加载和渲染完成
    setTimeout(async () => {
      console.log('⏰ 开始初始化视频播放...');
      await initializeVideo();
      isPlaying.value = true;
      isPaused.value = false;
      console.log('✅ 视频播放状态已更新');
    }, 2000); // 延迟2秒确保窗口完全加载
  } else {
    console.log('⚠️ 无法自动播放视频:');
    console.log('  - 视频URL:', videoUrl.value || '无');
    console.log('  - 视频窗口:', videoWindow.value ? videoWindow.value.label : '无');
  }
  
  // 监听当前窗口的关闭事件 - 使用简化版本
  const currentWindow = getCurrentWindow();
  console.log('设置窗口关闭监听器');
  
  // 使用window beforeunload事件作为备选方案
  window.addEventListener('beforeunload', () => {
    console.log('📱 页面即将卸载，清理资源');
    // 这里可以做一些同步的清理工作
  });
  
  // 添加页面可见性监听
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'hidden') {
      console.log('📱 页面变为不可见状态');
    }
  });
  
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

// 播放控制函数
async function togglePlayPause() {
  try {
    if (isPlaying.value) {
      await invoke('pause_video_command');
      isPlaying.value = false;
      isPaused.value = true;
    } else {
      if (videoUrl.value && !isPaused.value) {
        // 如果是第一次播放，需要初始化视频
        await initializeVideo();
      } else {
        await invoke('resume_video_command');
      }
      isPlaying.value = true;
      isPaused.value = false;
    }
  } catch (error) {
    console.error('播放控制失败:', error);
  }
}

async function initializeVideo() {
  try {
    if (!videoWindow.value) {
      console.error('❌ 视频窗口未找到');
      return;
    }
    
    // 获取视频窗口的标签和尺寸信息
    const windowLabel = videoWindow.value.label;
    const size = await videoWindow.value.innerSize();
    
    console.log('🎬 初始化视频播放参数:');
    console.log('  - 窗口标签:', windowLabel);
    console.log('  - 视频路径:', videoUrl.value);
    console.log('  - 窗口尺寸:', `${size.width}x${size.height}`);
    
    // 播放视频并绑定到指定的视频窗口
    console.log('📡 调用 play_video_in_window 命令...');
    await invoke('play_video_in_window', {
      windowLabel: windowLabel,
      uri: videoUrl.value,
      x: 0,
      y: 0,
      width: size.width,
      height: size.height
    });
    
    console.log('✅ 视频初始化命令发送成功');
    console.log(`🎥 视频开始播放，绑定到视频窗口 '${windowLabel}': ${size.width}x${size.height}`);
  } catch (error) {
    console.error('❌ 初始化视频播放失败:', error);
    console.error('错误详情:', JSON.stringify(error, null, 2));
  }
}

async function stopVideo() {
  try {
    await invoke('stop_video_command');
    isPlaying.value = false;
    isPaused.value = false;
    currentTime.value = 0;
  } catch (error) {
    console.error('停止播放失败:', error);
  }
}

async function selectVideoFile() {
  try {
    const selectedFile = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'Video Files',
        extensions: ['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm', 'm4v', '3gp']
      }]
    });
    
    if (selectedFile) {
      console.log('📁 选择的视频文件:', selectedFile);
      videoUrl.value = selectedFile;
      
      // 自动开始播放选择的视频
      if (videoWindow.value) {
        await initializeVideo();
        isPlaying.value = true;
        isPaused.value = false;
        console.log('✅ 开始播放选择的视频');
      }
    }
  } catch (error) {
    console.error('❌ 选择视频文件失败:', error);
  }
}

async function testGStreamer() {
  try {
    console.log('🧪 测试 GStreamer...');
    const result = await invoke('test_gstreamer');
    console.log('✅ GStreamer 测试成功:', result);
    alert(`GStreamer 测试成功!\n${result}`);
  } catch (error) {
    console.error('❌ GStreamer 测试失败:', error);
    alert(`GStreamer 测试失败!\n${error}`);
  }
}

async function testWindowHandle() {
  try {
    if (!videoWindow.value) {
      alert('❌ 没有找到视频窗口，无法测试窗口句柄');
      return;
    }
    
    console.log('🔍 测试窗口句柄...');
    const windowLabel = videoWindow.value.label;
    console.log('🎬 视频窗口标签:', windowLabel);
    
    const result = await invoke('test_window_handle', {
      windowLabel: windowLabel
    });
    
    console.log('✅ 窗口句柄测试成功:', result);
    alert(`窗口句柄测试成功!\n${result}`);
  } catch (error) {
    console.error('❌ 窗口句柄测试失败:', error);
    alert(`窗口句柄测试失败!\n${error}`);
  }
}

async function closePlayer() {
  console.log('🔴 开始关闭播放器');
  
  try {
    // 1. 停止视频播放
    if (isPlaying.value) {
      console.log('⏹️ 停止视频播放');
      await invoke('stop_video_command').catch((e) => {
        console.warn('停止视频播放时出现警告:', e);
      });
      isPlaying.value = false;
      isPaused.value = false;
    }

    // 2. 关闭视频窗口 - 使用简化方式
    if (videoWindow.value) {
      console.log('🎬 关闭视频窗口');
      try {
        await videoWindow.value.close();
        console.log('✅ 视频窗口关闭成功');
      } catch (error) {
        console.warn('关闭视频窗口时出现警告:', error);
        // 继续执行，不让这个错误阻止整个关闭流程
      }
    }

    // 3. 延迟一点时间确保清理完成
    await new Promise(resolve => setTimeout(resolve, 100));

    // 4. 关闭当前控制器窗口 - 使用更直接的方式
    console.log('🎮 关闭控制器窗口');
    const currentWindow = getCurrentWindow();
    await currentWindow.close();

  } catch (error) {
    console.error('❌ 关闭播放器过程中发生错误:', error);
    // 即使发生错误，也尝试强制关闭当前窗口
    try {
      const currentWindow = getCurrentWindow();
      await currentWindow.close();
    } catch (finalError) {
      console.error('❌ 强制关闭窗口失败:', finalError);
    }
  }
}

async function seek(seconds: number) {
  try {
    await invoke('seek_relative', { seconds });
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
  } catch (error) {
    console.error('进度跳转失败:', error);
  }
}

async function toggleMute() {
  try {
    isMuted.value = !isMuted.value;
    await invoke('set_muted', { muted: isMuted.value });
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
  } catch (error) {
    console.error('音量控制失败:', error);
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
</script>

<template>
  <div class="controller-window">
    <!-- 调试信息 -->
    <div style="background: red; color: white; padding: 10px; margin-bottom: 10px;">
      <h2>控制器调试信息</h2>
      <p>视频URL: {{ videoUrl || '无' }}</p>
      <p>视频窗口: {{ videoWindow?.label || '无' }}</p>
      <p>播放状态: {{ isPlaying ? '播放中' : '已停止' }}</p>
    </div>
    
    <!-- 视频信息区域 -->
    <div class="video-info">
      <h3>播放控制器</h3>
      <p v-if="videoUrl" class="video-path">{{ videoUrl.split('/').pop() || videoUrl.split('\\').pop() }}</p>
      <button v-if="!videoUrl" @click="selectVideoFile" class="select-video-btn">
        选择视频文件
      </button>
      <button @click="testGStreamer" class="test-btn">
        测试 GStreamer
      </button>
      <button @click="testWindowHandle" class="test-btn">
        测试窗口句柄
      </button>
    </div>

    <!-- 进度条区域 -->
    <div class="progress-section">
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
      
      <!-- 时间显示 -->
      <div class="time-display">
        <span class="current-time">{{ formatTime(currentTime) }}</span>
        <span class="time-separator">/</span>
        <span class="total-time">{{ formatTime(duration) }}</span>
      </div>
    </div>

    <!-- 控制按钮区域 -->
    <div class="controls-section">
      <div class="main-controls">
        <button @click="seek(-10)" class="control-btn" title="后退10秒">
          <BackwardIcon class="control-icon" />
        </button>

        <button @click="togglePlayPause" class="control-btn primary" title="播放/暂停">
          <PlayIcon v-if="!isPlaying" class="control-icon" />
          <PauseIcon v-else class="control-icon" />
        </button>

        <button @click="seek(10)" class="control-btn" title="前进10秒">
          <ForwardIcon class="control-icon" />
        </button>

        <button @click="stopVideo" class="control-btn" title="停止">
          <StopIcon class="control-icon" />
        </button>

        <button @click="closePlayer" class="control-btn close-btn" title="关闭播放器">
          <XMarkIcon class="control-icon" />
        </button>
      </div>

      <!-- 音量控制 -->
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
  </div>
</template>

<style scoped>
.controller-window {
  width: 100%;
  height: 100%;
  background: linear-gradient(145deg, #1e1e1e 0%, #2d2d2d 100%);
  color: white;
  display: flex;
  flex-direction: column;
  padding: 20px;
  box-sizing: border-box;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.video-info {
  text-align: center;
  margin-bottom: 30px;
}

.video-info h3 {
  margin: 0 0 10px 0;
  color: #42b883;
  font-size: 1.4em;
  font-weight: 500;
}

.video-path {
  margin: 0;
  font-size: 14px;
  opacity: 0.7;
  word-break: break-all;
  padding: 0 20px;
}

.select-video-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 20px;
  padding: 8px 20px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: 10px;
}

.select-video-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
}

.test-btn {
  background: linear-gradient(135deg, #ff6b6b 0%, #ffa500 100%);
  color: white;
  border: none;
  border-radius: 20px;
  padding: 8px 20px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-top: 10px;
  margin-left: 10px;
}

.test-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(255, 107, 107, 0.3);
  background: linear-gradient(135deg, #ffa500 0%, #ff6b6b 100%);
}

.progress-section {
  margin-bottom: 30px;
}

.progress-container {
  margin-bottom: 15px;
  cursor: pointer;
  padding: 10px 0;
}

.progress-track {
  position: relative;
  height: 6px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
  border: 1px solid rgba(255, 255, 255, 0.2);
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

.time-display {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
  color: white;
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 16px;
  font-weight: 500;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 8px 16px;
  border-radius: 8px;
  width: fit-content;
  margin: 0 auto;
}

.time-separator {
  opacity: 0.6;
}

.controls-section {
  display: flex;
  flex-direction: column;
  gap: 25px;
  align-items: center;
}

.main-controls {
  display: flex;
  align-items: center;
  gap: 15px;
  justify-content: center;
}

.control-btn {
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  color: white;
  cursor: pointer;
  padding: 12px;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 50px;
  min-height: 50px;
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.control-btn.primary {
  background: rgba(66, 184, 131, 0.8);
  border-color: #42b883;
}

.control-btn.primary:hover {
  background: rgba(66, 184, 131, 1);
  box-shadow: 0 4px 12px rgba(66, 184, 131, 0.4);
}

.control-btn.close-btn {
  background: rgba(220, 53, 69, 0.8);
  border-color: #dc3545;
}

.control-btn.close-btn:hover {
  background: rgba(220, 53, 69, 1);
  box-shadow: 0 4px 12px rgba(220, 53, 69, 0.4);
  transform: scale(1.1);
}

.control-icon {
  width: 24px;
  height: 24px;
}

.volume-controls {
  display: flex;
  align-items: center;
  gap: 15px;
}

.volume-slider-container {
  width: 120px;
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
  width: 16px;
  height: 16px;
  background: #42b883;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
}

.volume-slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
  box-shadow: 0 0 8px rgba(66, 184, 131, 0.6);
}

.volume-slider.muted {
  opacity: 0.5;
}

/* 全局样式重置 - 更强制性的重写 */
:global(html) {
  margin: 0 !important;
  padding: 0 !important;
  height: 100% !important;
  background: linear-gradient(145deg, #1e1e1e 0%, #2d2d2d 100%) !important;
}

:global(body) {
  margin: 0 !important;
  padding: 0 !important;
  overflow: hidden !important;
  background: linear-gradient(145deg, #1e1e1e 0%, #2d2d2d 100%) !important;
  display: block !important; /* 覆盖 style.css 中的 flex */
  place-items: unset !important; /* 移除居中对齐 */
  min-width: unset !important;
  min-height: unset !important;
  height: 100vh !important;
  width: 100vw !important;
}

:global(#app) {
  background: linear-gradient(145deg, #1e1e1e 0%, #2d2d2d 100%) !important;
  max-width: none !important; /* 覆盖 style.css 中的 max-width */
  margin: 0 !important; /* 覆盖居中对齐 */
  padding: 0 !important; /* 移除 padding */
  text-align: left !important; /* 移除居中文本对齐 */
  height: 100vh !important;
  width: 100vw !important;
  display: block !important;
}

:global(#app) {
  background: transparent !important;
  max-width: none !important;
  padding: 0 !important;
  margin: 0 !important;
  height: 100vh !important;
}
</style>
