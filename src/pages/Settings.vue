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
                    '抽完后仍可继续抽取魔法小礼袋' }}
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
            
            <!-- 单词配置模块 -->
            <div class="bg-emerald-50 rounded-xl p-5 border border-emerald-200 shadow-sm">
              <div class="flex justify-between items-center mb-3">
                <h4 class="text-lg font-medium text-emerald-700">转盘单词配置</h4>
                <div class="flex space-x-2">
                  <button 
                    @click="addNewWord" 
                    class="px-4 py-1.5 bg-emerald-500 text-white rounded-md hover:bg-emerald-600 transition-colors flex items-center text-sm"
                  >
                    <span class="mr-1">+</span> 添加单词
                  </button>
                  <button 
                    @click="triggerFileUpload" 
                    class="px-4 py-1.5 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors flex items-center text-sm"
                  >
                    <span class="mr-1">📤</span> 导入单词
                  </button>
                </div>
              </div>
              
              <!-- 文件上传组件 -->
              <input 
                type="file" 
                ref="fileInput" 
                @change="uploadFile" 
                accept=".json,.csv"
                class="hidden" 
              />
              
              <!-- 上传状态通知 -->
              <div 
                v-if="uploadStatus.show" 
                :class="[
                  'mb-4 p-3 rounded-md flex items-center',
                  uploadStatus.type === 'success' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'
                ]"
              >
                <span v-if="uploadStatus.type === 'success'" class="mr-2">✅</span>
                <span v-else class="mr-2">❌</span>
                <span>{{ uploadStatus.message }}</span>
                <button 
                  @click="uploadStatus.show = false" 
                  class="ml-auto text-gray-500 hover:text-gray-700"
                >
                  ×
                </button>
              </div>
              
              <!-- 单词列表 -->
              <div class="space-y-4 mt-4 max-h-96 overflow-y-auto pr-1">
                <div 
                  v-for="(word, index) in wordsList" 
                  :key="index"
                  class="bg-white rounded-lg border border-gray-200 p-4 transition-all hover:shadow-md"
                >
                  <div class="flex justify-between items-start mb-3">
                    <div class="font-bold text-lg text-gray-800">{{ word.english }}</div>
                    <div class="flex space-x-2">
                      <button 
                        @click="editWord(index)" 
                        class="text-blue-500 hover:text-blue-700"
                      >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path>
                        </svg>
                      </button>
                      <button 
                        @click="deleteWord(index)" 
                        class="text-red-500 hover:text-red-700"
                      >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                        </svg>
                      </button>
                    </div>
                  </div>
                  
                  <div class="flex space-x-4">
                    <div class="flex-shrink-0 w-20 h-20 bg-gray-100 rounded-md overflow-hidden">
                      <img 
                        :src="getDisplayImageUrl(word.imgSrc)" 
                        :alt="word.english" 
                        class="w-full h-full object-cover"
                        @error="handleImgError($event, index)"
                      />
                    </div>
                    <div class="flex-grow">
                      <div class="text-gray-500 mb-1">翻译: <span class="text-gray-700">{{ word.translation }}</span></div>
                      <div class="text-gray-500 mb-1">背景色: 
                        <span 
                          class="inline-block w-4 h-4 rounded-full border border-gray-300" 
                          :style="{ backgroundColor: word.bgColor }"
                        ></span>
                        <span class="text-gray-700 ml-1">{{ word.bgColor }}</span>
                      </div>
                      <div class="text-gray-500">字体颜色: 
                        <span 
                          class="inline-block w-4 h-4 rounded-full border border-gray-300" 
                          :style="{ backgroundColor: word.fontColor }"
                        ></span>
                        <span class="text-gray-700 ml-1">{{ word.fontColor }}</span>
                      </div>
                    </div>
                  </div>
                </div>
                
                <!-- 空状态提示 -->
                <div 
                  v-if="wordsList.length === 0" 
                  class="text-center py-8 bg-white rounded-lg border border-gray-200"
                >
                  <div class="text-gray-400 mb-2">暂无单词配置</div>
                  <div class="text-sm text-gray-500">点击上方"添加单词"按钮开始配置转盘单词</div>
                </div>
              </div>
            </div>
            
            <!-- 导出按钮 -->
            <div class="mt-6 flex justify-center">
              <button 
                @click="exportWords" 
                class="px-6 py-2 bg-indigo-500 text-white rounded-md hover:bg-indigo-600 transition-colors flex items-center text-sm shadow-sm"
                :disabled="wordsList.length === 0"
              >
                <span class="mr-2">📥</span> 导出单词列表
              </button>
            </div>
            
            <!-- 单词编辑弹窗 -->
            <div 
              v-if="showWordModal" 
              class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            >
              <div class="bg-white rounded-xl p-6 w-full max-w-md m-4">
                <h3 class="text-xl font-bold text-gray-800 mb-4">
                  {{ editingIndex === -1 ? '添加新单词' : '编辑单词' }}
                </h3>
                
                <div class="space-y-4">
                  <!-- 英语单词 -->
                  <div>
                    <label class="block text-gray-700 text-sm font-medium mb-1">英语单词</label>
                    <input 
                      v-model="editingWord.english" 
                      type="text" 
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
                      placeholder="请输入英语单词"
                    />
                  </div>
                  
                  <!-- 中文翻译 -->
                  <div>
                    <label class="block text-gray-700 text-sm font-medium mb-1">中文翻译</label>
                    <input 
                      v-model="editingWord.translation" 
                      type="text" 
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
                      placeholder="请输入中文翻译"
                    />
                  </div>
                  
                  <!-- 背景颜色 -->
                  <div>
                    <label class="block text-gray-700 text-sm font-medium mb-1">背景颜色</label>
                    <div class="flex items-center">
                      <input 
                        v-model="editingWord.bgColor" 
                        type="text" 
                        class="flex-grow px-3 py-2 border border-gray-300 rounded-l-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
                        placeholder="#RRGGBB 或颜色名称"
                      />
                      <input 
                        v-model="editingWord.bgColor" 
                        type="color" 
                        class="w-10 h-10 border-y border-r border-gray-300 rounded-r-md"
                      />
                    </div>
                  </div>
                  
                  <!-- 字体颜色 -->
                  <div>
                    <label class="block text-gray-700 text-sm font-medium mb-1">字体颜色</label>
                    <div class="flex items-center">
                      <input 
                        v-model="editingWord.fontColor" 
                        type="text" 
                        class="flex-grow px-3 py-2 border border-gray-300 rounded-l-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
                        placeholder="#RRGGBB 或颜色名称"
                      />
                      <input 
                        v-model="editingWord.fontColor" 
                        type="color" 
                        class="w-10 h-10 border-y border-r border-gray-300 rounded-r-md"
                      />
                    </div>
                  </div>
                  
                  <!-- 图片地址 -->
                  <div>
                    <label class="block text-gray-700 text-sm font-medium mb-1">图片设置</label>
                    
                    <!-- 图片上传区域 -->
                    <div 
                      class="w-full h-32 border-2 border-dashed border-gray-300 rounded-md mb-2 overflow-hidden flex items-center justify-center cursor-pointer hover:border-emerald-500 transition-all"
                      @click="triggerImageUpload"
                      @dragover.prevent 
                      @drop.prevent="onImageDrop"
                      @dragenter.prevent="isDragging = true"
                      @dragleave.prevent="isDragging = false"
                      :class="{'border-emerald-500 bg-emerald-50': isDragging}"
                    >
                      <template v-if="hasImageForPreview && !previewImgError">
                        <img 
                          :src="getDisplayImageUrlForPreview()" 
                          alt="预览" 
                          class="w-full h-full object-contain"
                          @error="previewImgError = true"
                        />
                      </template>
                      <div v-else class="text-center p-4">
                        <svg class="w-8 h-8 mx-auto text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                        </svg>
                        <p class="mt-1 text-sm text-gray-500">点击或拖放图片到这里上传</p>
                        <p class="text-xs text-gray-400">支持 JPG, PNG, GIF 格式</p>
                      </div>
                    </div>
                    
                    <!-- 隐藏的文件输入 -->
                    <input 
                      type="file"
                      ref="imageInput"
                      accept="image/*"
                      class="hidden"
                      @change="onImageSelected"
                    />
                    
                    <!-- 图片URL输入 -->
                    <div class="flex items-center">
                      <input 
                        v-model="editingWord.imgSrc" 
                        type="text" 
                        class="flex-grow px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-emerald-500"
                        placeholder="或输入图片URL地址"
                      />
                      <button 
                        type="button" 
                        class="ml-2 px-3 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200"
                        @click="resetImagePreview"
                      >
                        重置
                      </button>
                    </div>
                    <p class="text-xs text-gray-500 mt-1">支持外部链接或本地图片上传</p>
                  </div>
                </div>
                
                <div class="flex justify-end mt-6 space-x-3">
                  <button 
                    @click="closeWordModal" 
                    class="px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50"
                  >
                    取消
                  </button>
                  <button 
                    @click="saveWord" 
                    class="px-4 py-2 bg-emerald-500 text-white rounded-md hover:bg-emerald-600"
                    :disabled="!isWordFormValid"
                  >
                    保存
                  </button>
                </div>
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
import { ref, reactive, markRaw, h, computed, onMounted } from 'vue';
import { useWheelSettings, DrawMode, WordConfig } from '../utils/wheelSettings';
import * as fs from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import { appLocalDataDir } from '@tauri-apps/api/path';
// 获取转盘设置
const { 
  settings, 
  updateDrawMode, 
  updateLockAfterComplete,
  updateMaxDraws,
  updatePrizeWords
} = useWheelSettings();

