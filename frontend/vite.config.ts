import { defineConfig, loadEnv } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from 'node:path'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const frontendEnv = loadEnv(mode, process.cwd(), "")
  const repoEnv = loadEnv(mode, path.resolve(process.cwd(), ".."), "")
  const backendUrl =
    frontendEnv.BACKEND_URL ??
    frontendEnv.VITE_BACKEND_URL ??
    repoEnv.BACKEND_URL ??
    repoEnv.VITE_BACKEND_URL ??
    ""

  return {
    plugins: [svelte()],
    define: {
      "import.meta.env.BACKEND_URL": JSON.stringify(backendUrl),
    },
  }
})
