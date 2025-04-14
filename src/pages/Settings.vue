<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-3xl shadow-lg p-6 h-full flex flex-col border-4 border-dashed border-purple-300">
      <h2 class="text-3xl font-bold mb-6 text-center text-purple-700">系统设置</h2>

      <!-- 左侧导航栏 -->
      <div class="flex flex-grow overflow-hidden">
        <div class="w-64 bg-gray-50 rounded-xl mr-6 p-4 shadow-sm">
          <div class="text-lg font-bold text-gray-700 mb-4">设置菜单</div>
          <ul class="space-y-1">
            <li 
              v-for="section in settingSections" 
              :key="section.id"
              @click="activeSection = section.id"
              class="px-4 py-3 rounded-lg cursor-pointer transition-all flex items-center"
              :class="activeSection === section.id ? 'bg-purple-100 text-purple-700' : 'hover:bg-gray-100 text-gray-600'"
            >
              <component :is="section.icon" class="w-5 h-5 mr-2" />
              <span>{{ section.name }}</span>
            </li>
          </ul>
        </div>

        <!-- 右侧设置内容 -->
        <div class="flex-grow bg-gray-50 rounded-xl p-6 overflow-y-auto">
          <!-- 系统外观设置 -->
          <div v-if="activeSection === 'appearance'" class="space-y-6">
            <h3 class="text-xl font-bold text-gray-800 mb-4">系统外观设置</h3>
            
            <!-- 主题色调 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">主题色调</h4>
              <div class="grid grid-cols-2 gap-4">
                <div 
                  v-for="theme in themes" 
                  :key="theme.id"
                  @click="updateTheme(theme.id)"
                  class="relative rounded-lg overflow-hidden border-2 cursor-pointer transition-all"
                  :class="systemSettings.theme === theme.id ? 'border-purple-500 shadow-md' : 'border-transparent hover:border-gray-300'"
                >
                  <div class="h-20" :style="{background: theme.gradient}"></div>
                  <div class="p-3 bg-white">
                    <div class="font-medium">{{ theme.name }}</div>
                    <div class="text-xs text-gray-500">{{ theme.description }}</div>
                  </div>
                  <div 
                    v-if="systemSettings.theme === theme.id"
                    class="absolute top-2 right-2 bg-purple-500 text-white rounded-full p-1"
                  >
                    <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M5 13L9 17L19 7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 字体设置 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">字体设置</h4>
              <div class="grid grid-cols-1 gap-4">
                <div>
                  <label class="block text-gray-600 mb-2">字体大小</label>
                  <div class="flex items-center">
                    <button 
                      @click="decreaseFontSize" 
                      class="px-3 py-1 bg-gray-100 text-gray-700 rounded-l border border-gray-300"
                      :disabled="systemSettings.fontSize <= 12"
                    >-</button>
                    <span class="px-4 py-1 bg-white border-t border-b border-gray-300">{{ systemSettings.fontSize }}px</span>
                    <button 
                      @click="increaseFontSize" 
                      class="px-3 py-1 bg-gray-100 text-gray-700 rounded-r border border-gray-300"
                      :disabled="systemSettings.fontSize >= 20"
                    >+</button>
                  </div>
                </div>
                
                <div>
                  <label class="block text-gray-600 mb-2">字体样式</label>
                  <div class="grid grid-cols-3 gap-2">
                    <div 
                      v-for="font in fonts" 
                      :key="font.id"
                      @click="updateFont(font.id)"
                      class="px-3 py-2 rounded border cursor-pointer text-center"
                      :class="systemSettings.font === font.id ? 'bg-purple-50 border-purple-300 text-purple-700' : 'bg-white border-gray-200 hover:border-gray-300'"
                      :style="{fontFamily: font.family}"
                    >
                      {{ font.name }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 界面模式 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">界面模式</h4>
              <div class="flex items-center space-x-4">
                <div 
                  @click="updateMode('light')" 
                  class="flex-1 p-4 rounded-lg border-2 cursor-pointer text-center"
                  :class="systemSettings.mode === 'light' ? 'bg-orange-50 border-orange-300' : 'bg-white border-gray-200 hover:border-gray-300'"
                >
                  <div class="text-2xl mb-1">☀️</div>
                  <div class="font-medium">浅色模式</div>
                </div>
                <div 
                  @click="updateMode('dark')" 
                  class="flex-1 p-4 rounded-lg border-2 cursor-pointer text-center"
                  :class="systemSettings.mode === 'dark' ? 'bg-indigo-50 border-indigo-300' : 'bg-white border-gray-200 hover:border-gray-300'"
                >
                  <div class="text-2xl mb-1">🌙</div>
                  <div class="font-medium">深色模式</div>
                </div>
                <div 
                  @click="updateMode('auto')" 
                  class="flex-1 p-4 rounded-lg border-2 cursor-pointer text-center"
                  :class="systemSettings.mode === 'auto' ? 'bg-green-50 border-green-300' : 'bg-white border-gray-200 hover:border-gray-300'"
                >
                  <div class="text-2xl mb-1">🔄</div>
                  <div class="font-medium">自动模式</div>
                </div>
              </div>
              <p class="mt-2 text-sm text-gray-500">自动模式将根据系统设置自动切换浅色/深色模式</p>
            </div>
            
            <!-- 动画效果 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-lg font-medium text-gray-700">界面动画</h4>
                <div class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in">
                  <input 
                    type="checkbox" 
                    :checked="systemSettings.animations" 
                    @change="toggleAnimations" 
                    class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                  />
                  <label 
                    class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                    :class="systemSettings.animations ? 'bg-green-500' : ''"
                  ></label>
                </div>
              </div>
              <p class="text-sm text-gray-500">启用或禁用系统界面过渡动画效果</p>
            </div>
          </div>
          
          <!-- AI设置 -->
          <div v-if="activeSection === 'ai'" class="space-y-6">
            <h3 class="text-xl font-bold text-gray-800 mb-4">AI助手设置</h3>
            
            <!-- AI模型选择 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">AI模型</h4>
              <div class="space-y-3">
                <div 
                  v-for="model in aiModels" 
                  :key="model.id"
                  @click="updateAiModel(model.id)"
                  class="relative flex items-center p-4 rounded-lg border-2 cursor-pointer transition-all"
                  :class="aiSettings.model === model.id ? 'bg-blue-50 border-blue-300 shadow-sm' : 'bg-white border-gray-200 hover:border-gray-300'"
                >
                  <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-4 text-blue-600">
                    {{ model.icon }}
                  </div>
                  <div class="flex-grow">
                    <div class="font-medium">{{ model.name }}</div>
                    <div class="text-sm text-gray-500">{{ model.description }}</div>
                  </div>
                  <div 
                    class="w-6 h-6 rounded-full flex items-center justify-center border-2"
                    :class="aiSettings.model === model.id ? 'bg-blue-500 border-blue-500' : 'border-gray-300'"
                  >
                    <svg v-if="aiSettings.model === model.id" class="w-4 h-4 text-white" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M5 13L9 17L19 7" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </div>
                </div>
              </div>
              <p class="mt-2 text-sm text-gray-500">选择不同的AI模型将影响智能助手的能力和速度</p>
            </div>
            
            <!-- AI语言设置 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">AI语言偏好</h4>
              <div class="flex items-center mb-4">
                <label class="flex-grow text-gray-600">AI回答语言</label>
                <select 
                  v-model="aiSettings.language" 
                  class="py-2 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="zh">中文</option>
                  <option value="en">英文</option>
                  <option value="auto">自动检测</option>
                </select>
              </div>
              
              <div class="mt-4">
                <label class="text-gray-600 block mb-2">响应详细程度</label>
                <div class="flex items-center justify-between">
                  <span class="text-sm text-gray-500">简洁</span>
                  <input 
                    type="range" 
                    v-model="aiSettings.verbosity" 
                    min="1" 
                    max="5" 
                    class="w-2/3 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                  />
                  <span class="text-sm text-gray-500">详细</span>
                </div>
              </div>
            </div>
            
            <!-- AI功能设置 -->
            <div class="bg-white rounded-xl p-5 border border-gray-200 shadow-sm">
              <h4 class="text-lg font-medium text-gray-700 mb-3">AI功能设置</h4>
              <div class="space-y-3">
                <div class="flex items-center">
                  <label class="flex-grow text-gray-600">自动建议</label>
                  <div class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in">
                    <input 
                      type="checkbox" 
                      :checked="aiSettings.autoSuggestions" 
                      @change="toggleAutoSuggestions" 
                      class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                    />
                    <label 
                      class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                      :class="aiSettings.autoSuggestions ? 'bg-blue-500' : ''"
                    ></label>
                  </div>
                </div>
                <p class="text-sm text-gray-500 ml-0">启用后，AI将在适当时机提供智能建议</p>
                
                <div class="flex items-center mt-4">
                  <label class="flex-grow text-gray-600">学习数据收集</label>
                  <div class="relative inline-block w-12 align-middle select-none transition duration-200 ease-in">
                    <input 
                      type="checkbox" 
                      :checked="aiSettings.dataCollection" 
                      @change="toggleDataCollection" 
                      class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                    />
                    <label 
                      class="toggle-label block overflow-hidden h-6 rounded-full bg-gray-300 cursor-pointer"
                      :class="aiSettings.dataCollection ? 'bg-blue-500' : ''"
                    ></label>
                  </div>
                </div>
                <p class="text-sm text-gray-500 ml-0">允许系统收集使用数据以改进AI功能（不包含个人信息）</p>
              </div>
            </div>
          </div>
          
          <!-- 转盘设置 -->
          <div v-if="activeSection === 'wheel'" class="space-y-6">
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
                    '抽完后仍可继续抽取谢谢惠顾' }}
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
          </div>
          
          <!-- 关于系统 -->
          <div v-if="activeSection === 'about'" class="space-y-6">
            <h3 class="text-xl font-bold text-gray-800 mb-4">关于系统</h3>
            
            <div class="bg-white rounded-xl p-6 border border-gray-200 shadow-sm">
              <div class="flex items-center justify-center mb-6">
                <div class="w-24 h-24 bg-gradient-to-r from-purple-500 to-indigo-600 rounded-full flex items-center justify-center text-white text-2xl font-bold shadow-lg">
                  YRY
                </div>
              </div>
              
              <div class="text-center mb-6">
                <h4 class="text-xl font-bold text-gray-800">英语大转盘系统</h4>
                <p class="text-gray-500">版本 1.0.0</p>
              </div>
              
              <div class="space-y-4 text-center">
                <p class="text-gray-600">本系统旨在提供交互式的英语学习体验，通过游戏化的方式激发学习兴趣。</p>
                <p class="text-gray-600">Copyright © 2023 YRY教育科技</p>
                <button class="px-4 py-2 bg-indigo-50 text-indigo-600 rounded-md hover:bg-indigo-100 transition-colors">
                  检查更新
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, markRaw, h } from 'vue';
import { useWheelSettings, DrawMode } from '../utils/wheelSettings';

