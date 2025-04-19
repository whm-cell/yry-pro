<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits(["close"]);

const logs = ref<string[]>([]);
const unlistenFn = ref<(() => void) | undefined>();
const autoScroll = ref(true); // 是否自动滚动
const refreshInterval = ref<number | null>(null); // 刷新定时器
const isRefreshing = ref(false); // 是否正在刷新中
const lastRefreshTime = ref(new Date());

// 日志过滤
const filterText = ref("");
const filteredLogs = computed(() => {
  if (!filterText.value) return logs.value;
  const searchText = filterText.value.toLowerCase();
  return logs.value.filter(log => log.toLowerCase().includes(searchText));
});

// 监听可见性变化，当显示日志窗口时自动刷新日志和滚动到底部
watch(() => props.visible, (isVisible) => {
  if (isVisible) {
    refreshLogs(true);
    setTimeout(scrollToBottom, 100);
  }
});

onMounted(async () => {
  // 启动监听日志事件
  try {
    unlistenFn.value = await listen("log-event", (event: any) => {
      const logMessage = event.payload as string;
      
      // 避免重复日志
      if (!logs.value.includes(logMessage)) {
        logs.value.push(logMessage);
        if (autoScroll.value) {
          scrollToBottom();
        }
      }
    });
    
    // 获取现有日志
    await refreshLogs(true);
    
    // 设置自动刷新（每3秒刷新一次）
    refreshInterval.value = window.setInterval(() => refreshLogs(false), 3000);
  } catch (error) {
    logs.value.push(`获取日志失败: ${error}`);
  }
});

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenFn.value) {
    unlistenFn.value();
  }
  
  // 清理定时器
  if (refreshInterval.value !== null) {
    clearInterval(refreshInterval.value);
  }
});

// 刷新日志
const refreshLogs = async (force = false) => {
  // 避免频繁刷新
  if (isRefreshing.value) return;
  
  // 如果不是强制刷新且最后一次刷新在1秒内，则跳过
  if (!force) {
    const now = new Date();
    const diffTime = now.getTime() - lastRefreshTime.value.getTime();
    if (diffTime < 1000) return;
  }
  
  isRefreshing.value = true;
  
  try {
    const existingLogs = await invoke("get_recent_logs");
    lastRefreshTime.value = new Date();
    
    if (Array.isArray(existingLogs)) {
      // 合并日志，去除重复
      if (existingLogs.length > 0) {
        // 先获取当前没有的日志
        const newLogs = existingLogs.filter(log => !logs.value.includes(log));
        
        if (newLogs.length > 0) {
          // 添加新日志
          logs.value = [...logs.value, ...newLogs];
          
          if (autoScroll.value) {
            scrollToBottom();
          }
        }
      } else if (logs.value.length === 0) {
        logs.value = ['[INFO] 等待系统日志...'];
      }
    }
  } catch (error) {
    console.error("刷新日志失败:", error);
  } finally {
    isRefreshing.value = false;
  }
};

const scrollToBottom = () => {
  setTimeout(() => {
    const logContainer = document.getElementById("log-container");
    if (logContainer) {
      logContainer.scrollTop = logContainer.scrollHeight;
    }
  }, 50);
};

const clearLogs = () => {
  logs.value = [];
};

// 切换自动滚动
const toggleAutoScroll = () => {
  autoScroll.value = !autoScroll.value;
  if (autoScroll.value) {
    scrollToBottom();
  }
};

// 手动刷新日志
const manualRefresh = async () => {
  await refreshLogs(true);
};

// 根据日志级别确定CSS类
const getLogClass = (log: string): string => {
  const logLower = log.toLowerCase();
  if (logLower.includes('[error]')) return 'text-red-500';
  if (logLower.includes('[warn]')) return 'text-yellow-500';
  if (logLower.includes('[debug]')) return 'text-blue-400';
  if (logLower.includes('[trace]')) return 'text-gray-400';
  return 'text-green-400'; // 默认为info级别
};
</script>

