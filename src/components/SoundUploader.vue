<template>
  <div class="sound-uploader">
    <!-- 标题和说明 -->
    <div class="mb-4">
      <h3 class="text-lg font-medium text-gray-800 mb-1">音效管理</h3>
      <p class="text-sm text-gray-500">上传或选择音效文件，支持MP3、WAV、OGG格式</p>
    </div>
    
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
        <button @click.stop="clearPreview" class="clear-preview-btn">
          重新选择
        </button>
      </div>
      
      <div v-else class="upload-prompt">
        <div class="icon">🎵</div>
        <div class="text">点击或拖拽上传音频文件</div>
        <div class="sub-text">支持 MP3, WAV, OGG 格式</div>
      </div>
    </div>

    <!-- 预设音效列表 -->
    <div v-if="presetSounds.some(s => s.available)" class="preset-sounds mt-4">
      <h3 class="text-sm font-medium mb-2 flex items-center">
        <span>预设音效</span>
        <span class="ml-2 text-xs text-gray-500">(点击选择)</span>
      </h3>
      <div class="grid grid-cols-2 gap-2">
        <div 
          v-for="preset in presetSounds.filter(s => s.available)" 
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
    <div class="uploaded-sounds mt-4" v-if="audioList.length > 0">
      <h3 class="text-sm font-medium mb-2 flex items-center">
        <span>已上传音效</span>
        <span class="ml-2 text-xs text-gray-500">(点击选择)</span>
        <button @click="refreshAudioList" class="ml-auto text-xs text-blue-500 hover:text-blue-700">
          刷新列表
        </button>
      </h3>
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
              <button @click.stop="playAudio(audioName)" class="play-btn mr-1">
                ▶
              </button>
              <button @click.stop="deleteAudio(audioName)" class="delete-btn">
                🗑️
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 音频列表为空的提示 -->
    <div v-else-if="!loadingAudios" class="empty-list mt-4">
      <div class="text-center py-6 bg-gray-50 rounded-lg border border-dashed border-gray-300">
        <div class="text-4xl mb-2">🔈</div>
        <div class="text-gray-500">您还没有上传任何音效</div>
        <div class="text-sm text-gray-400 mt-1">请使用上方区域上传音频文件</div>
      </div>
    </div>
    
    <!-- 加载中提示 -->
    <div v-if="loadingAudios" class="loading-indicator">
      加载音效列表中...
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons mt-4">
      <button 
        class="secondary-btn mr-2" 
        @click="cancelSelection"
      >
        取消
      </button>
      <button 
        class="primary-btn" 
        @click="confirmAudioSelection" 
        :disabled="!selectedAudioName && !audioFile && !selectedPreset"
      >
        使用选中的音效
      </button>
    </div>

    <!-- 提示消息 -->
    <div v-if="message" class="message mt-2" :class="{ 'error': messageType === 'error' }">
      {{ message }}
    </div>
    
    <!-- 确认删除对话框 -->
    <div v-if="showDeleteConfirm" class="delete-confirm-overlay">
      <div class="delete-confirm-dialog">
        <div class="delete-confirm-header">确认删除</div>
        <div class="delete-confirm-content">
          确定要删除音频文件 "{{ audioToDelete }}" 吗？此操作不可恢复。
        </div>
        <div class="delete-confirm-actions">
          <button @click="cancelDelete" class="cancel-btn">取消</button>
          <button @click="confirmDelete" class="confirm-delete-btn">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts" name="SoundUploader">
import { ref, onMounted, defineExpose } from 'vue';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';

// 定义事件
const emit = defineEmits(['sound-selected', 'cancel']);

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

// 删除音频相关状态
const showDeleteConfirm = ref(false);
const audioToDelete = ref('');

// 预设音效 - 使用本地路径
const presetSounds = ref([
  { 
    name: '默认旋转音效', 
    url: '/Users/coolm/softs/temp_files/sounds/cjyx_01.mp3',
    icon: '🎮',
    description: '转盘旋转音效',
    available: false
  },
  { 
    name: '默认中奖音效', 
    url: '/Users/coolm/softs/temp_files/sounds/cjyx_02.mp3',
    icon: '🏆',
    description: '中奖音效',
    available: false
  }
]);

// 预览音频对象
let previewAudio: HTMLAudioElement | null = null;

// 组件加载时获取音频列表
onMounted(async () => {
  await refreshAudioList();
  
  // 检查预设音效是否可用
  await checkPresetSounds();
});

