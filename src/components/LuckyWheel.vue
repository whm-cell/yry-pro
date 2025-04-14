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
        </div>
        <div class="prize-content">
          <img :src="selectedPrize.imgSrc" :alt="selectedPrize.name">
          <div class="prize-name">{{ selectedPrize.name }}</div>
        </div>
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
  </div>
</template>

<script>
// 直接导入图片
import applePng from './ct-converted.png'  // 使用@别名指向src目录
import catPng from './ct-converted.png'
import ballPng from './ct-converted.png'
import dogPng from './ct-converted.png'
import starPng from './ct-converted.png'
import crownPng from './ct-converted.png'
import dotsPng from './ct-converted.png'

export default {
  data() {
    return {
      // 选中的奖品
      selectedPrize: null,
      isEnlarged: false, // 控制图片大小状态
      showImageDisplay: false, // 新属性控制显示状态
      
      // 添加抽奖记录跟踪
      prizeRecords: {
        "Apple / 苹果": 0,
        "Cat / 猫咪": 0,
        "Ball / 球": 0,
        "Dog / 小狗": 0,
        "谢谢惠顾": 0
      },
      showRecords: true, // 是否显示抽奖记录
      allPrizesDrawnOnce: false, // 是否所有奖品都至少抽中一次
      
      // 边框设计
      blocks: [
        { padding: '15px', background: 'linear-gradient(to right, #ff7979, #ffbe76)' },
        { padding: '2px', background: '#ffffff' }
      ],
      
      // 五个格子
      prizes: [
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
      ],
      
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
      }]
    }
  },
  methods: {
    startCallback() {
      // 只有在图片没有显示时才允许开始转盘
      if (!this.showImageDisplay) {
        this.$refs.myLucky.play()
        
        // 根据规则选择抽奖结果
        setTimeout(() => {
          const selectedIndex = this.getNextPrizeIndex();
          this.$refs.myLucky.stop(selectedIndex);
        }, 3000)
      }
    },
    
    // 获取下一个奖品索引
    getNextPrizeIndex() {
      // 检查是否所有普通奖品都至少抽中一次
      this.checkAllPrizesDrawnOnce();
      
      // 如果所有奖品都至少抽中一次，并且所有普通奖品都已经抽中2次，则返回"谢谢惠顾"的索引
      if (this.allPrizesDrawnOnce && this.areAllPrizesDrawnTwice()) {
        return 4; // "谢谢惠顾"的索引
      }
      
      // 获取可选的奖品索引
      const availablePrizes = this.getAvailablePrizeIndices();
      
      // 如果已经所有奖品都抽过一次，那么随机选择一个未达到2次的奖品
      if (this.allPrizesDrawnOnce) {
        if (availablePrizes.length > 0) {
          const randomIndex = Math.floor(Math.random() * availablePrizes.length);
          return availablePrizes[randomIndex];
        } else {
          return 4; // 如果没有可选奖品，返回"谢谢惠顾"
        }
      }
      
      // 如果还有未抽中过的奖品，优先选择它们
      const undrawnPrizes = this.getUndrawnPrizeIndices();
      if (undrawnPrizes.length > 0) {
        const randomIndex = Math.floor(Math.random() * undrawnPrizes.length);
        return undrawnPrizes[randomIndex];
      }
      
      // 如果所有奖品都抽过，随机选择一个未达到上限的奖品
      const randomIndex = Math.floor(Math.random() * availablePrizes.length);
      return availablePrizes[randomIndex];
    },
    
    // 获取未抽中过的奖品索引
    getUndrawnPrizeIndices() {
      const undrawnIndices = [];
      for (let i = 0; i < 4; i++) { // 只检查前4个普通奖品
        const prizeName = this.prizes[i].prizeInfo.name;
        if (this.prizeRecords[prizeName] === 0) {
          undrawnIndices.push(i);
        }
      }
      return undrawnIndices;
    },
    
    // 获取可选的奖品索引（次数小于2的奖品）
    getAvailablePrizeIndices() {
      const availableIndices = [];
      for (let i = 0; i < 4; i++) { // 只检查前4个普通奖品
        const prizeName = this.prizes[i].prizeInfo.name;
        if (this.prizeRecords[prizeName] < 2) {
          availableIndices.push(i);
        }
      }
      return availableIndices;
    },
    
    // 检查是否所有普通奖品都至少抽中一次
    checkAllPrizesDrawnOnce() {
      const prizeNames = Object.keys(this.prizeRecords).filter(name => name !== "谢谢惠顾");
      this.allPrizesDrawnOnce = prizeNames.every(name => this.prizeRecords[name] > 0);
    },
    
    // 检查是否所有普通奖品都已经抽中两次
    areAllPrizesDrawnTwice() {
      const prizeNames = Object.keys(this.prizeRecords).filter(name => name !== "谢谢惠顾");
      return prizeNames.every(name => this.prizeRecords[name] >= 2);
    },
    
    endCallback(prize) {
      // 设置选中的奖品
      const prizeIndex = this.prizes.findIndex(p => 
        p.fonts[0].text === prize.fonts[0].text);
      
      if (prizeIndex !== -1) {
        // 更新抽奖记录
        const prizeName = this.prizes[prizeIndex].prizeInfo.name;
        this.prizeRecords[prizeName]++;
        
        // 设置选中的奖品显示
        this.selectedPrize = this.prizes[prizeIndex].prizeInfo;
        this.isEnlarged = true; // 初始状态为放大
        this.showImageDisplay = true; // 显示图片
        
        // 检查是否所有奖品都至少抽中一次
        this.checkAllPrizesDrawnOnce();
        
        console.log('抽奖记录:', this.prizeRecords);
        console.log('是否所有奖品都至少抽中一次:', this.allPrizesDrawnOnce);
      }
    },
    
    // 点击切换图片显示
    toggleImageSize() {
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
    resetRecords() {
      for (const key in this.prizeRecords) {
        this.prizeRecords[key] = 0;
      }
      this.allPrizesDrawnOnce = false;
    }
  }
}
</script>

