# you need to read `MakeFile`
### to create new project $`cargo new yew-app`
### to run the app $`trunk serve`

# using [wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
1. $`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
2. $`wasm-pack build --target web`
3. $`wasm-pack build --target bundler`
4. for Bundle $`rollup ./main.js --format iife --file ./pkg/bundle.js`
4. $`python -m http.server 8000`
