<script setup lang="ts">
import { ref } from "vue";
import WindowHeader from "./components/WindowHeader.vue";
import Sidebar from "./components/Sidebar.vue";
import Home from "./pages/Home.vue";
import Profile from "./pages/Profile.vue";
import Results from "./pages/Results.vue";
import Settings from "./pages/Settings.vue";

// 当前活动页面
const activePage = ref('home');

// 切换页面
const changePage = (page: string) => {
  activePage.value = page;
};

// 背景装饰
const decorations = [
  { type: 'star', position: 'top-10 right-10', delay: '0s' },
  { type: 'star', position: 'top-20 left-20', delay: '0.5s' },
  { type: 'star', position: 'bottom-20 right-30', delay: '1s' },
  { type: 'star', position: 'bottom-40 left-40', delay: '1.5s' },
  { type: 'balloon', position: 'top-0 left-10', delay: '0s' },
  { type: 'balloon', position: 'top-5 right-20', delay: '2s' }
];
</script>

<template>
  <div class="bg-gradient-to-r from-blue-50 via-purple-50 to-pink-50 h-screen overflow-hidden flex flex-col">
    <!-- 背景装饰 -->
    <div 
      v-for="(decoration, index) in decorations" 
      :key="index" 
      :class="[
        'fixed', 
        decoration.position, 
        decoration.type === 'star' ? 'star text-2xl opacity-70' : 'float text-3xl',
      ]"
      :style="{ animationDelay: decoration.delay }"
    >
      {{ decoration.type === 'star' ? '✨' : '🎈' }}
    </div>
    
    <!-- 顶部标题栏 -->
    <WindowHeader />
    
    <!-- 主内容区域 -->
    <div class="flex flex-1 overflow-hidden">
      <!-- 侧边栏导航 -->
      <Sidebar :active-page="activePage" @change-page="changePage" />
      
      <!-- 页面内容区域 -->
      <div class="content-container flex-1 overflow-hidden relative bg-white bg-opacity-80 rounded-tl-3xl">
        <!-- 彩带装饰 -->
        <div class="absolute top-0 right-0 w-full h-6 bg-gradient-to-r from-red-300 via-yellow-300 to-green-300 opacity-70"></div>
        
        <!-- 页面内容 -->
        <div class="h-full overflow-auto pt-8 pb-4 px-2">
          <Home v-if="activePage === 'home'" />
          <Profile v-else-if="activePage === 'profile'" />
          <Results v-else-if="activePage === 'results'" />
          <Settings v-else-if="activePage === 'settings'" />
        </div>
      </div>
    </div>

    <!-- 底部装饰 -->
    <div class="fixed bottom-0 left-0 w-full flex justify-between py-2 px-4 space-x-4 text-2xl opacity-50 pointer-events-none">
      <div class="flex space-x-4">
        <div class="bounce" style="animation-delay: 0s;">🐰</div>
        <div class="bounce" style="animation-delay: 0.3s;">🐱</div>
        <div class="bounce" style="animation-delay: 0.6s;">🐶</div>
      </div>
      <div class="flex space-x-4">
        <div class="bounce" style="animation-delay: 0.9s;">🐼</div>
        <div class="bounce" style="animation-delay: 1.2s;">🦊</div>
        <div class="bounce" style="animation-delay: 1.5s;">🐨</div>
      </div>
    </div>
  </div>
</template>

<style>
@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');
@import url('https://fonts.googleapis.com/css2?family=Comic+Neue:wght@400;700&display=swap');

* {
  font-family: 'Comic Neue', 'Comic Sans MS', cursive, 'Microsoft YaHei', sans-serif;
}
</style>