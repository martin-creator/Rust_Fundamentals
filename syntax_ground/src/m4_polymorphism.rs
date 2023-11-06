use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}
 
impl EthereumAddress for &str{
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(self)
    }
}


fn get_ethereum_data<T: EthereumAddress>(address: T) -> Result<Address, &'static str> {
    let converted_address: Result<H160, &str> = address.convert_address().map(|addr| addr.as_fixed_bytes().into());

    converted_address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly(){
        let addr = Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap();
        assert_eq!(addr, Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap());

        let new_addr: Address = get_ethereum_data("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap();
        assert_eq!(new_addr, Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap());    
    }
}