// 获取转盘设置
const { 
  settings, 
  updateDrawMode, 
  updateLockAfterComplete,
  updateMaxDraws
} = useWheelSettings();

// 定义设置分类图标
const AppearanceIcon = markRaw({
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
        d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
      })
    ]);
  }
});

const AiIcon = markRaw({
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
        d: "M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
      })
    ]);
  }
});

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

const AboutIcon = markRaw({
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
        d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      })
    ]);
  }
});

// 设置分类列表
const settingSections = [
  // { id: 'appearance', name: '系统外观', icon: AppearanceIcon },
  // { id: 'ai', name: 'AI助手设置', icon: AiIcon },
  { id: 'wheel', name: '英语转盘设置', icon: WheelIcon },
  // { id: 'about', name: '关于系统', icon: AboutIcon }
];

// 当前激活的设置分类
const activeSection = ref('appearance');

// 系统外观设置
interface SystemSettings {
  theme: string;
  fontSize: number;
  font: string;
  mode: 'light' | 'dark' | 'auto';
  animations: boolean;
}

const systemSettings = reactive<SystemSettings>({
  theme: 'purple',
  fontSize: 14,
  font: 'default',
  mode: 'light',
  animations: true
});

// 主题选项
const themes = [
  { 
    id: 'purple', 
    name: '梦幻紫', 
    description: '明亮活泼的紫色主题',
    gradient: 'linear-gradient(135deg, #a78bfa, #7c3aed)'
  },
  { 
    id: 'blue', 
    name: '海洋蓝', 
    description: '清新稳重的蓝色主题',
    gradient: 'linear-gradient(135deg, #93c5fd, #3b82f6)'
  },
  { 
    id: 'green', 
    name: '自然绿', 
    description: '舒适平和的绿色主题',
    gradient: 'linear-gradient(135deg, #6ee7b7, #059669)'
  },
  { 
    id: 'orange', 
    name: '活力橙', 
    description: '充满活力的橙色主题',
    gradient: 'linear-gradient(135deg, #fdba74, #ea580c)'
  }
];