// 检查预设音效是否可用
async function checkPresetSounds() {
  for (const preset of presetSounds.value) {
    try {
      // 检查文件是否存在
      const exists = await invoke('plugin:fs|exists', { path: preset.url }) as boolean;
      preset.available = exists;
      
      if (exists) {
        // 尝试转换本地文件路径为Tauri可访问的URL
        try {
          const url = convertFileSrc(preset.url);
          preset.url = url;
        } catch (err) {
          console.warn(`无法转换音效URL: ${preset.url}`, err);
          preset.available = false;
        }
      }
    } catch (err) {
      console.warn(`检查预设音效失败: ${preset.url}`, err);
      preset.available = false;
    }
  }
}

// 刷新音频列表
async function refreshAudioList() {
  try {
    loadingAudios.value = true;
    // 确保音频目录存在
    await invoke('ensure_sounds_dir');
    // 获取音频列表
    const sounds = await invoke<string[]>('list_sounds');
    console.log('sounds', sounds);
    audioList.value = sounds || [];
  } catch (error) {
    showMessage(`获取音频列表失败: ${error}`, 'error');
    audioList.value = [];
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

// 清除当前预览
function clearPreview() {
  if (audioPreview.value && !selectedAudioName.value && !selectedPreset.value) {
    URL.revokeObjectURL(audioPreview.value);
  }
  audioPreview.value = null;
  audioFile.value = null;
  selectedAudioName.value = null;
  selectedPreset.value = null;
  
  if (previewAudio) {
    previewAudio.pause();
    previewAudio = null;
  }
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
      clearPreview();
      audioFile.value = file;
      audioPreview.value = URL.createObjectURL(file);
      selectedPreset.value = null;
      selectedAudioName.value = null;
    } else {
      showMessage('请选择有效的音频文件 (MP3, WAV, OGG)', 'error');
    }
    // 重置文件输入以便能再次选择同一文件
    target.value = '';
  }
}

// 文件拖放处理
async function onFileDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    const file = event.dataTransfer.files[0];
    if (isAudioFile(file)) {
      clearPreview();
      audioFile.value = file;
      audioPreview.value = URL.createObjectURL(file);
      selectedPreset.value = null;
      selectedAudioName.value = null;
    } else {
      showMessage('请选择有效的音频文件 (MP3, WAV, OGG)', 'error');
    }
  }
}

// 检查文件是否为有效的音频文件
function isAudioFile(file: File): boolean {
  const validTypes = ['audio/mp3', 'audio/wav', 'audio/ogg', 'audio/mpeg'];
  return validTypes.includes(file.type) || 
         file.name.endsWith('.mp3') || 
         file.name.endsWith('.wav') || 
         file.name.endsWith('.ogg');
}

// 上传文件到服务器
async function uploadFile(file: File) {
  if (!file) return;
  
  try {
    isUploading.value = true;
    uploadProgress.value = 0;
    
    // 生成安全的文件名
    const timestamp = new Date().getTime();
    const safeFileName = `${timestamp}_${file.name.replace(/[^a-zA-Z0-9.-]/g, '_')}`;
    
    // 读取文件
    const arrayBuffer = await file.arrayBuffer();
    
    // 计算进度
    const totalSize = arrayBuffer.byteLength;
    const chunkSize = 1024 * 1024; // 1MB
    let uploaded = 0;
    
    for (let offset = 0; offset < totalSize; offset += chunkSize) {
      const chunk = arrayBuffer.slice(offset, offset + chunkSize);
      
      // 创建Uint8Array来表示二进制数据
      const data = new Uint8Array(chunk);
      
      // 上传文件块
      await invoke('save_sound_file', {
        name: safeFileName,
        data: Array.from(data),
        offset,
        final: offset + chunkSize >= totalSize
      });
      
      // 更新进度
      uploaded += chunk.byteLength;
      uploadProgress.value = Math.round((uploaded / totalSize) * 100);
    }
    
    // 上传完成，设置为已选择的音频
    selectedAudioName.value = safeFileName;
    
    // 更新音频列表
    await refreshAudioList();
    
    // 显示成功消息
    showMessage('音频文件上传成功', 'info');
    
  } catch (error) {
    console.error('上传文件失败:', error);
    showMessage(`上传失败: ${error}`, 'error');
  } finally {
    isUploading.value = false;
  }
}

// 选择预设音效
function selectPresetSound(preset: any) {
  // 停止当前播放的音频
  if (previewAudio) {
    previewAudio.pause();
  }
  
  selectedPreset.value = preset.name;
  selectedAudioName.value = null;
  
  if (audioPreview.value && !audioFile.value) {
    URL.revokeObjectURL(audioPreview.value);
  }
  
  audioPreview.value = preset.url;
  audioFile.value = null;
}

