<template>
  <div class="sound-uploader">
    <!-- 上传区域 -->
    <div 
      class="upload-area"
      @click="triggerFileInput"
      @dragover.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
      @drop.prevent="onFileDrop"
      :class="{ 'is-dragging': isDragging }"
    >
      <input 
        type="file" 
        ref="fileInput" 
        @change="onFileSelected" 
        accept="audio/mp3,audio/wav,audio/ogg"
        class="hidden"
      />
      
      <div v-if="isUploading" class="upload-progress">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: `${uploadProgress}%` }"></div>
        </div>
        <div class="progress-text">上传中 {{ uploadProgress }}%</div>
      </div>
      
      <div v-else-if="audioPreview" class="preview">
        <audio controls :src="audioPreview" class="audio-preview"></audio>
        <div class="preview-name">{{ audioFile?.name || selectedAudioName }}</div>
      </div>
      
      <div v-else class="upload-prompt">
        <div class="icon">🎵</div>
        <div class="text">点击或拖拽上传音频文件</div>
        <div class="sub-text">支持 MP3, WAV, OGG 格式</div>
      </div>
    </div>

    <!-- 预设音效列表 -->
    <div class="preset-sounds">
      <h3 class="text-sm font-medium mb-2">预设音效</h3>
      <div class="grid grid-cols-2 gap-2">
        <div 
          v-for="preset in presetSounds" 
          :key="preset.name"
          @click="selectPresetSound(preset)"
          class="preset-item"
          :class="{ 'selected': selectedPreset === preset.name }"
        >
          <div class="preset-icon">{{ preset.icon }}</div>
          <div class="preset-info">
            <div class="preset-name">{{ preset.name }}</div>
            <div class="flex items-center">
              <button @click.stop="playPresetSound(preset)" class="play-btn">
                ▶
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 已上传音效列表 -->
    <div class="uploaded-sounds" v-if="audioList.length > 0">
      <h3 class="text-sm font-medium mb-2">已上传音效</h3>
      <div class="grid grid-cols-2 gap-2">
        <div 
          v-for="audioName in audioList" 
          :key="audioName"
          @click="selectAudio(audioName)"
          class="audio-item"
          :class="{ 'selected': selectedAudioName === audioName }"
        >
          <div class="audio-icon">🎵</div>
          <div class="audio-info">
            <div class="audio-name">{{ getShortName(audioName) }}</div>
            <div class="flex items-center">
              <button @click.stop="playAudio(audioName)" class="play-btn">
                ▶
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 加载中提示 -->
    <div v-if="loadingAudios" class="loading-indicator">
      加载音效列表中...
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button 
        class="primary-btn" 
        @click="confirmAudioSelection" 
        :disabled="!selectedAudioName && !audioFile && !selectedPreset"
      >
        使用选中的音效
      </button>
    </div>

    <!-- 提示消息 -->
    <div v-if="message" class="message" :class="{ 'error': messageType === 'error' }">
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts" name="SoundUploader">
import { ref, onMounted, defineExpose } from 'vue';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';

// 定义事件
const emit = defineEmits(['sound-selected']);

// 状态变量
const fileInput = ref<HTMLInputElement | null>(null);
const audioFile = ref<File | null>(null);
const audioPreview = ref<string | null>(null);
const isUploading = ref(false);
const uploadProgress = ref(0);
const isDragging = ref(false);
const audioList = ref<string[]>([]);
const loadingAudios = ref(true);
const selectedAudioName = ref<string | null>(null);
const selectedPreset = ref<string | null>(null);
const message = ref('');
const messageType = ref<'info' | 'error'>('info');

// 预设音效
const presetSounds = [
  { 
    name: 'spin', 
    url: 'https://assets.mixkit.co/sfx/preview/mixkit-arcade-game-jump-coin-216.mp3',
    icon: '🎮',
    description: '转盘旋转音效'
  },
  { 
    name: 'win', 
    url: 'https://assets.mixkit.co/sfx/preview/mixkit-achievement-bell-600.mp3',
    icon: '🏆',
    description: '中奖音效'
  }
];

// 预览音频对象
let previewAudio: HTMLAudioElement | null = null;

// 组件加载时获取音频列表
onMounted(async () => {
  await refreshAudioList();
});

