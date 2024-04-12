// first convert the wasm file to base64
const data = await Bun.file('./target/wasm32-unknown-unknown/release/optimized.wasm').arrayBuffer()
const uint8Array = Array.from(new Uint8Array(data))
const base64 = btoa(String.fromCharCode.apply(null, uint8Array))
const code = `export default '${base64}'\n`
await Bun.write('./lib/u-shaper.wasm.ts', code)

// now we can build the js glue
await Bun.build({
  entrypoints: ['./lib/index.ts'],
  outdir: './dist',
  minify: true,
  sourcemap: 'external',
  splitting: true,
  target: 'browser'
})
