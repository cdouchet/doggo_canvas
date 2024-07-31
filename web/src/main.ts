import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import Home from "./components/Home.vue";
import Board from "./components/Board.vue";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
  { path: "/", component: Home },
  {
    path: "/board/:id",
    component: Board,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
