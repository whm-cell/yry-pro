<template>
  <div class="wheel-container">
    <div class="relative w-80 h-80 md:w-96 md:h-96 mx-auto mb-8 transform hover:scale-105 transition-transform duration-300">
      <!-- 转盘装饰 - 彩虹边框 -->
      <div class="absolute inset-0 w-full h-full rounded-full border-8 border-dashed animate-spin-slow" style="border-image: linear-gradient(45deg, #ff6b6b, #feca57, #1dd1a1, #5f27cd, #ff9ff3) 1; animation-duration: 60s;"></div>

      <!-- 转盘指针 -->
      <div class="absolute top-0 left-1/2 transform -translate-x-1/2 -mt-6 z-20">
        <svg width="40" height="60" viewBox="0 0 40 60" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M20 0L40 30H0L20 0Z" fill="#FF4757" />
          <circle cx="20" cy="30" r="10" fill="#FFF" stroke="#FF4757" stroke-width="4" />
          <text x="20" y="34" text-anchor="middle" font-size="14" fill="#FF4757" font-weight="bold">🎯</text>
        </svg>
      </div>
      
      <!-- 转盘本体 -->
      <div id="wheel" 
        class="w-full h-full rounded-full bg-gradient-to-r from-yellow-300 to-yellow-400 border-8 border-yellow-500 relative overflow-hidden shadow-xl transform transition-transform duration-300"
        :style="{ transform: `rotate(${currentRotation}deg)` }">
        
        <!-- 转盘分区 - 5个扇形区域 -->
        <div v-for="(section, index) in wheelSections" :key="index" 
          class="wheel-section"
          :style="{
            '--rotate': `${index * 72}deg`,
            '--bg-color': section.color
          }">
          <div class="section-content">
            <div class="section-text">
              <div class="flex items-center">
                <span v-if="section.type === 'word'" class="text-lg font-bold section-word">{{ section.text }}</span>
                <span v-else-if="section.type === 'magic'" class="magic-bag">
                  <!-- 魔法袋子SVG -->
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M4 8H20L18 21H6L4 8Z" fill="#8A4FFF" />
                    <path d="M8 4C8 2.89543 8.89543 2 10 2H14C15.1046 2 16 2.89543 16 4V8H8V4Z" fill="#B388FF" />
                    <path d="M8 10L10 14L12 10L14 14L16 10" stroke="white" stroke-width="1.5" stroke-linecap="round" />
                    <circle cx="12" cy="6" r="1" fill="white" />
                  </svg>
                  <span class="ml-1">魔法袋子</span>
                </span>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 转盘分界线 -->
        <div v-for="index in 5" :key="`line-${index}`" 
          class="divider-line"
          :style="{ transform: `rotate(${index * 72}deg)` }">
        </div>
      </div>
      
      <!-- 中心花朵装饰 -->
      <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-10 pointer-events-none">
        <svg width="50" height="50" viewBox="0 0 50 50" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="25" cy="25" r="12" fill="#FFD166" />
          <path d="M25 13C25 13 28 5 25 5C22 5 25 13 25 13Z" fill="#FF6B6B" />
          <path d="M32 17C32 17 41 12 39 10C37 8 32 17 32 17Z" fill="#FF6B6B" />
          <path d="M33 25C33 25 43 25 43 22C43 19 33 25 33 25Z" fill="#FF6B6B" />
          <path d="M32 33C32 33 41 38 39 40C37 42 32 33 32 33Z" fill="#FF6B6B" />
          <path d="M25 37C25 37 28 45 25 45C22 45 25 37 25 37Z" fill="#FF6B6B" />
          <path d="M18 33C18 33 9 38 11 40C13 42 18 33 18 33Z" fill="#FF6B6B" />
          <path d="M17 25C17 25 7 25 7 22C7 19 17 25 17 25Z" fill="#FF6B6B" />
          <path d="M18 17C18 17 9 12 11 10C13 8 18 17 18 17Z" fill="#FF6B6B" />
        </svg>
      </div>
    </div>
    
    <!-- 选中项放大显示弹窗 -->
    <div class="winner-modal" v-if="showWinnerPopup">
      <div class="winner-content" :style="{ backgroundColor: selectedSection.color + '40' }">
        <button @click="closeWinnerPopup" class="absolute -top-4 -right-4 w-10 h-10 bg-red-500 hover:bg-red-600 text-white rounded-full flex items-center justify-center shadow-lg">
          <span>×</span>
        </button>
        
        <!-- 星星装饰SVG -->
        <svg class="absolute top-0 left-0 w-full h-full pointer-events-none opacity-50">
          <circle cx="30" cy="30" r="5" fill="gold" class="animate-pulse">
            <animate attributeName="r" values="5;7;5" dur="2s" repeatCount="indefinite" />
          </circle>
          <circle cx="180" cy="40" r="4" fill="gold" class="animate-pulse">
            <animate attributeName="r" values="4;6;4" dur="3s" repeatCount="indefinite" />
          </circle>
          <circle cx="50" cy="180" r="6" fill="gold" class="animate-pulse">
            <animate attributeName="r" values="6;8;6" dur="2.5s" repeatCount="indefinite" />
          </circle>
          <circle cx="160" cy="150" r="5" fill="gold" class="animate-pulse">
            <animate attributeName="r" values="5;7;5" dur="1.8s" repeatCount="indefinite" />
          </circle>
        </svg>
        
        <div class="text-center">
          <h3 class="text-3xl font-bold mb-6">
            <template v-if="selectedSection.type === 'word'">
              <div>{{ selectedSection.textEN }}</div>
              <div class="text-xl mt-2 text-purple-500">{{ selectedSection.text }}</div>
            </template>
            <template v-else>
              魔法袋子
            </template>
          </h3>
          
          <template v-if="selectedSection.type === 'magic'">
            <div class="mb-4">
              <!-- 大魔法袋子SVG -->
              <svg width="100" height="100" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M20 35H80L72 90H28L20 35Z" fill="#8A4FFF" />
                <path d="M35 15C35 10.5817 38.5817 7 43 7H57C61.4183 7 65 10.5817 65 15V35H35V15Z" fill="#B388FF" />
                <path d="M35 45L45 65L50 45L55 65L65 45" stroke="white" stroke-width="3" stroke-linecap="round" />
                <circle cx="50" cy="25" r="5" fill="white" />
                <path d="M35 45C25 35 15 55 35 65" stroke="white" stroke-width="2" />
                <path d="M65 45C75 35 85 55 65 65" stroke="white" stroke-width="2" />
              </svg>
            </div>
            <p class="text-lg">你发现了一个神秘的魔法袋子！</p>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { calculateRotation, animateWheel, playSound, createConfetti } from '../utils/wheelUtils';

