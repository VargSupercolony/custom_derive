
use function_name::named;

use erc20::ERC20;
use erc20_derive::ERC20;

#[derive(ERC20)]
pub struct ECR20Contract {
    name: String,
    symbol: String,
    total_supply: u128
}

impl ECR20Contract {
    fn new(name: String, symbol: String, total_supply: u128) -> Self {
        Self {
            name,
            symbol,
            total_supply
        }
    }
}

fn main() {

    let example_token = ECR20Contract::new("4irelabium".to_owned(), "4lb".to_owned(), 1000u128);

    println!("{}, {}, {}", &example_token.name, &example_token.symbol, &example_token.total_supply);
}
