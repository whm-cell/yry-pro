<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-lg shadow-lg p-6 h-full">
      <h2 class="text-2xl font-bold mb-6 text-gray-800">设置</h2>
      
      <!-- 设置选项卡 -->
      <div class="flex border-b border-gray-200 mb-6">
        <button 
          v-for="(tab, index) in tabs" 
          :key="index" 
          class="px-4 py-2" 
          :class="activeTab === index ? 'text-blue-600 border-b-2 border-blue-600 font-medium' : 'text-gray-500 hover:text-gray-700'"
          @click="activeTab = index"
        >
          {{ tab }}
        </button>
      </div>
      
      <!-- 常规设置面板 -->
      <div v-if="activeTab === 0">
        <!-- 主题设置 -->
        <div class="mb-6 pb-6 border-b border-gray-200">
          <h3 class="text-lg font-semibold mb-4">主题设置</h3>
          
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-2">应用主题</label>
            <div class="flex gap-4">
              <div 
                v-for="(theme, index) in themes" 
                :key="index"
                class="theme-option cursor-pointer p-1 rounded-md" 
                :class="selectedTheme === index ? 'border-2 border-blue-500' : 'border-2 border-gray-200'"
                @click="selectedTheme = index"
              >
                <div class="w-16 h-10" :style="{ backgroundColor: theme.color }"></div>
                <div class="text-center text-sm mt-1">{{ theme.name }}</div>
              </div>
            </div>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">转盘样式</label>
            <div class="flex gap-4">
              <div 
                v-for="(style, index) in wheelStyles" 
                :key="index"
                class="theme-option cursor-pointer p-1 rounded-md" 
                :class="selectedWheelStyle === index ? 'border-2 border-blue-500' : 'border-2 border-gray-200'"
                @click="selectedWheelStyle = index"
              >
                <div :class="style.class">
                  {{ style.label }}
                </div>
                <div class="text-center text-sm mt-1">{{ style.name }}</div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 声音设置 -->
        <div class="mb-6 pb-6 border-b border-gray-200">
          <h3 class="text-lg font-semibold mb-4">声音设置</h3>
          
          <div class="flex items-center justify-between mb-4 pb-2 border-b border-gray-100">
            <div>
              <div class="font-medium">背景音乐</div>
              <div class="text-sm text-gray-500">在应用中播放背景音乐</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="settings.backgroundMusic">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div class="flex items-center justify-between mb-4 pb-2 border-b border-gray-100">
            <div>
              <div class="font-medium">抽奖音效</div>
              <div class="text-sm text-gray-500">转盘旋转和中奖时播放音效</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="settings.spinSound">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div class="flex items-center justify-between mb-4">
            <div>
              <div class="font-medium">按钮音效</div>
              <div class="text-sm text-gray-500">点击按钮时播放音效</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="settings.buttonSound">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">音量</label>
            <input type="range" min="0" max="100" v-model="settings.volume" class="w-full">
          </div>
        </div>
        
        <!-- 性能设置 -->
        <div class="mb-6">
          <h3 class="text-lg font-semibold mb-4">性能设置</h3>
          
          <div class="flex items-center justify-between mb-4 pb-2 border-b border-gray-100">
            <div>
              <div class="font-medium">动画效果</div>
              <div class="text-sm text-gray-500">启用界面过渡动画</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="settings.animations">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div class="flex items-center justify-between">
            <div>
              <div class="font-medium">高质量图形</div>
              <div class="text-sm text-gray-500">使用高分辨率图像和效果</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="settings.highQuality">
              <span class="slider round"></span>
            </label>
          </div>
        </div>
        
        <!-- 保存按钮 -->
        <div class="mt-8 flex justify-end">
          <button class="px-6 py-2 bg-gray-200 text-gray-800 rounded-md mr-2 hover:bg-gray-300" @click="resetSettings">
            重置为默认
          </button>
          <button class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700" @click="saveSettings">
            保存设置
          </button>
        </div>
      </div>
      
      <!-- 其他面板可以在这里添加，使用v-if="activeTab === 1"等条件 -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';

const activeTab = ref(0);
const tabs = ['常规设置', '通知设置', '安全设置', '关于'];

const selectedTheme = ref(0);
const themes = [
  { name: '默认蓝', color: '#4F46E5' },
  { name: '清新绿', color: '#10B981' },
  { name: '高贵紫', color: '#8B5CF6' },
  { name: '暗黑', color: '#1F2937' }
];

const selectedWheelStyle = ref(0);
const wheelStyles = [
  { 
    name: '经典', 
    label: '经典样式',
    class: 'w-16 h-16 rounded-full bg-gradient-to-r from-yellow-400 to-yellow-600 flex items-center justify-center text-xs text-white'
  },
  { 
    name: '霓虹', 
    label: '霓虹样式',
    class: 'w-16 h-16 rounded-full bg-gradient-to-r from-pink-400 to-purple-600 flex items-center justify-center text-xs text-white'
  },
  { 
    name: '极简', 
    label: '极简样式',
    class: 'w-16 h-16 rounded-full bg-gradient-to-r from-blue-400 to-green-600 flex items-center justify-center text-xs text-white'
  }
];

// 设置状态
const settings = reactive({
  backgroundMusic: true,
  spinSound: true,
  buttonSound: false,
  volume: 70,
  animations: true,
  highQuality: true
});

// 重置设置
const resetSettings = () => {
  settings.backgroundMusic = true;
  settings.spinSound = true;
  settings.buttonSound = false;
  settings.volume = 70;
  settings.animations = true;
  settings.highQuality = true;
  selectedTheme.value = 0;
  selectedWheelStyle.value = 0;
};

// 保存设置
const saveSettings = () => {
  // 在实际应用中，这里应该将设置保存到本地存储或通过API保存
  alert('设置已保存！');
};
</script> 