import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../pages/HomePage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: HomePage },
    { path: '/query', component: () => import('../pages/QueryPage.vue') },
    { path: '/projects', component: () => import('../pages/ProjectsPage.vue') },
    { path: '/about', redirect: '/' },
  ],
})

export default router
