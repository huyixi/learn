import { createSSRApp } from "vue";
import App from "./App.vue";
import '@/assets/scss/index.scss'
export function createApp() {
  const app = createSSRApp(App);
  return {
    app,
  };
}
