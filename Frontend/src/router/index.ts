import { createRouter, createWebHistory } from 'vue-router'
import Checkout from '@/pages/checkout.vue'
import Home from '@/pages/index.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/checkout',
      name: 'checkout',
      component: Checkout
    }
  ],
})

export default router