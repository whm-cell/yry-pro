<template>
  <div class="image-uploader">
    <div class="uploader-header">
      <h3 class="uploader-title">图片上传</h3>
      <div class="uploader-actions">
        <button 
          class="refresh-btn" 
          @click="refreshImageList"
          title="刷新图片列表"
        >
          <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 0 1-9 9c-2.39 0-4.68-.94-6.4-2.6"></path>
            <path d="M3 12a9 9 0 0 1 9-9c2.39 0 4.68.94 6.4 2.6"></path>
            <path d="M9 18l-6-6 6-6"></path>
            <path d="M15 6l6 6-6 6"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- 上传区域 -->
    <div 
      class="upload-area"
      @dragover.prevent 
      @drop.prevent="onFileDrop"
      :class="{ 'dragging': isDragging }"
      @dragenter.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
    >
      <div v-if="isUploading" class="upload-progress">
        <div class="progress-bar">
          <div class="progress-inner" :style="{ width: `${uploadProgress}%` }"></div>
        </div>
        <div class="progress-text">上传中 {{ uploadProgress }}%</div>
      </div>
      <template v-else>
        <svg v-if="!imagePreview" viewBox="0 0 24 24" width="48" height="48" stroke="currentColor" stroke-width="1" fill="none" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        <img v-else :src="imagePreview" alt="预览" class="preview-image" />
        <div class="upload-text">
          <span v-if="!imagePreview">点击或拖放图片到这里</span>
          <span v-else>拖放新图片或点击更换</span>
        </div>
        <input
          type="file"
          ref="fileInput"
          accept="image/*"
          class="file-input"
          @change="onFileSelected"
        />
      </template>
    </div>

    <!-- 图片列表 -->
    <div class="image-list-container">
      <h4 class="image-list-title">已上传图片</h4>
      <div v-if="loadingImages" class="loading-images">
        <span>加载中...</span>
      </div>
      <div v-else-if="imageList.length === 0" class="no-images">
        <span>暂无图片</span>
      </div>
      <div v-else class="image-list">
        <div
          v-for="(image, index) in imageList"
          :key="index"
          class="image-item"
          :class="{ 'selected': selectedImageName === image }"
          @click="selectImage(image)"
        >
          <img :src="getImageUrl(image)" :alt="image" class="thumbnail" />
          <div class="image-name">{{ getShortName(image) }}</div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button 
        class="primary-btn" 
        @click="confirmImageSelection" 
        :disabled="!selectedImageName && !imageFile"
      >
        使用选中的图片
      </button>
    </div>

    <!-- 提示消息 -->
    <div v-if="message" class="message" :class="{ 'error': messageType === 'error' }">
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, defineExpose } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';

// 定义事件
const emit = defineEmits(['image-selected']);

// 状态变量
const fileInput = ref<HTMLInputElement | null>(null);
const imageFile = ref<File | null>(null);
const imagePreview = ref<string | null>(null);
const isUploading = ref(false);
const uploadProgress = ref(0);
const isDragging = ref(false);
const imageList = ref<string[]>([]);
const loadingImages = ref(true);
const selectedImageName = ref<string | null>(null);
const message = ref('');
const messageType = ref<'info' | 'error'>('info');

// 组件加载时获取图片列表
onMounted(async () => {
  await refreshImageList();
});

// 刷新图片列表
async function refreshImageList() {
  try {
    loadingImages.value = true;
    // 确保图片目录存在
    await invoke('ensure_images_dir');
    // 获取图片列表
    const images = await invoke<string[]>('list_images');
    imageList.value = images;
  } catch (error) {
    showMessage(`获取图片列表失败: ${error}`, 'error');
  } finally {
    loadingImages.value = false;
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

// 获取图片URL
function getImageUrl(imageName: string) {
  try {
    return convertFileSrc(`${appLocalDataDir}/images/${imageName}`);
  } catch (error) {
    console.error('获取图片URL失败:', error);
    return '';
  }
}

// 获取图片简短名称
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
    if (isImageFile(file)) {
      imageFile.value = file;
      imagePreview.value = URL.createObjectURL(file);
      await uploadFile(file);
    } else {
      showMessage('请选择有效的图片文件 (JPG, PNG, GIF)', 'error');
    }
  }
}

// 文件拖放处理
async function onFileDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    const file = event.dataTransfer.files[0];
    if (isImageFile(file)) {
      imageFile.value = file;
      imagePreview.value = URL.createObjectURL(file);
      await uploadFile(file);
    } else {
      showMessage('请选择有效的图片文件 (JPG, PNG, GIF)', 'error');
    }
  }
}

