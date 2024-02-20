import { createSSRApp } from "vue";
import App from "./App.vue";
import './assets/input.css'
export function createApp() {
  const app = createSSRApp(App);
  return {
    app,
  };
}
