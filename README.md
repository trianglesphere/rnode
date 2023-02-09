<img align="right" width="150" height="150" top="100" src="./assets/rsnode.png">

# rsnode • [![tests](https://github.com/trianglesphere/rsnode/actions/workflows/test.yml/badge.svg?label=tests)](https://github.com/trianglesphere/rsnode/actions/workflows/test.yml) ![license](https://img.shields.io/github/license/trianglesphere/rsnode?label=license) [![benches](https://github.com/trianglesphere/rsnode/actions/workflows/benchmarks.yml/badge.svg?label=benches)](https://github.com/trianglesphere/rsnode/actions/workflows/benchmarks.yml)

`rsnode` is an experimental fault-proof service built in pure Rust.

### Checklist

- [ ] Derivation Pipeline
    - [ ] TODO
- [ ] Execution revm Backend
    - [ ] New Transaction
    - [ ] State processing around new tx types
    - [ ] Execute transactions
    - [ ] Create post state
- [ ] Backend Metadata
    - [ ] Previous block hash
    - [ ] List of txs
    - [ ] Gas Limit
- [ ] Preimage Oracle
    - [ ] TODO

### Details

Execution is handled by a modified version of the [revm](https://github.com/bluealloy/revm) in a fork called [op-revm](https://github.com/refcell/op-revm).

Execution Differences:
- The modified transaction type introduces a few new fields: ``, ``, ``, ``...



### Usage

TODO

### License

TODO