// 播放预设音效
function playPresetSound(preset: any) {
  if (previewAudio) {
    previewAudio.pause();
    previewAudio = null;
  }
  
  previewAudio = new Audio(preset.url);
  previewAudio.volume = 0.5;
  previewAudio.play().catch(error => {
    console.error('播放预设音效失败:', error);
    showMessage('播放音效失败，请检查音频文件', 'error');
  });
}

// 选择已上传的音频
function selectAudio(audioName: string) {
  // 停止当前播放的音频
  if (previewAudio) {
    previewAudio.pause();
  }
  
  selectedAudioName.value = audioName;
  selectedPreset.value = null;
  
  if (audioPreview.value && !audioFile.value) {
    URL.revokeObjectURL(audioPreview.value);
  }
  
  try {
    const url = getAudioUrl(audioName);
    console.log('url', url);
    audioPreview.value = url;
    audioFile.value = null;
  } catch (error) {
    console.error('获取音频URL失败:', error);
    showMessage('获取音频文件失败', 'error');
  }
}

// 播放已上传的音频
function playAudio(audioName: string) {
  if (previewAudio) {
    previewAudio.pause();
    previewAudio = null;
  }
  
  try {
    const url = getAudioUrl(audioName);
    previewAudio = new Audio(url);
    previewAudio.volume = 0.5;
    previewAudio.play().catch(error => {
      console.error('播放音频失败:', error);
      showMessage('播放音效失败，请检查音频文件', 'error');
    });
  } catch (error) {
    console.error('获取音频URL失败:', error);
    showMessage('获取音频文件失败', 'error');
  }
}

// 删除音频文件
function deleteAudio(audioName: string) {
  audioToDelete.value = audioName;
  showDeleteConfirm.value = true;
}

// 确认删除
async function confirmDelete() {
  if (!audioToDelete.value) return;
  
  try {
    await invoke('delete_sound_file', { name: audioToDelete.value });
    
    // 如果删除的是当前选中的音频，清除选择
    if (selectedAudioName.value === audioToDelete.value) {
      clearPreview();
    }
    
    // 更新音频列表
    await refreshAudioList();
    
    showMessage('音频文件已删除', 'info');
  } catch (error) {
    console.error('删除音频文件失败:', error);
    showMessage(`删除失败: ${error}`, 'error');
  } finally {
    showDeleteConfirm.value = false;
    audioToDelete.value = '';
  }
}

// 取消删除
function cancelDelete() {
  showDeleteConfirm.value = false;
  audioToDelete.value = '';
}

// 取消选择
function cancelSelection() {
  clearPreview();
  emit('cancel');
}

// 确认音频选择
async function confirmAudioSelection() {
  // 如果有上传文件但还未上传，先上传
  if (audioFile.value && !isUploading.value && !selectedAudioName.value) {
    await uploadFile(audioFile.value);
  }
  
  // 根据选择类型，发出不同的音效数据
  if (selectedPreset.value) {
    // 如果选择了预设音效
    const preset = presetSounds.value.find(p => p.name === selectedPreset.value);
    if (preset) {
      const soundSetting = {
        name: preset.name,
        url: preset.url,
        type: 'preset',
        description: preset.description
      };
      emit('sound-selected', soundSetting);
    }
  } else if (selectedAudioName.value) {
    // 如果选择了已上传的音效
    try {
      const soundPath = `${await appLocalDataDir()}/sounds/${selectedAudioName.value}`;
      const soundSetting = {
        name: selectedAudioName.value,
        url: soundPath,
        type: 'custom',
        description: '自定义音效'
      };
      emit('sound-selected', soundSetting);
    } catch (error) {
      console.error('获取音频路径失败:', error);
      showMessage('获取音频文件路径失败', 'error');
    }
  } else {
    showMessage('请先选择或上传音频文件', 'error');
  }
}

// 公开方法给父组件
defineExpose({
  clearPreview,
  refreshAudioList
});
</script>

<script lang="ts">
export default {
  name: 'SoundUploader'
}
</script>

<style scoped>
.sound-uploader {
  width: 100%;
}

