## tinybacktest

Fast and hackable backtest engine written in Rust.

## Getting Started

### Installation

```bash
cargo build
cargo run
```

### Example

See main.rs for example


### Data format 

The .csv file you are using should have the following __core__ headers <code>["open", "low", "high", "close", "volume", "signal"]</code>.<br>

If you wish to generate signals on-the-fly, you can modify the <code>Dataframe</code> constructor to suit your needs.

### TODO

* Add the core backtest loop
* Create a <code>Strategy</code> abstract class with strategy logic
* Create <code>Trade</code> class to define a trade object
* Implement a simple toy strategy

## Important?

As per the "tiny" in the name, repo should not exceed 1000loc

