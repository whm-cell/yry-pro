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
        <img :src="selectedPrize.imgSrc" :alt="selectedPrize.name">
        <div class="prize-name">{{ selectedPrize.name }}</div>
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
        // 模拟抽奖
        setTimeout(() => {
          // 随机选择索引（0-4）
          const index = Math.floor(Math.random() * 5)
          this.$refs.myLucky.stop(index)
        }, 3000)
      }
    },
    endCallback(prize) {
      // 设置选中的奖品
      const prizeIndex = this.prizes.findIndex(p => 
        p.fonts[0].text === prize.fonts[0].text);
      
      if (prizeIndex !== -1) {
        this.selectedPrize = this.prizes[prizeIndex].prizeInfo;
        this.isEnlarged = true; // 初始状态为放大
        this.showImageDisplay = true; // 显示图片
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
  background-color: transparent; /* Changed from white to transparent */
  border-radius: 15px;
  padding: 10px;
  box-shadow: none; /* Remove shadow for better transparency effect */
  transition: all 0.3s ease;
  max-width: 150px;
  text-align: center;
}


.prize-image.enlarged {
  transform: scale(1.2); /* Slightly increase base scale */
  width: 95%; /* Increase from 80% to 95% */
  max-width: 95%; /* Increase from 80% to 95% */
  z-index: 30;
  box-shadow: none; /* Keep consistent with the transparent look */
}
.prize-image img {
  width: 100%;
  height: auto;
  transition: all 0.3s ease;
}

.prize-name {
  margin-top: 10px;
  font-size: 14px;
  color: #2d3436;
  font-weight: bold;
}

/* 提示文本 */
.prize-image::after {
  content: '点击可以隐藏/显示';
  position: absolute;
  bottom: -25px;
  left: 0;
  right: 0;
  font-size: 12px;
  color: #7f8c8d;
  opacity: 0.8;
}

@font-face {
  font-family: 'KidFont';
  src: url('./assets/kid-font.woff2') format('woff2');
}

.prize-name {
  font-family: 'KidFont', sans-serif;
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
</style>
