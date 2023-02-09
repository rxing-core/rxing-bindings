# rxing-bindings

Node.js bindings for the [rxing](https://github.com/hschimke/rxing) library, which is a Rust port of the [ZXing](https://github.com/zxing/zxing) barcode library.

## Installation

```bash
npm install @rxing/rxing
```

## Usage

### Decode a barcode from an image

```js
const { decode } = require('@rxing/rxing');
const result = decode('path/to/image.png'); // image path or a base64 encoded string

console.log(result);
```

### Encode a barcode

```js
const { encode } = require('@rxing/rxing');
const result = encode('hello world'/*, { outputFile: 'path/to/qrcode.jpg' } */);

fs.writeFileSync('hello-world.jpg', result);
```

## API

### decode(input: string, options?: DecodeOptions): DecodeResult |  Array\<DecodeResult\> | null

Decode a barcode from a file or base64 string.

#### input

Type: `string`

The path to the image file or a base64 encoded string.

#### options

Type: `DecodeOptions`

### encode(data: string, options?: EncodeOptions): Buffer

Encode a barcode from a string, returning a buffer representing the image.

#### data

Type: `string`

The data to encode.

#### options

Type: `EncodeOptions`

## [License](LICENSE)