// 文件输入引用
const fileInput = ref<HTMLInputElement | null>(null);

// 上传状态通知
const uploadStatus = reactive({
  show: false,
  type: 'success',
  message: ''
});

// 触发文件选择对话框
const triggerFileUpload = () => {
  fileInput.value?.click();
};

// 文件上传
const uploadFile = async (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  try {
    // 显示上传状态
    uploadStatus.show = true;
    uploadStatus.type = 'success';
    uploadStatus.message = '正在处理文件...';

    // 根据文件类型处理
    const fileExtension = file.name.split('.').pop()?.toLowerCase();
    
    // 读取文件内容
    const fileContent = await file.text();
    
    if (fileExtension === 'json') {
      // 解析JSON文件
      try {
        const jsonData = JSON.parse(fileContent);
        
        // 检查数据是否是数组且包含必要的字段
        if (Array.isArray(jsonData) && jsonData.length > 0) {
          // 验证数据结构
          const validWords = jsonData.filter(item => 
            item.english && 
            item.translation && 
            (item.bgColor || item.bgColor === '') && 
            (item.fontColor || item.fontColor === '') &&
            (item.imgSrc || item.imgSrc === '')
          );
          
          if (validWords.length > 0) {
            // 更新单词列表
            wordsList.value = validWords;
            saveWordsToSettings();
            
            uploadStatus.message = `成功导入 ${validWords.length} 个单词`;
            
            // 如果有无效数据
            if (validWords.length < jsonData.length) {
              uploadStatus.message += `，${jsonData.length - validWords.length} 个无效数据被忽略`;
            }
          } else {
            throw new Error('文件中没有有效的单词数据');
          }
        } else {
          throw new Error('JSON格式不正确，应为单词对象数组');
        }
      } catch (error) {
        uploadStatus.type = 'error';
        uploadStatus.message = `JSON解析错误: ${error instanceof Error ? error.message : '未知错误'}`;
      }
    } else if (fileExtension === 'csv') {
      // 解析CSV文件
      try {
        // 简单的CSV解析（假设第一行是表头）
        const lines = fileContent.split('\n');
        if (lines.length < 2) {
          throw new Error('CSV文件格式不正确，至少需要表头和一行数据');
        }
        
        const headers = lines[0].split(',').map(h => h.trim());
        
        // 检查必要的列是否存在
        const requiredColumns = ['english', 'translation', 'bgColor', 'fontColor', 'imgSrc'];
        const headerMap: Record<string, number> = {};
        
        requiredColumns.forEach(col => {
          const index = headers.indexOf(col);
          if (index !== -1) {
            headerMap[col] = index;
          }
        });
        
        if (!('english' in headerMap) || !('translation' in headerMap)) {
          throw new Error('CSV必须包含english和translation列');
        }
        
        // 处理数据行
        const newWords: WordConfig[] = [];
        
        for (let i = 1; i < lines.length; i++) {
          const line = lines[i].trim();
          if (!line) continue;
          
          const columns = line.split(',').map(col => col.trim());
          if (columns.length < Object.keys(headerMap).length) continue;
          
          const word: WordConfig = {
            english: headerMap['english'] !== undefined ? columns[headerMap['english']] : '',
            translation: headerMap['translation'] !== undefined ? columns[headerMap['translation']] : '',
            bgColor: headerMap['bgColor'] !== undefined ? columns[headerMap['bgColor']] : '#badc58',
            fontColor: headerMap['fontColor'] !== undefined ? columns[headerMap['fontColor']] : '#2d3436',
            imgSrc: headerMap['imgSrc'] !== undefined ? columns[headerMap['imgSrc']] : ''
          };
          
          if (word.english && word.translation) {
            newWords.push(word);
          }
        }
        
        if (newWords.length > 0) {
          wordsList.value = newWords;
          saveWordsToSettings();
          uploadStatus.message = `成功导入 ${newWords.length} 个单词`;
        } else {
          throw new Error('CSV文件中没有有效的单词数据');
        }
      } catch (error) {
        uploadStatus.type = 'error';
        uploadStatus.message = `CSV解析错误: ${error instanceof Error ? error.message : '未知错误'}`;
      }
    } else {
      uploadStatus.type = 'error';
      uploadStatus.message = '不支持的文件格式，请上传JSON或CSV文件';
    }
  } catch (error) {
    uploadStatus.type = 'error';
    uploadStatus.message = `文件处理错误: ${error instanceof Error ? error.message : '未知错误'}`;
  }
  
  // 重置文件输入，以便可以再次选择同一文件
  input.value = '';
  
  // 5秒后自动隐藏状态通知
  setTimeout(() => {
    uploadStatus.show = false;
  }, 5000);
};

