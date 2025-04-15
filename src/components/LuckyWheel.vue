<template>
  <div class="lucky-container">
    <!-- 转盘部分 -->
    <LuckyWheelCanvas
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
      :class="{ 'active': showImageDisplay, 'slide-out': isSlideOut }"
      @click.self="toggleImageSize"
    >
      <div 
        v-if="selectedPrize"
        class="prize-image" 
        :class="{ 'enlarged': isEnlarged, 'slide-out': isSlideOut }"
        @click="toggleImageSize"
      >
        <!-- 添加星星点缀装饰 -->
        <div class="slide-decorations">
          <div class="slide-star star-1" :class="{ 'active-decoration': isSlideOut }">⭐</div>
          <div class="slide-star star-2" :class="{ 'active-decoration': isSlideOut }">✨</div>
          <div class="slide-star star-3" :class="{ 'active-decoration': isSlideOut }">⭐</div>
          <div class="slide-star star-4" :class="{ 'active-decoration': isSlideOut }">✨</div>
          <div class="slide-star star-5" :class="{ 'active-decoration': isSlideOut }">⭐</div>
          <div class="slide-balloon balloon-1" :class="{ 'active-decoration': isSlideOut }">🎈</div>
          <div class="slide-balloon balloon-2" :class="{ 'active-decoration': isSlideOut }">🎈</div>
          <div class="slide-rainbow rainbow-1" :class="{ 'active-decoration': isSlideOut }">🌈</div>
          <div class="slide-confetti confetti-1" :class="{ 'active-decoration': isSlideOut }">🎊</div>
          <div class="slide-confetti confetti-2" :class="{ 'active-decoration': isSlideOut }">🎉</div>
          <!-- 新增点缀 -->
          <div class="slide-animal animal-1" :class="{ 'active-decoration': isSlideOut }">🦊</div>
          <div class="slide-animal animal-2" :class="{ 'active-decoration': isSlideOut }">🦁</div>
          <div class="slide-animal animal-3" :class="{ 'active-decoration': isSlideOut }">🐯</div>
          <div class="slide-dots dots-1" :class="{ 'active-decoration': isSlideOut }">🔴</div>
          <div class="slide-dots dots-2" :class="{ 'active-decoration': isSlideOut }">🟠</div>
          <div class="slide-dots dots-3" :class="{ 'active-decoration': isSlideOut }">🟡</div>
        </div>
       
        <div class="prize-content">
          <img  :src="selectedPrize.imgSrc" :alt="selectedPrize.name" 
         >
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
import { ref, reactive, computed, onMounted } from 'vue';
// 直接导入图片 - 使用占位图片路径
const applePng = '/images/placeholder.png';
const catPng = '/images/placeholder.png';
const ballPng = '/images/placeholder.png';
const dogPng = '/images/placeholder.png';
const starPng = '/images/placeholder.png';
const crownPng = '/images/placeholder.png';

// 导入设置钩子和类型
import { useWheelSettings, DrawMode } from '../utils/wheelSettings';

// 不需要此处的定义，因为LuckyWheel已经由插件注册全局组件
// const LuckyWheel = {
//   name: 'LuckyWheel'
// };

