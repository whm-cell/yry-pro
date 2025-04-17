/**
 * 转盘工具类 - 提供转盘相关的核心逻辑功能
 * @author YryPro
 */
import gsap from 'gsap';
import { useWheelSettings } from './wheelSettings';

/**
 * 旋转数据接口
 */
export interface RotationData {
  finalRotation: number;
  modRotation: number;
  sectionAngle: number;
  winnerIndex: number;
}

/**
 * 单词对定义
 */
export interface WordPair {
  cn: string;
  en: string;
}

/**
 * 转盘区域类型
 */
export type SectionType = 'word' | 'magic';

/**
 * 转盘区域定义
 */
export interface WheelSection {
  type: SectionType;
  text: string;
  textEN: string;
  color: string;
}

/**
 * 彩花效果返回值
 */
export interface ConfettiResult {
  cleanup: () => void;
}

/**
 * 转盘核心对象 - 提供所有转盘相关功能
 */
const WheelCore = {
  /**
   * 计算转盘旋转角度和最终位置
   * @param winnerIndex - 获奖区域索引
   * @param sectionCount - 转盘扇区总数
   * @returns 包含最终旋转角度和动画缓动等信息的对象
   */
  calculateRotation(winnerIndex: number, sectionCount = 5): RotationData {
    // 计算每个扇区的角度
    const sectionAngle = 360 / sectionCount;
    
    // 目标角度 - 指针指向的中心位置
    const targetAngle = (winnerIndex * sectionAngle) + (sectionAngle / 2);
    
    // 基础角度 - 确保指针指向获奖区域
    const baseAngle = 360 - targetAngle;
    
    // 添加小的随机偏移量，确保结果在扇区中央附近，但不会超出扇区
    const randomOffset = (Math.random() - 0.5) * (sectionAngle * 0.3);
    
    // 最终旋转角度 = 基础角度 + 随机偏移 + 固定圈数
    // 增加到8圈以确保有足够的旋转感，并且最后动画平滑
    const extraRotations = 8 * 360; // 固定8圈，避免不稳定性
    const finalRotation = baseAngle + randomOffset + extraRotations;
    
    return {
      finalRotation,
      modRotation: finalRotation % 360,
      sectionAngle,
      winnerIndex
    };
  },

  /**
   * 执行转盘旋转动画
   * @param wheelSelector - 转盘元素的选择器
   * @param rotationData - 旋转角度数据对象
   * @param onComplete - 动画完成后的回调函数
   */
  animateWheel(
    wheelSelector: string,
    rotationData: RotationData,
    onComplete?: (data: RotationData) => void
  ): void {
    // 旋转前先重置转盘样式，防止之前残留的transform属性影响
    gsap.set(wheelSelector, { rotation: 0 });
    
    // 使用GSAP动画库实现平滑减速动画
    gsap.to(wheelSelector, {
      duration: 6, // 增加持续时间，使动画更平滑
      rotation: rotationData.finalRotation,
      // 使用更平滑的缓动效果
      ease: "power2.out", // 更换为更稳定的缓动函数
      onComplete: () => {
        // 动画结束后设置最终位置
        gsap.set(wheelSelector, { rotation: rotationData.modRotation });
        
        if (onComplete && typeof onComplete === 'function') {
          onComplete(rotationData);
        }
      }
    });
  },

  /**
   * 播放音效
   * @param type - 音效类型：'spin'或'win' 
   * @param volume - 音量（0-1）
   */
  playSound(type: 'spin' | 'win', volume = 0.5): void {
    try {
      const { settings } = useWheelSettings();
      
      // 检查音效设置是否存在
      if (!settings.sounds || !settings.sounds[type]) {
        console.warn(`音效设置未找到: ${type}`);
        // 使用默认音效作为备选
        const audio = new Audio();
        if (type === 'spin') {
          audio.src = 'https://assets.mixkit.co/sfx/preview/mixkit-arcade-game-jump-coin-216.mp3';
        } else if (type === 'win') {
          audio.src = 'https://assets.mixkit.co/sfx/preview/mixkit-achievement-bell-600.mp3';
        }
        audio.volume = volume;
        audio.play().catch(e => console.error('无法播放音效', e));
        return;
      }
      
      const soundSetting = settings.sounds[type];
      const audio = new Audio();
      audio.src = soundSetting.url;
      audio.volume = volume;
      audio.play().catch(e => console.error('无法播放音效', e));
    } catch (e) {
      console.error('无法播放音效', e);
    }
  },

  /**
   * 创建彩花动画效果
   * @param duration - 彩花持续时间(毫秒)
   * @returns 包含清理彩花方法的对象
   */
  createConfetti(duration = 5000): ConfettiResult {
    if (typeof document === 'undefined') return { cleanup: () => {} };
    
    const confettiContainer = document.createElement('div');
    confettiContainer.classList.add('confetti-container');
    confettiContainer.style.position = 'fixed';
    confettiContainer.style.top = '0';
    confettiContainer.style.left = '0';
    confettiContainer.style.width = '100%';
    confettiContainer.style.height = '100%';
    confettiContainer.style.pointerEvents = 'none';
    confettiContainer.style.zIndex = '9999';
    document.body.appendChild(confettiContainer);
    
    const colors = ['#ff6b6b', '#4ecdc4', '#feca57', '#a78bfa', '#48dbfb'];
    
    // 创建更多彩花粒子
    for (let i = 0; i < 70; i++) {
      const confetti = document.createElement('div');
      confetti.style.position = 'absolute';
      confetti.style.width = `${Math.random() * 10 + 5}px`;
      confetti.style.height = `${Math.random() * 10 + 5}px`;
      confetti.style.backgroundColor = colors[Math.floor(Math.random() * colors.length)];
      confetti.style.borderRadius = '50%';
      confetti.style.left = `${Math.random() * 100}vw`;
      confetti.style.top = '0';
      confetti.style.opacity = '0.8';
      confetti.style.animation = `confetti-fall ${Math.random() * 3 + 2}s linear forwards`;
      confetti.style.animationDelay = `${Math.random() * 2}s`;
      confettiContainer.appendChild(confetti);
    }
    
    // 添加CSS动画规则
    const styleSheet = document.createElement('style');
    styleSheet.textContent = `
      @keyframes confetti-fall {
        0% {
          transform: translateY(0) rotate(0deg);
          opacity: 1;
        }
        100% {
          transform: translateY(100vh) rotate(720deg);
          opacity: 0;
        }
      }
    `;
    document.head.appendChild(styleSheet);
    
    // 返回清理函数
    const cleanup = () => {
      if (confettiContainer && confettiContainer.parentNode) {
        confettiContainer.parentNode.removeChild(confettiContainer);
      }
      if (styleSheet && styleSheet.parentNode) {
        styleSheet.parentNode.removeChild(styleSheet);
      }
    };
    
    // 设置持续时间后自动清理
    const timerId = setTimeout(cleanup, duration);
    
    return {
      cleanup: () => {
        clearTimeout(timerId);
        cleanup();
      }
    };
  },

  /**
   * 创建默认转盘区域数据
   * @param customWords - 自定义单词
   * @returns 转盘区域数据数组
   */
  createDefaultSections(customWords: WordPair[]): WheelSection[] {
    return [
      { type: 'word', text: customWords[0].cn, textEN: customWords[0].en, color: '#ff6b6b' },
      { type: 'word', text: customWords[1].cn, textEN: customWords[1].en, color: '#48dbfb' },
      { type: 'magic', text: '魔法袋子', textEN: 'Magic Bag', color: '#8A4FFF' },
      { type: 'word', text: customWords[2].cn, textEN: customWords[2].en, color: '#1dd1a1' },
      { type: 'word', text: customWords[3].cn, textEN: customWords[3].en, color: '#feca57' }
    ];
  },

  /**
   * 更新转盘区域数据
   * @param sections - 转盘区域数据
   * @param customWords - 自定义单词
   */
  updateSections(sections: WheelSection[], customWords: WordPair[]): void {
    sections.forEach((section: WheelSection, index: number) => {
      if (section.type === 'word') {
        const wordIndex = index >= 3 ? index - 1 : index;
        if (customWords[wordIndex]) {
          section.text = customWords[wordIndex].cn;
          section.textEN = customWords[wordIndex].en;
        }
      }
    });
  }
};

// 默认导出整个转盘核心对象
export default WheelCore;

// 同时导出所有单独函数，方便按需引入
export const {
  calculateRotation,
  animateWheel,
  playSound,
  createConfetti,
  createDefaultSections,
  updateSections
} = WheelCore; 