<template>
  <div 
    class="log-viewer-container fixed top-0 left-0 h-full bg-gray-900 text-green-400 shadow-xl z-50 transition-all duration-300 overflow-hidden flex flex-col border-r border-gray-700"
    :class="{ 'w-96': visible, 'w-0': !visible }"
  >
    <div class="log-header flex justify-between items-center p-4 bg-gradient-to-r from-gray-900 to-gray-800 border-b border-gray-700">
      <div class="flex items-center">
        <i class="fas fa-terminal mr-2 text-indigo-400"></i>
        <span class="font-mono font-bold text-indigo-300 text-lg">系统日志</span>
      </div>
      <div class="flex space-x-3">
        <button 
          @click="manualRefresh" 
          class="p-1.5 hover:bg-gray-700 rounded-md transition-colors duration-200 text-indigo-300 hover:text-indigo-200"
          title="刷新日志"
          :disabled="isRefreshing"
          :class="{'opacity-50': isRefreshing}"
        >
          <i class="fas" :class="isRefreshing ? 'fa-spinner fa-spin' : 'fa-sync-alt'"></i>
        </button>
        <button 
          @click="toggleAutoScroll" 
          class="p-1.5 hover:bg-gray-700 rounded-md transition-colors duration-200"
          :class="{'text-green-400 hover:text-green-300': autoScroll, 'text-indigo-300 hover:text-indigo-200': !autoScroll}"
          :title="autoScroll ? '关闭自动滚动' : '开启自动滚动'"
        >
          <i class="fas" :class="{'fa-scroll': autoScroll, 'fa-lock': !autoScroll}"></i>
        </button>
        <button 
          @click="clearLogs" 
          class="p-1.5 hover:bg-gray-700 rounded-md transition-colors duration-200 text-indigo-300 hover:text-indigo-200"
          title="清除日志"
        >
          <i class="fas fa-trash-alt"></i>
        </button>
        <button 
          @click="emit('close')" 
          class="p-1.5 hover:bg-red-900/50 rounded-md transition-colors duration-200 text-indigo-300 hover:text-red-300"
          title="关闭日志窗口"
        >
          <i class="fas fa-times"></i>
        </button>
      </div>
    </div>
    
    <!-- 日志过滤框 -->
    <div class="px-3 py-2 bg-gray-800 border-b border-gray-700 flex items-center">
      <i class="fas fa-search text-gray-500 mr-2"></i>
      <input 
        v-model="filterText"
        type="text" 
        placeholder="过滤日志..." 
        class="w-full bg-gray-700 text-gray-200 px-2 py-1 rounded border border-gray-600 focus:outline-none focus:border-indigo-500"
      />
    </div>

    <div 
      id="log-container" 
      class="flex-1 p-4 overflow-y-auto font-mono text-xs whitespace-pre-wrap bg-gray-900 bg-opacity-95"
      @scroll="autoScroll = false"
    >
      <div v-if="filteredLogs.length === 0" class="text-gray-500 italic flex flex-col items-center justify-center h-full">
        <i class="fas fa-inbox text-4xl mb-3 text-gray-700"></i>
        <span>暂无日志记录...</span>
      </div>
      <div 
        v-else 
        v-for="(log, index) in filteredLogs" 
        :key="index" 
        class="mb-2 py-1 px-2 rounded hover:bg-gray-800/50 transition-colors duration-200"
        :class="getLogClass(log)"
      >
        {{ log }}
      </div>
    </div>
    
    <!-- 滚动到底部按钮 -->
    <button 
      v-show="!autoScroll && visible && filteredLogs.length > 10" 
      @click="scrollToBottom(); autoScroll = true" 
      class="absolute bottom-4 right-4 bg-indigo-500 hover:bg-indigo-600 text-white p-2 rounded-full shadow-lg opacity-80 hover:opacity-100 transition-all animate-bounce"
      title="滚动到底部"
    >
      <i class="fas fa-arrow-down"></i>
    </button>
    
    <!-- 日志状态 -->
    <div class="py-1 px-3 text-xs text-gray-500 bg-gray-800 border-t border-gray-700 flex justify-between">
      <span>{{ filteredLogs.length }} 条日志</span>
      <span>最后更新: {{ lastRefreshTime.toLocaleTimeString() }}</span>
    </div>
  </div>
</template>

<style scoped>
.log-viewer-container {
  font-family: 'Courier New', Courier, monospace;
  backdrop-filter: blur(5px);
  box-shadow: -5px 0 15px rgba(0, 0, 0, 0.3);
}

#log-container::-webkit-scrollbar {
  width: 6px;
}

#log-container::-webkit-scrollbar-track {
  background: rgba(26, 26, 26, 0.5);
  border-radius: 3px;
}

#log-container::-webkit-scrollbar-thumb {
  background-color: rgba(99, 102, 241, 0.4);
  border-radius: 3px;
  transition: background-color 0.3s;
}

#log-container::-webkit-scrollbar-thumb:hover {
  background-color: rgba(99, 102, 241, 0.6);
}

/* 为每种日志类型添加前缀图标 */
.text-red-500::before {
  content: '❌ ';
}

.text-yellow-500::before {
  content: '⚠️ ';
}

.text-blue-400::before {
  content: '🔍 ';
}

.text-gray-400::before {
  content: '🔬 ';
}

.text-green-400::before {
  content: '✅ ';
}

/* 滚动提示按钮的上下跳动动画 */
@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5px); }
}

.animate-bounce {
  animation: bounce 1s infinite ease-in-out;
}
</style> 