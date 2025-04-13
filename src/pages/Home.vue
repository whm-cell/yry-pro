<template>
  <div class="p-6 h-full bg-gradient-to-b from-blue-100 to-purple-100">
    <div class="bg-white rounded-3xl shadow-lg p-6 h-full flex flex-col border-4 border-dashed border-pink-300">
      <h2 class="text-3xl font-bold mb-6 text-center rainbow-text transform -rotate-2" style="font-family: 'Comic Sans MS', cursive;">幼儿园英语单词抽奖转盘</h2>
      
      <!-- 云朵装饰 -->
      <div class="absolute top-16 right-32 opacity-70 animate-floating">
        <svg width="80" height="48" viewBox="0 0 80 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M76 48C78.2 48 80 46.2 80 44C80 41.8 78.2 40 76 40H72.7C71.1 28.4 61.1 20 49 20C44.8 20 40.9 21.2 37.5 23.2C34.6 9.9 22.8 0 9 0C4.1 0 0 4.1 0 9C0 11.4 1 13.5 2.5 15.1C1.1 17.2 0 19.5 0 22C0 29.2 5.8 35 13 35C14 35 15.1 34.9 16 34.6C20 42.8 28.5 48 38 48H76Z" fill="#FFFFFF"/>
        </svg>
      </div>
      <div class="absolute top-32 left-20 opacity-70 animate-floating-delay">
        <svg width="60" height="36" viewBox="0 0 80 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M76 48C78.2 48 80 46.2 80 44C80 41.8 78.2 40 76 40H72.7C71.1 28.4 61.1 20 49 20C44.8 20 40.9 21.2 37.5 23.2C34.6 9.9 22.8 0 9 0C4.1 0 0 4.1 0 9C0 11.4 1 13.5 2.5 15.1C1.1 17.2 0 19.5 0 22C0 29.2 5.8 35 13 35C14 35 15.1 34.9 16 34.6C20 42.8 28.5 48 38 48H76Z" fill="#FFFFFF"/>
        </svg>
      </div>
      
      <div class="flex flex-col md:flex-row h-full">
        <!-- 左侧单词设置 -->
        <div class="w-full md:w-1/3 pr-0 md:pr-6 mb-6 md:mb-0 animate-fadeIn">
          <div class="bg-gradient-to-r from-blue-50 to-purple-50 rounded-2xl border-2 border-blue-200 p-4 h-full overflow-auto">
            <h3 class="font-bold text-xl mb-4 text-blue-700 flex items-center">
              <span class="text-2xl mr-2 animate-bounce">📝</span> 中英文单词对照
            </h3>
            
            <div class="space-y-4">
              <div 
                v-for="(word, index) in wordList" 
                :key="index"
                class="bg-white p-3 rounded-xl shadow-sm flex items-center justify-between transform hover:scale-105 transition cursor-pointer w-full"
                :class="{ 'border-2 border-blue-400': selectedWordIndex === index }"
                @click="selectedWordIndex = index; newWordCN = word.cn; newWordEN = word.en"
              >
                <div class="flex items-center">
                  <div class="w-10 h-10 rounded-full flex items-center justify-center text-xl bg-blue-100">{{ index + 1 }}</div>
                  <div class="ml-3">
                    <div class="font-bold text-purple-600">{{ word.en }}</div>
                    <div class="text-sm text-gray-500">{{ word.cn }}</div>
                  </div>
                </div>
                <div class="text-blue-500">✏️</div>
              </div>
              
              <div class="mt-6 bg-blue-50 p-4 rounded-xl">
                <h4 class="font-medium text-blue-700 mb-2">修改单词</h4>
                <div class="space-y-3">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">英文单词</label>
                    <input 
                      v-model="newWordEN" 
                      type="text" 
                      class="w-full rounded-lg border border-gray-300 p-2 focus:ring-2 focus:ring-blue-400 focus:outline-none"
                      placeholder="输入英文单词"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">中文翻译</label>
                    <input 
                      v-model="newWordCN" 
                      type="text" 
                      class="w-full rounded-lg border border-gray-300 p-2 focus:ring-2 focus:ring-blue-400 focus:outline-none"
                      placeholder="输入中文翻译"
                    />
                  </div>
                  <div class="flex justify-between">
                    <button 
                      @click="updateWord"
                      class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transform transition hover:scale-105"
                    >
                      更新单词
                    </button>
                    <button 
                      @click="resetWords" 
                      class="bg-gray-200 hover:bg-gray-300 text-gray-700 px-4 py-2 rounded-lg"
                    >
                      重置
                    </button>
                  </div>
                </div>
                <div class="flex justify-between items-center mt-2">
                  <p class="text-sm text-gray-500">当前修改第 {{ selectedWordIndex + 1 }} 个单词</p>
                </div>
                
                <!-- 更新状态提示 -->
                <div v-if="updateStatus" class="mt-2 text-center text-green-500 bg-green-50 py-1 px-2 rounded-lg text-sm animate-pulse">
                  {{ updateStatus }}
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 右侧转盘 -->
        <div class="w-full md:w-2/3 flex flex-col items-center justify-center animate-fadeIn-delay">
          <!-- 转盘组件 -->
          <LuckyWheelComp ref="wheelRef" />
          
          <!-- 操作说明 -->
          <div class="bg-gradient-to-r from-blue-50 to-purple-50 rounded-2xl border-2 border-blue-200 p-4 max-w-md">
            <h3 class="font-bold text-lg mb-2 text-blue-700 flex items-center">
              <span class="text-xl mr-2">📝</span> 操作说明
            </h3>
            <ul class="space-y-2 text-sm">
              <li class="flex items-start">
                <span class="text-green-500 mr-2">✓</span>
                <span>左侧可以自定义英文和中文单词对照</span>
              </li>
              <li class="flex items-start">
                <span class="text-green-500 mr-2">✓</span>
                <span>点击"开始抽奖"按钮开始转动转盘</span>
              </li>
              <li class="flex items-start">
                <span class="text-green-500 mr-2">✓</span>
                <span>转盘停止后，被选中的单词会放大显示</span>
              </li>
              <li class="flex items-start">
                <span class="text-green-500 mr-2">✓</span>
                <span>如果抽到"魔法袋子"，会有特殊惊喜哦！</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
      
      <!-- 底部装饰 -->
      <div class="w-full flex justify-between mt-auto pt-4">
        <div class="text-4xl animate-bounce-delay-1">🦒</div>
        <div class="text-4xl animate-bounce-delay-2">🐘</div>
        <div class="text-4xl animate-bounce-delay-3">🦁</div>
        <div class="text-4xl animate-bounce-delay-4">🐯</div>
        <div class="text-4xl animate-bounce-delay-5">🦊</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, defineComponent, watch } from 'vue';