// 导出单词列表为JSON文件
const exportWords = async () => {
  try {
    if (wordsList.value.length === 0) {
      alert('没有单词可导出');
      return;
    }
    
    const jsonData = JSON.stringify(wordsList.value, null, 2);
    
    // 使用Tauri的对话框API保存文件
    // 这里仅模拟
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const fileName = `words_export_${timestamp}.json`;
    
    // 将字符串转换为Uint8Array
    const encoder = new TextEncoder();
    const data = encoder.encode(jsonData);
    
    await fs.writeFile(fileName, data, { baseDir: fs.BaseDirectory.AppData });
    
    uploadStatus.show = true;
    uploadStatus.type = 'success';
    uploadStatus.message = `单词列表已导出到 ${fileName}`;
    
    setTimeout(() => {
      uploadStatus.show = false;
    }, 5000);
  } catch (error) {
    uploadStatus.show = true;
    uploadStatus.type = 'error';
    uploadStatus.message = `导出失败: ${error instanceof Error ? error.message : '未知错误'}`;
    
    setTimeout(() => {
      uploadStatus.show = false;
    }, 5000);
  }
};

// 当前编辑的单词
const editingWord = reactive<WordConfig>({
  english: '',
  translation: '',
  bgColor: '#badc58',
  fontColor: '#2d3436',
  imgSrc: ''
});