// 定义扇形区域接口
interface WheelSection {
  type: 'word' | 'magic';
  text: string;    // 中文单词
  textEN: string;  // 英文单词
  color: string;
}

// 可自定义的单词对照
const customWords = ref<{cn: string, en: string}[]>([
  { cn: '苹果', en: 'Apple' },
  { cn: '香蕉', en: 'Banana' },
  { cn: '太阳', en: 'Sun' },
  { cn: '月亮', en: 'Moon' }
]);

// 创建轮盘区域数据 - 5个扇形区域
const wheelSections = reactive<WheelSection[]>([
  { type: 'word', text: customWords.value[0].cn, textEN: customWords.value[0].en, color: '#ff6b6b' },
  { type: 'word', text: customWords.value[1].cn, textEN: customWords.value[1].en, color: '#48dbfb' },
  { type: 'magic', text: '魔法袋子', textEN: 'Magic Bag', color: '#8A4FFF' },
  { type: 'word', text: customWords.value[2].cn, textEN: customWords.value[2].en, color: '#1dd1a1' },
  { type: 'word', text: customWords.value[3].cn, textEN: customWords.value[3].en, color: '#feca57' }
]);

// 转盘状态
const isSpinning = ref(false);
const currentRotation = ref(0);

// 弹窗状态
const showWinnerPopup = ref(false);
const selectedSection = ref<WheelSection>(wheelSections[0]);
const confettiActive = ref(false);

// 关闭弹窗
const closeWinnerPopup = () => {
  showWinnerPopup.value = false;
  confettiActive.value = false;
};

// 开始旋转 
const spin = () => {
  if (isSpinning.value) return false;
  
  isSpinning.value = true;
  showWinnerPopup.value = false;
  
  // 播放旋转音效
  playSound('spin');
  
  // 随机选择获奖区域
  const winnerIndex = Math.floor(Math.random() * wheelSections.length);
  const winner = wheelSections[winnerIndex];
  
  // 使用工具函数计算旋转角度
  const rotationData = calculateRotation(winnerIndex);
  
  // 使用工具函数执行动画
  animateWheel('#wheel', rotationData, () => {
    // 动画结束后
    isSpinning.value = false;
    currentRotation.value = rotationData.modRotation;
    
    // 播放获奖音效
    playSound('win');
    
    // 创建彩花效果
    createConfetti();
    
    // 显示弹窗
    selectedSection.value = winner;
    setTimeout(() => {
      showWinnerPopup.value = true;
    }, 500);
  });
  
  return true;
};

// 设置自定义单词
const setCustomWords = (words: {cn: string, en: string}[]) => {
  if (words.length >= 4) {
    customWords.value = words.slice(0, 4);
    
    // 更新轮盘上的文字
    wheelSections.forEach((section: WheelSection, index: number) => {
      if (section.type === 'word') {
        const wordIndex = index >= 3 ? index - 1 : index;
        section.text = customWords.value[wordIndex].cn;
        section.textEN = customWords.value[wordIndex].en;
      }
    });
  }
};

