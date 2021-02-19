<div align="center">

  <h1><code>rs-csv2all</code></h1>

  <strong>A project to convert your csv files to other types of file using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>
  <h2>Built with 🦀🕸 by Alexandre Vardai</a></h2>
</div>

## About

[**📚 Read about wasm-pack template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo install` to clone wasm-pack template

```
$ cargo install --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
$ cd my-project
```

### 🛠️ Build with `wasm-pack and run with npm`

```
$ wasm-pack build 
$ cd www 
$ npm install && npm run start
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
$ wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
$ wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
