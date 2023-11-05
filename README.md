# wasm canvas perfermance test

## run

```shell
wasm-pack build --release
npm install
npm run serve
```

access http://localhost:8080?count=100

## example results

    iteration: 10000

            wasm init: 0.19999998807907104
            js init: 0

            wasm draw: 242.30000001192093
            js draw: 221.29999999701977

    iteration: 100

        wasm init: 0.5
        js init: 0.10000000894069672

        wasm draw: 1.1000000089406967
        js draw: 0.8999999910593033

    teration: 1000

        wasm init: 0.4000000059604645
        js init: 0

        wasm draw: 10.399999991059303
        js draw: 3.7000000029802322

    iteration: 15000

        wasm init: 0.7000000029802322
        js init: 0

        wasm draw: 537.7000000029802
        js draw: 492.3999999910593
