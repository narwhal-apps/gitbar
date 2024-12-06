/** @type {import("prettier").Config} */
export default {
  tabWidth: 2,
  trailingComma: 'es5',
  semi: true,
  singleQuote: true,
  printWidth: 120,
  arrowParens: 'avoid',
  plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss'],
  overrides: [
    {
      files: '.src/**/*.svelte',
      options: {
        parser: 'svelte',
      },
    },
    {
      files: '.github/**/*',
      options: {
        tabWidth: 2,
        useTabs: false,
      },
    },
  ],
  tailwindFunctions: ['clsx', 'cn', 'tv'],
};
