import { object, string, regex, minLength, maxLength, pipe } from 'valibot';

export const loginSchema = object({
  token: pipe(
    string(),
    minLength(40, 'Token must be at least 40 characters long'),
    maxLength(255),
    regex(/^ghp_[A-Za-z0-9]{36}$/, "Must be a valid token starting with 'ghp_'")
  ),
  hostname: pipe(
    string(),
    minLength(4),
    maxLength(255),
    regex(/^github\.(com|[a-zA-Z0-9-]+\.[a-zA-Z0-9-]+)$/, 'Must be a valid hostname')
  ),
});

export type LoginSchema = typeof loginSchema;
