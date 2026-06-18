import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../pages/HomePage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: HomePage },
    { path: '/query', component: () => import('../pages/QueryPage.vue') },
    { path: '/use-cases', component: () => import('../pages/UseCasesPage.vue') },
    { path: '/projects', redirect: '/use-cases' },
    { path: '/about', redirect: '/' },
  ],
})

export default router
