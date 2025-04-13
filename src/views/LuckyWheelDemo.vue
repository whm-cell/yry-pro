<template>
  <div class="lucky-wheel-demo min-h-screen bg-gradient-to-br from-indigo-100 to-purple-100 py-8 px-4">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-center text-purple-800 mb-8">幸运单词转盘</h1>
      
      <div class="bg-white rounded-2xl shadow-xl p-6 mb-8">
        <div class="flex flex-col md:flex-row">
          <!-- 左侧：转盘组件 -->
          <div class="md:w-2/3 flex items-center justify-center">
            <LuckyWheel 
              ref="wheelRef"
              :initial-words="words"
              :spin-on-click="true"
              @spin-start="handleSpinStart"
              @spin-end="handleSpinEnd"
            />
          </div>
          
          <!-- 右侧：控制面板 -->
          <div class="md:w-1/3 mt-8 md:mt-0 md:ml-8">
            <div class="bg-purple-50 rounded-xl p-4 shadow-inner">
              <h2 class="text-xl font-bold text-purple-700 mb-4">控制面板</h2>
              
              <!-- 单词设置 -->
              <div class="mb-6">
                <h3 class="font-bold text-gray-700 mb-2">自定义单词</h3>
                <div class="space-y-2">
                  <div v-for="(word, index) in customWords" :key="index" class="flex space-x-2">
                    <input 
                      v-model="word.en" 
                      placeholder="英文单词" 
                      class="px-3 py-2 border border-gray-300 rounded text-sm flex-1" 
                    />
                    <input 
                      v-model="word.cn" 
                      placeholder="中文含义" 
                      class="px-3 py-2 border border-gray-300 rounded text-sm flex-1" 
                    />
                  </div>
                </div>
                
                <button 
                  @click="updateWords"
                  class="mt-3 w-full py-2 bg-purple-600 text-white font-medium rounded hover:bg-purple-700 transition"
                >
                  更新单词
                </button>
              </div>
              
              <!-- 转盘控制 -->
              <div>
                <button 
                  @click="spinWheel"
                  :disabled="isSpinning"
                  class="w-full py-3 bg-gradient-to-r from-yellow-500 to-orange-500 text-white font-bold rounded-lg shadow-lg hover:from-yellow-600 hover:to-orange-600 transition transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {{ isSpinning ? '旋转中...' : '转动转盘!' }}
                </button>
                
                <div class="mt-4 text-center text-gray-600 text-sm">
                  <p>{{ statusMessage }}</p>
                </div>
              </div>
            </div>
            
            <!-- 历史记录 -->
            <div class="mt-6 bg-blue-50 rounded-xl p-4 shadow-inner">
              <h3 class="font-bold text-blue-700 mb-2">历史结果</h3>
              <div class="max-h-40 overflow-y-auto">
                <div v-if="history.length === 0" class="text-gray-500 text-center py-2">
                  暂无历史记录
                </div>
                <div v-for="(item, index) in history" :key="index" class="py-2 border-b border-blue-100 last:border-0">
                  <div class="flex justify-between">
                    <span class="font-medium">{{ item.textEN }}</span>
                    <span :style="{ color: item.color }">{{ item.text }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 说明信息 -->
      <div class="text-center text-gray-600 text-sm">
        <p>点击转盘或使用转动按钮都可以启动。自定义单词会在转盘上更新。</p>
        <p class="mt-2">&copy; 2023 LuckyWheel 幸运转盘组件</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue';
import LuckyWheel from '../components/LuckyWheel.vue';
import { WheelSection, WordPair } from '../utils/wheelUtils';

// 定义LuckyWheel实例类型，包含公开的方法
interface WheelInstance {
  spin: () => boolean;
  setCustomWords: (words: WordPair[]) => void;
  closeWinnerPopup: () => void;
}

// 转盘引用
const wheelRef = ref<WheelInstance | null>(null);

// 转盘状态
const isSpinning = ref(false);
const statusMessage = ref('点击转盘或按钮开始');

// 单词配置
const words = ref<WordPair[]>([
  { cn: '苹果', en: 'Apple' },
  { cn: '香蕉', en: 'Banana' },
  { cn: '太阳', en: 'Sun' },
  { cn: '月亮', en: 'Moon' }
]);

// 自定义单词编辑区
const customWords = reactive<WordPair[]>([
  { cn: '苹果', en: 'Apple' },
  { cn: '香蕉', en: 'Banana' },
  { cn: '太阳', en: 'Sun' },
  { cn: '月亮', en: 'Moon' }
]);

// 历史记录
const history = reactive<WheelSection[]>([]);

// 转动转盘
const spinWheel = () => {
  if (wheelRef.value && !isSpinning.value) {
    wheelRef.value.spin();
  }
};

// 更新单词
const updateWords = () => {
  // 创建新数组以避免直接修改引用
  const newWords = customWords.map(word => ({ ...word }));
  words.value = newWords;
  
  // 更新转盘组件中的单词
  if (wheelRef.value) {
    wheelRef.value.setCustomWords(newWords);
  }
  
  statusMessage.value = '单词已更新';
};

// 转盘事件处理
const handleSpinStart = () => {
  isSpinning.value = true;
  statusMessage.value = '转盘旋转中...';
};

const handleSpinEnd = (section: WheelSection) => {
  isSpinning.value = false;
  
  // 添加到历史记录
  history.unshift(section);
  
  // 保持历史记录最多10项
  if (history.length > 10) {
    history.pop();
  }
  
  statusMessage.value = `停在了 ${section.textEN} (${section.text})`;
};

// 页面加载时
onMounted(() => {
  // 可以进行一些初始化操作
});
</script>

<style scoped>
/* 添加自定义样式 */
</style> 