<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-3xl shadow-lg p-6 h-full flex flex-col border-4 border-dashed border-purple-300">
      <h2 class="text-3xl font-bold mb-6 text-center text-purple-700">抽奖转盘设置</h2>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 flex-grow">
        <!-- 抽奖模式设置 -->
        <div class="bg-purple-50 rounded-xl p-5 border border-purple-200 shadow-sm">
          <h3 class="text-xl font-bold mb-4 text-purple-700 flex items-center">
            <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.4 15C19.2669 15.3016 19.2272 15.6362 19.286 15.9606C19.3448 16.285 19.4995 16.5843 19.73 16.82L19.79 16.88C19.976 17.0657 20.1235 17.2863 20.2241 17.5291C20.3248 17.7719 20.3766 18.0322 20.3766 18.295C20.3766 18.5578 20.3248 18.8181 20.2241 19.0609C20.1235 19.3037 19.976 19.5243 19.79 19.71C19.6043 19.896 19.3837 20.0435 19.1409 20.1441C18.8981 20.2448 18.6378 20.2966 18.375 20.2966C18.1122 20.2966 17.8519 20.2448 17.6091 20.1441C17.3663 20.0435 17.1457 19.896 16.96 19.71L16.9 19.65C16.6643 19.4195 16.365 19.2648 16.0406 19.206C15.7162 19.1472 15.3816 19.1869 15.08 19.32C14.7842 19.4468 14.532 19.6572 14.3543 19.9255C14.1766 20.1938 14.0813 20.5082 14.08 20.83V21C14.08 21.5304 13.8693 22.0391 13.4942 22.4142C13.1191 22.7893 12.6104 23 12.08 23C11.5496 23 11.0409 22.7893 10.6658 22.4142C10.2907 22.0391 10.08 21.5304 10.08 21V20.91C10.0723 20.579 9.96512 20.258 9.77251 19.9887C9.5799 19.7194 9.31074 19.5143 9 19.4C8.69838 19.2669 8.36381 19.2272 8.03941 19.286C7.71502 19.3448 7.41568 19.4995 7.18 19.73L7.12 19.79C6.93425 19.976 6.71368 20.1235 6.47088 20.2241C6.22808 20.3248 5.96783 20.3766 5.705 20.3766C5.44217 20.3766 5.18192 20.3248 4.93912 20.2241C4.69632 20.1235 4.47575 19.976 4.29 19.79C4.10405 19.6043 3.95653 19.3837 3.85588 19.1409C3.75523 18.8981 3.70343 18.6378 3.70343 18.375C3.70343 18.1122 3.75523 17.8519 3.85588 17.6091C3.95653 17.3663 4.10405 17.1457 4.29 16.96L4.35 16.9C4.58054 16.6643 4.73519 16.365 4.794 16.0406C4.85282 15.7162 4.81312 15.3816 4.68 15.08C4.55324 14.7842 4.34276 14.532 4.07447 14.3543C3.80618 14.1766 3.49179 14.0813 3.17 14.08H3C2.46957 14.08 1.96086 13.8693 1.58579 13.4942C1.21071 13.1191 1 12.6104 1 12.08C1 11.5496 1.21071 11.0409 1.58579 10.6658C1.96086 10.2907 2.46957 10.08 3 10.08H3.09C3.42099 10.0723 3.742 9.96512 4.0113 9.77251C4.28059 9.5799 4.48572 9.31074 4.6 9C4.73312 8.69838 4.77282 8.36381 4.714 8.03941C4.65519 7.71502 4.50054 7.41568 4.27 7.18L4.21 7.12C4.02405 6.93425 3.87653 6.71368 3.77588 6.47088C3.67523 6.22808 3.62343 5.96783 3.62343 5.705C3.62343 5.44217 3.67523 5.18192 3.77588 4.93912C3.87653 4.69632 4.02405 4.47575 4.21 4.29C4.39575 4.10405 4.61632 3.95653 4.85912 3.85588C5.10192 3.75523 5.36217 3.70343 5.625 3.70343C5.88783 3.70343 6.14808 3.75523 6.39088 3.85588C6.63368 3.95653 6.85425 4.10405 7.04 4.29L7.1 4.35C7.33568 4.58054 7.63502 4.73519 7.95941 4.794C8.28381 4.85282 8.61838 4.81312 8.92 4.68H9C9.29577 4.55324 9.54802 4.34276 9.72569 4.07447C9.90337 3.80618 9.99872 3.49179 10 3.17V3C10 2.46957 10.2107 1.96086 10.5858 1.58579C10.9609 1.21071 11.4696 1 12 1C12.5304 1 13.0391 1.21071 13.4142 1.58579C13.7893 1.96086 14 2.46957 14 3V3.09C14.0013 3.41179 14.0966 3.72618 14.2743 3.99447C14.452 4.26276 14.7042 4.47324 15 4.6C15.3016 4.73312 15.6362 4.77282 15.9606 4.714C16.285 4.65519 16.5843 4.50054 16.82 4.27L16.88 4.21C17.0657 4.02405 17.2863 3.87653 17.5291 3.77588C17.7719 3.67523 18.0322 3.62343 18.295 3.62343C18.5578 3.62343 18.8181 3.67523 19.0609 3.77588C19.3037 3.87653 19.5243 4.02405 19.71 4.21C19.896 4.39575 20.0435 4.61632 20.1441 4.85912C20.2448 5.10192 20.2966 5.36217 20.2966 5.625C20.2966 5.88783 20.2448 6.14808 20.1441 6.39088C20.0435 6.63368 19.896 6.85425 19.71 7.04L19.65 7.1C19.4195 7.33568 19.2648 7.63502 19.206 7.95941C19.1472 8.28381 19.1869 8.61838 19.32 8.92V9C19.4468 9.29577 19.6572 9.54802 19.9255 9.72569C20.1938 9.90337 20.5082 9.99872 20.83 10H21C21.5304 10 22.0391 10.2107 22.4142 10.5858C22.7893 10.9609 23 11.4696 23 12C23 12.5304 22.7893 13.0391 22.4142 13.4142C22.0391 13.7893 21.5304 14 21 14H20.91C20.5882 14.0013 20.2738 14.0966 20.0055 14.2743C19.7372 14.452 19.5268 14.7042 19.4 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            抽奖模式
          </h3>
          
          <div class="grid grid-cols-1 gap-4">
            <div 
              v-for="mode in drawModes" 
              :key="mode.value" 
              class="relative flex items-center p-4 rounded-lg cursor-pointer transition-all border-2"
              :class="settings.drawMode === mode.value ? 
                'bg-purple-100 border-purple-400 shadow-sm' : 
                'bg-white border-gray-200 hover:border-purple-300'"
              @click="updateDrawMode(mode.value)"
            >
              <span class="mr-3 text-2xl">{{ mode.icon }}</span>
              <div class="flex-grow">
                <h4 class="font-bold text-gray-800">{{ mode.name }}</h4>
                <p class="text-sm text-gray-600">{{ mode.description }}</p>
              </div>
              <div 
                class="w-6 h-6 rounded-full flex items-center justify-center border-2"
                :class="settings.drawMode === mode.value ? 
                  'bg-purple-500 border-purple-500' : 
                  'border-gray-300'"
              >
                <svg v-if="settings.drawMode === mode.value" class="w-4 h-4 text-white" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M5 13L9 17L19 7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 奖品设置 -->
        <div class="bg-blue-50 rounded-xl p-5 border border-blue-200 shadow-sm">
          <h3 class="text-xl font-bold mb-4 text-blue-700 flex items-center">
            <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M19 14V22H5V14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M21 9H3V14H21V9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 22V9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 9H7.5C6.83696 9 6.20107 8.73661 5.73223 8.26777C5.26339 7.79893 5 7.16304 5 6.5C5 5.83696 5.26339 5.20107 5.73223 4.73223C6.20107 4.26339 6.83696 4 7.5 4C11 4 12 9 12 9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M12 9H16.5C17.163 9 17.7989 8.73661 18.2678 8.26777C18.7366 7.79893 19 7.16304 19 6.5C19 5.83696 18.7366 5.20107 18.2678 4.73223C17.7989 4.26339 17.163 4 16.5 4C13 4 12 9 12 9Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            奖品设置
          </h3>
          
          <div class="mb-4">
            <div class="flex items-center mb-2">
              <label class="text-gray-700 font-medium flex-grow">抽完后锁定转盘</label>
              <div class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in">
                <input 
                  type="checkbox" 
                  :checked="settings.lockAfterComplete" 
                  @change="toggleLockAfterComplete" 
                  class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                />
                <label 
                  class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                  :class="settings.lockAfterComplete ? 'bg-blue-500' : ''"
                ></label>
              </div>
            </div>
            <p class="text-sm text-gray-500">
              {{ settings.lockAfterComplete ? 
                '抽完所有奖品后将锁定转盘，不能继续抽取' : 
                '抽完后仍可继续抽取谢谢惠顾' }}
            </p>
          </div>
          
          <div class="mb-4">
            <label class="block text-gray-700 font-medium mb-2">普通奖品最多抽取次数</label>
            <div class="flex items-center">
              <button 
                @click="decreaseMaxDraws" 
                class="px-3 py-1 bg-blue-100 text-blue-700 rounded-l border border-blue-300"
                :disabled="settings.maxDraws <= 1"
              >-</button>
              <span class="px-4 py-1 bg-white border-t border-b border-blue-300">{{ settings.maxDraws }}</span>
              <button 
                @click="increaseMaxDraws" 
                class="px-3 py-1 bg-blue-100 text-blue-700 rounded-r border border-blue-300"
                :disabled="settings.maxDraws >= 5"
              >+</button>
            </div>
            <p class="mt-1 text-sm text-gray-500">每个奖品最多可以被抽中的次数</p>
          </div>
          
          <button 
            @click="resetSettings" 
            class="mt-4 px-4 py-2 bg-red-100 text-red-600 rounded-md border border-red-200 hover:bg-red-200 transition-colors"
          >
            恢复默认设置
          </button>
        </div>
      </div>
      
      <!-- 设置说明 -->
      <div class="mt-6 bg-yellow-50 rounded-xl p-5 border border-yellow-200">
        <h3 class="text-xl font-bold mb-2 text-yellow-700 flex items-center">
          <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 16V12M12 8H12.01M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          设置说明
        </h3>
        <div class="space-y-2 text-gray-700">
          <p><strong>单次模式</strong>：每个普通奖品最多只能抽中一次，抽完后只能抽到"谢谢惠顾"。适合每人限抽一次的活动。</p>
          <p><strong>标准模式</strong>：每个普通奖品最多抽中设定的次数，超过次数的奖品将不再出现，转为抽取"谢谢惠顾"。</p>
          <p><strong>顺序模式</strong>：会先抽完所有普通奖品，每种最多抽中设定的次数，所有奖品抽完后才会出现"谢谢惠顾"。</p>
          <p><strong>注意</strong>：设置修改后将立即生效，并保存至下次使用。</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useWheelSettings, DrawMode } from '../utils/wheelSettings';

