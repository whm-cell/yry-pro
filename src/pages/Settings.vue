<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-3xl shadow-lg p-6 h-full flex flex-col border-4 border-dashed border-purple-300">
      <h2 class="text-3xl font-bold mb-6 text-center text-purple-700">系统设置</h2>

      <!-- 左侧导航栏 -->
      <div class="flex flex-grow overflow-hidden">
        <div class="w-64 bg-gray-50 rounded-xl mr-6 p-4 shadow-sm">
          <div class="text-lg font-bold text-gray-700 mb-4">设置菜单</div>
          <ul class="space-y-1">
            <!-- 英语转盘设置（唯一菜单项） -->
            <li 
              class="px-4 py-3 rounded-lg cursor-pointer transition-all flex items-center bg-purple-100 text-purple-700"
            >
              <component :is="WheelIcon" class="w-5 h-5 mr-2" />
              <span>英语转盘设置</span>
            </li>
          </ul>
        </div>

        <!-- 右侧设置内容 -->
        <div class="flex-grow bg-gray-50 rounded-xl p-6 overflow-y-auto">
          <!-- 转盘设置 -->
          <div class="space-y-6">
            <h3 class="text-xl font-bold text-gray-800 mb-4">英语转盘设置</h3>
            
            <!-- 抽奖模式设置 -->
            <div class="bg-purple-50 rounded-xl p-5 border border-purple-200 shadow-sm">
              <h4 class="text-lg font-medium text-purple-700 mb-3">抽奖模式</h4>
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

              <div class="mt-4">
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
                      :class="settings.lockAfterComplete ? 'bg-purple-500' : ''"
                    ></label>
                  </div>
                </div>
                <p class="text-sm text-gray-500">
                  {{ settings.lockAfterComplete ? 
                    '抽完所有奖品后将锁定转盘，不能继续抽取' : 
                    '抽完后仍可继续抽取Magic Bag' }}
                </p>
              </div>
            </div>
            
            <!-- 英语转盘特殊设置 -->
            <div class="bg-indigo-50 rounded-xl p-5 border border-indigo-200 shadow-sm">
              <h4 class="text-lg font-medium text-indigo-700 mb-3">英语转盘特殊设置</h4>
              <div class="mb-4">
              </div>

           
              <div class="mt-4">
                <label class="block text-gray-700 font-medium mb-2">普通奖品最多抽取次数</label>
                <div class="flex items-center">
                  <button 
                    @click="decreaseMaxDraws" 
                    class="px-3 py-1 bg-indigo-100 text-indigo-700 rounded-l border border-indigo-300"
                    :disabled="settings.maxDraws <= 1"
                  >-</button>
                  <span class="px-4 py-1 bg-white border-t border-b border-indigo-300">{{ settings.maxDraws }}</span>
                  <button 
                    @click="increaseMaxDraws" 
                    class="px-3 py-1 bg-indigo-100 text-indigo-700 rounded-r border border-indigo-300"
                    :disabled="settings.maxDraws >= 5"
                  >+</button>
                </div>
                <p class="mt-1 text-sm text-gray-500">每个奖品最多可以被抽中的次数</p>
              </div>
            </div>
            
            <!-- 音效设置 -->
            <div class="bg-orange-50 rounded-xl p-5 border border-orange-200 shadow-sm mt-6">
              <h4 class="text-lg font-medium text-orange-700 mb-3">音效设置</h4>
              
              <div class="space-y-4">
                <!-- 转盘旋转音效 -->
                <div>
                  <label class="block text-gray-700 font-medium mb-2">转盘旋转音效</label>
                  <div class="flex items-center">
                    <div class="flex-grow">
                      <div class="text-sm text-gray-600 truncate" :class="{'text-orange-500': !isAudioSet('spin')}">
                        {{ getSelectedSoundName('spin') }}
                      </div>
                    </div>
                    <div class="flex">
                      <button 
                        @click="playSelectedSound('spin')" 
                        class="mr-2 px-3 py-1 bg-orange-100 text-orange-700 rounded border border-orange-300 hover:bg-orange-200 transition-colors"
                        :disabled="!isAudioSet('spin')"
                      >
                        试听
                      </button>
                      <button 
                        @click="openSoundUploader('spin')" 
                        class="px-3 py-1 bg-orange-100 text-orange-700 rounded border border-orange-300 hover:bg-orange-200 transition-colors"
                      >
                        {{ isAudioSet('spin') ? '更改' : '选择' }}
                      </button>
                    </div>
                  </div>
                </div>
                
                <!-- 中奖音效 -->
                <div>
                  <label class="block text-gray-700 font-medium mb-2">中奖音效</label>
                  <div class="flex items-center">
                    <div class="flex-grow">
                      <div class="text-sm text-gray-600 truncate" :class="{'text-orange-500': !isAudioSet('win')}">
                        {{ getSelectedSoundName('win') }}
                      </div>
                    </div>
                    <div class="flex">
                      <button 
                        @click="playSelectedSound('win')" 
                        class="mr-2 px-3 py-1 bg-orange-100 text-orange-700 rounded border border-orange-300 hover:bg-orange-200 transition-colors"
                        :disabled="!isAudioSet('win')"
                      >
                        试听
                      </button>
                      <button 
                        @click="openSoundUploader('win')" 
                        class="px-3 py-1 bg-orange-100 text-orange-700 rounded border border-orange-300 hover:bg-orange-200 transition-colors"
                      >
                        {{ isAudioSet('win') ? '更改' : '选择' }}
                      </button>
                    </div>
                  </div>
                </div>
                
                <!-- 添加音频消息提示 -->
                <div 
                  v-if="showAudioMessage" 
                  class="p-3 rounded-md text-center transition-all duration-300 font-medium flex items-center justify-center"
                  :class="audioMessage.includes('失败') ? 'bg-red-100 text-red-700' : 'bg-green-100 text-green-700'"
                >
                  <span v-if="audioMessage.includes('失败')" class="mr-2">⚠️</span>
                  <span v-else class="mr-2">🔊</span>
                  {{ audioMessage }}
                </div>
                
                <p class="text-sm text-gray-500 mt-2">
                  音效可以使用预设音效或上传自定义音效（支持MP3、WAV格式）
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  
  <!-- 音效选择器对话框 -->
  <div v-if="showSoundUploader" class="sound-uploader-overlay" @click.self="closeSoundUploader">
    <div class="sound-uploader-modal">
      <div class="sound-uploader-header">
        <h3 class="text-xl font-bold text-gray-800">选择音效</h3>
        <button @click="closeSoundUploader" class="text-gray-500 hover:text-gray-700">✕</button>
      </div>
      
      <div class="sound-uploader-content">
        <SoundUploader @sound-selected="handleSoundSelected" @cancel="handleSoundCancel" />
      </div>
    </div>
  </div>
  
  <!-- 隐藏的音频元素 -->
  <audio ref="audioPlayer" style="display:none"></audio>