// 图片上传相关
const imageInput = ref<HTMLInputElement | null>(null);
const isDragging = ref(false);

// 单词列表
const wordsList = ref<WordConfig[]>([]);

// 编辑状态
const showWordModal = ref(false);
const editingIndex = ref(-1);
const previewImgError = ref(false);

// 验证表单是否有效
const isWordFormValid = computed(() => {
  // 英文和翻译必须填写
  const hasBasicInfo = editingWord.english.trim() !== '' && 
                       editingWord.translation.trim() !== '';
  
  // 检查是否有图片路径（公开的或内部的）
  let hasImagePath = editingWord.imgSrc.trim() !== '';
  
  // 检查内部存储的图片路径
  // @ts-ignore - 动态添加的属性
  if (editingWord._imgSrcInternal) {
    hasImagePath = true;
  }
  
  return hasBasicInfo && hasImagePath;
});

// 检查是否有图片可以预览
const hasImageForPreview = computed(() => {
  return !!editingWord.imgSrc || 
         // @ts-ignore - 动态添加的属性
         !!editingWord._imgSrcInternal;
});

// 获取预览图片的URL
function getDisplayImageUrlForPreview(): string {
  // @ts-ignore - 动态添加的属性
  const internalPath = editingWord._imgSrcInternal;
  
  // 如果有内部路径，优先使用
  if (internalPath) {
    return getDisplayImageUrl(internalPath);
  }
  
  // 否则使用普通路径
  return getDisplayImageUrl(editingWord.imgSrc);
}