// 刷新音频列表
async function refreshAudioList() {
  try {
    loadingAudios.value = true;
    // 确保音频目录存在
    await invoke('ensure_sounds_dir');
    // 获取音频列表
    const sounds = await invoke<string[]>('list_sounds');
    audioList.value = sounds;
  } catch (error) {
    showMessage(`获取音频列表失败: ${error}`, 'error');
  } finally {
    loadingAudios.value = false;
  }
}

// 显示消息
function showMessage(msg: string, type: 'info' | 'error' = 'info') {
  message.value = msg;
  messageType.value = type;
  setTimeout(() => {
    message.value = '';
  }, 3000);
}

// 获取音频URL
function getAudioUrl(audioName: string) {
  try {
    return convertFileSrc(`${appLocalDataDir}/sounds/${audioName}`);
  } catch (error) {
    console.error('获取音频URL失败:', error);
    return '';
  }
}

// 获取音频简短名称
function getShortName(name: string) {
  if (name.length > 15) {
    return name.substring(0, 12) + '...';
  }
  return name;
}

// 点击上传区域触发文件选择
function triggerFileInput() {
  if (!isUploading.value && fileInput.value) {
    fileInput.value.click();
  }
}

// 文件选择事件处理
async function onFileSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    const file = target.files[0];
    if (isAudioFile(file)) {
      audioFile.value = file;
      audioPreview.value = URL.createObjectURL(file);
      selectedPreset.value = null;
      await uploadFile(file);
    } else {
      showMessage('请选择有效的音频文件 (MP3, WAV, OGG)', 'error');
    }
  }
}

// 文件拖放处理
async function onFileDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    const file = event.dataTransfer.files[0];
    if (isAudioFile(file)) {
      audioFile.value = file;
      audioPreview.value = URL.createObjectURL(file);
      selectedPreset.value = null;
      await uploadFile(file);
    } else {
      showMessage('请选择有效的音频文件 (MP3, WAV, OGG)', 'error');
    }
  }
}

// 检查是否是音频文件
function isAudioFile(file: File): boolean {
  const validTypes = ['audio/mp3', 'audio/wav', 'audio/ogg', 'audio/mpeg'];
  return validTypes.includes(file.type);
}

// 上传文件到Tauri后端
async function uploadFile(file: File) {
  try {
    isUploading.value = true;
    uploadProgress.value = 0;
    
    // 创建文件名 (使用时间戳避免冲突)
    const timestamp = new Date().getTime();
    const ext = file.name.split('.').pop() || 'mp3';
    const fileName = `sound_${timestamp}.${ext}`;
    
    // 读取文件为base64
    const fileContent = await readFileAsDataURL(file);
    
    // 上传到后端
    // 模拟上传进度
    const interval = setInterval(() => {
      uploadProgress.value += 10;
      if (uploadProgress.value >= 90) {
        clearInterval(interval);
      }
    }, 100);
    
    // 调用Rust函数保存文件
    await invoke<string>('save_sound', { 
      fileData: fileContent,
      fileName: fileName
    });
    
    // 完成上传
    uploadProgress.value = 100;
    setTimeout(() => {
      isUploading.value = false;
      showMessage('音频上传成功!');
      refreshAudioList();
      selectedAudioName.value = fileName;
    }, 500);
    
  } catch (error) {
    isUploading.value = false;
    showMessage(`上传失败: ${error}`, 'error');
  }
}

// 读取文件为DataURL
function readFileAsDataURL(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error('文件读取失败'));
    reader.readAsDataURL(file);
  });
}

// 选择已上传的音频
function selectAudio(audioName: string) {
  selectedAudioName.value = audioName;
  audioFile.value = null;
  audioPreview.value = getAudioUrl(audioName);
  selectedPreset.value = null;
}

// 播放音频
function playAudio(audioName: string) {
  // 停止当前播放的预览
  if (previewAudio) {
    previewAudio.pause();
    previewAudio = null;
  }
  
  // 创建新的音频播放
  const audio = new Audio(getAudioUrl(audioName));
  previewAudio = audio;
  audio.play().catch(e => console.error('无法播放音频', e));
}

// 选择预设音效
function selectPresetSound(preset: typeof presetSounds[0]) {
  selectedPreset.value = preset.name;
  selectedAudioName.value = null;
  audioFile.value = null;
  audioPreview.value = preset.url;
}

// 播放预设音效
function playPresetSound(preset: typeof presetSounds[0]) {
  // 停止当前播放的预览
  if (previewAudio) {
    previewAudio.pause();
    previewAudio = null;
  }
  
  // 创建新的音频播放
  const audio = new Audio(preset.url);
  previewAudio = audio;
  audio.play().catch(e => console.error('无法播放音频', e));
}