// 字体选项
const fonts = [
  { id: 'default', name: '默认', family: 'system-ui, -apple-system, sans-serif' },
  { id: 'serif', name: '衬线体', family: 'serif' },
  { id: 'mono', name: '等宽体', family: 'monospace' }
];

// AI设置
interface AiSettings {
  model: string;
  language: string;
  verbosity: number;
  autoSuggestions: boolean;
  dataCollection: boolean;
}

const aiSettings = reactive<AiSettings>({
  model: 'default',
  language: 'zh',
  verbosity: 3,
  autoSuggestions: true,
  dataCollection: false
});

// AI模型选项
const aiModels = [
  { 
    id: 'default', 
    name: '通用模型', 
    description: '平衡性能与速度的基础模型',
    icon: '🤖'
  },
  { 
    id: 'education', 
    name: '教育专用模型', 
    description: '针对教育场景优化的专业模型',
    icon: '📚'
  },
  { 
    id: 'advanced', 
    name: '高级模型', 
    description: '更强大的理解和生成能力，但速度较慢',
    icon: '🧠'
  }
];

// 抽奖模式列表
const drawModes = [
  { 
    name: '有序模式', 
    value: DrawMode.ORDERLY, 
    icon: '📋',
    description: '每个奖品都要抽一次，抽完后只能抽到谢谢惠顾'
  },
  { 
    name: '随机模式', 
    value: DrawMode.RANDOM, 
    icon: '🎲',
    description: '奖品和谢谢惠顾完全随机，抽到哪个是哪个'
  }
];

