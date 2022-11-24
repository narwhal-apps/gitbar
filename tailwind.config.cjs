/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        blue: {
          50: '#eff7ff',
          100: '#dbedfe',
          200: '#bfe0fe',
          300: '#93cdfd',
          400: '#60b1fa',
          500: '#3b91f6',
          600: '#1f6feb',
          700: '#1d5dd8',
          800: '#1e4baf',
          900: '#1e438a',
        },
        slate: {
          50: '#f4f6f7',
          100: '#e2e8eb',
          200: '#c8d2d9',
          300: '#a2b3be',
          400: '#758c9b',
          500: '#5a7180',
          600: '#4d5e6d',
          700: '#43505b',
          800: '#3c454e',
          900: '#30363d',
        },
        stone: {
          50: '#f4f7f9',
          100: '#d9e4ee',
          200: '#b2c8dd',
          300: '#84a3c4',
          400: '#5a7ea7',
          500: '#41628b',
          600: '#324b6f',
          700: '#2b3f5a',
          800: '#263449',
          900: '#0d1117',
        },
        gray: {
          50: '#f2f6f9',
          100: '#dfe8ee',
          200: '#c2d3df',
          300: '#97b3c9',
          400: '#668caa',
          500: '#4a7090',
          600: '#405d7a',
          700: '#394d65',
          800: '#354355',
          900: '#161b22',
        },
      },
    },
  },
  plugins: [],
};
