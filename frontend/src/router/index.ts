import { createRouter, createWebHistory } from 'vue-router'
import AboutPage from '../pages/AboutPage.vue'
import HomePage from '../pages/HomePage.vue'
import ProjectsPage from '../pages/ProjectsPage.vue'
import QueryPage from '../pages/QueryPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: HomePage },
    { path: '/query', component: QueryPage },
    { path: '/projects', component: ProjectsPage },
    { path: '/about', component: AboutPage },
  ],
})

export default router
