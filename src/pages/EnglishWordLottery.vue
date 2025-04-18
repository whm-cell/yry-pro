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
      
      <!-- 设置按钮 -->
      <div class="absolute top-8 right-8">
        <button @click="toggleSettings" class="settings-btn">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
          </svg>
        </button>
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
    
    <!-- 设置面板 -->
    <div class="settings-panel" :class="{ 'show': showSettings }">
      <div class="settings-header">
        <h3>转盘设置</h3>
        <button @click="toggleSettings" class="settings-close-btn">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      <div class="settings-content">
        <WheelSettings />
      </div>
    </div>
    
    <!-- 设置面板遮罩 -->
    <div class="settings-overlay" :class="{ 'show': showSettings }" @click="toggleSettings"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineAsyncComponent } from 'vue';

// 使用异步组件导入
const LuckyWheelComp = defineAsyncComponent(() => import('../components/LuckyWheel.vue'));
const WheelSettings = defineAsyncComponent(() => import('../components/WheelSettings.vue'));

// 设置面板状态
const showSettings = ref(false);

// 切换设置面板显示状态
function toggleSettings() {
  showSettings.value = !showSettings.value;
}
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

/* 设置按钮样式 */
.settings-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background-color: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  color: #e17055;
  border: none;
  cursor: pointer;
  transition: all 0.3s ease;
}

.settings-btn:hover {
  transform: rotate(30deg);
  background-color: #e17055;
  color: #fff;
}

/* 设置面板样式 */
.settings-panel {
  position: fixed;
  top: 0;
  right: -400px;
  width: 380px;
  height: 100vh;
  background-color: white;
  box-shadow: -5px 0 25px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  transition: right 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  overflow-y: auto;
  padding: 0;
}

.settings-panel.show {
  right: 0;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #f0f0f0;
}

.settings-header h3 {
  margin: 0;
  font-size: 20px;
  color: #e17055;
}

.settings-close-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  color: #636e72;
  padding: 5px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.settings-close-btn:hover {
  background-color: #f0f0f0;
  color: #e17055;
}

.settings-content {
  padding: 20px;
}

/* 设置面板遮罩 */
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 999;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.3s ease, visibility 0s 0.3s;
}

.settings-overlay.show {
  opacity: 1;
  visibility: visible;
  transition: opacity 0.3s ease, visibility 0s;
}
</style> 