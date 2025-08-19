import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/pages/index.vue'
import Success from '@/pages/success.vue'
import Failure from '@/pages/failure.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/success',
      name: 'success',
      component: Success
    },
    {
      path: '/failure',
      name: 'failure',
      component: Failure
    }
  ],
})

export default router