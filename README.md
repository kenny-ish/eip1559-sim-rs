# eip1559-sim-rs

Simulates how the EIP-1559 base fee reacts to a demand shock. The mechanism works like a feedback
controller: blocks above the 50% target raise the base fee by up to 12.5% and blocks below it lower
the fee.

```bash
cargo run -- --base 10 --shock-blocks 20 --blocks 60
```

During the shock every block is full. After it, demand depends on price: the higher the fee is above
its starting level, the less gas users want, so blocks stay below the target until the fee comes
back down.

It prints the base fee and fullness of every block, and how many blocks it took to get back within
5% of the starting fee.

```bash
cargo test
```
