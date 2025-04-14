/**
 * 抽奖转盘逻辑类型声明
 */

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

// 抽奖结果接口
export interface DrawResult {
  prize: Prize;
  name: string;
  count: number;
}

// 奖品记录类型
export type PrizeRecords = Record<string, number>;

/**
 * 抽奖模式枚举
 * STANDARD - 标准模式：普通奖品最多抽中两次，之后只抽"谢谢惠顾"
 * SEQUENCE - 顺序模式：先抽完所有普通奖品，每种最多两次，然后才抽"谢谢惠顾"
 * SINGLE - 单次模式：每个奖品最多只能抽中一次
 */
export enum DrawMode {
  STANDARD = 'standard',
  SEQUENCE = 'sequence',
  SINGLE = 'single'
}

// 抽奖管理类
export class LuckyWheelManager {
  // 奖品配置
  private prizes: Prize[];
  
  // 抽奖记录
  private prizeRecords: PrizeRecords;
  
  // 标记所有奖品是否至少抽中一次
  public allPrizesDrawnOnce: boolean;
  
  // 抽奖模式
  private drawMode: DrawMode;
  
  // 是否在抽完后锁定转盘
  private lockAfterComplete: boolean;
  
  // 是否已完成抽奖
  private _isCompleted: boolean;
  
  constructor(prizes: Prize[], options?: { 
    drawMode?: DrawMode; 
    lockAfterComplete?: boolean;
    maxDraws?: number;
  });
  
  // 初始化奖品记录
  private _initPrizeRecords(): void;
  
  // 更新奖品配置
  public setPrizes(prizes: Prize[]): void;
  
  // 设置抽奖模式
  public setDrawMode(mode: DrawMode): void;
  
  // 设置每种奖品的最大抽取次数
  public setMaxDraws(maxDraws: number): void;
  
  // 设置抽完后是否锁定
  public setLockAfterComplete(lock: boolean): void;
  
  // 获取下一个奖品索引
  public getNextPrizeIndex(): number;
  
  // 获取"谢谢惠顾"的索引
  private getThanksIndex(): number;
  
  // 获取未抽中过的奖品索引
  public getUndrawnPrizeIndices(): number[];
  
  // 获取可选的奖品索引
  public getAvailablePrizeIndices(): number[];
  
  // 检查是否所有普通奖品都至少抽中一次
  public checkAllPrizesDrawnOnce(): boolean;
  
  // 检查是否所有普通奖品都已经抽中最大次数
  public areAllPrizesDrawnToMax(): boolean;
  
  // 更新奖品抽中记录
  public updatePrizeRecord(prizeIndex: number): DrawResult | null;
  
  // 重置抽奖记录
  public resetRecords(): void;
  
  // 获取当前抽奖记录
  public getPrizeRecords(): PrizeRecords;
  
  // 获取抽奖是否已完成
  public isCompleted(): boolean;
}

// 创建抽奖管理实例的工厂函数
export function createLuckyWheel(prizes: Prize[], options?: { 
  drawMode?: DrawMode; 
  lockAfterComplete?: boolean;
  maxDraws?: number;
}): LuckyWheelManager;

// 默认导出
export default createLuckyWheel; 