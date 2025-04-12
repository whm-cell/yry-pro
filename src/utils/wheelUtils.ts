/**
 * 转盘工具类 - 提供转盘相关的核心逻辑功能
 */
import gsap from 'gsap';

/**
 * 旋转数据接口
 */
export interface RotationData {
  finalRotation: number;
  modRotation: number;
  sectionAngle: number;
}

/**
 * 计算转盘旋转角度和最终位置
 * @param winnerIndex - 获奖区域索引
 * @param sectionCount - 转盘扇区总数
 * @returns 包含最终旋转角度和动画缓动等信息的对象
 */
export function calculateRotation(winnerIndex: number, sectionCount = 5): RotationData {
  // 计算每个扇区的角度
  const sectionAngle = 360 / sectionCount;
  
  // 目标角度 - 指针指向的中心位置
  const targetAngle = (winnerIndex * sectionAngle) + (sectionAngle / 2);
  
  // 基础角度 - 确保指针指向获奖区域
  const baseAngle = 360 - targetAngle;
  
  // 添加随机偏移量，使结果更随机（但不会超出扇区）
  const randomOffset = (Math.random() - 0.5) * (sectionAngle * 0.5);
  
  // 最终旋转角度 = 基础角度 + 随机偏移 + 6圈额外旋转
  // 增加到6圈以确保有足够的旋转感
  const finalRotation = baseAngle + randomOffset + (6 * 360);
  
  return {
    finalRotation,
    modRotation: finalRotation % 360,
    sectionAngle
  };
}

/**
 * 执行转盘旋转动画
 * @param wheelSelector - 转盘元素的选择器
 * @param rotationData - 旋转角度数据对象
 * @param onComplete - 动画完成后的回调函数
 */
export function animateWheel(
  wheelSelector: string,
  rotationData: RotationData,
  onComplete?: (data: RotationData) => void
): void {
  // 使用GSAP动画库实现平滑减速动画
  gsap.to(wheelSelector, {
    duration: 5,
    rotation: rotationData.finalRotation,
    // 使用cubic-bezier函数创建自定义缓动效果，更逼真的物理减速效果
    ease: "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
    // 确保转盘快结束时减速更平滑
    onUpdate: function() {
      // 获取当前进度(0-1之间)
      const progress = this.progress();
      
      // 在进度超过80%时，确保动画更平滑
      if (progress > 0.8) {
        // 在这个阶段应用更精细的动画控制
        const remainingProgress = (1 - progress) / 0.2; // 剩余进度的比例
        // 重新调整动画速度，使结尾更平滑
        if (this.timeScale) {
          this.timeScale(Math.max(0.5, remainingProgress));
        }
      }
    },
    onComplete: () => {
      if (onComplete && typeof onComplete === 'function') {
        onComplete(rotationData);
      }
    }
  });
}

/**
 * 播放音效
 * @param type - 音效类型：'spin'或'win' 
 * @param volume - 音量（0-1）
 */
export function playSound(type: 'spin' | 'win', volume = 0.5): void {
  try {
    const audio = new Audio();
    if (type === 'spin') {
      audio.src = 'https://assets.mixkit.co/sfx/preview/mixkit-arcade-game-jump-coin-216.mp3';
    } else if (type === 'win') {
      audio.src = 'https://assets.mixkit.co/sfx/preview/mixkit-achievement-bell-600.mp3';
    }
    audio.volume = volume;
    audio.play().catch(e => console.error('无法播放音效', e));
  } catch (e) {
    console.error('无法播放音效', e);
  }
}

interface ConfettiResult {
  cleanup: () => void;
}

/**
 * 创建彩花动画效果
 * @param duration - 彩花持续时间(毫秒)
 * @returns 包含清理彩花方法的对象
 */
export function createConfetti(duration = 5000): ConfettiResult {
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
} 