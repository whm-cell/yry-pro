<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-lg shadow-lg p-6 h-full">
      <h2 class="text-2xl font-bold mb-6 text-gray-800">中奖记录</h2>
      
      <!-- 筛选与搜索 -->
      <div class="flex flex-col md:flex-row justify-between mb-6">
        <div class="flex flex-wrap gap-2 mb-4 md:mb-0">
          <button 
            v-for="(filter, index) in filters" 
            :key="index"
            class="px-4 py-2 rounded-md" 
            :class="activeFilter === index ? 'bg-blue-600 text-white' : 'bg-gray-200 hover:bg-gray-300'"
            @click="activeFilter = index"
          >
            {{ filter }}
          </button>
        </div>
        
        <div class="relative">
          <input type="text" placeholder="搜索奖品名称" v-model="searchText" class="pl-10 pr-4 py-2 border border-gray-300 rounded-md w-64 focus:outline-none focus:ring-2 focus:ring-blue-500">
          <i class="fas fa-search absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"></i>
        </div>
      </div>
      
      <!-- 中奖记录表格 -->
      <div class="overflow-x-auto">
        <table class="min-w-full bg-white border border-gray-200">
          <thead>
            <tr class="bg-gray-100">
              <th class="py-3 px-4 text-left border-b">奖品名称</th>
              <th class="py-3 px-4 text-left border-b">奖品等级</th>
              <th class="py-3 px-4 text-left border-b">中奖时间</th>
              <th class="py-3 px-4 text-left border-b">状态</th>
              <th class="py-3 px-4 text-left border-b">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(record, index) in filteredRecords" :key="index" class="hover:bg-gray-50">
              <td class="py-3 px-4 border-b">{{ record.name }}</td>
              <td class="py-3 px-4 border-b">
                <span class="px-2 py-1 rounded-full text-xs font-medium" :class="getLevelClass(record.level)">
                  {{ record.level }}
                </span>
              </td>
              <td class="py-3 px-4 border-b">{{ record.time }}</td>
              <td class="py-3 px-4 border-b">
                <span class="px-2 py-1 rounded-full text-xs font-medium" :class="getStatusClass(record.status)">
                  {{ record.status }}
                </span>
              </td>
              <td class="py-3 px-4 border-b">
                <button 
                  v-if="record.status === '待领取'" 
                  class="text-blue-500 hover:text-blue-700"
                  @click="claimPrize(index)"
                >
                  领取
                </button>
                <button v-else class="text-gray-400 cursor-not-allowed">
                  {{ record.status }}
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <!-- 分页 -->
      <div class="mt-6 flex justify-between items-center">
        <div class="text-sm text-gray-500">
          显示 {{ startRange }} 到 {{ endRange }}，共 {{ records.length }} 条记录
        </div>
        
        <div class="flex">
          <button 
            class="w-8 h-8 flex items-center justify-center border border-gray-300 rounded-l-md hover:bg-gray-100" 
            :disabled="currentPage === 1"
            :class="{ 'opacity-50': currentPage === 1 }"
            @click="currentPage > 1 && (currentPage--)"
          >
            <i class="fas fa-chevron-left text-sm"></i>
          </button>
          
          <button 
            v-for="page in totalPages" 
            :key="page"
            class="w-8 h-8 flex items-center justify-center border-t border-b border-gray-300"
            :class="currentPage === page ? 'bg-blue-500 text-white' : 'hover:bg-gray-100'"
            @click="currentPage = page"
          >
            {{ page }}
          </button>
          
          <button 
            class="w-8 h-8 flex items-center justify-center border border-gray-300 rounded-r-md hover:bg-gray-100"
            :disabled="currentPage === totalPages"
            :class="{ 'opacity-50': currentPage === totalPages }"
            @click="currentPage < totalPages && (currentPage++)"
          >
            <i class="fas fa-chevron-right text-sm"></i>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

// 筛选项
const filters = ['全部', '待领取', '已领取', '已过期'];
const activeFilter = ref(0);
const searchText = ref('');

// 当前页面
const currentPage = ref(1);
const itemsPerPage = 5;