<style scoped>
.lucky-container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-items: center;
  gap: 30px;
  margin: 20px;
  padding: 20px;
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 20px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  position: relative; /* 添加相对定位 */
}

.image-display {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  position: absolute; /* 使用绝对定位进行覆盖 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 5; /* 默认较低的z-index */
  opacity: 0; /* 默认隐藏 */
  pointer-events: none; /* 默认不捕获点击 */
  transition: all 0.3s ease;
}

.image-display.active {
  opacity: 1;
  pointer-events: all; /* 可见时捕获点击 */
  z-index: 20; /* 显示时更高的z-index */
}

.prize-image {
  position: relative;
  cursor: pointer;
  background-color: transparent;
  border-radius: 15px;
  padding: 10px;
  box-shadow: none;
  transition: all 0.3s ease;
  max-width: 300px; /* 增加一点空间给心形背景 */
  text-align: center;
  z-index: 1;
}

.heart-background {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  z-index: -1;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: visible;
}

.heart-shape {
  position: relative;
  width: 100%;
  height: 90%;
  background-color: #FFB6C1; /* 浅粉色 */
  transform: scale(1.1);
  border-radius: 50%;
  border: 4px dashed #FF69B4; /* 深粉色虚线边框 */
  box-shadow: 0 0 15px rgba(255, 182, 193, 0.6);
}

.heart-shape:before,
.heart-shape:after {
  content: "";
  position: absolute;
  width: 70%;
  height: 70%;
  background-color: #FFB6C1; /* 浅粉色 */
  border-radius: 50%;
  border: 4px dashed #FF69B4; /* 深粉色虚线边框 */
}

.heart-shape:before {
  top: -35%;
  left: 15%;
}

