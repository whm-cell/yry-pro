<template>
  <div 
    class="sidebar text-white flex flex-col py-4 relative overflow-hidden transition-all duration-300"
    :class="[
      collapsed ? 'w-0' : 'w-64',
      'bg-gradient-to-b from-blue-500 to-purple-600'
    ]"
  >
    <!-- 内容容器，折叠时隐藏 -->
    <div v-if="!collapsed" class="sidebar-content w-full h-full flex flex-col">
      <!-- 气泡装饰 -->
      <div class="bubble absolute bottom-5 left-10" style="animation-delay: 0s;"></div>
      <div class="bubble absolute bottom-5 left-20" style="animation-delay: 1s;"></div>
      <div class="bubble absolute bottom-5 left-30" style="animation-delay: 2s;"></div>
      <div class="bubble absolute bottom-5 left-40" style="animation-delay: 3s;"></div>

      <div class="p-4 overflow-hidden">
        <div class="logo-container flex flex-col items-center justify-center mb-8">
          <div class="logo-circle w-20 h-20 rounded-full bg-gradient-to-r from-blue-400 to-purple-500 flex items-center justify-center shadow-lg mb-2 border-4 border-white">
            <div class="text-3xl font-bold">🎡</div>
          </div>
          <div class="logo-text text-center">
            <div class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-yellow-300 to-pink-300">AiMat</div>
          </div>
        </div>
        
        <nav>
          <ul>
            <li class="mb-2">
              <a href="#" class="nav-item flex items-center p-3 rounded-lg" 
                :class="{ 
                  'bg-blue-400 bg-opacity-20': activePage === 'englishWordLottery', 
                  'hover:bg-white hover:bg-opacity-10': activePage !== 'englishWordLottery'
                }" 
                @click.prevent="changePage('englishWordLottery')"
              >
                <i class="fas fa-spinner w-6 text-lg"></i>
                <span class="font-semibold ml-2">英语小转盘</span>
              </a>
            </li>
            <li class="mb-2">
              <a href="#" class="nav-item flex items-center p-3 rounded-lg" 
                :class="{ 
                  'bg-blue-400 bg-opacity-20': activePage === 'settings', 
                  'hover:bg-white hover:bg-opacity-10': activePage !== 'settings'
                }" 
                @click.prevent="changePage('settings')"
              >
                <i class="fas fa-cog w-6 text-lg"></i>
                <span class="font-semibold ml-2">小设置</span>
              </a>
            </li>
          </ul>
        </nav>
      </div>
      
      <div class="mt-auto p-4 text-sm text-gray-300 border-t border-white border-opacity-20">
        <div class="flex items-center mb-1">
          <i class="fas fa-heart text-pink-300 mr-1"></i>
          <div>Aimat v1.0</div>
        </div>
        <div class="flex items-center">
          <i class="fas fa-smile text-yellow-300 mr-1"></i>
          <div>© {{ new Date().getFullYear() }} 幼儿园</div>
        </div>
      </div>
    </div>
  </div>

  <!-- 折叠/展开按钮 - 放在组件外部以确保始终可见 -->
  <div 
    class="toggle-button fixed rounded-full w-6 h-6 flex items-center justify-center cursor-pointer shadow-md z-10 hover:bg-blue-300 transition-colors"
    :class="[
      collapsed ? 'bg-blue-500 left-2' : 'bg-blue-400 -translate-x-3',
      'top-4'
    ]"
    @click="toggleSidebar"
  >
    <i class="fas" :class="collapsed ? 'fa-chevron-right' : 'fa-chevron-left'" style="font-size: 0.7rem;"></i>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps<{
  activePage: string;
  collapsed: boolean;
}>();

const emit = defineEmits<{
  (e: 'change-page', page: string): void;
  (e: 'toggle-sidebar'): void;
}>();

const changePage = (page: string) => {
  emit('change-page', page);
};

const toggleSidebar = () => {
  emit('toggle-sidebar');
};
</script>

<style scoped>
.sidebar {
  background: linear-gradient(to bottom, #4c6ef5, #7c3aed);
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  border-radius: 0 16px 16px 0;
  min-width: 0;  /* 允许宽度为0 */
  overflow: hidden;
}

.sidebar.w-0 {
  padding: 0;
  border-radius: 0;
  opacity: 0;
}

/* 气泡动画 */
.bubble {
  width: 10px;
  height: 10px;
  background-color: rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  animation: bubbleFloat 5s infinite ease-in-out;
}

@keyframes bubbleFloat {
  0%, 100% { transform: translateY(0); opacity: 0.3; }
  50% { transform: translateY(-20px); opacity: 0.7; }
}

/* 过渡动画 */
.transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
}
</style> 