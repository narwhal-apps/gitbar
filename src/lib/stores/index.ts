import { getContext } from 'svelte';
import { AUTH_CONTEXT_KEY } from './constants';
import type { AuthContext } from './authContext.svelte';

export { default as AuthProvider } from './AuthProvider.svelte';
export function getAuthContext(): AuthContext {
  const context = getContext<AuthContext>(AUTH_CONTEXT_KEY);
  console.log('context :', context);

  if (!context) {
    throw new Error('Auth context must be used within an AuthProvider');
  }

  return context;
}