.heart-shape:after {
  top: -35%;
  right: 15%;
  background: linear-gradient(to right, #ff9a9e, #fad0c4, #a18cd1, #fbc2eb);
  border: none;
  opacity: 0.3;
  mix-blend-mode: screen;
}

/* 添加小云朵装饰 */
.heart-background .cloud {
  position: absolute;
  background-color: white;
  border-radius: 50%;
  box-shadow: 0 0 10px rgba(255,255,255,0.8);
  z-index: 1;
}

.heart-background .cloud-1 {
  width: 40px;
  height: 40px;
  top: -5%;
  left: 20%;
}

.heart-background .cloud-2 {
  width: 30px;
  height: 30px;
  top: -2%;
  left: 30%;
}

.heart-background .cloud-3 {
  width: 25px;
  height: 25px;
  top: 0%;
  left: 40%;
}

/* 星星装饰 */
.heart-background .star {
  position: absolute;
  font-size: 25px;
  color: #FFD700; /* 金黄色 */
  text-shadow: 0 0 5px #FF69B4;
  animation: twinkle 2s infinite alternate;
  z-index: 2;
}

.heart-background .star-1 {
  top: 5%;
  left: 15%;
  font-size: 30px;
}

.heart-background .star-2 {
  bottom: 10%;
  right: 15%;
  font-size: 25px;
  animation-delay: 0.5s;
}

.heart-background .star-3 {
  top: 15%;
  right: 20%;
  font-size: 20px;
  animation-delay: 1s;
}

@keyframes twinkle {
  from { opacity: 0.3; transform: scale(0.8) rotate(0deg); }
  to { opacity: 1; transform: scale(1.1) rotate(15deg); }
}

.prize-content {
  position: relative;
  z-index: 2;
  padding: 15px;
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0% { transform: translateY(0px); }
  50% { transform: translateY(-5px); }
  100% { transform: translateY(0px); }
}

.prize-image.enlarged {
  transform: scale(1.2);
  width: 95%;
  max-width: 350px; /* 放大后的最大宽度 */
  z-index: 30;
}

.prize-image.enlarged .heart-shape {
  box-shadow: 0 0 20px rgba(255, 182, 193, 0.7); /* 粉色阴影 */
  animation: heartbeat 1.5s infinite alternate; /* 心跳动画 */
}

@keyframes heartbeat {
  from { transform: scale(1.1); }
  to { transform: scale(1.2); }
}

.prize-image img {
  width: 75%;
  height: auto;
  transition: all 0.3s ease;
  border-radius: 12px;
  padding: 8px;
  background-color: white;
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
  border: 3px solid #FFC0CB; /* 粉色边框 */
}

.prize-name {
  margin-top: 15px;
  font-size: 18px;
  color: #ff6b81;
  font-weight: bold;
  text-shadow: 1px 1px 2px rgba(255,255,255,0.8);
  font-family: 'KidFont', sans-serif;
  letter-spacing: 1px;
  background: linear-gradient(45deg, #ff9a9e 0%, #fad0c4 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  filter: drop-shadow(1px 1px 1px rgba(255,255,255,0.8));
}

/* 提示文本 */
.prize-image::after {
  content: '';
  position: absolute;
  bottom: -25px;
  left: 0;
  right: 0;
  font-size: 14px;
  color: #ff6b81;
  opacity: 0.8;
  font-family: 'KidFont', sans-serif;
  animation: bounce 1s infinite alternate;
}

@keyframes bounce {
  from { transform: translateY(0); }
  to { transform: translateY(-5px); }
}

@font-face {
  font-family: 'KidFont';
  src: url('./assets/kid-font.woff2') format('woff2');
}

/* 添加响应式布局 */
@media (max-width: 768px) {
  .lucky-container {
    flex-direction: column;
  }
  
  .prize-image.enlarged {
    width: 90%; /* 在移动端稍微大一点 */
    max-width: 90%;
  }
}

/* 添加抽奖记录样式 */
.prize-records {
  position: absolute;
  top: 10px;
  right: 10px;
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 10px;
  padding: 10px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 4;
  max-width: 150px;
}

.prize-records h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  color: #ff6b81;
  text-align: center;
}

.records-list {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.record-item {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #555;
}

.record-item span:first-child {
  margin-right: 10px;
  font-weight: bold;
}
</style>
