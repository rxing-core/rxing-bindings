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

test('decode', (t) => {
    t.is(decode(path.join(__dirname, 'qrcode.jpg')).text, 'hello, world')
})

