# uqoin-js

A tool to accees Uqoin core entities and algorithms.

## Build

```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --release --target bundler --out-dir uqoin-js-pkg
```

## Use in Vue 3

Install: `npm in ../uqoin-js/uqoin-js-pkg`

Extra dependency: `"vite-plugin-wasm": "^3.4.1"`

App.vue:

```js
/* App.vue */
import { onMounted } from 'vue'

onMounted(async () => {
  const uqoin = await import('uqoin-js')
  console.log(uqoin.hashOfU256(
    "E7646626CB303A9EEBAAD078ACD5632862232A27EF6426CC7D7A92251FBFEE94" + 
    "E7646626CB303A9EEBAAD078ACD56328DC4BFFC745FD5063738D9E10BF513204"
  ))
})
```

Config:

```js
/* vite.config.js */

...
import wasm from "vite-plugin-wasm"

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    wasm(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
```

About `wasm-bindgen`: https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html