// 模拟数据
const records = ref([
  { name: 'iPhone 14 Pro', level: '一等奖', time: '2023-05-20 14:30:45', status: '待领取' },
  { name: 'AirPods Pro', level: '二等奖', time: '2023-05-15 09:22:10', status: '已领取' },
  { name: '200元购物券', level: '三等奖', time: '2023-05-10 16:45:30', status: '已过期' },
  { name: '50元购物券', level: '五等奖', time: '2023-05-05 11:12:33', status: '已领取' },
  { name: '10元购物券', level: '七等奖', time: '2023-04-28 08:15:20', status: '已领取' },
  { name: '1000元购物券', level: '特等奖', time: '2023-04-20 10:30:00', status: '已领取' },
  { name: '电动牙刷', level: '四等奖', time: '2023-04-15 16:20:15', status: '已过期' },
  { name: '充电宝', level: '四等奖', time: '2023-04-10 09:45:50', status: '已领取' },
  { name: '保温杯', level: '六等奖', time: '2023-04-05 14:05:30', status: '已领取' },
  { name: '笔记本电脑', level: '一等奖', time: '2023-03-28 11:30:00', status: '已过期' },
  { name: '平板电脑', level: '二等奖', time: '2023-03-20 15:45:20', status: '已领取' },
  { name: '20元话费券', level: '六等奖', time: '2023-03-15 10:10:10', status: '已领取' }
]);

// 筛选记录
const filteredRecords = computed(() => {
  let result = records.value;
  
  // 按状态筛选
  if (activeFilter.value > 0) {
    const filterStatus = filters[activeFilter.value];
    result = result.filter(record => record.status === filterStatus);
  }
  
  // 按搜索文本筛选
  if (searchText.value.trim() !== '') {
    const searchLower = searchText.value.toLowerCase();
    result = result.filter(record => 
      record.name.toLowerCase().includes(searchLower) || 
      record.level.toLowerCase().includes(searchLower)
    );
  }
  
  // 分页
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  
  return result.slice(start, end);
});

// 计算总页数
const totalPages = computed(() => {
  let filtered = records.value;
  
  // 按状态筛选
  if (activeFilter.value > 0) {
    const filterStatus = filters[activeFilter.value];
    filtered = filtered.filter(record => record.status === filterStatus);
  }
  
  // 按搜索文本筛选
  if (searchText.value.trim() !== '') {
    const searchLower = searchText.value.toLowerCase();
    filtered = filtered.filter(record => 
      record.name.toLowerCase().includes(searchLower) || 
      record.level.toLowerCase().includes(searchLower)
    );
  }
  
  return Math.ceil(filtered.length / itemsPerPage);
});

// 计算当前显示的记录范围
const startRange = computed(() => {
  let filtered = records.value;
  
  // 按状态筛选
  if (activeFilter.value > 0) {
    const filterStatus = filters[activeFilter.value];
    filtered = filtered.filter(record => record.status === filterStatus);
  }
  
  // 按搜索文本筛选
  if (searchText.value.trim() !== '') {
    const searchLower = searchText.value.toLowerCase();
    filtered = filtered.filter(record => 
      record.name.toLowerCase().includes(searchLower) || 
      record.level.toLowerCase().includes(searchLower)
    );
  }
  
  if (filtered.length === 0) return 0;
  
  return (currentPage.value - 1) * itemsPerPage + 1;
});

const endRange = computed(() => {
  let filtered = records.value;
  
  // 按状态筛选
  if (activeFilter.value > 0) {
    const filterStatus = filters[activeFilter.value];
    filtered = filtered.filter(record => record.status === filterStatus);
  }
  
  // 按搜索文本筛选
  if (searchText.value.trim() !== '') {
    const searchLower = searchText.value.toLowerCase();
    filtered = filtered.filter(record => 
      record.name.toLowerCase().includes(searchLower) || 
      record.level.toLowerCase().includes(searchLower)
    );
  }
  
  if (filtered.length === 0) return 0;
  
  const end = currentPage.value * itemsPerPage;
  return end > filtered.length ? filtered.length : end;
});

// 获取等级样式
const getLevelClass = (level: string) => {
  const levelMap: Record<string, string> = {
    '特等奖': 'bg-red-100 text-red-800',
    '一等奖': 'bg-red-100 text-red-800',
    '二等奖': 'bg-blue-100 text-blue-800',
    '三等奖': 'bg-green-100 text-green-800',
    '四等奖': 'bg-yellow-100 text-yellow-800',
    '五等奖': 'bg-yellow-100 text-yellow-800',
    '六等奖': 'bg-purple-100 text-purple-800',
    '七等奖': 'bg-blue-100 text-blue-800'
  };
  
  return levelMap[level] || 'bg-gray-100 text-gray-800';
};

// 获取状态样式
const getStatusClass = (status: string) => {
  const statusMap: Record<string, string> = {
    '待领取': 'bg-green-100 text-green-800',
    '已领取': 'bg-gray-100 text-gray-800',
    '已过期': 'bg-red-100 text-red-800'
  };
  
  return statusMap[status] || 'bg-gray-100 text-gray-800';
};

// 领取奖品
const claimPrize = (index: number) => {
  const originalIndex = (currentPage.value - 1) * itemsPerPage + index;
  if (records.value[originalIndex]) {
    records.value[originalIndex].status = '已领取';
    alert(`恭喜你成功领取了 ${records.value[originalIndex].name}！`);
  }
};
</script> 