// 挂载时初始化
onMounted(() => {
  // 确保所有文字正确显示
  setTimeout(() => {
    const wheel = document.getElementById('wheel');
    if (wheel) {
      wheel.style.opacity = '1';
    }
  }, 100);
});

// 导出方法给父组件使用
defineExpose({
  spin,
  setCustomWords
});
</script>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'LuckyWheel'
});
</script>

<style scoped>
/* 转盘区域样式 */
.wheel-section {
  position: absolute;
  width: 50%;
  height: 50%;
  top: 0;
  left: 0;
  transform-origin: bottom right;
  transform: rotate(calc(var(--rotate)));
  clip-path: polygon(0 0, 100% 0, 100% 100%);
  background-color: var(--bg-color);
  background-image: linear-gradient(45deg, var(--bg-color), transparent);
  transition: all 0.3s ease;
  overflow: visible;
  box-shadow: inset 0 0 10px rgba(255, 255, 255, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.wheel-section:hover {
  filter: brightness(1.1);
}

.section-content {
  position: relative;
  width: 100%;
  height: 100%;
}

/* 扇形文本定位 */
.section-text {
  position: absolute;
  top: 30%;
  left: 25%;
  color: white;
  font-weight: bold;
  font-size: 16px;
  text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
  white-space: nowrap;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 5;
  animation: pulse 2s infinite ease-in-out;
  -webkit-text-stroke: 0.5px #000;
  letter-spacing: 1px;
  transform: rotate(-45deg);
}

.section-word {
  white-space: nowrap;
  display: inline-block;
  background-color: rgba(0, 0, 0, 0.2);
  padding: 2px 6px;
  border-radius: 4px;
}

@keyframes pulse {
  0%, 100% {
    text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
    transform: rotate(-45deg) scale(1);
  }
  50% {
    text-shadow: 3px 3px 6px rgba(0,0,0,0.6);
    transform: rotate(-45deg) scale(1.05);
  }
}

/* 增加分界线 */
.divider-line {
  position: absolute;
  top: 0;
  left: 50%;
  width: 2px;
  height: 50%;
  background-color: rgba(255, 255, 255, 0.5);
  transform-origin: bottom center;
  z-index: 2;
}

.magic-bag {
  display: flex;
  align-items: center;
  font-weight: bold;
  background-color: rgba(0, 0, 0, 0.2);
  padding: 2px 6px;
  border-radius: 4px;
}

/* 获奖弹窗样式 */
.winner-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  perspective: 1000px;
  backdrop-filter: blur(5px);
}

.winner-content {
  width: 300px;
  height: 300px;
  background-color: white;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1), 0 0 30px rgba(255, 215, 0, 0.5);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: pop-in 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards;
  border: 8px double gold;
  overflow: hidden;
}

.winner-content::before {
  content: "";
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, 
    rgba(255,255,255,0) 0%, 
    rgba(255,255,255,0.2) 50%, 
    rgba(255,255,255,0) 100%);
  transform: rotate(45deg);
  animation: shine 3s infinite linear;
}

@keyframes shine {
  0% {
    transform: translateX(-100%) rotate(45deg);
  }
  100% {
    transform: translateX(100%) rotate(45deg);
  }
}

@keyframes pop-in {
  0% {
    transform: scale(0.1) rotate(-10deg);
    opacity: 0;
  }
  80% {
    transform: scale(1.1) rotate(5deg);
  }
  100% {
    transform: scale(1) rotate(0);
    opacity: 1;
  }
}

/* 转盘慢速旋转动画 */
@keyframes spin-slow {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.animate-spin-slow {
  animation: spin-slow 30s linear infinite;
}

/* 点击转盘添加震动效果 */
@keyframes wiggle {
  0%, 100% { transform: rotate(0); }
  25% { transform: rotate(1deg); }
  75% { transform: rotate(-1deg); }
}

#wheel:active {
  animation: wiggle 0.2s ease-in-out;
}

.wheel-container {
  position: relative;
}

.wheel-container::after {
  content: "";
  position: absolute;
  bottom: -10px;
  left: 50%;
  transform: translateX(-50%);
  width: 80%;
  height: 15px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  filter: blur(5px);
}

/* 彩花效果 */
.confetti-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 9999;
  overflow: hidden;
}

.confetti {
  position: absolute;
  top: -10px;
  width: 10px;
  height: 20px;
  background-color: #f00;
  opacity: 0.8;
  animation: confetti-fall linear forwards;
}

@keyframes confetti-fall {
  0% {
    transform: translateY(0) rotate(0deg);
    opacity: 1;
  }
  100% {
    transform: translateY(100vh) rotate(720deg);
    opacity: 0;
  }
}
</style> 