// 初始化单词列表
function initWordsList() {
  // 从settings中获取已配置的单词
  if (settings.prizeWords && settings.prizeWords.length > 0) {
    wordsList.value = [...settings.prizeWords];
  } else {
    // 使用默认值
    wordsList.value = [
      {
        english: 'Apple',
        translation: '苹果',
        bgColor: '#badc58',
        fontColor: '#2d3436',
        imgSrc: './ct-converted.png'
      },
      {
        english: 'Cat',
        translation: '猫咪',
        bgColor: '#ff9ff3',
        fontColor: '#2d3436',
        imgSrc: './ct-converted.png'
      },
      {
        english: 'Ball',
        translation: '球',
        bgColor: '#ffeaa7',
        fontColor: '#2d3436',
        imgSrc: './ct-converted.png'
      },
      {
        english: 'Dog',
        translation: '小狗',
        bgColor: '#74b9ff',
        fontColor: '#2d3436',
        imgSrc: './ct-converted.png'
      }
    ];
  }
}

// 添加新单词
function addNewWord() {
  // 重置编辑状态
  editingWord.english = '';
  editingWord.translation = '';
  editingWord.bgColor = '#badc58';
  editingWord.fontColor = '#2d3436';
  editingWord.imgSrc = '';
  editingIndex.value = -1;
  previewImgError.value = false;
  showWordModal.value = true;
}

// 编辑单词
function editWord(index: number) {
  const word = wordsList.value[index];
  editingWord.english = word.english;
  editingWord.translation = word.translation;
  editingWord.bgColor = word.bgColor;
  editingWord.fontColor = word.fontColor;
  
  // 检查图片路径是否以app://开头
  if (word.imgSrc && word.imgSrc.startsWith('app://')) {
    // 将实际路径存储在内部，但不显示在输入框中
    // @ts-ignore - 动态添加的属性
    editingWord._imgSrcInternal = word.imgSrc;
    editingWord.imgSrc = ''; // 清空输入框显示
  } else {
    // 常规URL或其他格式路径，直接显示
    editingWord.imgSrc = word.imgSrc;
    // 确保没有残留的内部路径
    // @ts-ignore - 动态添加的属性
    editingWord._imgSrcInternal = undefined;
  }
  
  editingIndex.value = index;
  previewImgError.value = false;
  showWordModal.value = true;
}

// 删除单词
function deleteWord(index: number) {
  if (confirm('确定要删除这个单词吗？')) {
    wordsList.value.splice(index, 1);
    saveWordsToSettings();
  }
}

