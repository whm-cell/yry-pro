<template>
  <div class="lucky-container">
    <!-- 转盘部分 -->
    <LuckyWheel
      ref="myLucky"
      width="300px"
      height="300px"
      :prizes="prizes"
      :blocks="blocks"
      :buttons="buttons"
      @start="startCallback"
      @end="endCallback"
      :style="{transform: 'scale(2)'}"
      style="scale-150"
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
        :class="{ 'enlarged': isEnlarged }"
        @click="toggleImageSize"
      >
        <div class="heart-background">
          <div class="heart-shape"></div>
          <!-- 添加云朵装饰 -->
          <div class="cloud cloud-1"></div>
          <div class="cloud cloud-2"></div>
          <div class="cloud cloud-3"></div>
          <!-- 添加星星装饰 -->
          <div class="star star-1">★</div>
          <div class="star star-2">★</div>
          <div class="star star-3">✦</div>
          <!-- 添加祝贺文字 -->
          <div class="congratulation-text">恭喜抽中</div>
        </div>
        <div class="prize-content">
          <img :src="selectedPrize.imgSrc" :alt="selectedPrize.name">
          <div class="prize-name">{{ selectedPrize.name }}</div>
          <div class="tap-to-close">点击关闭</div>
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
    
    <!-- 抽奖控制面板 -->
    <div class="control-panel" :class="{ 'expanded': showControlPanel }">
      <div class="control-panel-header" @click="toggleControlPanel">
        <h3>抽奖设置</h3>
        <div class="toggle-icon">
          {{ showControlPanel ? '▲' : '▼' }}
        </div>
      </div>
      
      <div class="control-panel-content" v-show="showControlPanel">
        <div class="mode-selection">
          <h4>抽奖模式：</h4>
          <div class="mode-options">
            <div 
              class="mode-option" 
              :class="{ 'active': drawMode === 'single' }"
              @click="setDrawMode('single')"
              role="button"
              tabindex="0"
              aria-label="选择单次模式"
            >
              <div class="mode-icon">🎯</div>
              <div class="mode-info">
                <div class="mode-name">单次模式</div>
                <div class="mode-desc">每个奖品最多抽中一次</div>
              </div>
              <div class="mode-check" v-if="drawMode === 'single'">✓</div>
            </div>
            
            <div 
              class="mode-option" 
              :class="{ 'active': drawMode === 'standard' }"
              @click="setDrawMode('standard')"
              role="button"
              tabindex="0"
              aria-label="选择标准模式"
            >
              <div class="mode-icon">🎲</div>
              <div class="mode-info">
                <div class="mode-name">标准模式</div>
                <div class="mode-desc">每个奖品最多抽中两次</div>
              </div>
              <div class="mode-check" v-if="drawMode === 'standard'">✓</div>
            </div>
            
            <div 
              class="mode-option" 
              :class="{ 'active': drawMode === 'sequence' }"
              @click="setDrawMode('sequence')"
              role="button"
              tabindex="0"
              aria-label="选择顺序模式"
            >
              <div class="mode-icon">📋</div>
              <div class="mode-info">
                <div class="mode-name">顺序模式</div>
                <div class="mode-desc">先抽完所有奖品再抽谢谢惠顾</div>
              </div>
              <div class="mode-check" v-if="drawMode === 'sequence'">✓</div>
            </div>
          </div>
        </div>
        
        <div class="mode-description">
          <div class="mode-detail-header">模式说明：</div>
          <div class="mode-detail-content">
            <div v-if="drawMode === 'single'">
              <strong>单次模式</strong>：每个普通奖品最多只能抽中一次，抽完后只能抽到"谢谢惠顾"。适合每人限抽一次的活动。
            </div>
            <div v-if="drawMode === 'standard'">
              <strong>标准模式</strong>：每个普通奖品最多抽中两次，超过次数的奖品将不再出现，转为抽取"谢谢惠顾"。
            </div>
            <div v-if="drawMode === 'sequence'">
              <strong>顺序模式</strong>：会先抽完所有普通奖品，每种最多抽中两次，所有奖品抽完后才会出现"谢谢惠顾"。
            </div>
          </div>
        </div>
        
        <div class="control-item">
          <label for="lock-toggle">抽完后锁定：</label>
          <div class="toggle-switch">
            <input 
              type="checkbox" 
              id="lock-toggle" 
              :checked="lockAfterComplete" 
              @change="toggleLockAfterComplete"
            />
            <label for="lock-toggle" class="toggle-label">
              <span class="toggle-inner"></span>
              <span class="toggle-switch-text">{{ lockAfterComplete ? '已开启' : '已关闭' }}</span>
            </label>
          </div>
          <div class="help-text">{{ lockAfterComplete ? '抽完所有奖品后将锁定转盘' : '抽完后仍可继续抽取谢谢惠顾' }}</div>
        </div>
      </div>
    </div>
    
    <!-- 完成抽奖提示 -->
    <div class="completion-tip" v-if="isCompleted && lockAfterComplete">
      <div class="completion-message">
        <svg viewBox="0 0 24 24" width="24" height="24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
        <span>抽奖已完成，点击重置按钮可重新开始</span>
      </div>
    </div>
    
    <!-- 添加抽奖记录展示 -->
    <div class="prize-records" v-if="showRecords">
      <h3>抽奖记录</h3>
      <div class="records-list">
        <div v-for="(count, name) in prizeRecords" :key="name" class="record-item">
          <span>{{ name }}:</span>
          <span>{{ count }}次</span>
        </div>
      </div>
    </div>
    
    <!-- 增加悬浮提示 -->
    <div class="tooltip" :class="{ 'active': showTooltip }">
      {{ tooltipText }}
    </div>
  </div>