.upload-area {
  border: 2px dashed #cbd5e0;
  border-radius: 0.5rem;
  padding: 1.5rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: #f7fafc;
  min-height: 150px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.upload-area:hover {
  border-color: #a0aec0;
  background-color: #edf2f7;
}

.upload-area.is-dragging {
  border-color: #4299e1;
  background-color: #ebf8ff;
}

.hidden {
  display: none;
}

.upload-prompt .icon {
  font-size: 2rem;
  margin-bottom: 0.5rem;
  color: #4a5568;
}

.upload-prompt .text {
  font-weight: 500;
  color: #4a5568;
  margin-bottom: 0.25rem;
}

.upload-prompt .sub-text {
  font-size: 0.75rem;
  color: #718096;
}

.preview {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.audio-preview {
  max-width: 100%;
  margin-bottom: 0.5rem;
}

.preview-name {
  font-size: 0.875rem;
  color: #4a5568;
  margin-bottom: 0.5rem;
  word-break: break-all;
  max-width: 100%;
}

.clear-preview-btn {
  font-size: 0.75rem;
  color: #4299e1;
  background: none;
  border: none;
  padding: 0.25rem 0.5rem;
  cursor: pointer;
  border-radius: 0.25rem;
}

.clear-preview-btn:hover {
  text-decoration: underline;
  color: #2b6cb0;
}

.upload-progress {
  width: 100%;
}

.progress-bar {
  width: 100%;
  height: 0.5rem;
  background-color: #edf2f7;
  border-radius: 0.25rem;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background-color: #4299e1;
  border-radius: 0.25rem;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 0.75rem;
  color: #4a5568;
}

.preset-sounds, .uploaded-sounds {
  margin-top: 1rem;
}

.preset-item, .audio-item {
  display: flex;
  align-items: center;
  padding: 0.75rem;
  border-radius: 0.375rem;
  border: 1px solid #e2e8f0;
  cursor: pointer;
  transition: all 0.2s ease;
  background-color: white;
}

.preset-item:hover, .audio-item:hover {
  border-color: #cbd5e0;
  background-color: #f7fafc;
}

.preset-item.selected, .audio-item.selected {
  border-color: #4299e1;
  background-color: #ebf8ff;
}

.preset-icon, .audio-icon {
  font-size: 1.5rem;
  margin-right: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preset-info, .audio-info {
  flex-grow: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-width: 0;
}

.preset-name, .audio-name {
  font-size: 0.875rem;
  font-weight: 500;
  color: #4a5568;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 130px;
}

.play-btn, .delete-btn {
  width: 1.5rem;
  height: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #edf2f7;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  font-size: 0.75rem;
  color: #4a5568;
  transition: all 0.2s ease;
}

.play-btn:hover {
  background-color: #4299e1;
  color: white;
}

.delete-btn:hover {
  background-color: #f56565;
  color: white;
}

.loading-indicator {
  text-align: center;
  padding: 1rem;
  color: #718096;
  font-size: 0.875rem;
}

.empty-list {
  color: #718096;
  text-align: center;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  margin-top: 1rem;
}

.primary-btn, .secondary-btn {
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s ease;
  cursor: pointer;
}

.primary-btn {
  background-color: #4299e1;
  color: white;
  border: 1px solid #4299e1;
}

.primary-btn:hover:not(:disabled) {
  background-color: #3182ce;
  border-color: #3182ce;
}

.primary-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary-btn {
  background-color: white;
  color: #4a5568;
  border: 1px solid #cbd5e0;
}

.secondary-btn:hover {
  background-color: #f7fafc;
  border-color: #a0aec0;
}

.message {
  margin-top: 0.5rem;
  padding: 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
  text-align: center;
  background-color: #c6f6d5;
  color: #2f855a;
}

.message.error {
  background-color: #fed7d7;
  color: #c53030;
}

/* 删除确认对话框样式 */
.delete-confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.delete-confirm-dialog {
  width: 90%;
  max-width: 400px;
  background-color: white;
  border-radius: 0.5rem;
  overflow: hidden;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1), 0 1px 3px rgba(0, 0, 0, 0.1);
}

.delete-confirm-header {
  padding: 1rem;
  background-color: #f7fafc;
  font-weight: 600;
  color: #4a5568;
  border-bottom: 1px solid #e2e8f0;
}

.delete-confirm-content {
  padding: 1.5rem 1rem;
  color: #4a5568;
}

.delete-confirm-actions {
  display: flex;
  justify-content: flex-end;
  padding: 1rem;
  background-color: #f7fafc;
  border-top: 1px solid #e2e8f0;
}

.cancel-btn, .confirm-delete-btn {
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 500;
  font-size: 0.875rem;
  transition: all 0.2s ease;
  cursor: pointer;
}

.cancel-btn {
  background-color: #edf2f7;
  color: #4a5568;
  border: 1px solid #e2e8f0;
  margin-right: 0.5rem;
}

.cancel-btn:hover {
  background-color: #e2e8f0;
}

.confirm-delete-btn {
  background-color: #f56565;
  color: white;
  border: 1px solid #f56565;
}

.confirm-delete-btn:hover {
  background-color: #e53e3e;
  border-color: #e53e3e;
}
</style> 