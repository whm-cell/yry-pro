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
      
      <!-- 抽奖设置面板 -->
      <div v-if="activeTab === 1">
        <h3 class="text-lg font-semibold mb-4">抽奖设置</h3>
        
        <!-- 一级标签选择器 -->
        <div class="mb-6">
          <label class="block text-sm font-medium text-gray-700 mb-2">抽奖类型</label>
          <div class="flex flex-wrap gap-3 mb-4">
            <div 
              v-for="(category, index) in lotteryCategories" 
              :key="index"
              class="cursor-pointer px-4 py-2 rounded-lg text-sm font-medium transition-all"
              :class="selectedCategory === index ? 'bg-blue-100 text-blue-700 border-2 border-blue-300' : 'bg-gray-100 text-gray-700 border-2 border-gray-200 hover:bg-gray-200'"
              @click="selectedCategory = index"
            >
              {{ category.name }}
            </div>
            <div 
              class="cursor-pointer px-4 py-2 rounded-lg text-sm font-medium bg-green-100 text-green-700 border-2 border-green-200 hover:bg-green-200 flex items-center"
              @click="showAddCategoryModal = true"
            >
              <span class="mr-1">+</span> 添加新类型
            </div>
          </div>
        </div>
        
        <!-- 二级标签设置 -->
        <div v-if="selectedCategory !== null" class="mb-6 pb-6 border-b border-gray-200">
          <div class="flex justify-between items-center mb-4">
            <h4 class="font-medium">{{ lotteryCategories[selectedCategory].name }} 抽奖模式</h4>
            <button 
              class="text-sm px-3 py-1 bg-blue-50 text-blue-600 rounded-md hover:bg-blue-100 flex items-center"
              @click="showAddModeModal = true"
            >
              <span class="mr-1">+</span> 添加新模式
            </button>
          </div>
          
          <!-- 抽奖模式列表 -->
          <div class="space-y-4">
            <div 
              v-for="(mode, modeIndex) in lotteryCategories[selectedCategory].modes" 
              :key="modeIndex"
              class="p-4 bg-gray-50 rounded-lg border border-gray-200 transition-all"
              :class="{'border-blue-300 bg-blue-50': selectedModeIndex === modeIndex}"
            >
              <div class="flex justify-between mb-3">
                <div class="flex items-center">
                  <input 
                    type="radio" 
                    :id="`mode-${modeIndex}`" 
                    :name="`lottery-mode-${selectedCategory}`"
                    :checked="selectedModeIndex === modeIndex"
                    @change="selectedModeIndex = modeIndex"
                    class="mr-2 h-4 w-4 text-blue-600"
                  >
                  <label :for="`mode-${modeIndex}`" class="font-medium">{{ mode.name }}</label>
                </div>
                <div class="flex gap-2">
                  <button 
                    class="text-xs px-2 py-1 bg-gray-200 text-gray-700 rounded hover:bg-gray-300"
                    @click="editMode(modeIndex)"
                  >
                    编辑
                  </button>
                  <button 
                    class="text-xs px-2 py-1 bg-red-100 text-red-600 rounded hover:bg-red-200"
                    @click="deleteMode(modeIndex)"
                  >
                    删除
                  </button>
                </div>
              </div>
              
              <!-- 模式说明 -->
              <div class="text-sm text-gray-600 mb-3">{{ mode.description }}</div>
              
              <!-- 模式配置预览 -->
              <div class="grid grid-cols-2 gap-3 text-xs">
                <div v-if="mode.config.minDraws" class="bg-white p-2 rounded">
                  <span class="text-gray-500">最少抽取:</span> {{ mode.config.minDraws }}次
                </div>
                <div v-if="mode.config.maxDraws" class="bg-white p-2 rounded">
                  <span class="text-gray-500">最多抽取:</span> {{ mode.config.maxDraws }}次
                </div>
                <div v-if="mode.config.specialPrizePosition !== undefined" class="bg-white p-2 rounded">
                  <span class="text-gray-500">特殊奖项位置:</span> {{ mode.config.specialPrizePosition === 'last' ? '最后' : '随机' }}
                </div>
                <div v-if="mode.config.drawOrder" class="bg-white p-2 rounded">
                  <span class="text-gray-500">抽取顺序:</span> {{ mode.config.drawOrder === 'sequential' ? '顺序' : '随机' }}
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 高级设置 -->
        <div class="mb-6">
          <h4 class="font-medium mb-3">高级设置</h4>
          
          <div class="flex items-center justify-between mb-4 pb-2 border-b border-gray-100">
            <div>
              <div class="font-medium">记录抽奖历史</div>
              <div class="text-sm text-gray-500">保存所有抽奖记录</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="lotterySettings.saveHistory">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div class="flex items-center justify-between mb-4 pb-2 border-b border-gray-100">
            <div>
              <div class="font-medium">显示概率</div>
              <div class="text-sm text-gray-500">在抽奖界面显示各奖项概率</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="lotterySettings.showProbability">
              <span class="slider round"></span>
            </label>
          </div>
          
          <div class="flex items-center justify-between">
            <div>
              <div class="font-medium">自动重置</div>
              <div class="text-sm text-gray-500">完成所有奖项后自动重置</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="lotterySettings.autoReset">
              <span class="slider round"></span>
            </label>
          </div>
        </div>
        
        <!-- 保存按钮 -->
        <div class="mt-8 flex justify-end">
          <button class="px-6 py-2 bg-gray-200 text-gray-800 rounded-md mr-2 hover:bg-gray-300" @click="resetLotterySettings">
            重置为默认
          </button>
          <button class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700" @click="saveLotterySettings">
            保存设置
          </button>
        </div>
      </div>
      
      <!-- 添加类型模态框 -->
      <div v-if="showAddCategoryModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg p-6 w-96 max-w-full">
          <h3 class="text-lg font-bold mb-4">添加抽奖类型</h3>
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-2">类型名称</label>
            <input type="text" v-model="newCategory.name" class="w-full px-3 py-2 border border-gray-300 rounded-md">
          </div>
          <div class="flex justify-end gap-2">
            <button class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300" @click="showAddCategoryModal = false">
              取消
            </button>
            <button class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700" @click="addCategory">
              添加
            </button>
          </div>
        </div>
      </div>
      
      <!-- 添加模式模态框 -->
      <div v-if="showAddModeModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg p-6 w-96 max-w-full">
          <h3 class="text-lg font-bold mb-4">{{ editingModeIndex === null ? '添加' : '编辑' }}抽奖模式</h3>
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-2">模式名称</label>
            <input type="text" v-model="newMode.name" class="w-full px-3 py-2 border border-gray-300 rounded-md mb-3">
            
            <label class="block text-sm font-medium text-gray-700 mb-2">模式描述</label>
            <textarea v-model="newMode.description" class="w-full px-3 py-2 border border-gray-300 rounded-md mb-3" rows="2"></textarea>
            
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">最少抽取次数</label>
                <input type="number" v-model="newMode.config.minDraws" min="1" class="w-full px-3 py-1 border border-gray-300 rounded-md">
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">最多抽取次数</label>
                <input type="number" v-model="newMode.config.maxDraws" min="1" class="w-full px-3 py-1 border border-gray-300 rounded-md">
              </div>
            </div>
            
            <div class="mb-3">
              <label class="block text-sm font-medium text-gray-700 mb-1">特殊奖项位置</label>
              <select v-model="newMode.config.specialPrizePosition" class="w-full px-3 py-2 border border-gray-300 rounded-md">
                <option value="last">最后出现</option>
                <option value="random">随机出现</option>
              </select>
            </div>
            
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">抽取顺序</label>
              <select v-model="newMode.config.drawOrder" class="w-full px-3 py-2 border border-gray-300 rounded-md">
                <option value="random">随机顺序</option>
                <option value="sequential">顺序抽取</option>
              </select>
            </div>
          </div>
          <div class="flex justify-end gap-2">
            <button class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300" @click="cancelModeEdit">
              取消
            </button>
            <button class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700" @click="saveMode">
              {{ editingModeIndex === null ? '添加' : '保存' }}
            </button>
          </div>
        </div>
      </div>
      
      <!-- 其他面板可以在这里添加，使用v-if="activeTab === x"等条件 -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';