// 检查是否是图片文件
function isImageFile(file: File): boolean {
  const validTypes = ['image/jpeg', 'image/png', 'image/gif'];
  return validTypes.includes(file.type);
}

// 上传文件到Tauri后端
async function uploadFile(file: File) {
  try {
    isUploading.value = true;
    uploadProgress.value = 0;
    
    // 创建文件名 (使用时间戳避免冲突)
    const timestamp = new Date().getTime();
    const ext = file.name.split('.').pop() || 'jpg';
    const fileName = `image_${timestamp}.${ext}`;
    
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
    const savedPath = await invoke<string>('save_image', { 
      fileData: fileContent,
      fileName: fileName
    });
    
    // 完成上传
    uploadProgress.value = 100;
    setTimeout(() => {
      isUploading.value = false;
      showMessage('图片上传成功!');
      refreshImageList();
      selectedImageName.value = fileName;
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

// 选择已上传的图片
function selectImage(imageName: string) {
  selectedImageName.value = imageName;
  imageFile.value = null;
  imagePreview.value = getImageUrl(imageName);
}

// 确认图片选择
function confirmImageSelection() {
  if (selectedImageName.value) {
    const imageUrl = getImageUrl(selectedImageName.value);
    emit('image-selected', {
      name: selectedImageName.value,
      url: imageUrl
    });
  } else if (imageFile.value && imagePreview.value) {
    emit('image-selected', {
      name: imageFile.value.name,
      url: imagePreview.value
    });
  }
}

// 对外暴露的方法
defineExpose({
  refreshImageList,
  showMessage,
  triggerFileInput
});

export default {}; // 添加默认导出以解决导入错误
</script>

<style scoped>
.image-uploader {
  width: 100%;
  background-color: #ffffff;
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
}

.uploader-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.uploader-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.refresh-btn {
  background: transparent;
  border: none;
  color: #666;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  transition: all 0.2s;
}

.refresh-btn:hover {
  background: #f0f0f0;
  color: #333;
}

.upload-area {
  width: 100%;
  min-height: 160px;
  border: 2px dashed #d1d5db;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  padding: 16px;
  position: relative;
  transition: all 0.2s;
  overflow: hidden;
  margin-bottom: 16px;
}

.upload-area:hover {
  border-color: #9ca3af;
}

.dragging {
  border-color: #60a5fa;
  background-color: rgba(96, 165, 250, 0.05);
}

.file-input {
  opacity: 0;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  cursor: pointer;
}

.upload-text {
  margin-top: 12px;
  color: #6b7280;
  font-size: 14px;
  text-align: center;
}

.preview-image {
  max-width: 100%;
  max-height: 200px;
  object-fit: contain;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.upload-progress {
  width: 100%;
  padding: 0 16px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-inner {
  height: 100%;
  background-color: #60a5fa;
  border-radius: 4px;
  transition: width 0.2s;
}

.progress-text {
  text-align: center;
  font-size: 14px;
  color: #6b7280;
}

.image-list-container {
  margin-bottom: 16px;
}

.image-list-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 8px;
  color: #374151;
}

.image-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 8px;
  max-height: 220px;
  overflow-y: auto;
  padding: 4px;
}

.image-item {
  position: relative;
  border-radius: 6px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s;
  border: 2px solid transparent;
  background-color: #f9fafb;
}

.image-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.image-item.selected {
  border-color: #60a5fa;
}

.thumbnail {
  width: 100%;
  height: 80px;
  object-fit: cover;
}

.image-name {
  padding: 4px;
  font-size: 12px;
  color: #4b5563;
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.action-buttons {
  display: flex;
  justify-content: center;
  margin-top: 16px;
}

.primary-btn {
  background-color: #60a5fa;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.primary-btn:hover {
  background-color: #3b82f6;
}

.primary-btn:disabled {
  background-color: #9ca3af;
  cursor: not-allowed;
}

.message {
  margin-top: 12px;
  padding: 8px;
  border-radius: 6px;
  background-color: #e0f2fe;
  color: #0369a1;
  font-size: 14px;
  text-align: center;
}

.message.error {
  background-color: #fee2e2;
  color: #b91c1c;
}

.loading-images, .no-images {
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #6b7280;
  font-size: 14px;
  background-color: #f9fafb;
  border-radius: 6px;
}
</style> 