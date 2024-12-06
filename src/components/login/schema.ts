import { z } from 'zod';

export const loginSchema = z.object({
  token: z
    .string()
    .min(40) // GitHub PATs are at least 40 characters
    .max(255) // Setting a reasonable maximum length
    .regex(/^ghp_[A-Za-z0-9]{36}$/, "Must be a valid token starting with 'ghp_'"),
  hostname: z
    .string()
    .min(4) // Minimum length for a valid hostname
    .max(255) // Maximum length for a hostname
    .regex(/^github\.(com|[a-zA-Z0-9-]+\.[a-zA-Z0-9-]+)$/, 'Must be a valid hostname'),
});

export type LoginSchema = typeof loginSchema;
