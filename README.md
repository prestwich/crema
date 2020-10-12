### Crema -- Espresso Testnet BLS fuzzer

This is a small fuzzing utility for the BLS precompiles on the Espresso Testnet.
It generates random inputs to the precompile functions and checks the output of 
the golang geth node against the rust
[eip1962 library](https://github.com/matter-labs/eip1962).


#### Running

- First start an Espresso testnet node.
    - see instructions [here](https://github.com/prestwich/go-ethereum)
- Then `$ cargo run`