</template>

<script setup lang="ts">
import { ref, markRaw, h, onMounted } from 'vue';
import { useWheelSettings, DrawMode, SoundSetting } from '../utils/wheelSettings';
import SoundUploader from '../components/SoundUploader.vue';
import * as tauriApi from '@tauri-apps/api/core';

// 获取转盘设置
const { 
  settings, 
  updateDrawMode, 
  updateLockAfterComplete,
  updateMaxDraws,
  updateSound
} = useWheelSettings();

// 抽奖模式列表
const drawModes = [
  { 
    name: '有序模式', 
    value: DrawMode.ORDERLY, 
    icon: '📋',
    description: '每个奖品都要抽一次，抽完后只能抽到Magic Bag'
  },
  { 
    name: '随机模式', 
    value: DrawMode.RANDOM, 
    icon: '🎲',
    description: '奖品和Magic Bag完全随机，抽到哪个是哪个'
  }
];

// 创建图标组件
const WheelIcon = markRaw({
  render() {
    return h('svg', {
      xmlns: "http://www.w3.org/2000/svg",
      fill: "none",
      viewBox: "0 0 24 24",
      stroke: "currentColor"
    }, [
      h('path', {
        'stroke-linecap': "round",
        'stroke-linejoin': "round",
        'stroke-width': "2",
        d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
      })
    ]);
  }
});

// 转盘设置相关函数
function toggleLockAfterComplete(): void {
  const currentSetting = settings.lockAfterComplete;
  updateLockAfterComplete(!currentSetting);
}