const activeTab = ref(0);
const tabs = ['常规设置', '抽奖设置', '通知设置', '安全设置', '关于'];

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

// 抽奖设置
const selectedCategory = ref(0);
const selectedModeIndex = ref(0);
const showAddCategoryModal = ref(false);
const showAddModeModal = ref(false);
const editingModeIndex = ref(null);

// 抽奖类型列表
const lotteryCategories = ref([
  {
    name: '英语大转盘',
    modes: [
      {
        name: '标准模式',
        description: '随机抽取奖品，每个奖品最多出现两次，谢谢惠顾最后出现',
        config: {
          minDraws: 1,
          maxDraws: 2,
          specialPrizePosition: 'last',
          drawOrder: 'random'
        }
      },
      {
        name: '初级学习模式',
        description: '按顺序抽取所有奖品，适合初学者练习',
        config: {
          minDraws: 1,
          maxDraws: 1,
          specialPrizePosition: 'last',
          drawOrder: 'sequential'
        }
      },
      {
        name: '随机模式',
        description: '完全随机抽取，谢谢惠顾可能随时出现',
        config: {
          minDraws: 0,
          maxDraws: 3,
          specialPrizePosition: 'random',
          drawOrder: 'random'
        }
      }
    ]
  },
  {
    name: '数学九宫格',
    modes: [
      {
        name: '数字练习',
        description: '抽取数字进行练习，每个数字只出现一次',
        config: {
          minDraws: 1,
          maxDraws: 1,
          specialPrizePosition: 'random',
          drawOrder: 'sequential'
        }
      }
    ]
  }
]);