// 暴露组件API
defineExpose({
  play: () => {
    if (myLucky.value) {
      (myLucky.value as any).play();
    }
  },
  stop: (index: number) => {
    if (myLucky.value) {
      (myLucky.value as any).stop(index);
    }
  }
});

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
const isSlideOut = ref(false);

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
  // 重置记录
  for (const key in prizeRecordsRaw.value) {
    prizeRecordsRaw.value[key] = 0;
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

// 开始转动回调
function startCallback(): void {
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
      isSlideOut.value = false; // 确保初始状态不是滑出
      isEnlarged.value = false; // 先确保不是放大状态
      
      // 使用下一帧来确保状态更新
      requestAnimationFrame(() => {
        showImageDisplay.value = true; // 显示图片层
        
        // 确保图层显示后再设置放大状态
        setTimeout(() => {
          isEnlarged.value = true; // 放大图片
        }, 50);
      });
      
      // 显示抽奖结果提示
      const isPrizeThanks = prizes.value[prizeIndex].prizeInfo.name === "魔法小礼袋";
      const count = prizeRecordsRaw.value[prizes.value[prizeIndex].prizeInfo.name];
      
      if (isPrizeThanks) {
        showTip('本次抽中: 魔法小礼袋', 1500);
      } else {
        showTip(`恭喜！抽中 ${prizes.value[prizeIndex].prizeInfo.name} (第${count}次)`, 1500);
      }
      
      // 如果抽奖已完成并且锁定，显示提示
      if (isCompletedFlag.value && lockAfterComplete.value) {
        setTimeout(() => {
          showTip("所有奖品已抽完，点击重置按钮重新开始", 5000);
        }, 2000);
      }
      
      // 设置2秒后开始滑出动画
      setTimeout(() => {
        isSlideOut.value = true; // 触发滑出动画
        
        // 动画完全结束后再隐藏图层 (slideOutLeft动画为2秒)
        // 这里不再直接隐藏图层，而是通过CSS的延迟过渡来处理
        // 在CSS中我们设置了2秒后再淡出整个图层
        
        // 确保在动画完全结束后重置所有状态
        setTimeout(() => {
          // 使用requestAnimationFrame确保在下一帧渲染周期中执行
          requestAnimationFrame(() => {
            // 动画已经完成，可以重置状态
            isSlideOut.value = false;
            isEnlarged.value = false;
            
            // 最后一步，在下一帧完全隐藏图层
            requestAnimationFrame(() => {
              showImageDisplay.value = false;
            });
          });
        }, 2500); // 延迟比动画时间更长一些
      }, 2000);
    }
  }
}

