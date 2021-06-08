
pub trait ERC20 {
    fn new(name: String, symbol: String, initial_supply: u128) -> Self;

    fn name(&self) -> &String;

    fn symbol(&self) -> &String;

    fn total_supply(&self) -> u128;
    // other methods may be added
}