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
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">音标</label>
          <input 
            v-model="newWord.phonetic" 
            type="text" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">例句</label>
          <input 
            v-model="newWord.example" 
            type="text" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
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
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">音标</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">图片</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in vocabularyList" :key="item.id" class="hover:bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{{ item.word }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.translation }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.phonetic || '-' }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <div v-if="item.image_path" class="w-12 h-12 overflow-hidden rounded-md">
                  <img :src="getImageUrl(item.image_path)" class="w-full h-full object-cover" alt="单词图片" />
                </div>
                <span v-else>-</span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { basename } from '@tauri-apps/api/path';
import { message } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';

// 状态
const newWord = ref({
  word: '',
  translation: '',
  phonetic: '',
  example: '',
});
const selectedImage = ref(null);
const imagePreview = ref('');
const isSaving = ref(false);
const isLoading = ref(true);
const isDeleting = ref(false);
const vocabularyList = ref([]);

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

// 初始化
onMounted(async () => {
  await loadVocabularyList();
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

// 获取图片URL
const getImageUrl = (imagePath) => {
  // 使用更通用的方式，适应不同系统环境
  try {
    // 判断是否已经是完整路径
    if (imagePath.startsWith('file://') || imagePath.startsWith('http://') || imagePath.startsWith('https://')) {
      return imagePath;
    }
    
    // 尝试使用Tauri的convertFileSrc API来获取正确的图片URL
    // 对于本地文件，该函数会创建一个特殊的URL，允许Tauri显示本地文件
    try {
      // 假设图片存储在固定目录
      return convertFileSrc(`/Users/coolm/softs/temp_files/images/${imagePath}`);
    } catch (e) {
      console.error('使用convertFileSrc失败:', e);
      // 回退到基本文件URL
      return `file:///Users/coolm/softs/temp_files/images/${imagePath}`;
    }
  } catch (error) {
    console.error('构建图片URL失败:', error);
    return ''; // 返回空字符串表示加载失败
  }
};

// 保存单词
const saveVocabulary = async () => {
  debugger
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
      image_path: fileName, // 修正参数名：从imagePath改为image_path
      phonetic: newWord.value.phonetic || null,
      example: newWord.value.example || null
    });
    
    console.log('单词保存结果:', result);
    
    // 3. 重置表单
    newWord.value = { word: '', translation: '', phonetic: '', example: '' };
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
</script>

<style scoped>
.vocabulary-manager {
  /* 不再使用@apply */
}
</style> 