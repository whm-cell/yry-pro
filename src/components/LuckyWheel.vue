<template>
  <div class="lucky-container" :class="{'large-scale': settings.wheelScale > 2}">
    <!-- 转盘部分 -->
    <div 
      class="wheel-wrapper" 
      :style="{ transform: `scale(${settings.wheelScale || 1})` }"
    >
      <LuckyWheel
        ref="myLucky"
        width="600px"
        height="600px"
        :prizes="prizes"
        :blocks="blocks"
        :buttons="buttons"
        @start="startCallback"
        @end="endCallback"
        @rotating="rotatingCallback"
      >
        <!-- 添加高亮边框覆盖层 -->
        <template #item="{index, transform, background, imgs, fonts}">
          <div
            class="prize-item"
            :style="{background, transform}"
            :class="{'highlight': settings.enableHighlight && highlightIndex === index}"
          >
            <div
              class="wheel-item-border"
              :class="{'active': settings.enableHighlight && highlightIndex === index}"
            ></div>
            <!-- 图片内容 -->
            <div v-for="(img, i) in imgs" :key="i">
              <img
                :src="img.src"
                :style="{
                  width: img.width,
                  top: img.top
                }"
              />
            </div>
            <!-- 文字内容 -->
            <div 
              v-for="(font, i) in fonts" 
              :key="i"
              :style="{
                color: font.fontColor,
                fontSize: font.fontSize,
                lineHeight: font.lineHeight || font.fontSize,
                top: font.top,
                fontWeight: font.fontWeight
              }"
            >
              {{font.text}}
            </div>
          </div>
        </template>
      </LuckyWheel>
    </div>
    
    <!-- 图片展示区域 -->
    <div 
      class="image-display" 
      :class="{ 'active': showImageDisplay, 'sliding': isSliding }"
      @click.self="toggleImageSize"
    >
      <div 
        v-if="selectedPrize"
        class="prize-image" 
        :class="{ 'enlarged': isEnlarged, 'sliding': isSliding, 'scale-70': scaleLevel === 1, 'scale-40': scaleLevel === 2 }"
        @click="toggleImageSize"
      >
        <!-- 添加自动关闭计时器指示，只在Magic Bag时显示 -->
        <div class="auto-close-indicator" v-if="isEnlarged && !isSliding && selectedPrize && selectedPrize.name === 'Magic Bag'">
          <svg viewBox="0 0 36 36" class="circular-timer">
            <path class="circle-bg"
              d="M18 2.0845
                a 15.9155 15.9155 0 0 1 0 31.831
                a 15.9155 15.9155 0 0 1 0 -31.831"
            />
            <path class="circle"
              :style="{ 'stroke-dasharray': `${autoCloseProgress}, 100` }"
              d="M18 2.0845
                a 15.9155 15.9155 0 0 1 0 31.831
                a 15.9155 15.9155 0 0 1 0 -31.831"
            />
            <text x="18" y="20.35" class="timer-text">{{ Math.ceil(autoCloseSecondsLeft) }}</text>
          </svg>
        </div>
       
        <!-- 添加点击次数提示 -->
        <div class="click-indicator" v-if="isEnlarged && !isSliding && clickCount > 0">
          再点击 {{ 3 - clickCount }} 次关闭
        </div>
        
        <div class="prize-content">
          <img :src="selectedPrize.imgSrc" :alt="selectedPrize.name">
        </div>
      </div>
    </div>
    
    <!-- 右下角重置按钮 -->
    <div class="reset-button" @click="resetRecords">
      <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12a9 9 0 0 1-9 9c-2.39 0-4.68-.94-6.4-2.6"></path>
        <path d="M9 6c0 .32.04.64.09.95"></path>
        <path d="M14.04 10H20v6"></path>
        <path d="M3 12a9 9 0 0 1 9-9c2.39 0 4.68.94 6.4 2.6"></path>
        <path d="M15 18c0-.32-.04-.64-.09-.95"></path>
        <path d="M9.96 14H4V8"></path>
      </svg>
      <span>重置转盘</span>
    </div>
    
    <!-- 完成抽奖提示 -->
    <div class="completion-tip" v-if="isCompleted && lockAfterComplete">
      <div class="completion-message">
        <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
        <span>单词转盘已完成，点击重置按钮可重新开始</span>
      </div>
    </div>
    
    <!-- 添加抽奖记录展示 -->
    <div class="prize-records" :style="{ display: 'block', opacity: 1 }">
      <h3>单词转盘记录</h3>
      <div class="records-list">
        <div v-for="(count, name) in prizeRecords" :key="name" class="record-item">
          <span>{{ name }}:</span>
          <span>{{ count }}次</span>
        </div>
      </div>
      <!-- 添加调试信息 -->
      <div class="debug-info">
        <p>记录数: {{ Object.keys(prizeRecords).length }}</p>
        <button @click="forceUpdateRecords">刷新记录</button>
      </div>
    </div>
    
    <!-- 增加悬浮提示 -->
    <div class="tooltip" :class="{ 'active': showTooltip }">
      {{ tooltipText }}
    </div>
    
    <!-- 增加隐藏的音频元素，添加id以便更容易获取 -->
    <audio ref="spinAudioEl" id="spinAudioEl" style="display:none"></audio>
    <audio ref="winAudioEl" id="winAudioEl" style="display:none"></audio>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount,  watch } from 'vue';
// @ts-ignore
import { invoke } from '@tauri-apps/api/core';
// @ts-ignore
import { convertFileSrc } from '@tauri-apps/api/core';

// 直接导入图片
import applePng from './ct-converted.png'  // 使用@别名指向src目录
import catPng from './ct-converted.png'
import ballPng from './ct-converted.png'
import dogPng from './ct-converted.png'
import starPng from './ct-converted.png'

// 导入设置钩子和类型
import { useWheelSettings } from '../utils/wheelSettings';

// 奖品信息类型
interface PrizeInfo {
  name: string;
  imgSrc: string;
  translation?: string; // 添加翻译字段
  phonetic?: string;    // 添加音标字段
  example?: string;     // 添加例句字段
}

// 文字设置接口
interface FontSetting {
  text: string;
  top: string;
  fontColor: string;
  fontSize: string;
  fontWeight?: string;
}

// 图片设置接口
interface ImageSetting {
  src: string;
  width: string;
  top: string;
}

