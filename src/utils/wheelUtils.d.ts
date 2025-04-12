/**
 * 转盘工具类类型定义
 */

/**
 * 旋转数据结构
 */
export interface RotationData {
  /**
   * 最终旋转角度
   */
  finalRotation: number;
  
  /**
   * 取模后的旋转角度（0-360）
   */
  modRotation: number;
  
  /**
   * 每个扇区的角度
   */
  sectionAngle: number;
}

/**
 * 计算转盘旋转角度和最终位置
 * @param winnerIndex - 获奖区域索引
 * @param sectionCount - 转盘扇区总数
 */
export function calculateRotation(winnerIndex: number, sectionCount?: number): RotationData;

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
): void;

/**
 * 播放音效
 * @param type - 音效类型：'spin'或'win' 
 * @param volume - 音量（0-1）
 */
export function playSound(type: 'spin' | 'win', volume?: number): void;

/**
 * 彩花效果控制器
 */
export interface ConfettiController {
  /**
   * 清理彩花效果
   */
  cleanup: () => void;
}

/**
 * 创建彩花动画效果
 * @param duration - 彩花持续时间(毫秒)
 */
export function createConfetti(duration?: number): ConfettiController; 