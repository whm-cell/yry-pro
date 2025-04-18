<template>
  <div class="settings-container">
    <h2>转盘设置</h2>
    
    <div class="settings-group">
      <h3>转盘旋转时间</h3>
      <div class="slider-container">
        <input 
          type="range" 
          min="1000" 
          max="10000" 
          step="500"
          v-model.number="spinDuration"
          @input="updateSpinDuration"
        />
        <div class="slider-value">{{ (spinDuration / 1000).toFixed(1) }}秒</div>
      </div>
    </div>
    
    <div class="settings-group">
      <h3>高亮边框效果</h3>
      <div class="toggle-container">
        <label class="toggle">
          <input 
            type="checkbox" 
            v-model="enableHighlight"
            @change="updateEnableHighlight"
          />
          <span class="toggle-slider"></span>
        </label>
        <span class="toggle-label">{{ enableHighlight ? '已开启' : '已关闭' }}</span>
      </div>
    </div>
    
    <div class="settings-actions">
      <button class="btn reset-btn" @click="resetSettings">重置为默认设置</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useWheelSettings } from '../utils/wheelSettings';

// 获取转盘设置
const { settings, updateSpinDuration: setSpinDuration, updateEnableHighlight: setEnableHighlight, resetSettings } = useWheelSettings();

// 创建本地状态
const spinDuration = ref(settings.spinDuration);
const enableHighlight = ref(settings.enableHighlight);

// 更新转盘旋转时间
function updateSpinDuration() {
  setSpinDuration(spinDuration.value);
}

// 更新高亮边框效果设置
function updateEnableHighlight() {
  setEnableHighlight(enableHighlight.value);
}

// 监听全局设置变化
onMounted(() => {
  // 同步初始值
  spinDuration.value = settings.spinDuration;
  enableHighlight.value = settings.enableHighlight;
});
</script>

<style>
.settings-container {
  background-color: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 500px;
  margin: 0 auto;
}

.settings-container h2 {
  color: #e17055;
  font-size: 24px;
  margin-top: 0;
  margin-bottom: 20px;
  text-align: center;
  border-bottom: 2px solid #fab1a0;
  padding-bottom: 10px;
}

.settings-group {
  margin-bottom: 25px;
}

.settings-group h3 {
  font-size: 18px;
  color: #2d3436;
  margin-bottom: 12px;
}

.slider-container {
  display: flex;
  align-items: center;
  gap: 15px;
}

.slider-container input[type="range"] {
  flex: 1;
  -webkit-appearance: none;
  height: 8px;
  background: #dfe6e9;
  border-radius: 4px;
  outline: none;
}

.slider-container input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 20px;
  height: 20px;
  background: #e17055;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;
}

.slider-container input[type="range"]::-webkit-slider-thumb:hover {
  background: #d35400;
  transform: scale(1.1);
}

.slider-value {
  font-size: 16px;
  color: #2d3436;
  min-width: 50px;
  text-align: center;
  font-weight: bold;
}

.toggle-container {
  display: flex;
  align-items: center;
  gap: 15px;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 52px;
  height: 28px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.4s;
  border-radius: 34px;
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 20px;
  width: 20px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: 0.4s;
  border-radius: 50%;
}

input:checked + .toggle-slider {
  background-color: #e17055;
}

input:checked + .toggle-slider:before {
  transform: translateX(24px);
}

.toggle-label {
  font-size: 16px;
  color: #2d3436;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 30px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.3s ease;
}

.reset-btn {
  background-color: #dfe6e9;
  color: #2d3436;
}

.reset-btn:hover {
  background-color: #b2bec3;
}
</style> 