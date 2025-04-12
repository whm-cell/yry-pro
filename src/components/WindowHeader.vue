<template>
  <div class="window-header px-4 py-2 flex justify-between items-center relative">
    <div class="flex items-center">
      <i class="fas fa-gift mr-2 text-xl"></i>
      <h1 class="text-lg font-bold text-white">幼儿园抽奖大转盘</h1>
    </div>
    <div class="absolute left-1/2 transform -translate-x-1/2 flex space-x-1">
      <div class="w-3 h-3 rounded-full bg-yellow-300 opacity-70"></div>
      <div class="w-3 h-3 rounded-full bg-green-300 opacity-70"></div>
      <div class="w-3 h-3 rounded-full bg-blue-300 opacity-70"></div>
      <div class="w-3 h-3 rounded-full bg-purple-300 opacity-70"></div>
      <div class="w-3 h-3 rounded-full bg-pink-300 opacity-70"></div>
    </div>
    <div class="window-controls flex">
      <button class="px-2 hover:bg-opacity-20 hover:bg-white text-white" @click="minimizeWindow"><i class="fas fa-minus"></i></button>
      <button class="px-2 hover:bg-opacity-20 hover:bg-white text-white" @click="maximizeWindow"><i class="fas fa-expand-alt"></i></button>
      <button class="px-2 hover:bg-opacity-20 hover:bg-white text-white hover:text-red-300" @click="closeWindow"><i class="fas fa-times"></i></button>
    </div>
  </div>
</template>

<script setup lang="ts">
// 暂时使用模拟功能，后续再集成Tauri窗口控制API
const minimizeWindow = () => {
  console.log('最小化窗口');
  alert('最小化窗口');
};

const maximizeWindow = () => {
  console.log('最大化/还原窗口');
  document.body.classList.toggle('maximized');
  alert('最大化/还原窗口');
};

const closeWindow = () => {
  if (confirm('确定要关闭应用吗？')) {
    console.log('关闭应用');
    alert('关闭应用');
  }
};
</script> 