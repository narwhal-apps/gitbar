import { fileURLToPath } from 'url';
import { $, cd, chalk } from 'zx';
import fs from 'fs/promises';

const srcFile = fileURLToPath(new URL('../src-tauri/bindings/index.ts', import.meta.url));
const destFile = fileURLToPath(new URL('../src/types/index.ts', import.meta.url));

cd('src-tauri');

await $`cargo test export_bindings`;

await $`mv ${srcFile} ${destFile}`;

const content = await fs.readFile(destFile, 'utf-8');

const typeNames = content.match(/export type (\w+)/g).map(match => match.replace('export type ', ''));

console.log(chalk.grey(destFile));
console.log(chalk.green(`Successfully generated ${typeNames.length} TypeScript types:`));
console.log(chalk.blueBright(typeNames.join(', ')));

await $`pnpm exec prettier --write ${destFile}`;
