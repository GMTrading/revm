
use crate::{
    primitives::U256, Address, Precompile, PrecompileWithAddress, PrecompileError,
};

pub mod celo {
    use core::str::FromStr;

    use super::*;

    const ZERO_ADDRESS: Address = crate::u64_to_address(0);
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

            if caller != Address::from_str("0xcB8710e072aC4700eE7eD0C63B2f2102366a7a39").unwrap() {
                return Err(PrecompileError::CustomError("Unable to call transfer from unpermissioned address".to_string()));
            }

            let from = Address::from_slice(&input[0..32]);
            let to = Address::from_slice(&input[32..64]);

            let value = U256::from_le_slice(&input[64..96]);

            if from == ZERO_ADDRESS {
                host.add_balance(&to, value)?;
            } else {
                if host.get_balance(from)? < value {
                    return Err(PrecompileError::CustomError("Unable to call transfer from unpermissioned address".to_string()));
                }
                host.transfer(&from, &to, value)?;
            }
            Ok((9000, input.to_vec()))
        })
    );
}