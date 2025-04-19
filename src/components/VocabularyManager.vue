<template>
  <div class="vocabulary-manager p-4">
    <h2 class="text-xl font-bold mb-4">单词管理</h2>
    
    <!-- 添加单词表单 -->
    <div class="bg-white p-4 rounded-lg shadow mb-6">
      <h3 class="text-lg font-semibold mb-3">添加新单词</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">单词</label>
          <input 
            v-model="newWord.word" 
            type="text" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">翻译</label>
          <input 
            v-model="newWord.translation" 
            type="text" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>
      
      <!-- 颜色选择器 -->
      <div class="mt-4">
        <label class="block text-sm font-medium text-gray-700 mb-1">颜色</label>
        <div class="flex items-center">
          <input 
            v-model="newWord.color" 
            type="color" 
            class="w-12 h-8 cursor-pointer border border-gray-300 rounded"
          />
          <input 
            v-model="newWord.color" 
            type="text" 
            class="ml-3 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="#636e72"
          />
          <div class="ml-3 w-8 h-8 rounded" :style="{backgroundColor: newWord.color}"></div>
        </div>
      </div>
      
      <!-- 图片上传 -->
      <div class="mt-4">
        <label class="block text-sm font-medium text-gray-700 mb-1">图片</label>
        <div class="flex items-center">
          <div class="relative">
            <input 
              type="file" 
              accept="image/*" 
              @change="handleImageSelect" 
              class="absolute inset-0 opacity-0 cursor-pointer"
            />
            <button 
              class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
            >
              选择图片
            </button>
          </div>
          <span v-if="selectedImage" class="ml-3 text-sm text-gray-600">
            已选择: {{ selectedImage.name }}
          </span>
          <div v-if="imagePreview" class="ml-4 w-16 h-16 overflow-hidden rounded-md">
            <img :src="imagePreview" class="w-full h-full object-cover" />
          </div>
        </div>
      </div>
      
      <div class="mt-4">
        <button 
          @click="saveVocabulary" 
          class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
          :disabled="isSaving"
        >
          {{ isSaving ? '保存中...' : '保存单词' }}
        </button>
      </div>
    </div>
    
    <!-- 活动单词管理 -->
    <div class="bg-white p-4 rounded-lg shadow mb-6">
      <h3 class="text-lg font-semibold mb-3">活动单词 ({{ activeWords.length }}/5)</h3>
      <div v-if="activeWords.length === 0" class="text-gray-500 mb-3">
        您尚未设置活动单词，请从下方单词列表中选择最多5个单词设为活动单词
      </div>
      <div v-else class="flex flex-wrap gap-2 mb-3">
        <div 
          v-for="word in activeWords" 
          :key="word.id" 
          :class="[
            'px-3 py-1 rounded-full flex items-center', 
            word.is_default ? 'bg-yellow-100 border border-yellow-300' : 'bg-blue-100'
          ]"
        >
          <span class="font-medium">{{ word.word }}</span>
          <span v-if="word.is_default" class="ml-1 text-xs text-orange-500">(默认)</span>
          <button 
            v-if="!word.is_default"
            @click="removeActiveWord(word.id)" 
            class="ml-2 text-red-500 hover:text-red-700"
          >
            &times;
          </button>
        </div>
      </div>
    </div>
    
    <!-- 单词列表 -->
    <div class="bg-white p-4 rounded-lg shadow">
      <h3 class="text-lg font-semibold mb-3">单词列表</h3>
      
      <div v-if="isLoading" class="py-4 text-center text-gray-500">
        加载中...
      </div>
      
      <div v-else-if="vocabularyList.length === 0" class="py-4 text-center text-gray-500">
        暂无单词，请添加一些单词
      </div>
      
      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">单词</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">翻译</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">图片</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">颜色</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in vocabularyList" :key="item.id" class="hover:bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{{ item.word }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.translation }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <div v-if="item.image_path" class="w-12 h-12 overflow-hidden rounded-md">
                  <img :src="convertFileSrc(item.image_path)" class="w-full h-full object-cover" alt="单词图片" />
                </div>
                <span v-else>-</span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <div class="flex items-center">
                  <div class="w-6 h-6 rounded mr-2" :style="{backgroundColor: item.color || '#636e72'}"></div>
                  <span>{{ item.color || '#636e72' }}</span>
                  <button 
                    v-if="!item.is_default"
                    @click="openColorEditor(item)" 
                    class="ml-2 text-blue-600 hover:text-blue-900"
                  >
                    修改
                  </button>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 space-x-2">
                <button 
                  v-if="!item.is_default"
                  @click="openEditDialog(item)" 
                  class="text-green-600 hover:text-green-900"
                >
                  编辑
                </button>
                <button 
                  v-if="item.is_default"
                  @click="openEditImageDialog(item)" 
                  class="text-green-600 hover:text-green-900"
                >
                  修改图片
                </button>
                <button 
                  v-if="!item.is_default || !isWordActive(item.id)"
                  @click="toggleActiveWord(item)" 
                  class="text-blue-600 hover:text-blue-900"
                  :disabled="isActiveWordUpdating"
                >
                  {{ isWordActive(item.id) ? '取消活动' : '设为活动' }}
                </button>
                <button 
                  v-if="!item.is_default"
                  @click="deleteVocabulary(item.id)" 
                  class="text-red-600 hover:text-red-900"
                  :disabled="isDeleting"
                >
                  删除
                </button>
                <span v-if="item.is_default" class="text-gray-400">默认单词</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 颜色编辑对话框 -->
    <div v-if="colorEditDialog.visible" class="fixed inset-0 flex items-center justify-center z-50">
      <div class="fixed inset-0 bg-black bg-opacity-50" @click="colorEditDialog.visible = false"></div>
      <div class="bg-white p-6 rounded-lg shadow-lg relative z-10 w-96">
        <h3 class="text-lg font-semibold mb-4">修改颜色</h3>
        <div class="flex items-center mb-4">
          <input 
            v-model="colorEditDialog.color" 
            type="color" 
            class="w-14 h-10 cursor-pointer border border-gray-300 rounded"
          />
          <input 
            v-model="colorEditDialog.color" 
            type="text" 
            class="ml-3 flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <div class="ml-3 w-10 h-10 rounded" :style="{backgroundColor: colorEditDialog.color}"></div>
        </div>
        <div class="flex justify-end space-x-2">
          <button 
            @click="colorEditDialog.visible = false" 
            class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
          >
            取消
          </button>
          <button 
            @click="saveColorChange" 
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            :disabled="isColorUpdating"
          >
            {{ isColorUpdating ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 单词编辑对话框 -->
    <div v-if="editDialog.visible" class="fixed inset-0 flex items-center justify-center z-50">
      <div class="fixed inset-0 bg-black bg-opacity-50" @click="editDialog.visible = false"></div>
      <div class="bg-white p-6 rounded-lg shadow-lg relative z-10 w-[600px] max-h-[90vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-4">编辑单词</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">单词</label>
            <input 
              v-model="editDialog.word" 
              type="text" 
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">翻译</label>
            <input 
              v-model="editDialog.translation" 
              type="text" 
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>
        
        <!-- 颜色选择器 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">颜色</label>
          <div class="flex items-center">
            <input 
              v-model="editDialog.color" 
              type="color" 
              class="w-12 h-8 cursor-pointer border border-gray-300 rounded"
            />
            <input 
              v-model="editDialog.color" 
              type="text" 
              class="ml-3 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <div class="ml-3 w-8 h-8 rounded" :style="{backgroundColor: editDialog.color}"></div>
          </div>
        </div>
        
        <!-- 当前图片 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">当前图片</label>
          <div v-if="editDialog.currentImagePath" class="w-24 h-24 overflow-hidden rounded-md">
            <img :src="editDialog.currentImageSrc" class="w-full h-full object-cover" alt="当前单词图片" />
          </div>
          <span v-else class="text-gray-500">暂无图片</span>
        </div>
        
        <!-- 图片上传 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">更新图片</label>
          <div class="flex items-center">
            <div class="relative">
              <input 
                type="file" 
                accept="image/*" 
                @change="handleEditImageSelect" 
                class="absolute inset-0 opacity-0 cursor-pointer"
              />
              <button 
                class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
              >
                选择新图片
              </button>
            </div>
            <span v-if="editDialog.selectedImage" class="ml-3 text-sm text-gray-600">
              已选择: {{ editDialog.selectedImage.name }}
            </span>
            <div v-if="editDialog.imagePreview" class="ml-4 w-16 h-16 overflow-hidden rounded-md">
              <img :src="editDialog.imagePreview" class="w-full h-full object-cover" />
            </div>
          </div>
        </div>
        
        <div class="flex justify-end space-x-2">
          <button 
            @click="editDialog.visible = false" 
            class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
          >
            取消
          </button>
          <button 
            @click="saveVocabularyEdit" 
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            :disabled="isEditing"
          >
            {{ isEditing ? '保存中...' : '保存修改' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 图片编辑对话框 -->
    <div v-if="imageEditDialog.visible" class="fixed inset-0 flex items-center justify-center z-50">
      <div class="fixed inset-0 bg-black bg-opacity-50" @click="imageEditDialog.visible = false"></div>
      <div class="bg-white p-6 rounded-lg shadow-lg relative z-10 w-[500px] max-h-[90vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-4">修改单词图片</h3>
        
        <!-- 当前图片 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">当前图片</label>
          <div v-if="imageEditDialog.currentImagePath" class="w-32 h-32 overflow-hidden rounded-md">
            <img :src="imageEditDialog.currentImageSrc" class="w-full h-full object-cover" alt="当前单词图片" />
          </div>
          <span v-else class="text-gray-500">暂无图片</span>
        </div>
        
        <!-- 图片上传 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">选择新图片</label>
          <div class="flex items-center">
            <div class="relative">
              <input 
                type="file" 
                accept="image/*" 
                @change="handleImageEditSelect" 
                class="absolute inset-0 opacity-0 cursor-pointer"
              />
              <button 
                class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
              >
                选择新图片
              </button>
            </div>
            <span v-if="imageEditDialog.selectedImage" class="ml-3 text-sm text-gray-600">
              已选择: {{ imageEditDialog.selectedImage.name }}
            </span>
            <div v-if="imageEditDialog.imagePreview" class="ml-4 w-16 h-16 overflow-hidden rounded-md">
              <img :src="imageEditDialog.imagePreview" class="w-full h-full object-cover" />
            </div>
          </div>
        </div>
        
        <div class="flex justify-end space-x-2">
          <button 
            @click="imageEditDialog.visible = false" 
            class="px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md"
          >
            取消
          </button>
          <button 
            @click="saveImageEdit" 
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            :disabled="isImageEditing"
          >
            {{ isImageEditing ? '保存中...' : '保存修改' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { basename } from '@tauri-apps/api/path';
import { message } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';

// 状态
const newWord = ref({
  word: '',
  translation: '',
  color: '#636e72', // 默认颜色值
  is_default: false // 添加is_default字段
});
const selectedImage = ref(null);
const imagePreview = ref('');
const isSaving = ref(false);
const isLoading = ref(true);
const isDeleting = ref(false);
const vocabularyList = ref([]);
const activeWords = ref([]);
const isActiveWordUpdating = ref(false);

// 编辑对话框状态
const editDialog = ref({
  visible: false,
  wordId: null,
  word: '',
  translation: '',
  color: '#636e72',
  currentImagePath: '',
  currentImageSrc: '',
  selectedImage: null,
  imagePreview: '',
  originalFileName: '',
  is_default: false
});
const isEditing = ref(false);

// 颜色编辑对话框状态
const colorEditDialog = ref({
  visible: false,
  wordId: null,
  color: '',
});
const isColorUpdating = ref(false);

// 图片编辑对话框状态
const imageEditDialog = ref({
  visible: false,
  wordId: null,
  word: '',
  translation: '',
  color: '',
  currentImagePath: '',
  currentImageSrc: '',
  selectedImage: null,
  imagePreview: '',
  originalFileName: '',
  is_default: false
});
const isImageEditing = ref(false);

// 获取单词列表
const loadVocabularyList = async () => {
  try {
    isLoading.value = true;
    const data = await invoke('get_all_vocabulary');
    console.log('获取到的单词列表数据:', data);
    vocabularyList.value = data || [];
  } catch (error) {
    console.error('获取单词列表失败:', error);
    await message('获取单词列表失败: ' + error, { title: '错误' });
    vocabularyList.value = [];
  } finally {
    isLoading.value = false;
  }
};

// 获取活动单词
const loadActiveWords = async () => {
  try {
    const data = await invoke('get_active_words');
    activeWords.value = data || [];
  } catch (error) {
    console.error('获取活动单词失败:', error);
    await message('获取活动单词失败: ' + error, { title: '错误' });
    activeWords.value = [];
  }
};

// 初始化
onMounted(async () => {
  await loadVocabularyList();
  await loadActiveWords();
});

// 处理图片选择
const handleImageSelect = (event) => {
  const file = event.target.files[0];
  if (!file) return;
  
  selectedImage.value = file;
  
  // 创建预览
  const reader = new FileReader();
  reader.onload = (e) => {
    imagePreview.value = e.target.result;
  };
  reader.readAsDataURL(file);
};

// 处理编辑图片选择
const handleEditImageSelect = (event) => {
  const file = event.target.files[0];
  if (!file) return;
  
  editDialog.value.selectedImage = file;
  
  // 创建预览
  const reader = new FileReader();
  reader.onload = (e) => {
    editDialog.value.imagePreview = e.target.result;
  };
  reader.readAsDataURL(file);
};

// 处理图片编辑选择
const handleImageEditSelect = (event) => {
  const file = event.target.files[0];
  if (!file) return;
  
  imageEditDialog.value.selectedImage = file;
  
  // 创建预览
  const reader = new FileReader();
  reader.onload = (e) => {
    imageEditDialog.value.imagePreview = e.target.result;
  };
  reader.readAsDataURL(file);
};

// 保存单词
const saveVocabulary = async () => {
  if (!newWord.value.word || !newWord.value.translation) {
    await message('单词和翻译不能为空', { title: '提示' });
    return;
  }
  
  if (!selectedImage.value) {
    await message('请选择图片', { title: '提示' });
    return;
  }
  
  try {
    isSaving.value = true;
    
    // 1. 上传图片
    const reader = new FileReader();
    reader.readAsDataURL(selectedImage.value);
    
    const imageData = await new Promise((resolve, reject) => {
      reader.onload = () => resolve(reader.result);
      reader.onerror = reject;
    });
    
    const fileName = Date.now() + '_' + selectedImage.value.name;
    const imagePath = await invoke('save_image', { 
      fileData: imageData, 
      fileName: fileName 
    });
    
    console.log('图片已保存:', imagePath);
    
    // 2. 保存单词信息到数据库
    const result = await invoke('add_vocabulary', {
      word: newWord.value.word,
      translation: newWord.value.translation,
      imagePath: fileName,
      color: newWord.value.color,
      is_default: newWord.value.is_default // 传递is_default字段
    });
    
    console.log('单词保存结果:', result);
    
    // 3. 重置表单
    newWord.value = { word: '', translation: '', color: '#636e72', is_default: false };
    selectedImage.value = null;
    imagePreview.value = '';
    
    // 4. 重新加载单词列表
    await loadVocabularyList();
    
    await message('单词添加成功', { title: '成功' });
  } catch (error) {
    console.error('保存单词失败:', error);
    await message('保存单词失败: ' + error, { title: '错误' });
  } finally {
    isSaving.value = false;
  }
};

// 删除单词
const deleteVocabulary = async (id) => {
  if (!id) return;
  
  try {
    isDeleting.value = true;
    
    // 如果是活动单词，先从活动单词中移除
    if (isWordActive(id)) {
      await removeActiveWord(id);
    }
    
    await invoke('delete_vocabulary', { id });
    await loadVocabularyList();
    await message('单词删除成功', { title: '成功' });
  } catch (error) {
    console.error('删除单词失败:', error);
    await message('删除单词失败: ' + error, { title: '错误' });
  } finally {
    isDeleting.value = false;
  }
};

// 检查单词是否为活动单词
const isWordActive = (wordId) => {
  return activeWords.value.some(word => word.id === wordId);
};

// 切换单词活动状态
const toggleActiveWord = async (word) => {
  if (isWordActive(word.id)) {
    // 检查是否为默认单词
    if (word.is_default) {
      await message('默认单词不能被移除活动状态', { title: '提示' });
      return;
    }
    await removeActiveWord(word.id);
  } else {
    await addActiveWord(word);
  }
};

// 添加活动单词
const addActiveWord = async (word) => {
  try {
    isActiveWordUpdating.value = true;
    
    if (activeWords.value.length >= 5) {
      await message('活动单词最多只能设置5个，请先移除一些', { title: '提示' });
      return;
    }
    
    await invoke('add_active_word', { wordId: word.id });
    activeWords.value.push(word);
    await message(`单词"${word.word}"已设为活动单词`, { title: '成功' });
  } catch (error) {
    console.error('设置活动单词失败:', error);
    await message('设置活动单词失败: ' + error, { title: '错误' });
  } finally {
    isActiveWordUpdating.value = false;
  }
};

// 移除活动单词
const removeActiveWord = async (wordId) => {
  try {
    // 先检查是否为默认单词
    const wordToRemove = vocabularyList.value.find(w => w.id === wordId);
    if (wordToRemove && wordToRemove.is_default) {
      await message('默认单词不能被移除活动状态', { title: '提示' });
      return;
    }
    
    isActiveWordUpdating.value = true;
    await invoke('remove_active_word', { wordId });
    activeWords.value = activeWords.value.filter(w => w.id !== wordId);
    await message('已从活动单词中移除', { title: '成功' });
  } catch (error) {
    console.error('移除活动单词失败:', error);
    await message('移除活动单词失败: ' + error, { title: '错误' });
  } finally {
    isActiveWordUpdating.value = false;
  }
};

// 打开颜色编辑对话框
const openColorEditor = (word) => {
  colorEditDialog.value = {
    visible: true,
    wordId: word.id,
    color: word.color || '#636e72',
  };
};

// 保存颜色修改
const saveColorChange = async () => {
  if (!colorEditDialog.value.wordId) return;
  
  try {
    isColorUpdating.value = true;
    
    await invoke('update_vocabulary_color', { 
      id: colorEditDialog.value.wordId,
      color: colorEditDialog.value.color
    });
    
    // 更新本地列表中的颜色值
    const index = vocabularyList.value.findIndex(w => w.id === colorEditDialog.value.wordId);
    if (index !== -1) {
      vocabularyList.value[index].color = colorEditDialog.value.color;
    }
    
    // 如果是活动单词，也更新活动单词列表
    const activeIndex = activeWords.value.findIndex(w => w.id === colorEditDialog.value.wordId);
    if (activeIndex !== -1) {
      activeWords.value[activeIndex].color = colorEditDialog.value.color;
    }
    
    colorEditDialog.value.visible = false;
    await message('颜色修改成功', { title: '成功' });
  } catch (error) {
    console.error('修改颜色失败:', error);
    await message('修改颜色失败: ' + error, { title: '错误' });
  } finally {
    isColorUpdating.value = false;
  }
};

// 打开编辑对话框
const openEditDialog = (word) => {
  // 从完整路径中提取文件名
  let imageFileName = '';
  if (word.image_path) {
    // 提取文件名部分
    imageFileName = word.image_path.split('/').pop();
  }
  
  editDialog.value = {
    visible: true,
    wordId: word.id,
    word: word.word,
    translation: word.translation,
    color: word.color || '#636e72',
    currentImagePath: word.image_path,
    currentImageSrc: word.image_path ? convertFileSrc(word.image_path) : '',
    selectedImage: null,
    imagePreview: '',
    originalFileName: imageFileName,
    is_default: word.is_default
  };
};

// 保存单词编辑
const saveVocabularyEdit = async () => {
  if (!editDialog.value.word || !editDialog.value.translation) {
    await message('单词和翻译不能为空', { title: '提示' });
    return;
  }
  
  try {
    isEditing.value = true;
    
    let fileName = editDialog.value.originalFileName;
    
    // 如果选择了新图片，先上传
    if (editDialog.value.selectedImage) {
      const reader = new FileReader();
      reader.readAsDataURL(editDialog.value.selectedImage);
      
      const imageData = await new Promise((resolve, reject) => {
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
      });
      
      fileName = Date.now() + '_' + editDialog.value.selectedImage.name;
      const imagePath = await invoke('save_image', { 
        fileData: imageData, 
        fileName: fileName 
      });
      
      console.log('新图片已保存:', imagePath);
    }
    
    // 更新单词信息
    await invoke('update_vocabulary', {
      id: editDialog.value.wordId,
      word: editDialog.value.word,
      translation: editDialog.value.translation,
      imagePath: fileName,  // 只传递文件名，后端会处理完整路径
      color: editDialog.value.color
    });
    
    // 更新本地列表中的信息
    await loadVocabularyList();
    await loadActiveWords();
    
    editDialog.value.visible = false;
    await message('单词修改成功', { title: '成功' });
    
  } catch (error) {
    console.error('修改单词失败:', error);
    await message('修改单词失败: ' + error, { title: '错误' });
  } finally {
    isEditing.value = false;
  }
};

// 打开图片编辑对话框
const openEditImageDialog = (word) => {
  // 从完整路径中提取文件名
  let imageFileName = '';
  if (word.image_path) {
    // 提取文件名部分
    imageFileName = word.image_path.split('/').pop();
  }
  
  imageEditDialog.value = {
    visible: true,
    wordId: word.id,
    word: word.word,
    translation: word.translation,
    color: word.color || '#636e72',
    currentImagePath: word.image_path,
    currentImageSrc: word.image_path ? convertFileSrc(word.image_path) : '',
    selectedImage: null,
    imagePreview: '',
    originalFileName: imageFileName,
    is_default: word.is_default
  };
};

// 保存图片编辑
const saveImageEdit = async () => {
  if (!imageEditDialog.value.selectedImage) {
    await message('请选择新图片', { title: '提示' });
    return;
  }
  
  try {
    isImageEditing.value = true;
    
    // 上传新图片
    const reader = new FileReader();
    reader.readAsDataURL(imageEditDialog.value.selectedImage);
    
    const imageData = await new Promise((resolve, reject) => {
      reader.onload = () => resolve(reader.result);
      reader.onerror = reject;
    });
    
    const fileName = Date.now() + '_' + imageEditDialog.value.selectedImage.name;
    const imagePath = await invoke('save_image', { 
      fileData: imageData, 
      fileName: fileName 
    });
    
    console.log('新图片已保存:', imagePath);
    
    // 根据是否是默认单词选择不同的API更新单词信息
    if (imageEditDialog.value.is_default) {
      // 如果是默认单词，只更新图片
      await invoke('update_vocabulary_image', {
        id: imageEditDialog.value.wordId,
        imagePath: fileName  // 只传递文件名，后端会处理完整路径
      });
    } else {
      // 如果不是默认单词，更新所有信息
      await invoke('update_vocabulary', {
        id: imageEditDialog.value.wordId,
        word: imageEditDialog.value.word,
        translation: imageEditDialog.value.translation,
        imagePath: fileName,  // 只传递文件名，后端会处理完整路径
        color: imageEditDialog.value.color
      });
    }
    
    // 更新本地列表中的信息
    await loadVocabularyList();
    await loadActiveWords();
    
    imageEditDialog.value.visible = false;
    await message('单词图片修改成功', { title: '成功' });
    
  } catch (error) {
    console.error('修改单词图片失败:', error);
    await message('修改单词图片失败: ' + error, { title: '错误' });
  } finally {
    isImageEditing.value = false;
  }
};
</script>

<style scoped>
.vocabulary-manager {
  /* 不再使用@apply */
}
</style> 