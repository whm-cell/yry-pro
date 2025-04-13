<template>
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
</template>

<script>
// 引入图片资源
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
      // 可爱的边框设计
      // blocks: [
      //   { padding: '15px', background: 'linear-gradient(to right, #ff7979, #ffbe76)' },
      //   { 
      //     padding: '2px', 
      //     background: '#ffffff', 
      //     imgs: [{ src: dotsPng, width: '100%', height: '100%' }] 
      //   }
      // ],
      // 五个格子：四个英语单词和一个谢谢惠顾
      prizes: [
        { 
          background: '#badc58', 
          fonts: [
            { text: 'Apple', top: '35%', fontColor: '#2d3436', fontSize: '20px', fontWeight: 'bold' },
            { text: '苹果', top: '55%', fontColor: '#2d3436', fontSize: '16px' }
          ],
          imgs: [{ src: applePng, width: '150px', top: '10%' }]
        },
        { 
          background: '#ff9ff3', 
          fonts: [
            { text: 'Cat', top: '35%', fontColor: '#2d3436', fontSize: '20px', fontWeight: 'bold' },
            { text: '猫咪', top: '55%', fontColor: '#2d3436', fontSize: '16px' }
          ],
          imgs: [{ src: catPng, width: '150px', top: '10%' }]
        },
        { 
          background: '#ffeaa7', 
          fonts: [
            { text: 'Ball', top: '35%', fontColor: '#2d3436', fontSize: '20px', fontWeight: 'bold' },
            { text: '球', top: '55%', fontColor: '#2d3436', fontSize: '16px' }
          ],
          imgs: [{ src: ballPng, width: '150px', top: '10%' }]
        },
        { 
          background: '#74b9ff', 
          fonts: [
            { text: 'Dog', top: '35%', fontColor: '#2d3436', fontSize: '20px', fontWeight: 'bold' },
            { text: '小狗', top: '55%', fontColor: '#2d3436', fontSize: '16px' }
          ],
          imgs: [{ src: dogPng, width: '150px', top: '10%' }]
        },
        { 
          background: '#fab1a0', 
          fonts: [
            { text: '谢谢惠顾', top: '40%', fontColor: '#2d3436', fontSize: '20px', fontWeight: 'bold' }
          ],
          imgs: [{ src: starPng, width: '150px', top: '10%' }]
        }
      ],
      // 可爱的中心按钮
      buttons: [{
        radius: '35%',
        background: '#ff7675',
        pointer: true,
        fonts: [
          { 
            text: '转一转', 
            top: '35%',
            fontColor: '#fff',
            fontSize: '24px',
            fontWeight: 'bold'
          }
        ],
        imgs: [
          { src: crownPng, width: '35px', top: '10%' }
        ]
      }]
    }
  },
  methods: {
    startCallback() {
      this.$refs.myLucky.play()
      // 模拟抽奖
      setTimeout(() => {
        // 随机选择索引（0-4）
        const index = Math.floor(Math.random() * 5)
        this.$refs.myLucky.stop(index)
      }, 3000)
    },
    endCallback(prize) {
      if (prize.fonts && prize.fonts[0].text === '谢谢惠顾') {
        alert('谢谢惠顾，再接再厉！')
      } else {
        alert('恭喜你抽到了: ' + prize.fonts[0].text)
      }
    },
  }
}
</script>

<style scoped>
/* 添加一些额外的可爱样式 */
.lucky-wheel-container {
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 20px;
  filter: drop-shadow(0 0 10px rgba(0,0,0,0.2));
}

@font-face {
  font-family: 'KidFont';
  src: url('@/assets/kid-font.woff2') format('woff2');
}

/* 应用可爱字体 */
.lucky-wheel-text {
  font-family: 'KidFont', sans-serif;
}
</style>
