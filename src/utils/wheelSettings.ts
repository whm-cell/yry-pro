/**
 * 抽奖转盘设置状态管理
 * 使用Vue 3的响应式API实现全局状态管理
 */
import { reactive, ref, watch } from 'vue';

// 抽奖模式枚举
export enum DrawMode {
  STANDARD = 'standard',
  SEQUENCE = 'sequence',
  SINGLE = 'single'
}

// 定义设置类型
export interface WheelSettings {
  drawMode: DrawMode;
  lockAfterComplete: boolean;
  maxDraws: number;
  prizes: any[];
}

// 设置存储键名
const STORAGE_KEY = 'lucky-wheel-settings';

// 默认设置
const defaultSettings: WheelSettings = {
  drawMode: DrawMode.STANDARD,
  lockAfterComplete: false,
  maxDraws: 2,
  prizes: [] // 奖品配置将保持为空，由组件初始化时提供
};

// 从本地存储加载设置
const loadSettings = (): WheelSettings => {
  try {
    const savedSettings = localStorage.getItem(STORAGE_KEY);
    if (savedSettings) {
      return JSON.parse(savedSettings);
    }
  } catch (error) {
    console.error('加载设置失败:', error);
  }
  return defaultSettings;
};

// 初始化设置状态
const settings = reactive<WheelSettings>(loadSettings());

// 保存设置到本地存储
const saveSettings = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    console.error('保存设置失败:', error);
  }
};

// 监听设置变化并保存
watch(settings, () => {
  saveSettings();
}, { deep: true });

// 提供给组件使用的钩子
export function useWheelSettings() {
  // 更新设置
  const updateDrawMode = (mode: DrawMode) => {
    settings.drawMode = mode;
  };

  const updateLockAfterComplete = (lock: boolean) => {
    settings.lockAfterComplete = lock;
  };

  const updateMaxDraws = (max: number) => {
    settings.maxDraws = max;
  };

  const updatePrizes = (prizes: any[]) => {
    settings.prizes = prizes;
  };
  
  // 重置设置
  const resetSettings = () => {
    Object.assign(settings, defaultSettings);
  };

  // 设置初始化完成的标志
  const isInitialized = ref(false);

  return {
    settings,
    updateDrawMode,
    updateLockAfterComplete,
    updateMaxDraws,
    updatePrizes,
    resetSettings,
    isInitialized
  };
} 