</template>

<script lang="ts">
// 直接导入图片
import applePng from './ct-converted.png'  // 使用@别名指向src目录
import catPng from './ct-converted.png'
import ballPng from './ct-converted.png'
import dogPng from './ct-converted.png'
import starPng from './ct-converted.png'
import crownPng from './ct-converted.png'
// 移除未使用的导入
// import dotsPng from './ct-converted.png'

// 导入抽奖逻辑管理器
import createLuckyWheel from '../utils/luckyWheelLogic';
import { defineComponent } from 'vue';

/**
 * 抽奖模式枚举
 * 与 luckyWheelLogic.ts 中的定义保持一致
 */
enum DrawMode {
  STANDARD = 'standard',
  SEQUENCE = 'sequence',
  SINGLE = 'single'
}

// 定义类型
interface PrizeInfo {
  name: string;
  imgSrc: string;
}

interface FontSetting {
  text: string;
  top: string;
  fontColor: string;
  fontSize: string;
  fontWeight?: string;
}

interface ImageSetting {
  src: string;
  width: string;
  top: string;
}

interface Prize {
  background: string;
  fonts: FontSetting[];
  imgs: ImageSetting[];
  prizeInfo: PrizeInfo;
}

type PrizeRecords = Record<string, number>;

// 转盘组件类型
interface LuckyWheelRef {
  play: () => void;
  stop: (index: number) => void;
}

// 转盘奖品类型
interface LuckyPrize {
  fonts: {
    text: string;
    [key: string]: any;
  }[];
  [key: string]: any;
}

