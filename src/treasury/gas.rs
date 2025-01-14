use near_sdk::Gas;

pub const GAS_FOR_GET_DEPOSITS: Gas = Gas(13_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(45_000_000_000_000);
pub const GAS_FOR_ADD_LIQUIDITY: Gas = Gas(20_000_000_000_000);
pub const GAS_FOR_REMOVE_LIQUIDITY: Gas = Gas(17_000_000_000_000);
pub const GAS_FOR_WITHDRAW: Gas = Gas(60_000_000_000_000);
pub const GAS_FOR_FINISH_BURNING: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_HANDLE_RESERVE: Gas = Gas(13_000_000_000_000);
#[cfg(not(feature = "mainnet"))]
pub const GAS_FOR_NEAR_DEPOSIT: Gas = Gas(25_000_000_000_000);
pub const GAS_FOR_NEAR_WITHDRAW: Gas = Gas(25_000_000_000_000);
#[cfg(not(feature = "mainnet"))]
pub const GAS_FOR_SWAP: Gas = Gas(30_000_000_000_000);
pub const GAS_FOR_HANDLE_EXCHANGE_RATE: Gas = Gas(15_000_000_000_000);
pub const GAS_SURPLUS: Gas = Gas(10_000_000_000_000);
