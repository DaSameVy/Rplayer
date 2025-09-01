import { createRouter, createWebHistory } from 'vue-router';
import App from '../App.vue';
import VideoPlayerView from '../views/VideoPlayerView.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: App
  },
  {
    path: '/video-player',
    name: 'VideoPlayer',
    component: VideoPlayerView
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
