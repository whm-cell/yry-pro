<template>
  <div class="p-6 h-full">
    <div class="bg-white rounded-lg shadow-lg p-6 h-full">
      <h2 class="text-2xl font-bold mb-6 text-gray-800">个人资料</h2>
      
      <div class="flex flex-col md:flex-row">
        <!-- 左侧个人信息 -->
        <div class="w-full md:w-1/3 flex flex-col items-center p-4">
          <div class="w-40 h-40 rounded-full overflow-hidden border-4 border-blue-500 mb-4">
            <img src="https://via.placeholder.com/200" alt="用户头像" class="w-full h-full object-cover">
          </div>
          
          <button class="mt-2 bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded">
            更换头像
          </button>
          
          <div class="mt-6 w-full">
            <div class="bg-blue-50 rounded-lg p-4 mb-4">
              <div class="text-sm text-gray-500 mb-1">账户等级</div>
              <div class="flex items-center">
                <i class="fas fa-crown text-yellow-500 mr-2"></i>
                <span class="font-medium">黄金会员</span>
              </div>
            </div>
            
            <div class="bg-blue-50 rounded-lg p-4">
              <div class="text-sm text-gray-500 mb-1">抽奖次数</div>
              <div class="flex items-center">
                <i class="fas fa-ticket-alt text-green-500 mr-2"></i>
                <span class="font-medium">{{ userProfile.drawCount }}次/天</span>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 右侧资料编辑 -->
        <div class="w-full md:w-2/3 p-4">
          <div class="bg-gray-50 p-6 rounded-lg">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200">基本信息</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">用户名</label>
                <input type="text" v-model="userProfile.username" class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500">
              </div>
              
              <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">手机号</label>
                <input type="text" v-model="userProfile.phone" class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500">
              </div>
              
              <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">邮箱</label>
                <input type="email" v-model="userProfile.email" class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500">
              </div>
              
              <div class="form-group">
                <label class="block text-sm font-medium text-gray-700 mb-1">生日</label>
                <input type="date" v-model="userProfile.birthday" class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500">
              </div>
            </div>
          </div>
          
          <div class="mt-6 bg-gray-50 p-6 rounded-lg">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200">收货地址</h3>
            
            <div class="mb-4" v-for="(address, index) in userProfile.addresses" :key="index">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center">
                  <span class="font-medium">{{ address.name }}</span>
                  <span class="mx-2 text-gray-500">{{ address.phone }}</span>
                </div>
                <div>
                  <span class="text-blue-500 cursor-pointer hover:text-blue-700 mr-2" @click="editAddress(index)">编辑</span>
                  <span class="text-red-500 cursor-pointer hover:text-red-700" @click="deleteAddress(index)">删除</span>
                </div>
              </div>
              <div class="text-gray-600">
                {{ address.address }}
              </div>
            </div>
            
            <button class="flex items-center text-blue-500 hover:text-blue-700" @click="addAddress">
              <i class="fas fa-plus-circle mr-1"></i>
              <span>添加新地址</span>
            </button>
          </div>
          
          <div class="mt-6 flex justify-end">
            <button class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-8 rounded-lg" @click="saveProfile">
              保存修改
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue';

// 用户资料
const userProfile = reactive({
  username: '张三',
  phone: '138****6789',
  email: 'example@email.com',
  birthday: '1990-01-01',
  drawCount: 5,
  addresses: [
    {
      name: '张三',
      phone: '138****6789',
      address: '北京市海淀区中关村南大街5号'
    }
  ]
});

// 编辑地址
const editAddress = (index: number) => {
  alert(`编辑第${index + 1}个地址`);
};

// 删除地址
const deleteAddress = (index: number) => {
  if (confirm('确定要删除这个地址吗？')) {
    userProfile.addresses.splice(index, 1);
  }
};

// 添加地址
const addAddress = () => {
  userProfile.addresses.push({
    name: userProfile.username,
    phone: userProfile.phone,
    address: '请填写新地址'
  });
};

// 保存资料
const saveProfile = () => {
  alert('个人资料已更新！');
};
</script> 