// 抽奖高级设置
const lotterySettings = reactive({
  saveHistory: true,
  showProbability: false,
  autoReset: true
});

// 新类型模板
const newCategory = reactive({
  name: '',
  modes: []
});

// 新模式模板
const newMode = reactive({
  name: '',
  description: '',
  config: {
    minDraws: 1,
    maxDraws: 2,
    specialPrizePosition: 'last',
    drawOrder: 'random'
  }
});

// 添加新类型
const addCategory = () => {
  if (newCategory.name.trim()) {
    lotteryCategories.value.push({
      name: newCategory.name,
      modes: []
    });
    selectedCategory.value = lotteryCategories.value.length - 1;
    newCategory.name = '';
    showAddCategoryModal.value = false;
  }
};

// 编辑模式
const editMode = (index) => {
  const mode = lotteryCategories.value[selectedCategory.value].modes[index];
  newMode.name = mode.name;
  newMode.description = mode.description;
  newMode.config = { ...mode.config };
  editingModeIndex.value = index;
  showAddModeModal.value = true;
};

// 删除模式
const deleteMode = (index) => {
  if (confirm('确定要删除这个模式吗？')) {
    lotteryCategories.value[selectedCategory.value].modes.splice(index, 1);
    if (selectedModeIndex.value >= lotteryCategories.value[selectedCategory.value].modes.length) {
      selectedModeIndex.value = lotteryCategories.value[selectedCategory.value].modes.length - 1;
    }
  }
};

// 取消模式编辑
const cancelModeEdit = () => {
  showAddModeModal.value = false;
  editingModeIndex.value = null;
  newMode.name = '';
  newMode.description = '';
  newMode.config = {
    minDraws: 1,
    maxDraws: 2,
    specialPrizePosition: 'last',
    drawOrder: 'random'
  };
};

// 保存模式
const saveMode = () => {
  if (!newMode.name.trim()) return;
  
  if (editingModeIndex.value !== null) {
    // 编辑现有模式
    lotteryCategories.value[selectedCategory.value].modes[editingModeIndex.value] = {
      name: newMode.name,
      description: newMode.description,
      config: { ...newMode.config }
    };
  } else {
    // 添加新模式
    lotteryCategories.value[selectedCategory.value].modes.push({
      name: newMode.name,
      description: newMode.description,
      config: { ...newMode.config }
    });
    selectedModeIndex.value = lotteryCategories.value[selectedCategory.value].modes.length - 1;
  }
  
  cancelModeEdit();
};

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

// 重置抽奖设置
const resetLotterySettings = () => {
  lotterySettings.saveHistory = true;
  lotterySettings.showProbability = false;
  lotterySettings.autoReset = true;
  
  // 重置为默认抽奖配置
  if (confirm('是否要重置所有抽奖类型和模式？')) {
    lotteryCategories.value = [
      {
        name: '英语大转盘',
        modes: [
          {
            name: '标准模式',
            description: '随机抽取奖品，每个奖品最多出现两次，谢谢惠顾最后出现',
            config: {
              minDraws: 1,
              maxDraws: 2,
              specialPrizePosition: 'last',
              drawOrder: 'random'
            }
          }
        ]
      }
    ];
    selectedCategory.value = 0;
    selectedModeIndex.value = 0;
  }
};

// 保存设置
const saveSettings = () => {
  // 在实际应用中，这里应该将设置保存到本地存储或通过API保存
  alert('设置已保存！');
};

// 保存抽奖设置
const saveLotterySettings = () => {
  // 在实际应用中，这里应该将抽奖设置保存到本地存储或通过API保存
  alert('抽奖设置已保存！');
};
</script>

<style scoped>
/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .4s;
}

input:checked + .slider {
  background-color: #3B82F6;
}

input:focus + .slider {
  box-shadow: 0 0 1px #3B82F6;
}

input:checked + .slider:before {
  transform: translateX(24px);
}

.slider.round {
  border-radius: 24px;
}

.slider.round:before {
  border-radius: 50%;
}
</style> 