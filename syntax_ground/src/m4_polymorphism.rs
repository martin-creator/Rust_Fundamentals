use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Resukt<Address, &'static str>;
}

impl EthereumAddress for &str{
    fn convert_address(&self) -> Resukt<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid address"),
        }
    }
}

imp EthereumAddress for Address {
    fn convert_address(&self) -> Resukt<Address, &'static str> {
        Ok(self)
    }
}


fn get_ethereum_data<T: EthereumAddress>(address: T) -> Result<Address, &'static str> {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]

mode test {
    use super::*;

    #[test]
    fn test_poly(){
        let addr = Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635");
        .unwrap();
     }
     assert_eql!(addr, Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap() );

    let new_addr: Address = get_ethereum_data("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap();
    assert_eql!(new_addr, Address::from_str("0xf773566C15111f0a994028Bb0941e7DB5d8d2635").unwrap() );    

}