import LuckyWheelComp from '../components/LuckyWheel.vue';

// 学生数据
const students = [
  { name: '小明', class: '向日葵班', avatar: '👦', color: '#ff6b6b' },
  { name: '小红', class: '向日葵班', avatar: '👧', color: '#4ecdc4' },
  { name: '小刚', class: '蒲公英班', avatar: '👦', color: '#ff9f1c' },
  { name: '小丽', class: '蒲公英班', avatar: '👧', color: '#a78bfa' },
  { name: '小华', class: '星星班', avatar: '👦', color: '#ffbe0b' },
  { name: '小美', class: '星星班', avatar: '👧', color: '#fb5607' },
  { name: '小亮', class: '彩虹班', avatar: '👦', color: '#8ecae6' },
  { name: '小芳', class: '彩虹班', avatar: '👧', color: '#8ac926' }
];

// 转盘引用
const isSpinning = ref(false);

// 自定义单词
const wordList = ref<{cn: string, en: string}[]>([
  { cn: '苹果', en: 'Apple' },
  { cn: '香蕉', en: 'Banana' },
  { cn: '太阳', en: 'Sun' },
  { cn: '月亮', en: 'Moon' }
]);
const newWordCN = ref('');
const newWordEN = ref('');
const selectedWordIndex = ref(0);
const updateStatus = ref('');

// 更新自定义单词
const updateWord = () => {
  if (newWordEN.value.trim() && newWordCN.value.trim()) {
    wordList.value[selectedWordIndex.value] = {
      en: newWordEN.value.trim(),
      cn: newWordCN.value.trim()
    };
    
    newWordEN.value = '';
    newWordCN.value = '';
  }
};

// 重置单词
const resetWords = () => {
  wordList.value = [
    { cn: '苹果', en: 'Apple' },
    { cn: '香蕉', en: 'Banana' },
    { cn: '太阳', en: 'Sun' },
    { cn: '月亮', en: 'Moon' }
  ];
  
  newWordEN.value = '';
  newWordCN.value = '';
};
</script>

<style scoped>
/* 彩虹文字 */
.rainbow-text {
  background: linear-gradient(to right, #ff6b6b, #feca57, #1dd1a1, #5f27cd, #ff9ff3);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  color: transparent;
}

/* 按钮发光效果 */
.button-glow {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
  transform: translateX(-100%);
  animation: button-shine 3s infinite;
}

@keyframes button-shine {
  0% {
    transform: translateX(-100%);
  }
  20% {
    transform: translateX(100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.spin-button:active {
  transform: scale(0.95);
}

/* 淡入动画 */
.animate-fadeIn {
  animation: fadeIn 0.5s ease-in forwards;
}

.animate-fadeIn-delay {
  animation: fadeIn 0.5s ease-in 0.2s forwards;
  opacity: 0;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 浮动动画 */
.animate-floating {
  animation: floating 6s ease-in-out infinite;
}

.animate-floating-delay {
  animation: floating 6s ease-in-out 2s infinite;
}

@keyframes floating {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

/* 震动动画 */
.animate-shake {
  animation: shake 0.5s cubic-bezier(.36,.07,.19,.97) both;
}

@keyframes shake {
  10%, 90% {
    transform: translate3d(-1px, 0, 0);
  }
  20%, 80% {
    transform: translate3d(2px, 0, 0);
  }
  30%, 50%, 70% {
    transform: translate3d(-4px, 0, 0);
  }
  40%, 60% {
    transform: translate3d(4px, 0, 0);
  }
}

/* 弹跳动画 */
.animate-bounce-delay-1 {
  animation: bounce 2s infinite 0.1s;
}
.animate-bounce-delay-2 {
  animation: bounce 2s infinite 0.3s;
}
.animate-bounce-delay-3 {
  animation: bounce 2s infinite 0.5s;
}
.animate-bounce-delay-4 {
  animation: bounce 2s infinite 0.7s;
}
.animate-bounce-delay-5 {
  animation: bounce 2s infinite 0.9s;
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}
</style> 