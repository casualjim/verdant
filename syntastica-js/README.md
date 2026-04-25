# `verdant-js`

Modern and easy syntax highlighting using tree-sitter; use
[`verdant`](https://crates.io/crates/verdant) from JavaScript/TypeScript.

The full JavaScript/TypeScript API docs can be found
[here](https://rubixdev.github.io/verdant/js/).

## Basic Usage

```ts
import verdant from '@verdant/core'

// initialize the module
await verdant.init()

// load some languages
await verdant.loadLanguage('node_modules/@verdant/lang-rust/rust.wasm')
await verdant.loadLanguage(
    'node_modules/@verdant/lang-javascript/javascript.wasm',
)

// highlight a piece of code once
const rustInput = `fn main() {\n    println!("Hello, World!");\n}`
const rustOutput = verdant.highlight(rustInput, 'rust', 'one::dark')
document.getElementById('rust-code').innerHTML = rustOutput

// highlight a piece of code multiple times
const jsInput = `console.log('Hello, World!')`
const highlights = verdant.process(jsInput, 'javascript')

// - first to HTML again
const jsOutput1 = verdant.render(highlights, 'gruvbox::dark')
document.getElementById('js-code').innerHTML = jsOutput1

// - then for terminals (e.g. for usage in nodejs CLIs)
const jsOutput2 = verdant.render(highlights, 'one::deep', 'terminal')
console.log(jsOutput2)
```
