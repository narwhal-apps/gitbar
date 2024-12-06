import { z } from 'zod';

export const loginSchema = z.object({
  token: z
    .string()
    .min(40, {message: 'Token must be at least 40 characters long'})
    .max(255)
    .regex(/^ghp_[A-Za-z0-9]{36}$/, "Must be a valid token starting with 'ghp_'"),
  hostname: z
    .string()
    .min(4)
    .max(255)
    .regex(/^github\.(com|[a-zA-Z0-9-]+\.[a-zA-Z0-9-]+)$/, 'Must be a valid hostname'),
});
