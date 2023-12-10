
use crate::{
    primitives::U256, Address, Precompile, PrecompileWithAddress, PrecompileError,
};

pub mod celo {
    use core::str::FromStr;

    use revm_primitives::B256;

    use super::*;

    const ADDRESS: Address = crate::u64_to_address(0xFF - 2);


    pub const TRANSFER: PrecompileWithAddress = PrecompileWithAddress(
        ADDRESS,
        Precompile::Ext(|input, target_gas, caller, host | {
            if 9000 > target_gas {
                return Err(PrecompileError::OutOfGas);
            }

            if input.len() < 96 {
                return Err(PrecompileError::CustomError("Wrong data input length".to_string()));
            }

            if caller != Address::from_str("0x471ece3750da237f93b8e339c536989b8978a438").unwrap() {
                return Err(PrecompileError::CustomError("Unable to call transfer from unpermissioned address".to_string()));
            }

            let from = Address::from_slice(&input[12..32]);
            let to = Address::from_slice(&input[44..64]);

            let value = U256::from_be_slice(&input[64..96]);

            if from == Address::ZERO {
                host.add_balance(&to, value)?;
            } else {
                if host.get_balance(from)? < value {
                    return Err(PrecompileError::CustomError(format!("Address {from:?} lacks balance: {}, transfer value: {}", host.get_balance(from)?, value)));
                }
                host.transfer(&from, &to, value)?;
            }
            Ok((9000, input.to_vec()))
        })
    );
}