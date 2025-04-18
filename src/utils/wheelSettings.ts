/**
 * 抽奖转盘设置状态管理
 * 使用Vue 3的响应式API实现全局状态管理
 */
import { reactive, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 抽奖模式枚举
export enum DrawMode {
  ORDERLY = 'orderly',   // 模式1：每个奖品都要抽一次，最大是1次，最后可以抽到Magic Bag
  RANDOM = 'random'      // 模式2：奖品和Magic Bag完全随机，抽到哪个是哪个，抽完后对应扇形变灰色
}

// 单词配置接口
export interface WordConfig {
  english: string;
  translation: string;
  bgColor: string;
  fontColor: string;
  imgSrc: string;
}

// 音效类型
export interface SoundSetting {
  type: 'preset' | 'uploaded';
  name: string;
  url: string;
}

// 定义设置类型
export interface WheelSettings {
  drawMode: DrawMode;
  lockAfterComplete: boolean;
  maxDraws: number;
  prizes: any[];
  prizeWords: WordConfig[]; // 添加单词配置
  sounds: {
    spin: SoundSetting;
    win: SoundSetting;
  };
}

// 设置存储键名
const STORAGE_KEY = 'lucky-wheel-settings';

// 默认设置
const defaultSettings: WheelSettings = {
  drawMode: DrawMode.ORDERLY,
  lockAfterComplete: false,
  maxDraws: 1,
  prizes: [], // 奖品配置将保持为空，由组件初始化时提供
  prizeWords: [], // 单词配置，初始为空
  sounds: {
    spin: {
      type: 'preset',
      name: '默认旋转音效',
      url: '/Users/coolm/softs/temp_files/sounds/cjyx_01.mp3'
    },
    win: {
      type: 'preset',
      name: '默认中奖音效',
      url: '/Users/coolm/softs/temp_files/sounds/cjyx_02.mp3'
    }
  }
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

// 从数据库加载maxDraws设置
const loadMaxDrawsFromDb = async () => {
  try {
    const result = await invoke<any>('get_system_setting', { key: 'max_prize_draws' });
    if (result && result.value) {
      const maxDraws = parseInt(result.value, 10);
      if (!isNaN(maxDraws)) {
        settings.maxDraws = maxDraws;
        console.log('从数据库加载maxDraws设置:', maxDraws);
      }
    }
  } catch (error) {
    console.error('从数据库加载maxDraws设置失败:', error);
  }
};

// 保存设置到本地存储
const saveSettings = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (error) {
    console.error('保存设置失败:', error);
  }
};

// 更新数据库中的maxDraws设置
const updateMaxDrawsInDb = async (value: number) => {
  try {
    await invoke<boolean>('update_system_setting', { 
      key: 'max_prize_draws', 
      value: value.toString() 
    });
    console.log('更新数据库中的maxDraws设置:', value);
  } catch (error) {
    console.error('更新数据库中的maxDraws设置失败:', error);
  }
};

// 监听设置变化并保存
watch(settings, () => {
  saveSettings();
}, { deep: true });

// 初始化加载数据库设置
loadMaxDrawsFromDb();

// 提供给组件使用的钩子
export function useWheelSettings() {
  // 更新设置
  const updateDrawMode = (mode: DrawMode) => {
    settings.drawMode = mode;
  };

  const updateLockAfterComplete = (lock: boolean) => {
    settings.lockAfterComplete = lock;
  };

  const updateMaxDraws = async (max: number) => {
    settings.maxDraws = max;
    // 同时更新数据库
    await updateMaxDrawsInDb(max);
  };

  const updatePrizes = (prizes: any[]) => {
    settings.prizes = prizes;
  };
  
  // 添加更新单词配置的方法
  const updatePrizeWords = (words: WordConfig[]) => {
    settings.prizeWords = words;
  };
  
  // 添加更新音效设置的方法
  const updateSound = (type: 'spin' | 'win', sound: SoundSetting) => {
    settings.sounds[type] = sound;
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
    updatePrizeWords,
    updateSound,
    resetSettings,
    isInitialized
  };
} 