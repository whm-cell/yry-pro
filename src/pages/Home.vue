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
        
        <!-- 右侧转盘 -->
        <div class="w-full flex flex-col items-center justify-center animate-fadeIn-delay">
          <!-- 转盘组件 -->
          <LuckyWheelComp ref="wheelRef" />
          
          
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