// 保存单词
function saveWord() {
  if (!isWordFormValid.value) return;
  
  // 获取实际的图片路径
  let actualImgSrc = editingWord.imgSrc.trim();
  
  // 如果有内部存储的路径，优先使用它
  // @ts-ignore - 动态添加的属性
  if (editingWord._imgSrcInternal) {
    // @ts-ignore - 动态添加的属性
    actualImgSrc = editingWord._imgSrcInternal;
  }
  
  const newWord: WordConfig = {
    english: editingWord.english.trim(),
    translation: editingWord.translation.trim(),
    bgColor: editingWord.bgColor,
    fontColor: editingWord.fontColor,
    imgSrc: actualImgSrc
  };
  
  if (editingIndex.value === -1) {
    // 添加新单词
    wordsList.value.push(newWord);
  } else {
    // 更新现有单词
    wordsList.value[editingIndex.value] = newWord;
  }
  
  // 保存到全局设置
  saveWordsToSettings();
  
  // 关闭弹窗
  closeWordModal();
}

// 关闭编辑弹窗
function closeWordModal() {
  showWordModal.value = false;
}

// 获取显示用的图片URL
function getDisplayImageUrl(storedPath: string): string {
  // 如果已经在缓存中，直接返回
  if (imageUrlCache[storedPath]) {
    return imageUrlCache[storedPath];
  }
  
  // 如果是app://开头的路径，但尚未缓存，触发异步加载
  if (storedPath.startsWith('app://')) {
    // 返回占位图片，handleImgError会在图片加载失败时处理
    return './ct-converted.png';
  }
  
  // 正常返回路径
  return storedPath;
}

// 处理图片加载错误
async function handleImgError(event: Event, index: number) {
  const target = event.target as HTMLImageElement;
  
  try {
    // 获取存储的路径
    const storedPath = wordsList.value[index].imgSrc;
    console.log("处理图片加载错误:", storedPath);
    
    // 如果是app://开头但尚未处理过或处理出错
    if (storedPath.startsWith('app://')) {
      // 检查是否已缓存
      if (imageUrlCache[storedPath]) {
        console.log("使用缓存的URL:", imageUrlCache[storedPath]);
        target.src = imageUrlCache[storedPath];
        return;
      }
      
      // 重新尝试获取正确的URL
      try {
        const url = await getImageUrl(storedPath);
        console.log("重新获取的图片URL:", url);
        imageUrlCache[storedPath] = url;
        target.src = url;
        return;
      } catch (urlError) {
        console.error("获取图片URL失败:", urlError);
      }
    }
  } catch (error) {
    console.error('处理图片URL错误:', error);
  }
  
  // 如果上述处理失败，使用默认图片
  console.log("使用默认图片");
  target.src = './ct-converted.png';
}

// 保存单词到settings
function saveWordsToSettings() {
  updatePrizeWords(wordsList.value);
}

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

// 设置分类列表
const settingSections = [
  // { id: 'appearance', name: '系统外观', icon: AppearanceIcon },
  // { id: 'ai', name: 'AI助手设置', icon: AiIcon },
  { id: 'wheel', name: '英语转盘设置', icon: WheelIcon },
  // { id: 'about', name: '关于系统', icon: AboutIcon }
];

// 当前激活的设置分类
const activeSection = ref('wheel');

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
    description: '每个奖品都要抽一次，抽完后只能抽到魔法小礼袋'
  },
  { 
    name: '随机模式', 
    value: DrawMode.RANDOM, 
    icon: '🎲',
    description: '奖品和魔法小礼袋完全随机，抽到哪个是哪个'
  }
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

// 触发图片上传对话框
function triggerImageUpload() {
  imageInput.value?.click();
}

// 图片选择处理
function onImageSelected(event: Event) {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const file = input.files[0];
    if (isImageFile(file)) {
      handleImageFile(file);
    } else {
      alert('请选择有效的图片文件 (JPG, PNG, GIF)');
    }
    // 重置input，以便能够选择同一个文件
    input.value = '';
  }
}

