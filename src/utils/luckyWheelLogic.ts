/**
 * 抽奖转盘逻辑
 * 处理抽奖游戏的核心规则和逻辑
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

// 存储奖品配置和记录的类
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
  
  // 每种奖品的最大抽取次数
  private maxDraws: number;
  
  constructor(prizes: Prize[] = [], options: { 
    drawMode?: DrawMode; 
    lockAfterComplete?: boolean;
    maxDraws?: number;
  } = {}) {
    this.prizes = prizes;
    this.prizeRecords = {};
    this.allPrizesDrawnOnce = false;
    this.drawMode = options.drawMode || DrawMode.STANDARD;
    this.lockAfterComplete = options.lockAfterComplete !== undefined ? options.lockAfterComplete : false;
    this._isCompleted = false;
    this.maxDraws = options.maxDraws || 2; // 默认每种奖品最多抽中2次
    
    // 初始化记录
    this._initPrizeRecords();
  }
  
  // 初始化奖品记录
  private _initPrizeRecords(): void {
    this.prizeRecords = {};
    this.prizes.forEach(prize => {
      if (prize.prizeInfo && prize.prizeInfo.name) {
        this.prizeRecords[prize.prizeInfo.name] = 0;
      }
    });
  }
  
  // 更新奖品配置
  public setPrizes(prizes: Prize[]): void {
    this.prizes = prizes;
    this._initPrizeRecords();
    this._isCompleted = false;
  }
  
  // 设置抽奖模式
  public setDrawMode(mode: DrawMode): void {
    this.drawMode = mode;
  }
  
  // 设置每种奖品的最大抽取次数
  public setMaxDraws(maxDraws: number): void {
    this.maxDraws = maxDraws;
  }
  
  // 设置抽完后是否锁定
  public setLockAfterComplete(lock: boolean): void {
    this.lockAfterComplete = lock;
  }
  
  // 获取下一个奖品索引
  public getNextPrizeIndex(): number {
    // 如果已完成抽奖且设置了锁定，则不允许继续抽奖
    if (this._isCompleted && this.lockAfterComplete) {
      // 返回"谢谢惠顾"的索引
      const thanksIndex = this.getThanksIndex();
      return thanksIndex;
    }
    
    // 检查是否所有普通奖品都至少抽中一次
    this.checkAllPrizesDrawnOnce();
    
    // 获取"谢谢惠顾"的索引
    const thanksIndex = this.getThanksIndex();
    
    // 如果所有奖品都已经达到最大抽取次数，则返回"谢谢惠顾"的索引
    if (this.areAllPrizesDrawnToMax()) {
      // 标记抽奖已完成
      this._isCompleted = true;
      return thanksIndex;
    }
    
    // 根据不同模式处理抽奖逻辑
    if (this.drawMode === DrawMode.SEQUENCE) {
      // 顺序模式：先抽完所有普通奖品，再抽"谢谢惠顾"
      
      // 获取可选的奖品索引（次数小于最大抽取次数的奖品）
      const availablePrizes = this.getAvailablePrizeIndices();
      
      // 如果还有可抽的普通奖品，就抽普通奖品
      if (availablePrizes.length > 0) {
        // 如果未抽过的奖品，优先抽未抽过的
        const undrawnPrizes = this.getUndrawnPrizeIndices();
        if (undrawnPrizes.length > 0) {
          const randomIndex = Math.floor(Math.random() * undrawnPrizes.length);
          return undrawnPrizes[randomIndex];
        }
        
        // 否则随机抽一个未达到上限的
        const randomIndex = Math.floor(Math.random() * availablePrizes.length);
        return availablePrizes[randomIndex];
      }
      
      // 如果没有可抽的普通奖品，抽"谢谢惠顾"
      this._isCompleted = true;
      return thanksIndex;
    } else {
      // 标准模式：普通奖品和"谢谢惠顾"混合抽取
      
      // 获取可选的奖品索引
      const availablePrizes = this.getAvailablePrizeIndices();
      
      // 如果已经所有奖品都抽过一次，那么随机选择一个未达到最大抽取次数的奖品
      if (this.allPrizesDrawnOnce) {
        if (availablePrizes.length > 0) {
          const randomIndex = Math.floor(Math.random() * availablePrizes.length);
          return availablePrizes[randomIndex];
        } else {
          this._isCompleted = true;
          return thanksIndex; // 如果没有可选奖品，返回"谢谢惠顾"
        }
      }
      
      // 如果还有未抽中过的奖品，优先选择它们
      const undrawnPrizes = this.getUndrawnPrizeIndices();
      if (undrawnPrizes.length > 0) {
        const randomIndex = Math.floor(Math.random() * undrawnPrizes.length);
        return undrawnPrizes[randomIndex];
      }
      
      // 如果所有奖品都抽过，随机选择一个未达到上限的奖品
      if (availablePrizes.length > 0) {
        const randomIndex = Math.floor(Math.random() * availablePrizes.length);
        return availablePrizes[randomIndex];
      }
      
      // 如果没有可用奖品，返回"谢谢惠顾"
      this._isCompleted = true;
      return thanksIndex;
    }
  }
  
  // 获取"谢谢惠顾"的索引
  private getThanksIndex(): number {
    const thanksIndex = this.prizes.findIndex(prize => 
      prize.prizeInfo && prize.prizeInfo.name === "谢谢惠顾");
    return thanksIndex >= 0 ? thanksIndex : this.prizes.length - 1; // 默认最后一个是"谢谢惠顾"
  }
  
  // 获取未抽中过的奖品索引
  public getUndrawnPrizeIndices(): number[] {
    const undrawnIndices: number[] = [];
    // 只检查非"谢谢惠顾"的普通奖品
    const regularPrizeCount = this.prizes.filter(prize => 
      prize.prizeInfo && prize.prizeInfo.name !== "谢谢惠顾").length;
      
    for (let i = 0; i < regularPrizeCount; i++) {
      const prizeName = this.prizes[i].prizeInfo.name;
      if (this.prizeRecords[prizeName] === 0) {
        undrawnIndices.push(i);
      }
    }
    return undrawnIndices;
  }
  
  // 获取可选的奖品索引（次数小于最大抽取次数的奖品）
  public getAvailablePrizeIndices(): number[] {
    const availableIndices: number[] = [];
    // 只检查非"谢谢惠顾"的普通奖品
    const regularPrizeCount = this.prizes.filter(prize => 
      prize.prizeInfo && prize.prizeInfo.name !== "谢谢惠顾").length;
      
    for (let i = 0; i < regularPrizeCount; i++) {
      const prizeName = this.prizes[i].prizeInfo.name;
      if (this.prizeRecords[prizeName] < this.maxDraws) {
        availableIndices.push(i);
      }
    }
    return availableIndices;
  }
  
  // 检查是否所有普通奖品都至少抽中一次
  public checkAllPrizesDrawnOnce(): boolean {
    const prizeNames = Object.keys(this.prizeRecords).filter(name => name !== "谢谢惠顾");
    this.allPrizesDrawnOnce = prizeNames.every(name => this.prizeRecords[name] > 0);
    return this.allPrizesDrawnOnce;
  }
  
  // 检查是否所有普通奖品都已经抽中最大次数
  public areAllPrizesDrawnToMax(): boolean {
    const prizeNames = Object.keys(this.prizeRecords).filter(name => name !== "谢谢惠顾");
    return prizeNames.every(name => this.prizeRecords[name] >= this.maxDraws);
  }
  
  // 更新奖品抽中记录
  public updatePrizeRecord(prizeIndex: number): DrawResult | null {
    if (prizeIndex >= 0 && prizeIndex < this.prizes.length) {
      const prizeName = this.prizes[prizeIndex].prizeInfo.name;
      if (Object.prototype.hasOwnProperty.call(this.prizeRecords, prizeName)) {
        this.prizeRecords[prizeName]++;
        
        // 检查是否所有奖品都至少抽中一次
        this.checkAllPrizesDrawnOnce();
        
        // 检查是否抽完（所有普通奖品都抽中最大次数）
        if (this.areAllPrizesDrawnToMax()) {
          this._isCompleted = true;
        }
        
        return {
          prize: this.prizes[prizeIndex],
          name: prizeName,
          count: this.prizeRecords[prizeName]
        };
      }
    }
    return null;
  }
  
  // 重置抽奖记录
  public resetRecords(): void {
    for (const key in this.prizeRecords) {
      if (Object.prototype.hasOwnProperty.call(this.prizeRecords, key)) {
        this.prizeRecords[key] = 0;
      }
    }
    this.allPrizesDrawnOnce = false;
    this._isCompleted = false;
  }
  
  // 获取当前抽奖记录
  public getPrizeRecords(): PrizeRecords {
    return { ...this.prizeRecords };
  }
  
  // 获取抽奖是否已完成
  public isCompleted(): boolean {
    return this._isCompleted;
  }
}

// 创建抽奖管理实例的工厂函数
export function createLuckyWheel(prizes: Prize[], options?: { 
  drawMode?: DrawMode; 
  lockAfterComplete?: boolean;
  maxDraws?: number;
}): LuckyWheelManager {
  return new LuckyWheelManager(prizes, options);
}

// 默认导出简便函数
export default createLuckyWheel; 