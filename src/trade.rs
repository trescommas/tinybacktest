pub struct Trade{
    entry_price: f64,
    exit_price: f64,
    entry_time: u32,
    exit_time: u32,
}

impl Trade{
	pub	fn new() -> Trade{
		Trade{
			entry_price: 0.0,
			exit_price: 0.0,
			entry_time: 0,
			exit_time: 0,
		}
	
	}

}
