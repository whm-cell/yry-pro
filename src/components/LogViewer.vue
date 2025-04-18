<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits(["close"]);

const logs = ref<string[]>([]);
const unlistenFn = ref<(() => void) | undefined>();

onMounted(async () => {
  // 启动监听日志事件
  try {
    unlistenFn.value = await listen("log-event", (event: any) => {
      const logMessage = event.payload as string;
      logs.value.push(logMessage);
      scrollToBottom();
    });
    
    // 获取现有日志
    const existingLogs = await invoke("get_recent_logs");
    if (Array.isArray(existingLogs)) {
      logs.value = existingLogs;
      scrollToBottom();
    }
  } catch (error) {
    logs.value.push(`获取日志失败: ${error}`);
  }
});

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenFn.value) {
    unlistenFn.value();
  }
});

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

// 测试发送不同级别的日志
const testLogLevels = async () => {
  try {
    await invoke("log_message", { level: "info", message: "这是一条信息日志" });
    await invoke("log_message", { level: "warn", message: "这是一条警告日志" });
    await invoke("log_message", { level: "error", message: "这是一条错误日志" });
    await invoke("log_message", { level: "debug", message: "这是一条调试日志" });
  } catch (error) {
    logs.value.push(`发送测试日志失败: ${error}`);
  }
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
          @click="testLogLevels" 
          class="p-1.5 hover:bg-gray-700 rounded-md transition-colors duration-200 text-indigo-300 hover:text-indigo-200"
          title="测试日志"
        >
          <i class="fas fa-vial"></i>
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

    <div 
      id="log-container" 
      class="flex-1 p-4 overflow-y-auto font-mono text-sm whitespace-pre-wrap bg-gray-900 bg-opacity-95"
    >
      <div v-if="logs.length === 0" class="text-gray-500 italic flex flex-col items-center justify-center h-full">
        <i class="fas fa-inbox text-4xl mb-3 text-gray-700"></i>
        <span>暂无日志记录...</span>
      </div>
      <div 
        v-else 
        v-for="(log, index) in logs" 
        :key="index" 
        class="mb-2 py-1 px-2 rounded hover:bg-gray-800/50 transition-colors duration-200"
        :class="getLogClass(log)"
      >
        {{ log }}
      </div>
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
</style> 