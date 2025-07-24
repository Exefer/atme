import adapter from "@sveltejs/adapter-static";
import { Config } from "@sveltejs/kit";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
  // Consult https://svelte.dev/docs/kit/integrations
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      pages: process.env.NODE_ENV === "production" ? "build" : "../backend/static"
    }),
    paths: {
      relative: false
    }
  }
} satisfies Config;
