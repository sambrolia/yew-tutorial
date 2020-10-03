cargo install wasm-pack
apt install npm
npm install --global rollup

# Build and run
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
python3 -m http.server 8080