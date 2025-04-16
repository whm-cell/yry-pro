const resolve = require('path').resolve
module.exports = {
  content: [resolve(__dirname, 'index.html'), resolve(__dirname, 'src/**/*.{vue,ts}')],
  presets: [{
    theme: {
      extend: {},
    },
    css: {
      apply: true, // 启用@apply指令
    },
    prefix: '',
    shortcuts: {},
    rules: [
      // padding
      ['p-4', { padding: '1rem' }],
      ['p-6', { padding: '1.5rem' }],
      ['px-6', { 'padding-left': '1.5rem', 'padding-right': '1.5rem' }],
      ['px-4', { 'padding-left': '1rem', 'padding-right': '1rem' }],
      ['px-3', { 'padding-left': '0.75rem', 'padding-right': '0.75rem' }],
      ['py-4', { 'padding-top': '1rem', 'padding-bottom': '1rem' }],
      ['py-2', { 'padding-top': '0.5rem', 'padding-bottom': '0.5rem' }],
      ['py-3', { 'padding-top': '0.75rem', 'padding-bottom': '0.75rem' }],
      
      // margin
      ['mb-4', { 'margin-bottom': '1rem' }],
      ['mb-6', { 'margin-bottom': '1.5rem' }],
      ['mb-3', { 'margin-bottom': '0.75rem' }],
      ['mb-1', { 'margin-bottom': '0.25rem' }],
      ['mt-4', { 'margin-top': '1rem' }],
      ['ml-4', { 'margin-left': '1rem' }],
      ['ml-3', { 'margin-left': '0.75rem' }],
      
      // backgrounds
      ['bg-white', { 'background-color': 'white' }],
      ['bg-gray-50', { 'background-color': 'rgb(249, 250, 251)' }],
      ['bg-gray-200', { 'background-color': 'rgb(229, 231, 235)' }],
      ['bg-gray-300', { 'background-color': 'rgb(209, 213, 219)' }],
      ['bg-blue-500', { 'background-color': 'rgb(59, 130, 246)' }],
      ['bg-blue-600', { 'background-color': 'rgb(37, 99, 235)' }],
      
      // text
      ['text-white', { 'color': 'white' }],
      ['text-gray-500', { 'color': 'rgb(107, 114, 128)' }],
      ['text-gray-600', { 'color': 'rgb(75, 85, 99)' }],
      ['text-gray-700', { 'color': 'rgb(55, 65, 81)' }],
      ['text-gray-900', { 'color': 'rgb(17, 24, 39)' }],
      ['text-red-600', { 'color': 'rgb(220, 38, 38)' }],
      ['text-red-900', { 'color': 'rgb(127, 29, 29)' }],
      
      // font sizes
      ['text-sm', { 'font-size': '0.875rem', 'line-height': '1.25rem' }],
      ['text-lg', { 'font-size': '1.125rem', 'line-height': '1.75rem' }],
      ['text-xl', { 'font-size': '1.25rem', 'line-height': '1.75rem' }],
      
      // font weights
      ['font-medium', { 'font-weight': '500' }],
      ['font-semibold', { 'font-weight': '600' }],
      ['font-bold', { 'font-weight': '700' }],
      
      // borders, rounded
      ['rounded-md', { 'border-radius': '0.375rem' }],
      ['rounded-lg', { 'border-radius': '0.5rem' }],
      ['border', { 'border-width': '1px' }],
      ['border-gray-200', { 'border-color': 'rgb(229, 231, 235)' }],
      ['border-gray-300', { 'border-color': 'rgb(209, 213, 219)' }],
      ['shadow', { 'box-shadow': '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)' }],
      
      // flex, grid
      ['grid', { 'display': 'grid' }],
      ['flex', { 'display': 'flex' }],
      ['items-center', { 'align-items': 'center' }],
      ['grid-cols-1', { 'grid-template-columns': 'repeat(1, minmax(0, 1fr))' }],
      ['gap-4', { 'gap': '1rem' }],
      ['whitespace-nowrap', { 'white-space': 'nowrap' }],
      ['overflow-x-auto', { 'overflow-x': 'auto' }],
      ['overflow-hidden', { 'overflow': 'hidden' }],
      
      // width, height
      ['w-full', { 'width': '100%' }],
      ['min-w-full', { 'min-width': '100%' }],
      ['w-12', { 'width': '3rem' }],
      ['w-16', { 'width': '4rem' }],
      ['h-12', { 'height': '3rem' }],
      ['h-16', { 'height': '4rem' }],
      
      // object
      ['object-cover', { 'object-fit': 'cover' }],
      
      // more can be added as needed
    ],
  }],
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