// 确认音频选择
function confirmAudioSelection() {
  if (selectedAudioName.value) {
    emit('sound-selected', {
      type: 'uploaded',
      name: selectedAudioName.value,
      url: getAudioUrl(selectedAudioName.value)
    });
  } else if (selectedPreset.value) {
    const preset = presetSounds.find(p => p.name === selectedPreset.value);
    if (preset) {
      emit('sound-selected', {
        type: 'preset',
        name: preset.name,
        url: preset.url
      });
    }
  }
}

// 暴露方法给父组件
defineExpose({
  selectAudio,
  selectPresetSound
});
</script>

<script lang="ts">
export default {
  name: 'SoundUploader'
}
</script>

<style scoped>
.sound-uploader {
  background-color: white;
  border-radius: 0.75rem;
  padding: 1rem;
  border: 1px solid rgb(229, 231, 235);
}

.upload-area {
  border: 2px dashed rgb(209, 213, 219);
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  min-height: 120px;
}

.upload-area.is-dragging {
  border-color: rgb(59, 130, 246);
  background-color: rgb(239, 246, 255);
}

.hidden {
  display: none;
}

.upload-prompt {
  text-align: center;
}

.upload-prompt .icon {
  font-size: 1.875rem;
  margin-bottom: 0.5rem;
}

.upload-prompt .text {
  color: rgb(75, 85, 99);
  font-weight: 500;
}

.upload-prompt .sub-text {
  font-size: 0.75rem;
  color: rgb(107, 114, 128);
  margin-top: 0.25rem;
}

.preview {
  width: 100%;
  text-align: center;
}

.audio-preview {
  width: 100%;
  max-width: 20rem;
  margin-left: auto;
  margin-right: auto;
  margin-bottom: 0.5rem;
}

.preview-name {
  font-size: 0.875rem;
  color: rgb(75, 85, 99);
}

.upload-progress {
  width: 100%;
  padding: 0.5rem;
}

.progress-bar {
  width: 100%;
  height: 0.75rem;
  background-color: rgb(229, 231, 235);
  border-radius: 9999px;
  overflow: hidden;
  margin-bottom: 0.25rem;
}

.progress-fill {
  height: 100%;
  background-color: rgb(59, 130, 246);
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

.progress-text {
  font-size: 0.75rem;
  text-align: center;
  color: rgb(75, 85, 99);
}

.preset-sounds, .uploaded-sounds {
  margin-bottom: 1rem;
}

.preset-item, .audio-item {
  display: flex;
  align-items: center;
  padding: 0.5rem;
  border-radius: 0.5rem;
  cursor: pointer;
  border: 1px solid rgb(229, 231, 235);
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

.preset-item:hover, .audio-item:hover {
  background-color: rgb(249, 250, 251);
}

.preset-item.selected, .audio-item.selected {
  background-color: rgb(239, 246, 255);
  border-color: rgb(147, 197, 253);
}

.preset-icon, .audio-icon {
  margin-right: 0.5rem;
  font-size: 1.25rem;
}

.preset-info, .audio-info {
  flex-grow: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.preset-name, .audio-name {
  font-size: 0.875rem;
  font-weight: 500;
  color: rgb(55, 65, 81);
}

.play-btn {
  font-size: 0.75rem;
  background-color: rgb(243, 244, 246);
  border-radius: 9999px;
  width: 1.5rem;
  height: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: rgb(55, 65, 81);
}

.play-btn:hover {
  background-color: rgb(229, 231, 235);
}

.action-buttons {
  margin-top: 1rem;
  display: flex;
  justify-content: flex-end;
}

.primary-btn {
  padding-left: 1rem;
  padding-right: 1rem;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  background-color: rgb(59, 130, 246);
  color: white;
  border-radius: 0.5rem;
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}

.primary-btn:hover {
  background-color: rgb(37, 99, 235);
}

.primary-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.loading-indicator {
  text-align: center;
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
  font-size: 0.875rem;
  color: rgb(107, 114, 128);
}

.message {
  margin-top: 0.5rem;
  font-size: 0.875rem;
  text-align: center;
  padding: 0.5rem;
  border-radius: 0.5rem;
  background-color: #e6f7ff;
  color: #1890ff;
}

.message.error {
  background-color: #fff2f0;
  color: #ff4d4f;
}
</style> 