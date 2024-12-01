import adapter from '@sveltejs/adapter-static' // This was changed from adapter-auto
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  onWarn: (warning) => {
    if (warning.code === 'a11y_no_static_element_interactions') return
    if (warning.code === 'a11y_click_events_have_key_events') return
    if (warning.code === 'a11y_no_noninteractive_element_interactions') return
    if (warning.code === 'a11y_consider_explicit_label') return
  },
  kit: {
    adapter: adapter(),
  },
}

export default config

