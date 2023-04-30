#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, BankMsg, coin, coins, from_binary, Coin, Deps, DepsMut, Uint128};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse, GetBalanceResponse};
    use crate::state::Config;

    fn mock_init(deps: DepsMut) {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "usei"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn assert_balance(deps: Deps, address: Addr, expected_balance: Uint128) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::GetBalance {
                address: address
            },
        )
        .unwrap();

        let value: GetBalanceResponse = from_binary(&res).unwrap();
        assert_eq!(expected_balance, value.balance);
    }

    #[test]
    fn init_and_check_addr_from_config() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1, "usei"));
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
        let info = mock_info("Alice", &coins(2, "usei"));
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner{}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!("creator", value.address)
    }

    #[test]
    fn transfer_basic() {
        let mut deps = mock_dependencies();
        mock_init(deps.as_mut());

        // Querying for the owner of the contract results in address "creator", as defined in mock_init.
        let info_alice = mock_info("Alice", &coins(1000, "usei"));
        let bob_addr = Addr::unchecked("Bob");
        let carl_addr = Addr::unchecked("Carl");

        let transfer_msg = ExecuteMsg::Transfer {
            address1: bob_addr.clone(),
            address2: carl_addr.clone(),
            amount: Uint128::from(1000u32),
        };

        let res = execute(deps.as_mut(), mock_env(), info_alice, transfer_msg)
            .expect("Alice successfully transferes 1000 usei");
        assert_balance(deps.as_ref(), bob_addr, Uint128::from(500u32));
        assert_balance(deps.as_ref(), carl_addr, Uint128::from(500u32));
    }

    #[test]
    fn transfer_odd_amount() {
        let mut deps = mock_dependencies();
        mock_init(deps.as_mut());

        // Querying for the owner of the contract results in address "creator", as defined in mock_init.
        let info_alice = mock_info("Alice", &coins(5, "usei"));
        let bob_addr = Addr::unchecked("Bob");
        let carl_addr = Addr::unchecked("Carl");

        let transfer_msg = ExecuteMsg::Transfer {
            address1: bob_addr.clone(),
            address2: carl_addr.clone(),
            amount: Uint128::from(5u32),
        };

        let res = execute(deps.as_mut(), mock_env(), info_alice, transfer_msg)
            .expect("Alice successfully transferes 1000 usei");
        assert_balance(deps.as_ref(), bob_addr, Uint128::from(2u32));
        assert_balance(deps.as_ref(), carl_addr, Uint128::from(2u32));
    }

    #[test]
    fn get_balance_empty() {
        let mut deps = mock_dependencies();
        mock_init(deps.as_mut());

        // Bob's balance should be 0 usei as there have been no transfers to his addr.
        let bob_addr = Addr::unchecked("Bob");
        assert_balance(deps.as_ref(), bob_addr, Uint128::from(0u32));
    }

    #[test]
    fn withdraw_basic() {
        let mut deps = mock_dependencies();
        mock_init(deps.as_mut());

        let info_alice = mock_info("Alice", &coins(1000, "usei"));
        let bob_addr = Addr::unchecked("Bob");
        let carl_addr = Addr::unchecked("Carl");

        let transfer_msg = ExecuteMsg::Transfer {
            address1: bob_addr.clone(),
            address2: carl_addr.clone(),
            amount: Uint128::from(1000u32),
        };

        let transfer_res = execute(deps.as_mut(), mock_env(), info_alice, transfer_msg)
            .expect("Alice successfully transferes 1000 usei");

        // After Alice's transfer, Bob's balance should be 500.
        assert_balance(deps.as_ref(), bob_addr.clone(), Uint128::from(500u32));

        // Now attempt to withdraw 400 tokens as Bob.
        let info_bob = mock_info("Bob", &coins(0, "usei"));
        let withdraw_res = execute(deps.as_mut(), mock_env(), info_bob, ExecuteMsg::Withdraw { amount: Uint128::from(400u32)});

        // After withdraw, Bob's balance should be 100.
        assert_balance(deps.as_ref(), bob_addr.clone(), Uint128::from(100u32));

        // Unsure of this part:  To test Bob actually receives the funds, check 'withdraw_res' contains the expected BankMsg?
    }
}
