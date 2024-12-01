export function createThemeStore() {
  let theme = $state<'light' | 'dark'>('dark');
  const isDark = $derived(theme === 'dark'); 

  function toggle() {
    theme = isDark ? 'light' : 'dark';
  }
  
  $effect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  })

  return {
    get theme() {
      return theme;
    },
    set theme(value: 'light' | 'dark') {
      theme = value;
    },
    get isDark() {
      return isDark;
    },
    toggle
  };
}

export type ThemeStore = ReturnType<typeof createThemeStore>;
