import fs from 'fs/promises'
import test from 'ava'

import {decode, encode} from '../index.js'
import * as url from 'url';
import path from 'path';

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

test('encode', async (t) => {
    const result = encode('hello, world')
    await fs.writeFile(path.join(__dirname, '__qrcode.jpg'), result)
    t.is(decode(path.join(__dirname, '__qrcode.jpg')).text, 'hello, world')
})

test('encode output to file', async (t) => {
    encode('hello, world', { outputFile: path.join(__dirname, '__output.jpg') })
    t.is(decode(path.join(__dirname, '__output.jpg')).text, 'hello, world')
})

test('decode', (t) => {
    t.is(decode(path.join(__dirname, 'qrcode.jpg')).text, 'hello, world')
})

test('decode base64', async (t) => {
    const base64 = await fs.readFile(path.join(__dirname, 'qrcode.jpg'), { encoding: 'base64' });
    t.is(decode(base64).text, 'hello, world')
})

test('decode base64 data url', async (t) => {
    const base64 = await fs.readFile(path.join(__dirname, 'qrcode.jpg'), { encoding: 'base64' });
    t.is(decode(`data:image/jpeg;base64,${base64}`).text, 'hello, world')
})
