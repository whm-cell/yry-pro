<template>
  <div class="lucky-container">
    <!-- 转盘部分 -->
    <LuckyWheel
      ref="myLucky"
      width="600px"
      height="600px"
      :prizes="prizes"
      :blocks="blocks"
      :buttons="buttons"
      @start="startCallback"
      @end="endCallback"
    />
    
    <!-- 图片展示区域 -->
    <div 
      class="image-display" 
      :class="{ 'active': showImageDisplay }"
      @click.self="toggleImageSize"
    >
      <div 
        v-if="selectedPrize"
        class="prize-image" 
        :class="{ 'enlarged': isEnlarged, 'sliding': isSliding }"
        @click="toggleImageSize"
      >
        <!-- 添加自动关闭计时器指示，只在魔法小礼袋时显示 -->
        <div class="auto-close-indicator" v-if="isEnlarged && !isSliding && selectedPrize && selectedPrize.name === '魔法小礼袋'">
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
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onBeforeUnmount } from 'vue';
// 直接导入图片
import applePng from './ct-converted.png'  // 使用@别名指向src目录
import catPng from './ct-converted.png'
import ballPng from './ct-converted.png'
import dogPng from './ct-converted.png'
import starPng from './ct-converted.png'
import crownPng from './ct-converted.png'

// 导入设置钩子和类型
import { useWheelSettings, DrawMode } from '../utils/wheelSettings';