export default defineComponent({
  data() {
    // 定义奖品数据
    const prizes: Prize[] = [
      { 
        background: '#badc58', 
        fonts: [
          { text: 'Apple', top: '35%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
          { text: '苹果', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
        ],
        imgs: [{ src: applePng, width: '40px', top: '10%' }],
        // 额外信息
        prizeInfo: {
          name: "Apple / 苹果",
          imgSrc: applePng
        }
      },
      { 
        background: '#ff9ff3', 
        fonts: [
          { text: 'Cat', top: '35%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
          { text: '猫咪', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
        ],
        imgs: [{ src: catPng, width: '40px', top: '10%' }],
        prizeInfo: {
          name: "Cat / 猫咪",
          imgSrc: catPng
        }
      },
      { 
        background: '#ffeaa7', 
        fonts: [
          { text: 'Ball', top: '35%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
          { text: '球', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
        ],
        imgs: [{ src: ballPng, width: '40px', top: '10%' }],
        prizeInfo: {
          name: "Ball / 球",
          imgSrc: ballPng
        }
      },
      { 
        background: '#74b9ff', 
        fonts: [
          { text: 'Dog', top: '35%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' },
          { text: '小狗', top: '55%', fontColor: '#2d3436', fontSize: '14px' }
        ],
        imgs: [{ src: dogPng, width: '40px', top: '10%' }],
        prizeInfo: {
          name: "Dog / 小狗",
          imgSrc: dogPng
        }
      },
      { 
        background: '#fab1a0', 
        fonts: [
          { text: '谢谢惠顾', top: '40%', fontColor: '#2d3436', fontSize: '16px', fontWeight: 'bold' }
        ],
        imgs: [{ src: starPng, width: '40px', top: '10%' }],
        prizeInfo: {
          name: "谢谢惠顾",
          imgSrc: starPng
        }
      }
    ];

    // 抽奖模式
    const drawMode = DrawMode.STANDARD;
    // 是否在抽完后锁定
    const lockAfterComplete = false;

    // 创建抽奖管理器并使用类型断言
    const wheelManager = createLuckyWheel(prizes) as any;
    
    // 设置抽奖模式和锁定状态
    wheelManager.setDrawMode?.(drawMode);
    wheelManager.setLockAfterComplete?.(lockAfterComplete);

    return {
      // 选中的奖品
      selectedPrize: null as PrizeInfo | null,
      isEnlarged: false, // 控制图片大小状态
      showImageDisplay: false, // 新属性控制显示状态
      
      // 抽奖管理器
      wheelManager,
      
      // 抽奖模式
      drawMode,
      // 是否抽完后锁定
      lockAfterComplete,
      
      // 展示抽奖记录
      showRecords: true, // 是否显示抽奖记录
      
      // 边框设计
      blocks: [
        { padding: '15px', background: 'linear-gradient(to right, #ff7979, #ffbe76)' },
        { padding: '2px', background: '#ffffff' }
      ],
      
      // 奖品配置
      prizes,
      
      // 中心按钮
      buttons: [{
        radius: '35%',
        background: '#ff7675',
        pointer: true,
        fonts: [
          { 
            text: '转一转', 
            top: '35%',
            fontColor: '#fff',
            fontSize: '18px',
            fontWeight: 'bold'
          }
        ],
        imgs: [
          { src: crownPng, width: '25px', top: '10%' }
        ]
      }],
      // 是否显示控制面板
      showControlPanel: true,
      // 提示工具
      showTooltip: false,
      tooltipText: '',
      tooltipTimer: null as any,
    }
  },
  computed: {
    // 获取抽奖记录
    prizeRecords(): PrizeRecords {
      return this.wheelManager.getPrizeRecords();
    },
    
    // 判断是否所有奖品都已抽中一次
    allPrizesDrawnOnce(): boolean {
      return this.wheelManager.allPrizesDrawnOnce;
    },
    
    // 判断是否已完成抽奖
    isCompleted(): boolean {
      return (this.wheelManager as any).isCompleted?.() || false;
    }
  },
  methods: {
    // 显示工具提示
    showTip(text: string, duration: number = 2000): void {
      this.tooltipText = text;
      this.showTooltip = true;
      
      // 清除之前的定时器
      if (this.tooltipTimer) {
        clearTimeout(this.tooltipTimer);
      }
      
      // 设置自动关闭
      this.tooltipTimer = setTimeout(() => {
        this.showTooltip = false;
      }, duration);
    },
    
    // 设置抽奖模式
    setDrawMode(mode: string): void {
      // 类型安全：确保传入的模式是有效的DrawMode值
      if (mode !== 'standard' && mode !== 'sequence' && mode !== 'single') {
        console.error('无效的抽奖模式:', mode);
        return;
      }
      
      this.drawMode = mode as DrawMode;
      
      if (mode === 'single') {
        // 单次模式：设置最大抽奖次数为1
        (this.wheelManager as any).setDrawMode?.(DrawMode.STANDARD);
        (this.wheelManager as any).setMaxDraws?.(1);
        this.showTip('已切换到单次模式: 每个奖品最多抽中一次');
      } else if (mode === 'standard') {
        // 标准模式
        (this.wheelManager as any).setDrawMode?.(mode as DrawMode);
        (this.wheelManager as any).setMaxDraws?.(2); // 恢复默认值
        this.showTip('已切换到标准模式: 每个奖品最多抽中两次');
      } else {
        // 顺序模式
        (this.wheelManager as any).setDrawMode?.(mode as DrawMode);
        (this.wheelManager as any).setMaxDraws?.(2); // 恢复默认值
        this.showTip('已切换到顺序模式: 先抽完所有奖品再抽谢谢惠顾');
      }
    },
    
    // 切换抽完后是否锁定
    toggleLockAfterComplete(): void {
      this.lockAfterComplete = !this.lockAfterComplete;
      (this.wheelManager as any).setLockAfterComplete?.(this.lockAfterComplete);
      
      if (this.lockAfterComplete) {
        this.showTip('已开启抽完锁定: 抽完所有奖品后将锁定转盘');
      } else {
        this.showTip('已关闭抽完锁定: 抽完后仍可继续抽取谢谢惠顾');
      }
    },
    
    startCallback(): void {
      // 如果抽奖已完成并且锁定，显示提示而不启动转盘
      if ((this.wheelManager as any).isCompleted?.() && this.lockAfterComplete) {
        alert("抽奖已完成，请点击重置按钮重新开始");
        return;
      }
      
      // 只有在图片没有显示时才允许开始转盘
      if (!this.showImageDisplay) {
        (this.$refs.myLucky as unknown as LuckyWheelRef).play();
        
        // 根据规则选择抽奖结果
        setTimeout(() => {
          const selectedIndex = this.wheelManager.getNextPrizeIndex();
          (this.$refs.myLucky as unknown as LuckyWheelRef).stop(selectedIndex);
        }, 3000);
      }
    },
    
    endCallback(prize: LuckyPrize): void {
      // 获取中奖索引
      const prizeIndex = this.prizes.findIndex((p: Prize) => 
        p.fonts[0].text === prize.fonts[0].text);
      
      if (prizeIndex !== -1) {
        // 更新抽奖记录
        const result = this.wheelManager.updatePrizeRecord(prizeIndex);
        
        if (result) {
          // 设置选中的奖品显示
          this.selectedPrize = this.prizes[prizeIndex].prizeInfo;
          this.isEnlarged = true; // 初始状态为放大
          this.showImageDisplay = true; // 显示图片
          
          // 显示抽奖结果提示
          const isPrizeThanks = this.prizes[prizeIndex].prizeInfo.name === "谢谢惠顾";
          const count = this.wheelManager.getPrizeRecords()[this.prizes[prizeIndex].prizeInfo.name];
          
          if (isPrizeThanks) {
            this.showTip('本次抽中: 谢谢惠顾', 1500);
          } else {
            this.showTip(`恭喜！抽中 ${this.prizes[prizeIndex].prizeInfo.name} (第${count}次)`, 1500);
          }
          
          console.log('抽奖记录:', this.wheelManager.getPrizeRecords());
          console.log('是否所有奖品都至少抽中一次:', this.wheelManager.allPrizesDrawnOnce);
          console.log('抽奖是否已完成:', (this.wheelManager as any).isCompleted?.());
          
          // 如果抽奖已完成并且锁定，显示提示
          if ((this.wheelManager as any).isCompleted?.() && this.lockAfterComplete) {
            setTimeout(() => {
              this.showTip("所有奖品已抽完，点击重置按钮重新开始", 5000);
            }, 2000);
          }
        }
      }
    },
    
    // 点击切换图片显示
    toggleImageSize(): void {
      if (this.isEnlarged) {
        // 如果已经放大，隐藏图片展示
        this.showImageDisplay = false;
        // 添加小延迟重置属性
        setTimeout(() => {
          this.isEnlarged = false;
        }, 300); // 匹配过渡动画持续时间
      } else {
        // 如果没有放大，显示并放大
        this.showImageDisplay = true;
        this.isEnlarged = true;
      }
    },
    
    // 重置抽奖记录
    resetRecords(): void {
      this.wheelManager.resetRecords();
      this.showTip('抽奖记录已重置，可以重新开始抽奖！', 3000);
    },
    // 切换控制面板显示
    toggleControlPanel(): void {
      this.showControlPanel = !this.showControlPanel;
    },
  }
});
</script>

<style scoped>
.lucky-container {
  /* position: relative; */
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

/* 抽奖控制面板样式 */
.control-panel {
  position: absolute;
  top: 20px;
  right: 20px;
  width: 350px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  z-index: 10;
  transition: all 0.3s ease;
  overflow: hidden;
}

.control-panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  cursor: pointer;
  background-color: rgba(255, 255, 255, 0.95);
  border-bottom: 2px solid #fab1a0;
}

.control-panel-header:hover {
  background-color: #fff7f7;
}

.control-panel-header h3 {
  margin: 0;
  color: #e17055;
  font-size: 18px;
}

.toggle-icon {
  color: #e17055;
  font-size: 16px;
  transition: transform 0.3s ease;
}

.control-panel.expanded .toggle-icon {
  transform: rotate(180deg);
}

.control-panel-content {
  padding: 15px;
  max-height: 500px;
  overflow-y: auto;
  transition: max-height 0.3s ease;
}

.control-panel h4 {
  margin: 10px 0;
  color: #2d3436;
  font-size: 16px;
}

/* 抽奖模式选择 */
.mode-selection {
  margin-bottom: 15px;
}

.mode-options {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.mode-option {
  flex: 1;
  min-width: 100px;
  background: #f5f6fa;
  border-radius: 8px;
  padding: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  border: 2px solid transparent;
  position: relative;
}

.mode-option:hover {
  background: #dfe6e9;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.mode-option.active {
  background: #e6fcff;
  border-color: #55efc4;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.mode-check {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 20px;
  height: 20px;
  background: #55efc4;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 14px;
}

.mode-icon {
  font-size: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 30px;
}

.mode-info {
  display: flex;
  flex-direction: column;
}

.mode-name {
  font-weight: bold;
  color: #2d3436;
}

.mode-desc {
  font-size: 12px;
  color: #636e72;
}

/* 模式详细说明 */
.mode-description {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 15px;
  border-left: 3px solid #55efc4;
}

.mode-detail-header {
  font-weight: bold;
  margin-bottom: 5px;
  color: #2d3436;
}

.mode-detail-content {
  font-size: 14px;
  line-height: 1.5;
  color: #636e72;
}

/* 控制项 */
.control-item {
  display: flex;
  flex-direction: column;
  gap: 5px;
  margin-bottom: 15px;
}

.control-item label {
  font-weight: bold;
  color: #2d3436;
}

/* 开关样式 */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 30px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-label {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  border-radius: 34px;
  cursor: pointer;
  transition: .4s;
  display: flex;
  align-items: center;
  padding-left: 8px;
  padding-right: 8px;
  justify-content: space-between;
}

.toggle-inner {
  position: absolute;
  height: 24px;
  width: 24px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: .4s;
}

.toggle-switch-text {
  font-size: 10px;
  color: white;
  position: relative;
  z-index: 1;
  margin-left: 20px;
}

input:checked + .toggle-label {
  background-color: #55efc4;
}

input:checked + .toggle-label .toggle-inner {
  transform: translateX(30px);
}

.help-text {
  font-size: 12px;
  color: #636e72;
  margin-top: 5px;
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
  transition: all 0.3s ease;
}

.image-display.active {
  opacity: 1;
  visibility: visible;
}

.prize-image {
  position: relative;
  width: 300px;
  height: 300px;
  transition: all 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  cursor: pointer;
  transform-origin: center;
  transform: scale(0.8);
}

.prize-image.enlarged {
  transform: scale(1);
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
}

.prize-content img {
  width: 120px;
  height: 120px;
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
</style>