async function increaseMaxDraws(): Promise<void> {
  const currentMaxDraws = settings.maxDraws;
  if (currentMaxDraws < 5) {
    await updateMaxDraws(currentMaxDraws + 1);
  }
}

async function decreaseMaxDraws(): Promise<void> {
  const currentMaxDraws = settings.maxDraws;
  if (currentMaxDraws > 1) {
    await updateMaxDraws(currentMaxDraws - 1);
  }
}

// 音效设置相关变量
const selectedSoundType = ref<'spin' | 'win'>('spin');
const showSoundUploader = ref(false);

// 添加消息提示的状态
const audioMessage = ref('');
const showAudioMessage = ref(false);

// 音频播放器引用
const audioPlayer = ref<HTMLAudioElement | null>(null);

// 获取当前选中音效名称
function getSelectedSoundName(type: 'spin' | 'win'): string {
  if (!settings.sounds || !settings.sounds[type]) {
    return '未设置';
  }
  
  const sound = settings.sounds[type];
  if (sound.type === 'preset') {
    return `预设: ${sound.name}`;
  } else {
    return `自定义: ${sound.name.split('_').slice(1).join('_')}`;
  }
}

// 检查音效是否已设置
function isAudioSet(type: 'spin' | 'win'): boolean {
  return !!(settings.sounds && settings.sounds[type]);
}

// 播放选中的音效
async function playSelectedSound(type: 'spin' | 'win'): Promise<void> {
  // 检查音效设置是否存在
  if (!isAudioSet(type)) {
    audioMessage.value = '请先选择音效';
    showAudioMessage.value = true;
    setTimeout(() => { showAudioMessage.value = false; }, 3000);
    return;
  }
  
  try {
    // 显示音效信息
    const soundName = settings.sounds[type].name;
    audioMessage.value = `正在准备播放${type === 'spin' ? '旋转' : '中奖'}音效: ${soundName}`;
    showAudioMessage.value = true;
    // 获取并转换音效URL
    let soundUrl = settings.sounds[type].url;
    
    // 创建新的Audio实例
    const audio = new Audio();
    
    // 设置音频事件处理
    const audioPromise = new Promise((resolve, reject) => {
      let hasError = false;
      
      // 监听音频加载完成事件
      audio.addEventListener('loadeddata', () => {
        if (!hasError) {
          resolve(audio);
        }
      });
      
      // 监听音频加载错误事件
      audio.addEventListener('error', () => {
        hasError = true;
        console.error('音频加载错误:', audio.error);
        reject(new Error(`音频加载失败: ${audio.error?.message || '未知错误'}`));
      });
      
      // 设置超时，防止无限等待
      setTimeout(() => {
        if (!hasError) {
          reject(new Error('音频加载超时'));
        }
      }, 5000);
    });
    
    // 设置音频源并等待加载
    audio.src = soundUrl;
    audio.volume = 0.5;
    
    try {
      // 等待音频加载完成
      await audioPromise;
      
      try {
        // 尝试播放音频
        await audio.play();
        
        // 播放完成后的清理
        audio.addEventListener('ended', () => {
          audio.remove();
          audioMessage.value = '播放完成';
          showAudioMessage.value = true;
          setTimeout(() => { showAudioMessage.value = false; }, 1500);
        });
        
      } catch (playError: any) {
        // 处理播放音频时的常见错误
        if (playError.name === 'NotAllowedError') {
          // 浏览器要求用户交互后才能播放音频
          audioMessage.value = '需要用户交互才能播放声音，请点击页面任意位置';
          showAudioMessage.value = true;
          
          // 添加一次性点击事件监听器
          const playAfterInteraction = async () => {
            try {
              await audio.play();
            } catch (e) {
              audioMessage.value = '播放失败，请检查音频文件是否正确';
              showAudioMessage.value = true;
            }
            document.removeEventListener('click', playAfterInteraction);
          };
          
          document.addEventListener('click', playAfterInteraction, { once: true });
        } else {
          throw playError;
        }
      }
    } catch (error: any) {
      // 处理音频加载失败
      console.error('音频处理失败:', error);
      audioMessage.value = `音频处理失败: ${error.message || '未知错误'}`;
      showAudioMessage.value = true;
      audio.remove();
    }
    
  } catch (error: any) {
    // 处理整体播放过程中的错误
    console.error('播放音效失败:', error);
    audioMessage.value = `播放音效失败: ${error.message || '未知错误'}`;
    showAudioMessage.value = true;
    setTimeout(() => { showAudioMessage.value = false; }, 3000);
  }
}

