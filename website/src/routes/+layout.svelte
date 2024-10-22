<script lang="ts">
  import { page } from '$app/stores'
  import '../app.css'
  import Revealed from './Revealed.svelte'
  import { base } from '$app/paths'

  function removeTrailingSlash(str: string) {
    return str.replace(/\/+$/, '')
  }
</script>

<svelte:head>
  <title>GitBar</title>
  <meta name="description" content="Keep track of your github reviews" />

  <meta property="og:type" content="website" />
  <!-- <meta property="og:url" content="https://gitbar.space" /> -->
  <meta property="og:title" content="Gitbar" />
  <meta property="og:description" content="Keep track of your github reviews" />
  <!-- <meta property="og:image" content="https://gitbar.space/screenshot.png" /> -->

  <meta name="twitter:card" content="summary_large_image" />
  <!-- <meta property="twitter:domain" content="gitbar.space" /> -->
  <!-- <meta property="twitter:url" content="https://gitbar.space" /> -->
  <meta name="twitter:title" content="Gitbar" />
  <meta name="twitter:description" content="Keep track of your github reviews" />
  <!-- <meta name="twitter:image" content="https://gitbar.space/screenshot.png" /> -->
</svelte:head>

<nav
  class="mx-auto flex max-w-5xl items-center py-4 px-6 text-base text-white text-opacity-60 sm:justify-between"
>
  <Revealed class="sm:w-32" options={{ opacity: 0, x: -10, duration: 1000, delay: 300 }}>
    <a
      href={`${base}/`}
      class="flex items-center text-lg font-medium text-white ease-md sm:p-1.5 sm:text-xl"
      style="-webkit-tap-highlight-color: hsla(0, 0%, 100%, 0.1);"
    >
      <span class=" border-2 rounded-lg border-white px-2">GitBar</span>
    </a>
  </Revealed>
  <Revealed class="mx-1.5" options={{ opacity: 0, y: -10, duration: 1000, delay: 500 }}>
    <a
      class="mx-1.5 py-2 text-sm transition-colors duration-200 ease-md hover:text-white sm:text-base"
      class:font-medium={removeTrailingSlash($page.url.pathname) === base}
      class:text-white={removeTrailingSlash($page.url.pathname) === base}
      href={`${base}/`}>Overview</a
    >
  </Revealed>
  <Revealed
    class="ml-auto flex justify-end sm:ml-0 sm:w-32"
    options={{ opacity: 0, x: 10, duration: 1000, delay: 300 }}
  >
    <a
      class="icon github ease-md sm:p-1.5"
      href="https://github.com/narwhal-apps/gitbar"
      style="-webkit-tap-highlight-color: hsla(0, 0%, 100%, 0.1);"
      ><svg height="24" width="24" viewBox="-2 -2 28 28" xmlns="http://www.w3.org/2000/svg"
        ><path
          d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
          fill="none"
          stroke="#ffffff"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
        /></svg
      ></a
    >
  </Revealed>
</nav>

<slot />

<style lang="sass">
  :global(html)
    background-color: #000000
    -webkit-tap-highlight-color: transparent
  :global(body)
    font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif, Apple Color Emoji, Segoe UI Emoji
    font-size: 18px
    color: #f2f2f2
</style>