// 奖品接口
interface Prize {
  background: string;
  fonts: FontSetting[];
  imgs: ImageSetting[];
  prizeInfo: PrizeInfo;
}

// 获取全局设置
const { 
  settings, 
  isInitialized 
} = useWheelSettings();

// 转盘ref
const myLucky = ref(null);

// 选中的奖品
const selectedPrize = ref<PrizeInfo | null>(null);
const isEnlarged = ref(false);
const showImageDisplay = ref(false);
const isSliding = ref(false);
const isTransitioning = ref(false); // 添加过渡状态锁，防止在过渡期间触发其他操作
let autoSlideTimer: number | null = null; // 添加自动滑动计时器
let autoCloseInterval: number | null = null; // 添加进度条更新计时器

// 新增：用于高亮边框的状态
const highlightIndex = ref<number | null>(null);
const isRotating = ref(false);

// 自动关闭倒计时
const autoCloseSecondsLeft = ref(5);
const autoCloseProgress = computed(() => (autoCloseSecondsLeft.value / 5) * 100);

// 添加点击计数器和缩放级别
const clickCount = ref(0);
const scaleLevel = ref(0);

// 定义默认奖品数据
const defaultPrizes: Prize[] = [
  { 
    background: '#badc58', 
    fonts: [
      { text: 'Apple', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
    ],
    imgs: [{ src: applePng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "Apple / 苹果",
      imgSrc: applePng
    }
  },
  { 
    background: '#ff9ff3', 
    fonts: [
      { text: 'Cat', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
    ],
    imgs: [{ src: catPng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "Cat / 猫咪",
      imgSrc: catPng
    }
  },
  { 
    background: '#ffeaa7', 
    fonts: [
      { text: 'Ball', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
    ],
    imgs: [{ src: ballPng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "Ball / 球",
      imgSrc: ballPng
    }
  },
  { 
    background: '#74b9ff', 
    fonts: [
      { text: 'Dog', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
    ],
    imgs: [{ src: dogPng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "Dog / 小狗",
      imgSrc: dogPng
    }
  },
  { 
    background: '#fab1a0', 
    fonts: [
      { text: 'Magic Bag', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' }
    ],
    imgs: [{ src: starPng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "Magic Bag",
      imgSrc: starPng
    }
  }
];

// 奖品数据
const prizes = ref<Prize[]>([...defaultPrizes]);

// 锁定后属性（从设置中获取）
const lockAfterComplete = computed(() => settings.lockAfterComplete);

// 提示工具
const showTooltip = ref(false);
const tooltipText = ref('');
let tooltipTimer: number | null = null;

// 转盘设计元素
const blocks = [
  { padding: '15px', background: 'linear-gradient(to right, #ff7979, #ffbe76)' },
  { padding: '2px', background: '#ffffff' }
];
// 中心按钮
const buttons = [{
  radius: '25%',
  background: '#ff7675',
  pointer: true,
  fonts: [
    { 
      text: '转一转', 
      fontColor: '#fff',
      fontSize: '18px',
      fontWeight: 'bold'
    }
  ],
}];

// 抽奖记录
const prizeRecordsRaw = ref<Record<string, number>>({});
// 标记是否所有奖品都至少抽中一次
const allPrizesDrawnOnce = ref(false);
// 标记是否已完成抽奖
const isCompletedFlag = ref(false);

// 检查是否是魔法礼袋词条
const isMagicBagItem = (item: any): boolean => {
  // 检查单词是否为"☆"或者翻译中包含"礼袋"
  return item.word === "☆" || item.translation.includes("礼袋");
};

// 修改颜色生成逻辑，确保颜色不重复
// 创建一个颜色池
const colorPool = [
  '#badc58', '#ff9ff3', '#ffeaa7', '#74b9ff', 
  '#a29bfe', '#55efc4', '#fab1a0', '#81ecec',
  '#e17055', '#0984e3', '#6c5ce7', '#00b894',
  '#fd79a8', '#fdcb6e', '#e84393', '#00cec9',
  '#2d3436', '#636e72', '#b2bec3', '#dfe6e9'
];

// 跟踪已使用的颜色索引
let usedColorIndices: number[] = [];

// 获取不重复的颜色
const getRandomColor = () => {
  // 如果所有颜色都已使用，重置已使用颜色数组
  if (usedColorIndices.length >= colorPool.length) {
    usedColorIndices = [];
  }
  
  // 找到一个未使用的颜色索引
  let colorIndex;
  do {
    colorIndex = Math.floor(Math.random() * colorPool.length);
  } while (usedColorIndices.includes(colorIndex));
  
  // 记录已使用的颜色索引
  usedColorIndices.push(colorIndex);
  
  return colorPool[colorIndex];
};

// 加载单词数据
const loadVocabularyFromDatabase = async () => {
  try {
    // 先尝试加载活动单词
    const activeWords: any[] = await invoke('get_active_words');
    console.log('活动单词原始数据:', activeWords);
    
    if (activeWords && Array.isArray(activeWords) && activeWords.length > 0) {
      console.log('加载活动单词成功:', activeWords.length);
      
      // 重置已使用颜色索引
      usedColorIndices = [];
      
      // 将活动单词转换为奖品格式
      const activePrizes: Prize[] = activeWords.map((item: any) => {
        // 使用convertFileSrc处理图片路径
        const imgSrc = convertFileSrc(item.image_path);
        console.log(`处理图片路径: ${item.image_path} -> ${imgSrc}`);
        
        // 判断是否是魔法礼袋
        const isMagicBag = isMagicBagItem(item);
        
        // 使用数据库中的自定义颜色或默认颜色
        const itemColor = item.color || null;
        console.log(`单词颜色: ${item.word} -> ${itemColor || '未设置，使用随机颜色'}`);
        
        return {
          background: isMagicBag ? '#fab1a0' : (itemColor || getRandomColor()),
          fonts: [
            { 
              text: isMagicBag ? 'Magic Bag' : "", 
              top: isMagicBag?'80%':'55%', 
              fontColor: '#2d3436', 
              fontSize: '16px', 
              fontWeight: 'bold' 
            },
          ],
          imgs: [{ src: imgSrc, width: '100px', top: '10%' }],
          prizeInfo: {
            name: isMagicBag ? "Magic Bag" : `${item.word} / ${item.translation}`,
            imgSrc: imgSrc,
            translation: item.translation,
            phonetic: item.phonetic,
            example: item.example
          }
        };
      });
      
      console.log('处理后的转盘奖品数据:', activePrizes);
      
      // 设置奖品列表（引用替换而不是修改属性）
      prizes.value = activePrizes;
      
      return; // 如果找到活动单词，直接返回，不加载所有单词
    } else {
      console.log('没有找到活动单词，使用所有单词');
    }
    
    // 如果没有活动单词，则加载所有单词
    const vocabularyData: any[] = await invoke('get_all_vocabulary');
    console.log('所有单词原始数据:', vocabularyData);
    
    if (vocabularyData && Array.isArray(vocabularyData) && vocabularyData.length > 0) {
      // 重置已使用颜色索引
      usedColorIndices = [];
      
      // 将词汇数据转换为奖品格式
      const databasePrizes: Prize[] = vocabularyData.map((item: any) => {
        const imgSrc = convertFileSrc(item.image_path);
        console.log(`处理图片路径: ${item.image_path} -> ${imgSrc}`);
        
        // 判断是否是魔法礼袋
        const isMagicBag = isMagicBagItem(item);
        
        // 使用数据库中的自定义颜色或默认颜色
        const itemColor = item.color || null;
        console.log(`单词颜色: ${item.word} -> ${itemColor || '未设置，使用随机颜色'}`);
        
        return {
          background: isMagicBag ? '#fab1a0' : (itemColor || getRandomColor()),
          fonts: [
            { 
              text: isMagicBag ? 'Magic Bag' : item.word, 
              top: '55%', 
              fontColor: '#2d3436', 
              fontSize: '16px', 
              fontWeight: 'bold' 
            },
          ],
          imgs: [{ src: imgSrc, width: '100px', top: '10%' }],
          prizeInfo: {
            name: isMagicBag ? "Magic Bag" : `${item.word} / ${item.translation}`,
            imgSrc: imgSrc,
            translation: item.translation,
            phonetic: item.phonetic,
            example: item.example
          }
        };
      });
      
      console.log('处理后的转盘奖品数据:', databasePrizes);
      
      // 设置奖品列表（引用替换而不是修改属性）
      prizes.value = databasePrizes;
      
      console.log('从数据库加载了词汇数据:', vocabularyData.length);
    } else {
      // 如果没有数据，使用默认奖品
      console.log('数据库中没有词汇数据，使用默认数据');
      
      // 重置已使用颜色索引
      usedColorIndices = [];
      
      // 确保默认奖品使用不同颜色
      prizes.value = defaultPrizes.map((prize) => {
        // 为"Magic Bag"保留原来的颜色
        if (prize.prizeInfo.name === "Magic Bag") {
          return prize;
        }
        return {
          ...prize,
          background: getRandomColor()
        };
      });
    }
  } catch (error) {
    console.error('加载数据库词汇数据失败:', error);
    
    // 重置已使用颜色索引
    usedColorIndices = [];
    
    // 确保默认奖品使用不同颜色
    prizes.value = defaultPrizes.map((prize) => {
      // 为"Magic Bag"保留原来的颜色
      if (prize.prizeInfo.name === "Magic Bag") {
        return prize;
      }
      return {
        ...prize,
        background: getRandomColor()
      };
    });
  }
};

// 添加音频相关状态
const spinAudioEl = ref<HTMLAudioElement | null>(null);
const winAudioEl = ref<HTMLAudioElement | null>(null);
const spinAudio = ref<HTMLAudioElement | null>(null);
const winAudio = ref<HTMLAudioElement | null>(null);

// 音频状态
const isSpinAudioLoaded = ref(false);
const isWinAudioLoaded = ref(false);
const isSpinAudioPlaying = ref(false);

// 修改onMounted钩子
onMounted(async () => {
  console.log('LuckyWheel组件挂载');
  
  // 确保转盘组件已经初始化
  await new Promise(resolve => setTimeout(resolve, 300));
  
  // 加载数据库中的单词
  await loadVocabularyFromDatabase();
  
  // 初始化抽奖记录
  console.log('初始化抽奖记录');
  initializePrizeRecords();
  
  // 打印初始化后的记录和奖品列表
  console.log('初始化后的记录:', JSON.stringify(prizeRecordsRaw.value));
  console.log('当前奖品列表:', prizes.value.map(p => p.prizeInfo?.name || '未命名'));
  
  // 确保DOM已完全挂载
  await new Promise(resolve => setTimeout(resolve, 200));
  
  // 检查音频元素是否存在
  if (!spinAudioEl.value) {
    spinAudioEl.value = document.querySelector('audio[ref="spinAudioEl"]');
    console.log('通过DOM查询获取spinAudioEl:', spinAudioEl.value ? '成功' : '失败');
  }
  
  if (!winAudioEl.value) {
    winAudioEl.value = document.querySelector('audio[ref="winAudioEl"]');
    console.log('通过DOM查询获取winAudioEl:', winAudioEl.value ? '成功' : '失败');
  }
  
  // 加载音频资源
  console.log('开始加载音频资源');
  await loadAudios();
  
  // 标记已初始化
  isInitialized.value = true;
});

// 添加对prizes的监听，当奖品变化时更新记录
watch(prizes, () => {
  console.log('奖品列表变化，更新记录');
  initializePrizeRecords();
}, { deep: true });

// 监听maxDraws设置的变化
watch(() => settings.maxDraws, (newValue) => {
  console.log(`最大抽取次数更改为: ${newValue}`);
}, { immediate: true });

// 监听音效设置的变化
watch(() => settings.sounds, () => {
  console.log('音效设置变化，重新加载音频');
  loadAudios();
}, { deep: true });

// 手动初始化奖品记录
function initializePrizeRecords() {
  const records: Record<string, number> = {};
  
  // 先确保旧记录的值被保留
  const oldRecords = { ...prizeRecordsRaw.value };
  
  // 遍历所有奖品
  prizes.value.forEach(prize => {
    if (prize.prizeInfo && prize.prizeInfo.name) {
      // 如果旧记录中有值，则保留，否则设为0
      records[prize.prizeInfo.name] = oldRecords[prize.prizeInfo.name] || 0;
      console.log(`初始化奖品记录: ${prize.prizeInfo.name} = ${records[prize.prizeInfo.name]}`);
    }
  });
  
  // 替换整个记录对象，保证响应式更新
  prizeRecordsRaw.value = records;
  console.log('初始化后的记录:', prizeRecordsRaw.value);
}

// 强制更新记录
function forceUpdateRecords(): void {
  console.log('强制更新前的记录:', JSON.stringify(prizeRecordsRaw.value));
  // 手动刷新一次奖品记录
  initializePrizeRecords();
  console.log('强制更新后的记录:', JSON.stringify(prizeRecordsRaw.value));
  showTip('记录已刷新', 1500);
}

// 检查是否所有普通奖品都至少抽中一次
function checkAllPrizesDrawnOnce(): boolean {
  const prizeNames = Object.keys(prizeRecordsRaw.value).filter(name => name !== "Magic Bag");
  allPrizesDrawnOnce.value = prizeNames.every(name => prizeRecordsRaw.value[name] > 0);
  return allPrizesDrawnOnce.value;
}

// 检查是否所有普通奖品都已经抽中最大次数
function areAllPrizesDrawnToMax(): boolean {
  const maxDraws = settings.maxDraws || 1;
  const prizeNames = Object.keys(prizeRecordsRaw.value).filter(name => name !== "Magic Bag");
  return prizeNames.every(name => prizeRecordsRaw.value[name] >= maxDraws);
}

// 获取下一个奖品索引
function getNextPrizeIndex(): number {
  // 检查是否所有普通奖品都至少抽中一次
  checkAllPrizesDrawnOnce();
  
  // 如果抽奖已完成并且锁定，则不允许继续抽奖
  if (isCompletedFlag.value && lockAfterComplete.value) {
    // 返回"Magic Bag"的索引
    return getThanksIndex();
  }
  
  // 获取当前抽奖模式
  const drawMode = settings.drawMode;
  
  if (drawMode === 'orderly') {
    // 有序模式：每个奖品都要抽一次，最大抽取次数为settings.maxDraws次，抽完后只能抽到Magic Bag
    
    // 获取未抽中过的奖品索引
    const undrawnPrizes = getUndrawnPrizeIndices();
    
    // 如果还有未抽中的普通奖品，从中随机选择一个
    if (undrawnPrizes.length > 0) {
      const randomIndex = Math.floor(Math.random() * undrawnPrizes.length);
      return undrawnPrizes[randomIndex];
    }
    
    // 获取未达到最大抽取次数的奖品
    const availablePrizes = getAvailablePrizeIndices();
    
    // 如果还有未达到最大抽取次数的奖品，从中随机选择一个
    if (availablePrizes.length > 0) {
      const randomIndex = Math.floor(Math.random() * availablePrizes.length);
      return availablePrizes[randomIndex];
    }
    
    // 如果所有奖品都抽到最大次数，返回"Magic Bag"
    isCompletedFlag.value = true;
    return getThanksIndex();
  } else {
    // 随机模式：奖品和Magic Bag完全随机，但要遵循最大抽取次数限制
    
    // 获取未达到最大抽取次数的奖品索引（包括Magic Bag）
    const availablePrizes = getAvailablePrizeIndicesWithMagicBag();
    
    // 如果还有可用奖品，从中随机选择一个
    if (availablePrizes.length > 0) {
      const randomIndex = Math.floor(Math.random() * availablePrizes.length);
      return availablePrizes[randomIndex];
    }
    
    // 如果所有普通奖品都已经抽中最大次数，标记为完成并返回Magic Bag
    isCompletedFlag.value = true;
    return getThanksIndex();
  }
}

// 获取"Magic Bag"的索引
function getThanksIndex(): number {
  const thanksIndex = prizes.value.findIndex(prize => 
    prize.prizeInfo && prize.prizeInfo.name === "Magic Bag");
  return thanksIndex >= 0 ? thanksIndex : prizes.value.length - 1; // 默认最后一个是"Magic Bag"
}

// 获取未抽中过的奖品索引
function getUndrawnPrizeIndices(): number[] {
  const undrawnIndices: number[] = [];
  // 只检查非"Magic Bag"的普通奖品
  prizes.value.forEach((prize, index) => {
    if (prize.prizeInfo && prize.prizeInfo.name !== "Magic Bag" && 
        prizeRecordsRaw.value[prize.prizeInfo.name] === 0) {
      undrawnIndices.push(index);
    }
  });
  return undrawnIndices;
}

// 获取未达到最大抽取次数的奖品索引（不包括Magic Bag）
function getAvailablePrizeIndices(): number[] {
  const maxDraws = settings.maxDraws || 1;
  const availableIndices: number[] = [];
  
  prizes.value.forEach((prize, index) => {
    if (prize.prizeInfo && prize.prizeInfo.name !== "Magic Bag") {
      const count = prizeRecordsRaw.value[prize.prizeInfo.name] || 0;
      if (count < maxDraws) {
        availableIndices.push(index);
      }
    }
  });
  
  return availableIndices;
}

// 获取未达到最大抽取次数的奖品索引（包括Magic Bag）
function getAvailablePrizeIndicesWithMagicBag(): number[] {
  const maxDraws = settings.maxDraws || 1;
  const availableIndices: number[] = [];
  const magicBagIndex = getThanksIndex();
  
  prizes.value.forEach((prize, index) => {
    if (prize.prizeInfo) {
      if (prize.prizeInfo.name === "Magic Bag") {
        // Magic Bag总是可用的
        availableIndices.push(index);
      } else {
        // 普通奖品检查抽取次数
        const count = prizeRecordsRaw.value[prize.prizeInfo.name] || 0;
        if (count < maxDraws) {
          availableIndices.push(index);
        }
      }
    }
  });
  
  // 如果没有可用奖品，至少返回Magic Bag
  if (availableIndices.length === 0 && magicBagIndex >= 0) {
    availableIndices.push(magicBagIndex);
  }
  
  return availableIndices;
}

// 更新奖品抽中记录
function updatePrizeRecord(prizeIndex: number) {
  console.log(`更新奖品记录，索引: ${prizeIndex}`);
  
  if (prizeIndex >= 0 && prizeIndex < prizes.value.length) {
    const prizeName = prizes.value[prizeIndex].prizeInfo.name;
    console.log(`奖品名称: ${prizeName}`);
    
    // 打印当前的记录状态
    console.log('当前记录状态:', JSON.stringify(prizeRecordsRaw.value));
    
    // 如果奖品名称存在但记录中没有，先初始化
    if (prizeName && prizeRecordsRaw.value[prizeName] === undefined) {
      console.log(`记录中没有此奖品，正在初始化: ${prizeName}`);
      prizeRecordsRaw.value[prizeName] = 0;
    }
    
    if (prizeRecordsRaw.value[prizeName] !== undefined) {
      // 增加计数
      prizeRecordsRaw.value[prizeName]++;
      console.log(`更新后的记录: ${prizeName} = ${prizeRecordsRaw.value[prizeName]}`);
      
      // 检查是否所有奖品都至少抽中一次
      checkAllPrizesDrawnOnce();
      
      // 检查是否抽完（所有普通奖品都抽中最大次数）
      if (areAllPrizesDrawnToMax()) {
        isCompletedFlag.value = true;
        console.log('所有奖品已达到最大抽取次数，标记为完成');
      }
      
      // 确保实时更新UI
      // 使用Vue的响应式系统强制更新视图
      prizeRecordsRaw.value = { ...prizeRecordsRaw.value };
      
      return {
        prize: prizes.value[prizeIndex],
        name: prizeName,
        count: prizeRecordsRaw.value[prizeName]
      };
    } else {
      console.error(`奖品 ${prizeName} 的记录依然不存在，无法更新`);
    }
  } else {
    console.error(`奖品索引 ${prizeIndex} 超出范围`);
  }
  return null;
}

// 重置抽奖记录
function resetRecords(): void {
  // 如果当前正在过渡中，不执行任何操作
  if (isTransitioning.value) {
    return;
  }

  // 设置过渡锁
  isTransitioning.value = true;
  
  // 停止所有音频
  stopSpinSound();
  
  // 清除自动滑动定时器
  if (autoSlideTimer) {
    clearTimeout(autoSlideTimer);
    autoSlideTimer = null;
  }
  
  // 清除进度条更新计时器
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
    autoCloseInterval = null;
  }
  
  // 重置记录
  for (const key in prizeRecordsRaw.value) {
    prizeRecordsRaw.value[key] = 0;
  }
  
  // 隐藏图片显示
  if (showImageDisplay.value) {
    // 检查是否正在滑动
    const isInSlidingAnimation = isSliding.value;
    
    // 如果正在执行滑动动画，取消滑动
    if (isInSlidingAnimation) {
      isSliding.value = false;
      
      // 移除可能存在的滑动结束类
      const displayEl = document.querySelector('.image-display');
      if (displayEl) {
        displayEl.classList.remove('sliding-end');
      }
    }
    
    // 直接关闭显示
    showImageDisplay.value = false;
    
    // 等待过渡完成后再重置其他状态
    setTimeout(() => {
      isEnlarged.value = false;
      autoCloseSecondsLeft.value = 5; // 重置倒计时
      selectedPrize.value = null; // 完全清除选中的奖品，避免下次显示时再次从右侧滑入
      
      // 释放过渡锁
      isTransitioning.value = false;
    }, 600);
  } else {
    // 如果没有显示图片，直接释放锁
    isTransitioning.value = false;
  }
  
  // 这部分代码也可以移除，因为我们不再改变扇形颜色
  // 但为了保险起见，保留这段代码以确保任何可能的颜色变化都被重置
  prizes.value.forEach(prize => {
    // 移除灰色滤镜
    if (prize.background.includes('linear-gradient')) {
      // 提取原始颜色
      const originalColor = prize.background.split('), ')[1];
      prize.background = originalColor;
      
      // 恢复字体颜色
      prize.fonts.forEach(font => {
        font.fontColor = '#2d3436';
      });
    }
  });
  
  allPrizesDrawnOnce.value = false;
  isCompletedFlag.value = false;
  showTip('单词转盘记录已重置，可以重新开始抽单词！', 3000);
}

// 在组件卸载前清除所有定时器和音频资源
onBeforeUnmount(() => {
  if (autoSlideTimer) {
    clearTimeout(autoSlideTimer);
    autoSlideTimer = null;
  }
  
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
    autoCloseInterval = null;
  }
  
  if (tooltipTimer) {
    clearTimeout(tooltipTimer);
    tooltipTimer = null;
  }
  
  // 清理音频资源
  stopSpinSound();
  if (spinAudio.value) {
    spinAudio.value.pause();
    spinAudio.value.src = '';
  }
  
  if (winAudio.value) {
    winAudio.value.pause();
    winAudio.value.src = '';
  }
});

// 开始转动回调
function startCallback(): void {
  // 如果当前正在过渡中，不执行任何操作
  if (isTransitioning.value) {
    return;
  }
  
  // 清除自动滑动定时器
  if (autoSlideTimer) {
    clearTimeout(autoSlideTimer);
    autoSlideTimer = null;
  }
  
  // 如果抽奖已完成并且锁定，显示提示而不启动转盘
  if (isCompletedFlag.value && lockAfterComplete.value) {
    // 确保停止任何可能正在播放的音效
    stopSpinSound();
    alert("单词转盘已完成，请点击重置按钮重新开始");
    return;
  }
  
  // 只有在图片没有显示时才允许开始转盘
  if (!showImageDisplay.value && myLucky.value) {
    // 设置转盘旋转状态为开始
    isRotating.value = true;
    
    // 开始转盘旋转
    (myLucky.value as any).play();
    
    // 播放旋转音效
    playSpinSound();
    
    // 根据规则选择抽奖结果
    setTimeout(() => {
      const selectedIndex = getNextPrizeIndex();
      if (myLucky.value) {
        (myLucky.value as any).stop(selectedIndex);
      }
    }, settings.spinDuration); // 使用设置中的转盘旋转时间
  } else {
    // 如果转盘无法开始旋转，确保停止任何可能正在播放的音效
    stopSpinSound();
  }
}

// 添加转盘旋转中回调
function rotatingCallback(data: any): void {
  // 不管是否启用高亮边框效果，都记录当前索引
  // 这样当用户打开高亮边框效果设置时，可以立即看到效果
  if (data.currIndex !== highlightIndex.value) {
    highlightIndex.value = data.currIndex;
  }
}

// 结束转动回调
function endCallback(prize: any): void {
  console.log('转盘停止，中奖信息:', prize);
  
  // 停止旋转音效
  stopSpinSound();
  
  // 设置转盘旋转状态为结束
  isRotating.value = false;
  
  // 清空高亮索引
  setTimeout(() => {
    highlightIndex.value = null;
  }, 1000); // 在转盘停止1秒后移除高亮效果
  
  // 获取中奖索引
  let prizeIndex = -1;
  
  // 尝试通过文本匹配查找
  if (prize.fonts && prize.fonts.length > 0 && prize.fonts[0].text) {
    prizeIndex = prizes.value.findIndex((p: Prize) => 
      p.fonts[0].text === prize.fonts[0].text);
    console.log(`通过文本匹配查找奖品，文本: ${prize.fonts[0].text}, 索引: ${prizeIndex}`);
  }
  
  // 如果找不到，尝试通过背景颜色匹配
  if (prizeIndex === -1 && prize.background) {
    prizeIndex = prizes.value.findIndex((p: Prize) => 
      p.background === prize.background);
    console.log(`通过背景颜色匹配查找奖品，颜色: ${prize.background}, 索引: ${prizeIndex}`);
  }
  
  // 如果匹配到了有效的奖品索引
  if (prizeIndex !== -1) {
    // 记录奖品名称以便调试
    const prizeName = prizes.value[prizeIndex].prizeInfo?.name || '未知奖品';
    console.log(`匹配到奖品：${prizeName}，索引：${prizeIndex}`);
    
    // 播放中奖音效
    playWinSound();
    
    // 更新抽奖记录
    const result = updatePrizeRecord(prizeIndex);
    
    if (result) {
      // 设置选中的奖品显示
      selectedPrize.value = prizes.value[prizeIndex].prizeInfo;
      
      // 判断是否为Magic Bag
      const isMagicBag = selectedPrize.value.name === "Magic Bag";
      
      // 直接显示图片，不使用滑入效果
      showImageDisplay.value = true; // 显示图片
      
      // 确保DOM已更新
      setTimeout(() => {
        isEnlarged.value = true; // 放大图片
        
        // 只有Magic Bag才启动倒计时和自动滑动
        if (isMagicBag) {
          // 启动自动关闭倒计时
          startAutoCloseCountdown();
          
          // 设置5秒后自动滑走
          if (autoSlideTimer) {
            clearTimeout(autoSlideTimer);
          }
          autoSlideTimer = window.setTimeout(() => {
            autoSlideImage();
          }, 5000);
        }
      }, 50);
      
      // 显示抽奖结果提示
      const isPrizeThanks = prizes.value[prizeIndex].prizeInfo.name === "Magic Bag";
      const count = prizeRecordsRaw.value[prizes.value[prizeIndex].prizeInfo.name];
      
      if (isPrizeThanks) {
        showTip('本次抽中: Magic Bag', 1500);
      } else {
        showTip(`恭喜！抽中 ${prizes.value[prizeIndex].prizeInfo.name} (第${count}次)`, 1500);
      }
      
      console.log('抽奖后的记录:', JSON.stringify(prizeRecordsRaw.value));
      
      // 如果抽奖已完成并且锁定，显示提示
      if (isCompletedFlag.value && lockAfterComplete.value) {
        setTimeout(() => {
          showTip("所有奖品已抽完，点击重置按钮重新开始", 5000);
        }, 2000);
      }
    } else {
      console.error(`更新奖品记录失败: ${prizeName}`);
    }
  } else {
    console.error('未能找到匹配的奖品索引');
    // 如果找不到匹配的奖品，可能是初始结构有问题，尝试重新初始化记录
    initializePrizeRecords();
  }
}

// 添加自动滑动图片函数
function autoSlideImage(): void {
  // 如果当前正在过渡中，不执行任何操作
  if (isTransitioning.value) {
    return;
  }
  
  // 设置过渡锁
  isTransitioning.value = true;
  
  // 停止所有音效
  stopSpinSound();
  
  // 清除自动关闭计时器
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
    autoCloseInterval = null;
  }
  
  // 检查是否为Magic Bag
  const isMagicBag = selectedPrize.value && selectedPrize.value.name === "Magic Bag";
  
  // 如果当前正在显示图片，则触发滑出动画
  if (showImageDisplay.value && isEnlarged.value && isMagicBag) {
    // 停止倒计时
    autoCloseSecondsLeft.value = 0;
    
    // 开始向左滑动
    isSliding.value = true;
    
    // 给滑动动画充分时间完成
    const slideTime = 4000; // 4秒滑动时间
    
    // 在滑动即将结束时应用淡出效果
    setTimeout(() => {
      // 添加滑动结束的类，触发淡出
      const displayEl = document.querySelector('.image-display');
      if (displayEl) {
        displayEl.classList.add('sliding-end');
      }
      
      // 给淡出动画一些时间完成
      setTimeout(() => {
        // 完全隐藏并重置所有状态
        showImageDisplay.value = false;
        isSliding.value = false;
        
        // 稍微延迟后重置其他状态，确保DOM已完全更新
        setTimeout(() => {
          // 移除滑动结束类，避免影响下次动画
          if (displayEl) {
            displayEl.classList.remove('sliding-end');
          }
          
          isEnlarged.value = false;
          autoCloseSecondsLeft.value = 5;
          selectedPrize.value = null;
          isTransitioning.value = false;
        }, 50);
      }, 300); // 等待淡出动画完成
      
    }, slideTime - 100); // 在滑动完成前稍微提前开始淡出
  } else if (showImageDisplay.value && isEnlarged.value) {
    // 非Magic Bag直接关闭
    showImageDisplay.value = false;
    
    // 简单延迟后重置状态
    setTimeout(() => {
      isEnlarged.value = false;
      autoCloseSecondsLeft.value = 5;
      selectedPrize.value = null;
      isTransitioning.value = false;
    }, 600);
  } else {
    // 如果不满足条件，直接释放锁
    isTransitioning.value = false;
  }
}

// 启动自动关闭倒计时
function startAutoCloseCountdown(): void {
  // 重置计时
  autoCloseSecondsLeft.value = 5;
  
  // 清除之前可能存在的计时器
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
  }
  
  // 设置每100ms更新一次进度条
  autoCloseInterval = window.setInterval(() => {
    autoCloseSecondsLeft.value -= 0.1;
    
    // 当倒计时结束，清除定时器
    if (autoCloseSecondsLeft.value <= 0) {
      if (autoCloseInterval) {
        clearInterval(autoCloseInterval);
        autoCloseInterval = null;
      }
    }
  }, 100);
}

// 点击切换图片显示
function toggleImageSize(): void {
  // 如果当前正在过渡中，不执行任何操作
  if (isTransitioning.value) {
    return;
  }
  
  // 停止所有音效
  stopSpinSound();
  
  // 清除自动滑动定时器
  if (autoSlideTimer) {
    clearTimeout(autoSlideTimer);
    autoSlideTimer = null;
  }
  
  // 清除进度条更新计时器
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
    autoCloseInterval = null;
  }
  
  // 判断当前奖品是否为Magic Bag
  const isMagicBag = selectedPrize.value && selectedPrize.value.name === "Magic Bag";
  
  if (isEnlarged.value) {
    // 如果已经放大，根据奖品类型和点击次数决定如何操作
    
    if (isMagicBag) {
      // Magic Bag仍然使用原来的滑动效果
      // 设置过渡锁
      isTransitioning.value = true;
      
      // Magic Bag使用滑动效果
      isSliding.value = true;
      
      // 给滑动动画充分时间完成
      const slideTime = 4000; // 4秒滑动时间
      
      // 在滑动即将结束时应用淡出效果
      setTimeout(() => {
        // 添加滑动结束的类，触发淡出
        const displayEl = document.querySelector('.image-display');
        if (displayEl) {
          displayEl.classList.add('sliding-end');
        }
        
        // 给淡出动画一些时间完成
        setTimeout(() => {
          // 完全隐藏并重置所有状态
          showImageDisplay.value = false;
          isSliding.value = false;
          
          // 稍微延迟后重置其他状态，确保DOM已完全更新
          setTimeout(() => {
            // 移除滑动结束类，避免影响下次动画
            if (displayEl) {
              displayEl.classList.remove('sliding-end');
            }
            
            isEnlarged.value = false;
            autoCloseSecondsLeft.value = 5;
            selectedPrize.value = null;
            isTransitioning.value = false;
            // 重置点击计数和缩放级别
            clickCount.value = 0;
            scaleLevel.value = 0;
          }, 50);
        }, 300); // 等待淡出动画完成
        
      }, slideTime - 100); // 在滑动完成前稍微提前开始淡出
    } else {
      // 普通奖品使用三次点击缩小效果
      clickCount.value++;
      
      if (clickCount.value === 1) {
        // 第一次点击：缩小到70%
        scaleLevel.value = 1;
      } else if (clickCount.value === 2) {
        // 第二次点击：缩小到40%
        scaleLevel.value = 2;
      } else if (clickCount.value >= 3) {
        // 第三次点击：关闭弹框
        // 设置过渡锁
        isTransitioning.value = true;
        
        // 普通奖品直接淡出
        showImageDisplay.value = false;
        
        // 简单延迟后重置状态
        setTimeout(() => {
          isEnlarged.value = false;
          autoCloseSecondsLeft.value = 5;
          selectedPrize.value = null;
          isTransitioning.value = false;
          // 重置点击计数和缩放级别
          clickCount.value = 0;
          scaleLevel.value = 0;
        }, 600);
      }
    }
  } else {
    // 设置过渡锁
    isTransitioning.value = true;
    
    // 如果没有放大，直接显示并放大
    showImageDisplay.value = true;
    
    // 重置点击计数和缩放级别
    clickCount.value = 0;
    scaleLevel.value = 0;
    
    // 给一个短暂延迟，让DOM渲染完成
    setTimeout(() => {
      isEnlarged.value = true; // 放大
      
      // 只有Magic Bag才启动倒计时和自动滑动
      if (isMagicBag) {
        // 启动自动关闭倒计时
        startAutoCloseCountdown();
        
        // 设置5秒后自动滑走
        autoSlideTimer = window.setTimeout(() => {
          autoSlideImage();
        }, 5000);
      }
      
      // 释放过渡锁
      setTimeout(() => {
        isTransitioning.value = false;
      }, 800);
    }, 50);
  }
}

// 计算属性：获取抽奖记录
const prizeRecords = computed(() => {
  return prizeRecordsRaw.value;
});

// 计算属性：判断是否已完成抽奖
const isCompleted = computed(() => {
  return isCompletedFlag.value;
});

// 显示工具提示
function showTip(text: string, duration: number = 2000): void {
  tooltipText.value = text;
  showTooltip.value = true;
  
  // 清除之前的定时器
  if (tooltipTimer) {
    clearTimeout(tooltipTimer);
  }
  
  // 设置自动关闭
  tooltipTimer = window.setTimeout(() => {
    showTooltip.value = false;
  }, duration);
}

// 添加音频控制相关函数
// 加载音频资源
async function loadAudios() {
  console.log('开始加载音频资源');
  
  // 检查是否有音效设置
  if (settings.sounds?.spin?.url) {
    try {
      let spinAudioUrl = settings.sounds.spin.url;
      
      console.log('加载旋转音效:', spinAudioUrl);
      
      // 首先尝试使用DOM元素
      if (spinAudioEl.value) {
        spinAudioEl.value.src = spinAudioUrl;
        spinAudioEl.value.loop = true; // 旋转音效需要循环播放
        spinAudioEl.value.volume = 0.5;
        spinAudio.value = spinAudioEl.value;
      } else {
        // 备用方案：直接创建新的Audio对象
        console.log('找不到旋转音效DOM元素，创建新的Audio对象');
        spinAudio.value = new Audio(spinAudioUrl);
        spinAudio.value.loop = true;
        spinAudio.value.volume = 0.5;
      }
      
      // 监听加载完成事件
      spinAudio.value.addEventListener('canplaythrough', () => {
        console.log('旋转音效加载完成');
        isSpinAudioLoaded.value = true;
      });
      
      // 监听错误事件
      spinAudio.value.addEventListener('error', (e) => {
        console.error('旋转音效加载失败:', e, spinAudio.value?.error);
        isSpinAudioLoaded.value = false;
        
        // 尝试使用备用方法加载
        if (typeof Audio !== 'undefined') {
          console.log('尝试使用备用方法加载旋转音效');
          const backupAudio = new Audio();
          backupAudio.src = spinAudioUrl;
          backupAudio.loop = true;
          backupAudio.volume = 0.5;
          
          backupAudio.addEventListener('canplaythrough', () => {
            console.log('备用方法：旋转音效加载完成');
            spinAudio.value = backupAudio;
            isSpinAudioLoaded.value = true;
          });
          
          backupAudio.addEventListener('error', (err) => {
            console.error('备用方法：旋转音效加载失败:', err);
          });
          
          backupAudio.load();
        }
      });
      
      // 预加载音频
      spinAudio.value.load();
    } catch (error) {
      console.error('创建旋转音效失败:', error);
      isSpinAudioLoaded.value = false;
    }
  } else {
    console.log('未设置旋转音效');
  }
  
  if (settings.sounds?.win?.url) {
    try {
      let winAudioUrl = settings.sounds.win.url;
      
      
      console.log('加载中奖音效:', winAudioUrl);
      
      // 首先尝试使用DOM元素
      if (winAudioEl.value) {
        winAudioEl.value.src = winAudioUrl;
        winAudioEl.value.loop = false; // 中奖音效不循环
        winAudioEl.value.volume = 0.6;
        winAudio.value = winAudioEl.value;
      } else {
        // 备用方案：直接创建新的Audio对象
        console.log('找不到中奖音效DOM元素，创建新的Audio对象');
        winAudio.value = new Audio(winAudioUrl);
        winAudio.value.loop = false;
        winAudio.value.volume = 0.6;
      }
      
      // 监听加载完成事件
      winAudio.value.addEventListener('canplaythrough', () => {
        console.log('中奖音效加载完成');
        isWinAudioLoaded.value = true;
      });
      
      // 监听错误事件
      winAudio.value.addEventListener('error', (e) => {
        console.error('中奖音效加载失败:', e, winAudio.value?.error);
        isWinAudioLoaded.value = false;
        
        // 尝试使用备用方法加载
        if (typeof Audio !== 'undefined') {
          console.log('尝试使用备用方法加载中奖音效');
          const backupAudio = new Audio();
          backupAudio.src = winAudioUrl;
          backupAudio.loop = false;
          backupAudio.volume = 0.6;
          
          backupAudio.addEventListener('canplaythrough', () => {
            console.log('备用方法：中奖音效加载完成');
            winAudio.value = backupAudio;
            isWinAudioLoaded.value = true;
          });
          
          backupAudio.addEventListener('error', (err) => {
            console.error('备用方法：中奖音效加载失败:', err);
          });
          
          backupAudio.load();
        }
      });
      
      // 预加载音频
      winAudio.value.load();
    } catch (error) {
      console.error('创建中奖音效失败:', error);
      isWinAudioLoaded.value = false;
    }
  } else {
    console.log('未设置中奖音效');
  }
}

// 播放旋转音效
async function playSpinSound() {
  if (!spinAudio.value) {
    console.log('旋转音效未初始化');
    return;
  }
  
  if (!isSpinAudioLoaded.value) {
    console.log('旋转音效未加载完成，重新尝试加载');
    // 尝试重新加载
    try {
      spinAudio.value.load();
      await new Promise(resolve => setTimeout(resolve, 500));
    } catch (e) {
      console.error('重新加载旋转音效失败:', e);
      return;
    }
  }
  
  try {
    console.log('开始播放旋转音效');
    // 确保音频已就绪
    spinAudio.value.currentTime = 0;
    const playPromise = spinAudio.value.play();
    
    if (playPromise !== undefined) {
      playPromise.then(() => {
        console.log('旋转音效开始播放');
        isSpinAudioPlaying.value = true;
      }).catch(error => {
        console.error('播放旋转音效失败:', error);
        
        // 如果是用户交互错误，则添加点击一次后重新尝试
        if (error.name === 'NotAllowedError') {
          console.log('需要用户交互才能播放音频，将在下一次用户交互时尝试');
          
          // 一次性事件监听器，在用户下一次交互时尝试播放
          const playAfterInteraction = () => {
            try {
              if (spinAudio.value) {
                spinAudio.value.currentTime = 0;
                spinAudio.value.play()
                  .then(() => {
                    isSpinAudioPlaying.value = true;
                  })
                  .catch(e => {
                    console.error('用户交互后播放失败:', e);
                  });
              }
            } catch (e) {
              console.error('用户交互后播放处理失败:', e);
            }
            document.removeEventListener('click', playAfterInteraction);
          };
          
          document.addEventListener('click', playAfterInteraction, { once: true });
        }
      });
    }
  } catch (error) {
    console.error('播放旋转音效出现异常:', error);
  }
}

// 停止旋转音效
function stopSpinSound() {
  if (spinAudio.value && isSpinAudioPlaying.value) {
    console.log('停止旋转音效');
    spinAudio.value.pause();
    spinAudio.value.currentTime = 0;
    isSpinAudioPlaying.value = false;
  }
}

// 播放中奖音效
async function playWinSound() {
  if (!winAudio.value) {
    console.log('中奖音效未初始化');
    return;
  }
  
  if (!isWinAudioLoaded.value) {
    console.log('中奖音效未加载完成，重新尝试加载');
    // 尝试重新加载
    try {
      winAudio.value.load();
      await new Promise(resolve => setTimeout(resolve, 500));
    } catch (e) {
      console.error('重新加载中奖音效失败:', e);
      return;
    }
  }
  
  try {
    console.log('开始播放中奖音效');
    // 确保音频重置到开始位置
    winAudio.value.currentTime = 0;
    const playPromise = winAudio.value.play();
    
    if (playPromise !== undefined) {
      playPromise.then(() => {
        console.log('中奖音效开始播放');
      }).catch(error => {
        console.error('播放中奖音效失败:', error);
        
        // 如果是用户交互错误，则添加点击一次后重新尝试
        if (error.name === 'NotAllowedError') {
          console.log('需要用户交互才能播放音频，将在下一次用户交互时尝试');
          
          // 一次性事件监听器，在用户下一次交互时尝试播放
          const playAfterInteraction = () => {
            try {
              if (winAudio.value) {
                winAudio.value.currentTime = 0;
                winAudio.value.play()
                  .catch(e => {
                    console.error('用户交互后播放失败:', e);
                  });
              }
            } catch (e) {
              console.error('用户交互后播放处理失败:', e);
            }
            document.removeEventListener('click', playAfterInteraction);
          };
          
          document.addEventListener('click', playAfterInteraction, { once: true });
        }
      });
    }
  } catch (error) {
    console.error('播放中奖音效出现异常:', error);
  }
}
</script>

<style>
@import '../styles/luckyWheel.css';
</style>
