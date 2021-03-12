### Build app
'''
wasm-pack build --target web --out-name wasm --out-dir ./static
'''

### Run app
'''
miniserve ./static --index index.html
'''
