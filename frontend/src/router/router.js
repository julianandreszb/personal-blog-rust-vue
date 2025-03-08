import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      children: [
        {
          path: '',
          name: 'posts',
          default: true,
          component: () => import('../views/PostsView.vue'),
        },
        {
          path: '/:slug',
          name: 'post',
          component: () => import('../views/PostView.vue'),
          props: true
        },
      ]
    },
  ],
})

export default router
