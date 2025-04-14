<template>
  <div class="demo-container">
    <h1>抽奖转盘演示</h1>
    
    <div class="wheel-container">
      <LuckyWheel ref="luckyWheel" />
    </div>
    
    <div class="controls">
      <button @click="resetGame" class="control-button">重置抽奖</button>
      <button @click="startGame" class="control-button primary">开始抽奖</button>
    </div>
    
    <div class="description">
      <h2>使用说明</h2>
      <p>1. 点击"开始抽奖"按钮或直接点击转盘开始抽奖</p>
      <p>2. 每个普通奖品最多抽中两次</p>
      <p>3. 所有奖品都抽中一次后，优先抽取未达到两次的奖品</p>
      <p>4. 所有普通奖品都抽中两次后，只能抽中"谢谢惠顾"</p>
      <p>5. 点击"重置抽奖"可以清空抽奖记录，重新开始</p>
    </div>
    
    <div class="code-example">
      <h2>如何在其他组件中使用抽奖逻辑</h2>
      <pre><code>
import { createLuckyWheel, Prize } from '../utils/luckyWheelLogic';

// 创建抽奖管理器
const wheelManager = createLuckyWheel(prizes);

// 获取下一个奖品索引
const nextPrizeIndex = wheelManager.getNextPrizeIndex();

// 更新抽奖记录
wheelManager.updatePrizeRecord(prizeIndex);

// 重置抽奖记录
wheelManager.resetRecords();

// 获取当前抽奖记录
const records = wheelManager.getPrizeRecords();
      </code></pre>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import LuckyWheel from '../components/LuckyWheel.vue';

interface LuckyWheelInstance {
  startCallback: () => void;
  resetRecords: () => void;
}

export default defineComponent({
  components: {
    LuckyWheel
  },
  methods: {
    // 开始抽奖
    startGame(): void {
      // 通过引用调用组件的方法
      (this.$refs.luckyWheel as LuckyWheelInstance).startCallback();
    },
    
    // 重置抽奖
    resetGame(): void {
      // 通过引用调用组件的方法
      (this.$refs.luckyWheel as LuckyWheelInstance).resetRecords();
      // 显示提示
      alert('抽奖记录已重置');
    }
  }
});
</script>

<style scoped>
.demo-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family: 'Arial', sans-serif;
}

h1 {
  text-align: center;
  color: #ff6b81;
  margin-bottom: 30px;
}

.wheel-container {
  display: flex;
  justify-content: center;
  margin-bottom: 30px;
  position: relative;
  min-height: 600px;
}

.controls {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 40px;
}

.control-button {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: #f1f2f6;
  color: #333;
}

.control-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.control-button.primary {
  background-color: #ff6b81;
  color: white;
}

.description, .code-example {
  background-color: white;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.05);
}

h2 {
  color: #ff6b81;
  margin-top: 0;
  margin-bottom: 15px;
}

pre {
  background-color: #f8f9fa;
  padding: 15px;
  border-radius: 8px;
  overflow-x: auto;
}

code {
  font-family: 'Courier New', monospace;
  font-size: 14px;
  color: #333;
}
</style> 