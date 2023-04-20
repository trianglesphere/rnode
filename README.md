<img align="right" width="150" height="150" top="100" src="./assets/rsnode.png">

# rnode • [![tests](https://github.com/trianglesphere/rnode/actions/workflows/test.yml/badge.svg?label=tests)](https://github.com/trianglesphere/rnode/actions/workflows/test.yml) ![license](https://img.shields.io/github/license/trianglesphere/rnode?label=license)

`rnode` is an experimental version of the optimism protocol built specifically to be
a fault proof program. It must go through the pre-image oracle & be fully determinisitic
with no multi-threaded or networked parts inside the core of derivation.

### Checklist

- [ ] Derivation Pipeline
    - [ ] System Config
    - [x] Inbox Address check
    - [x] Filter from authorized batcher
    - [x] Parse frames (basic)
    - [x] Parse frames (resilient to malformed data)
    - [x] Channel from frames
    - [x] Decode batches from channel
    - [ ] RLP bytes limit on channel
    - [ ] Batch Queue stage
    - [ ] Batch -> Attributes
- [ ] Execution revm Backend
    - [ ] New Deposit Transaction Type
    - [ ] State processing of deposits
    - [ ] Fee modifications
        - [ ] L1 Cost on non-deposits
        - [ ] Basefee to address
    - [ ] Execute transactions
    - [ ] Create post state
- [ ] L1 Preimage Oracle
    - [ ] MPT for transaction/receipts
    - [ ] Persist pre-images to disk
    - [ ] Run in online or offline pre-image mode
    - [ ] Run in pre-image generation mode
- [ ] L2 Preimage Oracle
    - [ ] State DB for execution
    - [ ] Implement pre-image oracle of MPT

### TODO

- Fake RPC provider for tests
- Use own block types
- Remove ethers core RLP & use reth-rlp
- Finish MPT
- Finish derivation
- Client / L1 Preimage Oracle API
    - Will be several layers here
- CLI command + usage

### Usage

TODO

### License

TODO

