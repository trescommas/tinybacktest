# tinybacktest (In development)

Fast and hackable backtest engine written in Rust.

## Getting Started

### Installation

```bash
cargo build
cargo run
```

### Example

See <code>/src/main.rs</code> 


### Data format 

The .csv file in the example has the following __core__ headers <code>["open", "low", "high", "close", "volume", "signal"]</code>.<br>

If your data is different, you can modify the <code>Dataframe</code> struct to suit your needs.

### TODO

* Add the core backtest loop
* Create a <code>Strategy</code> abstract class with strategy logic
* Implement a simple toy strategy

## Important?

As per the "tiny" in the name, repo should not exceed 1000loc