// 英语转盘特殊设置
interface EnglishSettings {
  wordLevel: 'elementary' | 'intermediate' | 'advanced';
  displayMode: 'word' | 'phonetic' | 'both';
  autoPronounciation: boolean;
}

const englishSettings = reactive<EnglishSettings>({
  wordLevel: 'elementary',
  displayMode: 'both',
  autoPronounciation: true
});

const wordLevels = [
  { name: '初级', value: 'elementary' as const },
  { name: '中级', value: 'intermediate' as const },
  { name: '高级', value: 'advanced' as const }
];

const wordDisplayModes = [
  { name: '仅单词', value: 'word' as const },
  { name: '仅音标', value: 'phonetic' as const },
  { name: '单词和音标', value: 'both' as const }
];

// 系统外观设置相关函数
function updateTheme(themeId: string): void {
  systemSettings.theme = themeId;
}

function increaseFontSize(): void {
  if (systemSettings.fontSize < 20) {
    systemSettings.fontSize += 1;
  }
}

function decreaseFontSize(): void {
  if (systemSettings.fontSize > 12) {
    systemSettings.fontSize -= 1;
  }
}

function updateFont(fontId: string): void {
  systemSettings.font = fontId;
}

function updateMode(mode: 'light' | 'dark' | 'auto'): void {
  systemSettings.mode = mode;
}

function toggleAnimations(): void {
  systemSettings.animations = !systemSettings.animations;
}

// AI设置相关函数
function updateAiModel(modelId: string): void {
  aiSettings.model = modelId;
}

function toggleAutoSuggestions(): void {
  aiSettings.autoSuggestions = !aiSettings.autoSuggestions;
}

function toggleDataCollection(): void {
  aiSettings.dataCollection = !aiSettings.dataCollection;
}

// 转盘设置相关函数
function toggleLockAfterComplete(): void {
  const currentSetting = settings.lockAfterComplete;
  updateLockAfterComplete(!currentSetting);
}

function increaseMaxDraws(): void {
  const currentMaxDraws = settings.maxDraws;
  if (currentMaxDraws < 5) {
    updateMaxDraws(currentMaxDraws + 1);
  }
}

function decreaseMaxDraws(): void {
  const currentMaxDraws = settings.maxDraws;
  if (currentMaxDraws > 1) {
    updateMaxDraws(currentMaxDraws - 1);
  }
}

// 英语转盘特殊设置相关函数
function updateWordLevel(level: EnglishSettings['wordLevel']): void {
  englishSettings.wordLevel = level;
}

function updateWordDisplayMode(mode: EnglishSettings['displayMode']): void {
  englishSettings.displayMode = mode;
}

function toggleAutoPronounciation(): void {
  englishSettings.autoPronounciation = !englishSettings.autoPronounciation;
}
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
</style>