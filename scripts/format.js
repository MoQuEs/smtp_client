import { exec } from './exec.js';

exec('cargo fmt');
exec('npx prettier --write src');
