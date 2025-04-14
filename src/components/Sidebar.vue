<template>
  <div class="sidebar text-white w-64 flex flex-col py-4 relative overflow-hidden">
    <!-- 气泡装饰 -->
    <div class="bubble absolute bottom-5 left-10" style="animation-delay: 0s;"></div>
    <div class="bubble absolute bottom-5 left-20" style="animation-delay: 1s;"></div>
    <div class="bubble absolute bottom-5 left-30" style="animation-delay: 2s;"></div>
    <div class="bubble absolute bottom-5 left-40" style="animation-delay: 3s;"></div>

    <div class="p-4">
      <div class="user-profile flex items-center mb-8">
        <div class="relative">
          <img src="https://via.placeholder.com/50" alt="用户头像" class="rounded-full w-12 h-12 border-4 border-white">
          <div class="absolute -top-2 -right-2 bg-yellow-400 text-white rounded-full w-6 h-6 flex items-center justify-center text-xs shadow-md">
            👶
          </div>
        </div>
        <div class="ml-3">
          <div class="font-bold">小朋友</div>
          <div class="text-sm text-gray-200">幸运星</div>
        </div>
      </div>
      
      <nav>
        <ul>
          <li class="mb-2">
            <a href="#" class="nav-item flex items-center p-3 rounded-lg" :class="{ ' bg-blue-400 bg-opacity-20': activePage === 'englishWordLottery', 'hover:bg-white hover:bg-opacity-10': activePage !== 'englishWordLottery' }" @click.prevent="changePage('englishWordLottery')">
              <i class="fas fa-spinner w-6 text-lg"></i>
<span class="font-semibold">英语小转盘</span>

            </a>
          </li>
          <li class="mb-2">
            <a href="#" class="nav-item flex items-center p-3 rounded-lg" :class="{ 'bg-blue-400 bg-opacity-20': activePage === 'settings', 'hover:bg-white hover:bg-opacity-10': activePage !== 'settings' }" @click.prevent="changePage('settings')">
              <i class="fas fa-cog w-6 text-lg"></i>
              <span class="font-semibold">小设置</span>
            </a>
          </li>
        </ul>
      </nav>
    </div>
    
    <div class="mt-auto p-4 text-sm text-gray-300 border-t border-white border-opacity-20">
      <div class="flex items-center mb-1">
        <i class="fas fa-heart text-pink-300 mr-1"></i>
        <div>幼儿园抽奖大转盘 v1.0</div>
      </div>
      <div class="flex items-center">
        <i class="fas fa-smile text-yellow-300 mr-1"></i>
        <div>© {{ new Date().getFullYear() }} 幼儿园</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps<{
  activePage: string
}>();

const emit = defineEmits<{
  (e: 'change-page', page: string): void
}>();

const changePage = (page: string) => {
  emit('change-page', page);
};
</script> 