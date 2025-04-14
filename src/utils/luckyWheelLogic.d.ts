/**
 * 抽奖转盘逻辑类型声明
 */

import { DrawMode } from './wheelSettings';

// 奖品信息接口
export interface PrizeInfo {
  name: string;
  imgSrc: string;
}

// 文字设置接口
export interface FontSetting {
  text: string;
  top: string;
  fontColor: string;
  fontSize: string;
  fontWeight?: string;
}

// 图片设置接口
export interface ImageSetting {
  src: string;
  width: string;
  top: string;
}

// 奖品接口
export interface Prize {
  background: string;
  fonts: FontSetting[];
  imgs: ImageSetting[];
  prizeInfo: PrizeInfo;
}

// 奖品记录类型
export type PrizeRecords = Record<string, number>;

// 抽奖结果接口
export interface DrawResult {
  prize: Prize;
  name: string;
  count: number;
}

// 抽奖管理器选项接口
export interface LuckyWheelOptions {
  drawMode?: DrawMode;
  lockAfterComplete?: boolean;
  maxDraws?: number;
}

// 抽奖管理类
export interface LuckyWheelManager {
  // 标记所有奖品是否至少抽中一次
  allPrizesDrawnOnce: boolean;
  
  // 设置抽奖模式
  setDrawMode(mode: DrawMode): void;
  
  // 设置每种奖品的最大抽取次数
  setMaxDraws(maxDraws: number): void;
  
  // 设置抽完后是否锁定
  setLockAfterComplete(lock: boolean): void;
  
  // 获取下一个奖品索引
  getNextPrizeIndex(): number;
  
  // 更新奖品抽中记录
  updatePrizeRecord(prizeIndex: number): DrawResult | null;
  
  // 重置抽奖记录
  resetRecords(): void;
  
  // 获取当前抽奖记录
  getPrizeRecords(): PrizeRecords;
  
  // 获取抽奖是否已完成
  isCompleted(): boolean;
}

// 声明创建抽奖管理器的函数
declare function createLuckyWheel(prizes: Prize[], options?: LuckyWheelOptions): LuckyWheelManager;

export default createLuckyWheel; 