// 图片拖放处理
function onImageDrop(event: DragEvent) {
  isDragging.value = false;
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    const file = event.dataTransfer.files[0];
    if (isImageFile(file)) {
      handleImageFile(file);
    } else {
      alert('请选择有效的图片文件 (JPG, PNG, GIF)');
    }
  }
}

// 检查是否是图片文件
function isImageFile(file: File): boolean {
  // 通过MIME类型检查
  const validTypes = ['image/jpeg', 'image/png', 'image/gif'];
  if (!validTypes.includes(file.type)) return false;
  
  // 通过文件扩展名检查
  const validExtensions = ['.jpg', '.jpeg', '.png', '.gif'];
  const fileName = file.name.toLowerCase();
  return validExtensions.some(ext => fileName.endsWith(ext));
}

// 处理图片文件
async function handleImageFile(file: File) {
  try {
    // 检查文件大小限制 (5MB)
    const maxSize = 5 * 1024 * 1024; // 5MB
    if (file.size > maxSize) {
      alert(`图片大小不能超过5MB，当前大小：${(file.size / (1024 * 1024)).toFixed(2)}MB`);
      return;
    }

    // 添加加载中的提示
    const loadingMsg = document.createElement('div');
    loadingMsg.textContent = '图片处理中...';
    loadingMsg.style.position = 'absolute';
    loadingMsg.style.padding = '5px 10px';
    loadingMsg.style.background = 'rgba(0,0,0,0.7)';
    loadingMsg.style.color = 'white';
    loadingMsg.style.borderRadius = '4px';
    loadingMsg.style.zIndex = '1000';
    document.body.appendChild(loadingMsg);
    
    try {
      // 创建唯一文件名
      const timestamp = new Date().getTime();
      const ext = file.name.split('.').pop() || 'jpg';
      const fileName = `word_image_${timestamp}.${ext}`;
      
      // 读取文件为base64（仅用于传输）
      const fileDataUrl = await readFileAsDataURL(file);
      
      // 调用后端保存图片
      await invoke('ensure_images_dir'); // 确保目录存在
      
      // 可选：如果图片较大，进行压缩
      let fileData = fileDataUrl;
      if (file.size > 1024 * 1024) { // 如果大于1MB
        fileData = await compressImage(fileDataUrl, 800, 800, 0.8);
      }
      
      // 调用Rust函数保存文件（使用try-catch包裹以防止未处理的异常）
      try {
        await invoke('save_image', { 
          fileData: fileData,
          fileName: fileName
        });
      } catch (saveError) {
        console.error('保存图片失败:', saveError);
        alert(`保存图片失败: ${saveError instanceof Error ? saveError.message : '未知错误'}`);
        return;
      }
      
      // 设置图片相对路径（保存在数据中），但不显示在输入框中
      const relativePath = `app://image/${fileName}`;
      
      // 创建预览URL（使用convertFileSrc函数正确处理路径）
      try {
        const appDataDir = await appLocalDataDir();
        const fullImagePath = `${appDataDir}/images/${fileName}`;
        const previewUrl = convertFileSrc(fullImagePath);
        
        // 创建图片元素预加载图片，确保显示正常
        const preloadImg = new Image();
        preloadImg.src = previewUrl;
        
        // 添加到缓存，避免后续重复转换
        imageUrlCache[relativePath] = previewUrl;
        
        // 保存内部存储的路径
        // @ts-ignore - 动态添加的属性
        editingWord._imgSrcInternal = relativePath;
        
        // 清空输入框（不显示技术路径）
        editingWord.imgSrc = '';
        
        preloadImg.onload = () => {
          console.log("图片预加载成功:", previewUrl);
        };
        
        preloadImg.onerror = (err) => {
          console.error("图片预加载失败:", previewUrl, err);
          previewImgError.value = true;
        };
      } catch (urlError) {
        console.error('创建预览URL失败:', urlError);
        previewImgError.value = true;
      }
      
    } finally {
      // 移除加载提示
      document.body.removeChild(loadingMsg);
    }
  } catch (error) {
    console.error('图片处理失败:', error);
    alert(`图片处理失败: ${error instanceof Error ? error.message : '未知错误'}`);
  }
}

