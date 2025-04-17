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
          class="bg-blue-100 px-3 py-1 rounded-full flex items-center"
        >
          <span class="font-medium">{{ word.word }}</span>
          <button 
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
                    @click="openColorEditor(item)" 
                    class="ml-2 text-blue-600 hover:text-blue-900"
                  >
                    修改
                  </button>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 space-x-2">
                <button 
                  @click="toggleActiveWord(item)" 
                  class="text-blue-600 hover:text-blue-900"
                  :disabled="isActiveWordUpdating"
                >
                  {{ isWordActive(item.id) ? '取消活动' : '设为活动' }}
                </button>
                <button 
                  @click="deleteVocabulary(item.id)" 
                  class="text-red-600 hover:text-red-900"
                  :disabled="isDeleting"
                >
                  删除
                </button>
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
});
const selectedImage = ref(null);
const imagePreview = ref('');
const isSaving = ref(false);
const isLoading = ref(true);
const isDeleting = ref(false);
const vocabularyList = ref([]);
const activeWords = ref([]);
const isActiveWordUpdating = ref(false);

// 颜色编辑对话框状态
const colorEditDialog = ref({
  visible: false,
  wordId: null,
  color: '',
});
const isColorUpdating = ref(false);

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
      color: newWord.value.color, // 添加颜色值
    });
    
    console.log('单词保存结果:', result);
    
    // 3. 重置表单
    newWord.value = { word: '', translation: '', color: '#636e72' };
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
</script>

<style scoped>
.vocabulary-manager {
  /* 不再使用@apply */
}
</style> 