// 打开音效选择器对话框
function openSoundUploader(type: 'spin' | 'win'): void {
  selectedSoundType.value = type;
  showSoundUploader.value = true;
}

// 关闭音效选择器对话框
function closeSoundUploader(): void {
  showSoundUploader.value = false;
}

// 处理音效选择
function handleSoundSelected(sound: SoundSetting): void {
  updateSound(selectedSoundType.value, sound);
  closeSoundUploader();
  
  // 显示成功消息
  audioMessage.value = `已设置${selectedSoundType.value === 'spin' ? '旋转' : '中奖'}音效: ${sound.name}`;
  showAudioMessage.value = true;
  setTimeout(() => { showAudioMessage.value = false; }, 3000);
}

// 取消选择音效
function handleSoundCancel(): void {
  closeSoundUploader();
}

// 组件初始化
onMounted(async () => {
  // 检查audio元素是否成功绑定
  if (!audioPlayer.value) {
    const audioElement = document.querySelector('audio');
    if (audioElement) {
      audioPlayer.value = audioElement as HTMLAudioElement;
    }
  }
  
  try {
    // 确保音频目录存在
    if (tauriApi.invoke) {
      try {
        await tauriApi.invoke('ensure_sounds_dir');
      } catch (err) {
        console.warn('确认音频目录失败:', err);
      }
    }
    
    // 检查当前音效文件是否存在并可用
    const testAudio = async (soundPath: string): Promise<boolean> => {
      return new Promise((resolve) => {
        if (tauriApi.convertFileSrc) {
          try {
            const audio = new Audio();
            audio.src = tauriApi.convertFileSrc(soundPath);
            
            audio.addEventListener('canplaythrough', () => {
              resolve(true);
            });
            
            audio.addEventListener('error', () => {
              resolve(false);
            });
            
            // 设置超时，防止无限等待
            setTimeout(() => resolve(false), 2000);
          } catch (e) {
            resolve(false);
          }
        } else {
          resolve(false);
        }
      });
    };
    
    // 验证旋转音效是否可用
    if (settings.sounds?.spin?.url) {
      const spinExists = await testAudio(settings.sounds.spin.url);
      if (!spinExists) {
        audioMessage.value = "当前旋转音效不可用，请重新选择";
        showAudioMessage.value = true;
        setTimeout(() => { showAudioMessage.value = false; }, 3000);
      }
    }
    
    // 验证中奖音效是否可用
    if (settings.sounds?.win?.url) {
      const winExists = await testAudio(settings.sounds.win.url);
      if (!winExists) {
        audioMessage.value = "当前中奖音效不可用，请重新选择";
        showAudioMessage.value = true;
        setTimeout(() => { showAudioMessage.value = false; }, 3000);
      }
    }
    
  } catch (error: any) {
    console.error('音效初始化失败:', error);
  }
});
</script>

<style scoped>
/* 开关样式 */
.toggle-checkbox {
  right: 0;
  border-color: #d1d5db;
  z-index: 10;
}

.toggle-checkbox:checked {
  right: 0;
  transform: translateX(100%);
  border-color: white;
}

.toggle-label {
  transition: background-color 0.2s ease-in;
}

/* 滑块样式 */
input[type=range] {
  height: 6px;
  -webkit-appearance: none;
  margin: 10px 0;
  background: #edf2f7;
  border-radius: 5px;
}

input[type=range]::-webkit-slider-thumb {
  height: 16px;
  width: 16px;
  border-radius: 50%;
  background: #3b82f6;
  cursor: pointer;
  -webkit-appearance: none;
}

/* 音效选择器样式 */
.sound-uploader-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 100;
}

.sound-uploader-modal {
  background-color: white;
  border-radius: 0.75rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.sound-uploader-header {
  padding: 1rem 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid rgba(229, 231, 235);
}

.sound-uploader-content {
  padding: 1.5rem;
  overflow-y: auto;
  flex-grow: 1;
}
</style>