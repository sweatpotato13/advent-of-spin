import { componentize } from '@bytecodealliance/componentize-js';
import { readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const enableAot = process.env.ENABLE_AOT == '1'

const jsSource = await readFile('scoring.js', 'utf8');

const { component } = await componentize(jsSource, {
  witPath: resolve('scoring.wit'),
  enableAot
});

await writeFile('scoring.component.wasm', component);