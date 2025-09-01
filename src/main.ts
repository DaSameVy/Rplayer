import { createApp } from 'vue'
import './style.css'
import MainApp from './MainApp.vue'
import router from './router'

createApp(MainApp).use(router).mount('#app')
