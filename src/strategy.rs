
trait Strategy {
    fn backtest(&self, df: &DataFrame); // change it to a result struct
    fn take_profit(&self);
    fn take_stop_loss(&self);
    fn check_entry(&self);
    fn check_exit(&self);
}

pub struct BasicStrategy{
    trades: Vec<Trade>,
    net_pnl: f64,
    cost: f64,
    balance: f64,
    equity: f64,
}

impl BasicStrategy{

    fn new() -> BasicStrategy{
        BasicStrategy{
            trades: Vec::new(),
            net_pnl: 0.0,
            cost: 0.0,
            balance: 0.0,
            equity: 0.0
        }
    }

    /// The main backtest loop
    ///
    fn backtest(&mut self, df: &DataFrame) -> f64 {
        
        for (i, date) in df.time.iter().enumerate() {
            continue;
        }

        return 42.0;
    }

}
