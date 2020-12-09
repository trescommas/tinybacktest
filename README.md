## tinybacktest

Simple, fast and editable backtest engine written in Rust.

## Getting Started

### Data structure

tiny-backtest assumes that the .csv file you are using has the following (core) headers <code>["open", "low", "high", "close", "volume", "signals"]</code>.<br>

If you wish to generate signals on-the-fly, you can modify the <code>Dataframe</code> constructor to suit your needs.

### TODO

* Add the core backtest loop
* Create a <code>Strategy</code> abstract class with strategy logic
* Create <code>Trade</code> class to define a trade object
* Implement a simple toy strategy