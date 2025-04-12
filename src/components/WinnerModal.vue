<template>
  <div class="modal-overlay" :class="{ 'active': isActive }">
    <div class="modal-content bg-white p-8 rounded-3xl shadow-2xl w-80 text-center relative">
      <!-- 关闭按钮 -->
      <button @click="close" class="absolute -top-4 -right-4 w-10 h-10 bg-red-500 hover:bg-red-600 text-white rounded-full flex items-center justify-center shadow-lg">
        <i class="fas fa-times"></i>
      </button>
      
      <!-- 恭喜标题 -->
      <h3 class="text-2xl font-bold mb-6 text-blue-600">恭喜获奖！</h3>
      
      <!-- 获奖者信息 -->
      <div class="mb-6">
        <div class="w-24 h-24 rounded-full mx-auto mb-4 flex items-center justify-center text-4xl" :style="{ backgroundColor: winner.color + '40' }">
          {{ winner.avatar }}
        </div>
        <div class="text-xl font-bold">{{ winner.name }}</div>
        <div class="text-gray-500">{{ winner.class }}</div>
      </div>
      
      <!-- 礼花效果容器 -->
      <div id="starsContainer" class="absolute top-0 left-0 w-full h-full pointer-events-none overflow-hidden"></div>
      
      <!-- 按钮 -->
      <div class="flex justify-center gap-4">
        <button @click="close" class="px-6 py-2 bg-gray-200 text-gray-800 rounded-full hover:bg-gray-300 transition">
          关闭
        </button>
        <button class="px-6 py-2 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-full hover:from-blue-600 hover:to-purple-600 transition">
          领取奖品
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, defineProps, defineEmits } from 'vue';
import gsap from 'gsap';

interface Winner {
  name: string;
  class: string;
  avatar: string;
  color: string;
}

const props = defineProps<{
  isActive: boolean;
  winner: Winner;
}>();

const emit = defineEmits<{
  (e: 'close'): void
}>();

const close = () => {
  emit('close');
};

watch(() => props.isActive, (newValue) => {
  if (newValue) {
    createStars();
  }
});

// 创建星星动画效果
function createStars() {
  const container = document.getElementById('starsContainer');
  if (!container) return;
  
  // 清除现有的星星
  container.innerHTML = '';
  
  // 定义可能的表情
  const emojis = ['✨', '🎉', '🎊', '🎇', '🎆', '⭐', '🌟', '💫', '🥳'];
  
  // 创建15个随机星星
  for (let i = 0; i < 15; i++) {
    const star = document.createElement('div');
    const randomEmoji = emojis[Math.floor(Math.random() * emojis.length)];
    
    star.textContent = randomEmoji;
    star.className = 'celebrate';
    star.style.position = 'absolute';
    star.style.fontSize = Math.random() * 24 + 12 + 'px';
    star.style.left = Math.random() * 100 + '%';
    star.style.top = Math.random() * 100 + '%';
    star.style.opacity = '0';
    star.style.zIndex = '10';
    
    // 随机的延迟开始动画
    star.style.animationDelay = (Math.random() * 1.5) + 's';
    
    container.appendChild(star);
  }
}
</script> 