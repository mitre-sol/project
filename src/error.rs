use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},

    #[error("Insufficient balance to meet the requested withdrawal amount")]
    InsufficientBalanceForWithdraw {},
}
