import { exec_and_forget } from './exec.js';

exec_and_forget('cargo fmt');
exec_and_forget('npx prettier --write src');