// 读取文件为DataURL
function readFileAsDataURL(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error('文件读取失败'));
    reader.readAsDataURL(file);
  });
}

// 压缩图片函数
function compressImage(base64: string, maxWidth: number, maxHeight: number, quality: number): Promise<string> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.src = base64;
    img.onload = () => {
      const canvas = document.createElement('canvas');
      let width = img.width;
      let height = img.height;
      
      // 计算缩放比例
      if (width > maxWidth || height > maxHeight) {
        const ratio = Math.min(maxWidth / width, maxHeight / height);
        width = width * ratio;
        height = height * ratio;
      }
      
      canvas.width = width;
      canvas.height = height;
      const ctx = canvas.getContext('2d');
      
      if (!ctx) {
        reject(new Error('无法创建Canvas上下文'));
        return;
      }
      
      ctx.drawImage(img, 0, 0, width, height);
      const compressedBase64 = canvas.toDataURL('image/jpeg', quality);
      resolve(compressedBase64);
    };
    img.onerror = () => reject(new Error('图片加载失败'));
  });
}

// 转换存储的图片路径为可显示的URL
async function getImageUrl(storedPath: string): Promise<string> {
  try {
    // 检查是否是app://格式的路径
    if (storedPath.startsWith('app://')) {
      // 提取文件名部分
      const fileName = storedPath.replace('app://image/', '');
      
      try {
        // 使用appLocalDataDir获取应用数据目录
        const appDataDir = await appLocalDataDir();
        // 构建图片文件的完整路径
        const fullPath = `${appDataDir}/images/${fileName}`;
        // 将本地文件路径转换为有效的URL
        return convertFileSrc(fullPath);
      } catch (pathError) {
        console.error('无法获取应用数据目录:', pathError);
        return './ct-converted.png'; // 返回默认图片
      }
    }
    
    // 如果是base64图片，直接返回
    if (storedPath.startsWith('data:image/')) {
      return storedPath;
    }
    
    // 如果是相对路径，尝试转换
    if (!storedPath.includes('://')) {
      return convertFileSrc(storedPath);
    }
    
    // 其他情况直接返回
    return storedPath;
  } catch (error) {
    console.error('图片URL转换失败:', error);
    return './ct-converted.png'; // 出错时返回默认图片
  }
}

// 重置图片预览
function resetImagePreview() {
  editingWord.imgSrc = '';
  // 清除内部存储的路径
  // @ts-ignore - 动态添加的属性
  editingWord._imgSrcInternal = undefined;
  previewImgError.value = false;
}

// 图片URL缓存
const imageUrlCache = reactive<Record<string, string>>({});

// 组件初始化
initWordsList();

// 确保images目录存在
onMounted(async () => {
  try {
    console.log("组件初始化中...");
    // 确保images目录存在
    await invoke('ensure_images_dir');
    console.log("images目录已确保存在");
    
    // 预处理单词列表中的图片，并缓存URL
    const processPromises = [];
    for (const word of wordsList.value) {
      if (word.imgSrc && word.imgSrc.startsWith('app://')) {
        processPromises.push(
          (async () => {
            try {
              console.log("处理图片路径:", word.imgSrc);
              // 转换URL并缓存
              const url = await getImageUrl(word.imgSrc);
              console.log("图片URL已转换:", url);
              imageUrlCache[word.imgSrc] = url;
            } catch (err) {
              console.error('预处理图片URL失败:', word.imgSrc, err);
            }
          })()
        );
      }
    }
    
    // 等待所有图片处理完成
    if (processPromises.length > 0) {
      await Promise.allSettled(processPromises);
      console.log("所有图片路径处理完成");
    }
  } catch (error) {
    console.error('初始化图片目录失败:', error);
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
</style>