// 获取全局设置
const { 
  settings, 
  updateDrawMode, 
  updateLockAfterComplete,
  updateMaxDraws,
  resetSettings
} = useWheelSettings();

// 抽奖模式列表
const drawModes = [
  { 
    name: '单次模式', 
    value: DrawMode.SINGLE, 
    icon: '🎯',
    description: '每个奖品最多抽中一次，适合每人限抽一次的场景'
  },
  { 
    name: '标准模式', 
    value: DrawMode.STANDARD, 
    icon: '🎲',
    description: '每个奖品最多抽中指定次数，超过则抽谢谢惠顾'
  },
  { 
    name: '顺序模式', 
    value: DrawMode.SEQUENCE, 
    icon: '📋',
    description: '先抽完所有奖品，然后才抽谢谢惠顾'
  }
];

// 切换是否抽完后锁定
function toggleLockAfterComplete() {
  updateLockAfterComplete(!settings.lockAfterComplete);
}

// 增加最大抽奖次数
function increaseMaxDraws() {
  if (settings.maxDraws < 5) {
    updateMaxDraws(settings.maxDraws + 1);
  }
}

// 减少最大抽奖次数
function decreaseMaxDraws() {
  if (settings.maxDraws > 1) {
    updateMaxDraws(settings.maxDraws - 1);
  }
}
</script>

<style scoped>
/* 开关样式 */
.toggle-checkbox:checked {
  right: 0;
  transform: translateX(100%);
  border-color: white;
}

.toggle-label {
  transition: background-color 0.2s ease-in;
}
</style>