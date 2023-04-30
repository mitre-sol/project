#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, coin, coins, from_binary, Coin, Deps, DepsMut};

    // use crate::contract::{execute, instantiate, query};
    use crate::contract::{instantiate, query};
    use crate::error::ContractError;
    // use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse};
    use crate::msg::{InstantiateMsg, QueryMsg, GetOwnerResponse};
    use crate::state::Config;

    fn mock_init(deps: DepsMut) {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    #[test]
    fn init_and_check_addr_from_config() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {})
            .expect("contract successfully handles InstantiateMsg");

        assert_config_state(
            deps.as_ref(),
            Config {
                owner: info.sender,
            }
        );
    }

    #[test]
    fn init_and_check_addr_from_query() {
        let mut deps = mock_dependencies();
        mock_init(deps.as_mut());

        // Querying for the owner of the contract results in address "creator", as defined in mock_init.
        let info = mock_info("Alice", &coins(2, "token"));
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner{}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!("creator", value.address)
    }

}