// 点击切换图片显示
function toggleImageSize(): void {
  // 如果正在滑出动画中，不做任何操作
  if (isSlideOut.value) return;
  
  if (isEnlarged.value) {
    // 如果已经放大，渐变隐藏
    showImageDisplay.value = false;
    setTimeout(() => {
      isEnlarged.value = false;
    }, 500);
  } else {
    // 如果没有放大，显示并放大
    isEnlarged.value = true;
    showImageDisplay.value = true;
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
  pointer-events: none;
  transition: opacity 0.5s ease, visibility 0.5s, background-color 0.5s;
  /* 添加硬件加速 */
  will-change: opacity, visibility;
  transform: translateZ(0);
  backface-visibility: hidden;
}

.image-display.active {
  opacity: 1;
  visibility: visible;
  pointer-events: all;
}

.image-display.slide-out {
  background-color: transparent;
  pointer-events: none;
  /* 增加过渡延迟，等待子元素动画完成 */
  transition: background-color 0.5s ease, opacity 0.1s linear 2s, visibility 0.1s linear 2s;
}

.prize-image {
  position: relative;
  width: 500px;
  height: 500px;
  transition: all 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  cursor: pointer;
  transform-origin: center;
  transform: scale(0.8);
}

.prize-image.enlarged {
  transform: scale(1);
}

.prize-image.slide-out {
  transform: scale(0.8);
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
  will-change: transform;
  backface-visibility: hidden;
  transform: translateZ(0);
}

.prize-content img {
  width: 500px;
  height: 500px;
  object-fit: contain;
  margin-bottom: 15px;
  filter: drop-shadow(0 5px 15px rgba(0, 0, 0, 0.3));
  animation: float 3s infinite ease-in-out;
  will-change: transform;
  backface-visibility: hidden;
  transform: translateZ(0);
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

/* 添加滑出动画相关样式 */
.image-display.slide-out .prize-image {
  animation: slideOutLeft 2s cubic-bezier(0.25, 0.1, 0.25, 1.0) forwards;
  will-change: transform, opacity;
  pointer-events: none;
  opacity: 1 !important;
  visibility: visible !important;
  position: absolute;
  z-index: 100;
  /* 添加硬件加速属性 */
  transform: translateZ(0);
  backface-visibility: hidden;
}

@keyframes slideOutLeft {
  0% {
    transform: translateX(0) scale(1) rotate(0);
    opacity: 1;
  }
  90% {
    opacity: 1;
  }  
  100% {
    transform: translateX(-150vw) scale(0.8) rotate(-5deg);
    opacity: 0;
    visibility: hidden;
  }
}

/* 滑出时的星星和其他装饰点缀 */
.slide-decorations {
  position: absolute;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 10;
  will-change: contents;
}

.slide-star, 
.slide-balloon, 
.slide-rainbow, 
.slide-confetti,
.slide-animal,
.slide-dots {
  position: absolute;
  opacity: 0;
  visibility: hidden;
  transform: translateX(0);
  filter: drop-shadow(0 0 5px rgba(255, 255, 255, 0.7));
  /* 硬件加速 */
  will-change: transform, opacity;
  backface-visibility: hidden;
  transform: translateZ(0);
}

.active-decoration {
  opacity: 1 !important;
  visibility: visible !important;
  z-index: 11;
}

.slide-star {
  font-size: 28px;
}

.slide-star.active-decoration {
  animation: twinkleFade 1s infinite alternate, followImage 1.5s ease-in-out forwards;
}

.star-1 {
  top: 10%;
  left: 20%;
  animation-delay: 0.1s;
}

.star-2 {
  top: 20%;
  left: 70%;
  animation-delay: 0.3s;
  font-size: 32px;
}

.star-3 {
  top: 60%;
  left: 10%;
  animation-delay: 0.2s;
}

.star-4 {
  top: 75%;
  left: 60%;
  animation-delay: 0.4s;
}

.star-5 {
  top: 40%;
  left: 80%;
  animation-delay: 0.5s;
  font-size: 30px;
}

.slide-balloon {
  font-size: 36px;
}

.slide-balloon.active-decoration {
  animation: floatUp 3s ease-in-out infinite, followImage 1.5s ease-in-out forwards;
}

.balloon-1 {
  top: 5%;
  left: 85%;
  animation-delay: 0.2s;
}

.balloon-2 {
  top: 70%;
  left: 5%;
  animation-delay: 0.6s;
}

.slide-rainbow {
  font-size: 40px;
}

.slide-rainbow.active-decoration {
  animation: fadeInOut 2s ease-in-out infinite, followImage 1.5s ease-in-out forwards;
}

.rainbow-1 {
  top: 10%;
  left: 50%;
  transform: translateX(-50%);
}

.slide-confetti {
  font-size: 30px;
}

.slide-confetti.active-decoration {
  animation: spin 2s linear infinite, followImage 1.5s ease-in-out forwards;
}

.confetti-1 {
  top: 15%;
  left: 30%;
  animation-delay: 0.1s;
}

.confetti-2 {
  bottom: 15%;
  right: 30%;
  animation-delay: 0.4s;
}

/* 新增动物点缀 */
.slide-animal {
  font-size: 34px;
}

.slide-animal.active-decoration {
  animation: bounce 1.5s infinite alternate, followImage 1.5s ease-in-out forwards;
}

.animal-1 {
  top: 30%;
  left: 15%;
  animation-delay: 0.2s;
}

.animal-2 {
  bottom: 30%;
  left: 35%;
  animation-delay: 0.3s;
}

.animal-3 {
  top: 50%;
  right: 20%;
  animation-delay: 0.4s;
}

/* 新增彩色点点 */
.slide-dots {
  font-size: 24px;
}

.slide-dots.active-decoration {
  animation: pulseScale 1.2s infinite alternate, followImage 1.5s ease-in-out forwards;
}

.dots-1 {
  top: 25%;
  right: 40%;
  animation-delay: 0.1s;
}

.dots-2 {
  bottom: 40%;
  right: 25%;
  animation-delay: 0.3s;
}

.dots-3 {
  top: 65%;
  left: 45%;
  animation-delay: 0.5s;
}

@keyframes pulseScale {
  0% {
    transform: scale(0.8);
    opacity: 0.7;
  }
  100% {
    transform: scale(1.2);
    opacity: 1;
  }
}

@keyframes bounce {
  0% {
    transform: translateY(0);
  }
  100% {
    transform: translateY(-15px);
  }
}

@keyframes followImage {
  0% {
    transform: translateX(0);
    opacity: 1;
    visibility: visible;
  }
  90% {
    opacity: 1;
  }
  100% {
    transform: translateX(-150vw);
    opacity: 0;
    visibility: hidden;
  }
}

@keyframes twinkleFade {
  0% {
    opacity: 0.3;
    transform: scale(0.8);
  }
  100% {
    opacity: 1;
    transform: scale(1.1);
  }
}

@keyframes floatUp {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-15px);
  }
}

@keyframes fadeInOut {
  0%, 100% {
    opacity: 0.7;
  }
  50% {
    opacity: 1;
  }
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>