// 奖品信息类型
interface PrizeInfo {
  name: string;
  imgSrc: string;
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

// 自动关闭倒计时
const autoCloseSecondsLeft = ref(5);
const autoCloseProgress = computed(() => (autoCloseSecondsLeft.value / 5) * 100);

// 定义默认奖品数据
const defaultPrizes: Prize[] = [
  { 
    background: '#badc58', 
    fonts: [
      { text: 'Apple', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
      // { text: '苹果', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
    ],
    imgs: [{ src: applePng, width: '100px', top: '10%' }],
    // 额外信息
    prizeInfo: {
      name: "Apple / 苹果",
      imgSrc: applePng
    }
  },
  { 
    background: '#ff9ff3', 
    fonts: [
      { text: 'Cat', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
      // { text: '猫咪', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
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
      // { text: '球', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
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
      // { text: '小狗', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
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
      { text: '魔法小礼袋', top: '55%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' }
    ],
    imgs: [{ src: starPng, width: '100px', top: '10%' }],
    prizeInfo: {
      name: "魔法小礼袋",
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
      // top: '35%',
      fontColor: '#fff',
      fontSize: '18px',
      fontWeight: 'bold'
    }
  ],
  // imgs: [
  //   { src: crownPng, width: '25px', top: '10%' }
  // ]
}];

// 抽奖记录
const prizeRecordsRaw = ref<Record<string, number>>({});
// 标记是否所有奖品都至少抽中一次
const allPrizesDrawnOnce = ref(false);
// 标记是否已完成抽奖
const isCompletedFlag = ref(false);

// 初始化
onMounted(() => {
  // 从全局设置中获取值
  if (settings) {
    // 如果全局设置中有奖品数据，使用它
    if (settings.prizes && settings.prizes.length > 0) {
      prizes.value = [...settings.prizes];
    }
  }
  
  // 初始化抽奖记录
  initializePrizeRecords();
  
  // 标记已初始化
  isInitialized.value = true;
});

// 手动初始化奖品记录
function initializePrizeRecords() {
  const records: Record<string, number> = {};
  prizes.value.forEach(prize => {
    if (prize.prizeInfo && prize.prizeInfo.name) {
      records[prize.prizeInfo.name] = 0;
    }
  });
  prizeRecordsRaw.value = records;
}

// 检查是否所有普通奖品都至少抽中一次
function checkAllPrizesDrawnOnce(): boolean {
  const prizeNames = Object.keys(prizeRecordsRaw.value).filter(name => name !== "魔法小礼袋");
  allPrizesDrawnOnce.value = prizeNames.every(name => prizeRecordsRaw.value[name] > 0);
  return allPrizesDrawnOnce.value;
}

// 检查是否所有普通奖品都已经抽中最大次数
function areAllPrizesDrawnToMax(): boolean {
  const maxDraws = settings.maxDraws || 2;
  const prizeNames = Object.keys(prizeRecordsRaw.value).filter(name => name !== "魔法小礼袋");
  return prizeNames.every(name => prizeRecordsRaw.value[name] >= maxDraws);
}

// 获取下一个奖品索引
function getNextPrizeIndex(): number {
  // 检查是否所有普通奖品都至少抽中一次
  checkAllPrizesDrawnOnce();
  
  // 如果抽奖已完成并且锁定，则不允许继续抽奖
  if (isCompletedFlag.value && lockAfterComplete.value) {
    // 返回"魔法小礼袋"的索引
    return getThanksIndex();
  }
  
  // 获取当前抽奖模式
  const drawMode = settings.drawMode;
  
  if (drawMode === 'orderly') {
    // 有序模式：每个奖品都要抽一次，最大是1次，抽完后只能抽到魔法小礼袋
    
    // 获取未抽中过的奖品索引
    const undrawnPrizes = getUndrawnPrizeIndices();
    
    // 如果还有未抽中的普通奖品，从中随机选择一个
    if (undrawnPrizes.length > 0) {
      const randomIndex = Math.floor(Math.random() * undrawnPrizes.length);
      return undrawnPrizes[randomIndex];
    }
    
    // 如果所有奖品都抽过一次，返回"魔法小礼袋"
    isCompletedFlag.value = true;
    return getThanksIndex();
  } else {
    // 随机模式：奖品和魔法小礼袋完全随机
    
    // 所有奖品的索引（包括"魔法小礼袋"）
    const allPrizes = prizes.value.length;
    
    // 如果所有普通奖品都已经抽中最大次数，标记为完成
    if (areAllPrizesDrawnToMax()) {
      isCompletedFlag.value = true;
    }
    
    // 随机选择一个奖品索引
    return Math.floor(Math.random() * allPrizes);
  }
}

// 获取"魔法小礼袋"的索引
function getThanksIndex(): number {
  const thanksIndex = prizes.value.findIndex(prize => 
    prize.prizeInfo && prize.prizeInfo.name === "魔法小礼袋");
  return thanksIndex >= 0 ? thanksIndex : prizes.value.length - 1; // 默认最后一个是"魔法小礼袋"
}

// 获取未抽中过的奖品索引
function getUndrawnPrizeIndices(): number[] {
  const undrawnIndices: number[] = [];
  // 只检查非"魔法小礼袋"的普通奖品
  prizes.value.forEach((prize, index) => {
    if (prize.prizeInfo && prize.prizeInfo.name !== "魔法小礼袋" && 
        prizeRecordsRaw.value[prize.prizeInfo.name] === 0) {
      undrawnIndices.push(index);
    }
  });
  return undrawnIndices;
}

// 获取可选的奖品索引（次数小于最大抽取次数的奖品）
function getAvailablePrizeIndices(): number[] {
  const availableIndices: number[] = [];
  const maxDraws = settings.maxDraws || 2;
  
  // 检查非"魔法小礼袋"的普通奖品
  prizes.value.forEach((prize, index) => {
    if (prize.prizeInfo && prize.prizeInfo.name !== "魔法小礼袋" && 
        prizeRecordsRaw.value[prize.prizeInfo.name] < maxDraws) {
      availableIndices.push(index);
    }
  });
  return availableIndices;
}

// 更新奖品抽中记录
function updatePrizeRecord(prizeIndex: number) {
  if (prizeIndex >= 0 && prizeIndex < prizes.value.length) {
    const prizeName = prizes.value[prizeIndex].prizeInfo.name;
    
    if (prizeRecordsRaw.value[prizeName] !== undefined) {
      prizeRecordsRaw.value[prizeName]++;
      
      // 检查是否所有奖品都至少抽中一次
      checkAllPrizesDrawnOnce();
      
      // 检查是否抽完（所有普通奖品都抽中最大次数）
      if (areAllPrizesDrawnToMax()) {
        isCompletedFlag.value = true;
      }
      
      return {
        prize: prizes.value[prizeIndex],
        name: prizeName,
        count: prizeRecordsRaw.value[prizeName]
      };
    }
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
    showImageDisplay.value = false;
    
    // 等待过渡完成后再重置其他状态
    setTimeout(() => {
      isEnlarged.value = false;
      isSliding.value = false;
      selectedPrize.value = null; // 完全清除选中的奖品，避免下次显示时再次从右侧滑入
      
      // 释放过渡锁
      isTransitioning.value = false;
    }, 1000); // 等待足够长的时间让过渡效果完成
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

// 在组件卸载前清除所有定时器
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
    alert("单词转盘已完成，请点击重置按钮重新开始");
    return;
  }
  
  // 只有在图片没有显示时才允许开始转盘
  if (!showImageDisplay.value && myLucky.value) {
    (myLucky.value as any).play();
    
    // 根据规则选择抽奖结果
    setTimeout(() => {
      const selectedIndex = getNextPrizeIndex();
      if (myLucky.value) {
        (myLucky.value as any).stop(selectedIndex);
      }
    }, 3000);
  }
}

// 结束转动回调
function endCallback(prize: any): void {
  // 获取中奖索引
  const prizeIndex = prizes.value.findIndex((p: Prize) => 
    p.fonts[0].text === prize.fonts[0].text);
  
  if (prizeIndex !== -1) {
    // 更新抽奖记录
    const result = updatePrizeRecord(prizeIndex);
    
    if (result) {
      // 设置选中的奖品显示
      selectedPrize.value = prizes.value[prizeIndex].prizeInfo;
      
      // 判断是否为魔法小礼袋
      const isMagicBag = selectedPrize.value.name === "魔法小礼袋";
      
      // 直接显示图片，不使用滑入效果
      showImageDisplay.value = true; // 显示图片
      
      // 确保DOM已更新
      setTimeout(() => {
        isEnlarged.value = true; // 放大图片
        
        // 只有魔法小礼袋才启动倒计时和自动滑动
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
      const isPrizeThanks = prizes.value[prizeIndex].prizeInfo.name === "魔法小礼袋";
      const count = prizeRecordsRaw.value[prizes.value[prizeIndex].prizeInfo.name];
      
      if (isPrizeThanks) {
        showTip('本次抽中: 魔法小礼袋', 1500);
      } else {
        showTip(`恭喜！抽中 ${prizes.value[prizeIndex].prizeInfo.name} (第${count}次)`, 1500);
      }
      
      console.log('抽奖记录:', prizeRecordsRaw.value);
      
      // 如果抽奖已完成并且锁定，显示提示
      if (isCompletedFlag.value && lockAfterComplete.value) {
        setTimeout(() => {
          showTip("所有奖品已抽完，点击重置按钮重新开始", 5000);
        }, 2000);
      }
    }
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
  
  // 清除自动关闭计时器
  if (autoCloseInterval) {
    clearInterval(autoCloseInterval);
    autoCloseInterval = null;
  }
  
  // 检查是否为魔法小礼袋
  const isMagicBag = selectedPrize.value && selectedPrize.value.name === "魔法小礼袋";
  
  // 如果当前正在显示图片，则触发滑出动画
  if (showImageDisplay.value && isEnlarged.value && isMagicBag) {
    // 开始向左滑动
    isSliding.value = true;
    
    // 等待滑动动画完成后再隐藏
    setTimeout(() => {
      showImageDisplay.value = false;
      
      // 重置状态
      setTimeout(() => {
        isEnlarged.value = false;
        isSliding.value = false;
        autoCloseSecondsLeft.value = 5; // 重置倒计时
        selectedPrize.value = null; // 完全清除选中的奖品，避免下次显示时再次从右侧滑入
        isTransitioning.value = false; // 释放过渡锁
      }, 100);
    }, 800);
  } else if (showImageDisplay.value && isEnlarged.value) {
    // 非魔法小礼袋直接关闭
    showImageDisplay.value = false;
    
    // 简单延迟后重置状态
    setTimeout(() => {
      isEnlarged.value = false;
      autoCloseSecondsLeft.value = 5;
      selectedPrize.value = null;
      isTransitioning.value = false; // 释放过渡锁
    }, 1000);
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
  
  // 判断当前奖品是否为魔法小礼袋
  const isMagicBag = selectedPrize.value && selectedPrize.value.name === "魔法小礼袋";
  
  if (isEnlarged.value) {
    // 如果已经放大，根据奖品类型决定如何关闭
    if (isMagicBag) {
      // 设置过渡锁
      isTransitioning.value = true;
      
      // 魔法小礼袋使用滑动效果
      isSliding.value = true;
      
      // 等待滑动动画完成后再隐藏
      setTimeout(() => {
        showImageDisplay.value = false;
        
        // 重置状态
        setTimeout(() => {
          isEnlarged.value = false;
          isSliding.value = false;
          autoCloseSecondsLeft.value = 5; // 重置倒计时
          selectedPrize.value = null; // 完全清除选中的奖品，避免下次显示时再次从右侧滑入
          isTransitioning.value = false; // 释放过渡锁
        }, 100);
      }, 800); // 增加等待时间，让动画更完整
    } else {
      // 设置过渡锁
      isTransitioning.value = true;
      
      // 普通奖品直接淡出
      showImageDisplay.value = false;
      
      // 简单延迟后重置状态
      setTimeout(() => {
        isEnlarged.value = false;
        autoCloseSecondsLeft.value = 5;
        selectedPrize.value = null;
        isTransitioning.value = false; // 释放过渡锁
      }, 1000); // 增加等待时间以匹配CSS过渡时间
    }
  } else {
    // 设置过渡锁
    isTransitioning.value = true;
    
    // 如果没有放大，直接显示并放大
    showImageDisplay.value = true;
    
    // 给一个短暂延迟，让DOM渲染完成
    setTimeout(() => {
      isEnlarged.value = true; // 放大
      
      // 只有魔法小礼袋才启动倒计时和自动滑动
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

// 强制更新记录
function forceUpdateRecords(): void {
  console.log('当前记录:', prizeRecordsRaw.value);
}

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
</script>

<style scoped>
.lucky-container {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  height: 100%;
  width: 100%;
  overflow: visible;
}

/* 抽奖记录 */
.prize-records {
  position: absolute;
  top: 20px;
  left: 20px;
  width: 200px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  padding: 15px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.prize-records h3 {
  margin-top: 0;
  margin-bottom: 10px;
  color: #e17055;
  font-size: 18px;
  text-align: center;
  border-bottom: 2px solid #fab1a0;
  padding-bottom: 8px;
}

.records-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.record-item {
  display: flex;
  justify-content: space-between;
  font-size: 14px;
  padding: 5px 0;
  border-bottom: 1px dashed #dfe6e9;
}

.record-item:last-child {
  border-bottom: none;
}

/* 重置按钮 */
.reset-button {
  position: absolute;
  bottom: 20px;
  right: 20px;
  background: #ff7675;
  color: white;
  border-radius: 50px;
  padding: 10px 20px;
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  transition: all 0.3s ease;
  z-index: 20;
}

.reset-button:hover {
  background: #e17055;
  transform: translateY(-2px);
}

.reset-button svg {
  width: 20px;
  height: 20px;
}

/* 完成抽奖提示 */
.completion-tip {
  position: absolute;
  bottom: 80px;
  right: 20px;
  background: rgba(254, 211, 48, 0.9);
  border-radius: 8px;
  padding: 10px 15px;
  display: flex;
  align-items: center;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  z-index: 10;
  animation: bounce 1s ease infinite;
}

.completion-message {
  display: flex;
  align-items: center;
  gap: 8px;
}

.completion-message svg {
  color: #2ecc71;
}

@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5px); }
}

/* 图片展示区域 */
.image-display {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 50;
  opacity: 0;
  visibility: hidden;
  transition: opacity 1s ease, visibility 0s 1s; /* 重要：确保visibility在opacity完全消失后再变化 */
  will-change: opacity; /* 提示浏览器优化动画性能 */
  overflow: hidden; /* 确保内容不会溢出 */
  pointer-events: auto; /* 确保在过渡期间仍然可以捕获点击事件 */
}

.image-display.active {
  opacity: 1;
  visibility: visible;
  transition: opacity 1s ease, visibility 0s; /* 显示时立即改变visibility */
}

.prize-image {
  position: relative;
  width: 500px;
  height: 500px;
  transition: transform 0.8s cubic-bezier(0.34, 1.56, 0.64, 1), opacity 0.4s ease-in; /* 调整普通过渡速度 */
  cursor: pointer;
  transform-origin: center;
  transform: translateX(0) scale(0.8); /* 初始位置在中央，不是在屏幕右侧 */
  opacity: 0; /* 初始时不可见 */
  will-change: transform, opacity; /* 提示浏览器优化动画性能 */
  -webkit-backface-visibility: hidden; /* 防止Safari中的闪烁 */
  backface-visibility: hidden;
  -webkit-transform-style: preserve-3d; /* 在移动设备上提高性能 */
  transform-style: preserve-3d;
}

.prize-image.enlarged {
  transform: translateX(0) scale(1); /* 放大但保持在中央 */
  opacity: 1; /* 显示 */
}

.prize-image.sliding {
  transform: translateX(-120vw) scale(1); /* 向左滑出屏幕 */
  opacity: 0;
  transition: transform 15s cubic-bezier(0.22, 0.61, 0.36, 1), opacity 1s ease 0.5s; /* 增加滑动时间到15秒，使效果更慢 */
  pointer-events: none; /* 防止在滑动时被点击 */
}

/* 确保图片顺利显示，不要使用中间状态 */
.image-display.active .prize-image {
  transition: transform 0.8s cubic-bezier(0.34, 1.56, 0.64, 1), opacity 0.4s ease;
}

/* 为移动设备优化显示和动画 */
@media (max-width: 768px) {
  .prize-image {
    width: 90vw; /* 使用视口宽度的百分比 */
    height: auto; /* 高度自适应 */
    max-height: 80vh; /* 最大高度不超过视口高度的80% */
  }
  
  .prize-content img {
    width: 100%;
    height: auto;
    max-height: 70vh;
    object-fit: contain;
  }
}

/* 为特小屏幕设备优化动画 */
@media (max-width: 320px) {
  .prize-image {
    transform: translateX(0) scale(0.8); /* 初始位置在中央 */
  }
  
  .prize-image.sliding {
    transform: translateX(-150vw) scale(1); /* 小屏幕可能需要更大的移动距离 */
    transition: transform 15s cubic-bezier(0.22, 0.61, 0.36, 1), opacity 1s ease 0.5s; /* 确保小屏幕也使用相同的过渡时间 */
  }
}

.heart-background {
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: transparent;
  overflow: hidden;
}

.heart-shape {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #e84393;
  border-radius: 50%;
  animation: heartbeat 1.5s infinite ease-in-out;
}

.prize-content {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  z-index: 2;
  opacity: 0; /* 初始透明 */
  transition: opacity 0.3s ease 0.2s; /* 延迟显示内容，让容器先出现 */
}

.prize-image.enlarged .prize-content {
  opacity: 1; /* 放大时显示内容 */
}

.prize-content img {
  width: 500px;
  height: 500px;
  object-fit: contain;
  margin-bottom: 15px;
  filter: drop-shadow(0 5px 15px rgba(0, 0, 0, 0.3));
  animation: float 3s infinite ease-in-out;
}

.prize-name {
  color: white;
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  margin-bottom: 10px;
}

.tap-to-close {
  color: white;
  font-size: 14px;
  opacity: 0.8;
  margin-top: 20px;
  padding: 5px 15px;
  border: 1px solid rgba(255, 255, 255, 0.5);
  border-radius: 20px;
  animation: pulse 2s infinite;
}

.congratulation-text {
  position: absolute;
  top: 40px;
  left: 0;
  width: 100%;
  text-align: center;
  color: white;
  font-size: 28px;
  font-weight: bold;
  text-shadow: 0 2px 5px rgba(0, 0, 0, 0.5);
  z-index: 3;
  animation: slideDown 0.8s ease-out;
}

@keyframes pulse {
  0% { opacity: 0.6; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.05); }
  100% { opacity: 0.6; transform: scale(1); }
}

@keyframes slideDown {
  0% { transform: translateY(-20px); opacity: 0; }
  100% { transform: translateY(0); opacity: 1; }
}

@keyframes heartbeat {
  0% { transform: scale(1); }
  50% { transform: scale(1.1); }
  100% { transform: scale(1); }
}

/* 云朵装饰 */
.cloud {
  position: absolute;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 50%;
  filter: blur(4px);
}

.cloud-1 {
  width: 60px;
  height: 60px;
  top: 10%;
  left: 10%;
  animation: float 4s infinite ease-in-out;
}

.cloud-2 {
  width: 40px;
  height: 40px;
  top: 20%;
  right: 15%;
  animation: float 5s 1s infinite ease-in-out;
}

.cloud-3 {
  width: 35px;
  height: 35px;
  bottom: 25%;
  right: 20%;
  animation: float 3s 0.5s infinite ease-in-out;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

/* 星星装饰 */
.star {
  position: absolute;
  color: #ffeaa7;
  text-shadow: 0 0 10px #fdcb6e, 0 0 20px #fdcb6e;
  animation: twinkle 2s infinite ease-in-out;
}

.star-1 {
  top: 15%;
  right: 20%;
  font-size: 28px;
  animation-delay: 0.2s;
}

.star-2 {
  bottom: 20%;
  left: 15%;
  font-size: 22px;
  animation-delay: 0.5s;
}

.star-3 {
  top: 40%;
  left: 10%;
  font-size: 20px;
  animation-delay: 0.8s;
}

@keyframes twinkle {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(0.8); }
}

/* 工具提示 */
.tooltip {
  position: fixed;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%) translateY(20px);
  background-color: rgba(52, 73, 94, 0.9);
  color: #fff;
  padding: 10px 15px;
  border-radius: 6px;
  font-size: 14px;
  z-index: 100;
  opacity: 0;
  visibility: hidden;
  transition: all 0.3s ease;
  max-width: 90%;
  text-align: center;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
}

.tooltip.active {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0);
}

.tooltip:after {
  content: '';
  position: absolute;
  bottom: -10px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 10px 10px 0;
  border-style: solid;
  border-color: rgba(52, 73, 94, 0.9) transparent transparent;
}

/* 自动关闭计时器样式 */
.auto-close-indicator {
  position: absolute;
  top: 15px;
  right: 15px;
  width: 36px;
  height: 36px;
  z-index: 10;
}

.circular-timer {
  transform: rotate(-90deg);
  width: 36px;
  height: 36px;
}

.circle-bg {
  fill: none;
  stroke: rgba(255, 255, 255, 0.3);
  stroke-width: 2.8;
}

.circle {
  fill: none;
  stroke: white;
  stroke-width: 2.8;
  stroke-linecap: round;
  transition: stroke-dasharray 0.1s linear;
}

.timer-text {
  fill: white;
  font-size: 10px;
  text-anchor: middle;
  transform: rotate(90deg);
  transform-origin: center;
  font-family: Arial, sans-serif;
  font-weight: bold;
}
</style>
