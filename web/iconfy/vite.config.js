import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite';
import  UnocssIcons from "@unocss/preset-icons";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),
  UnoCSS({
    presets: [
      UnocssIcons({
        prefix: 'i-',
        extraProperties: {
          display: 'inline-block'
        }
      })
    ]
  })],
})
