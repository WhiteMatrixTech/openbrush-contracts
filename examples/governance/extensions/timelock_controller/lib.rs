#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(AccessControl, TimelockController)]
#[openbrush::contract]
pub mod my_timelock_controller {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            timelock_controller::Internal::_init_with_admin(
                &mut instance,
                Some(caller),
                min_delay,
                proposers,
                executors,
            );

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::timelock_controller::timelockcontroller_external::TimelockController;

        #[rustfmt::skip]
        use super::*;

        use test_helpers::address_of;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn can_schedule<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();
            let address = contract.account_id;
            let transaction = Transaction {
                callee: Some(address.clone()),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let id = {
                let _msg = call.hash_operation(transaction.clone(), None, salt);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let is_operation_pending = {
                let _msg =call.is_operation_pending(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_pending, false);

            let schedule_tx = {
                let _msg = call.schedule(transaction.clone(), None, salt, 0);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let is_operation_pending = {
                let _msg =call.is_operation_pending(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_pending, true);

            let is_operation_ready = {
                let _msg =call.is_operation_ready(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_ready, true);

            let is_operation_done = {
                let _msg =call.is_operation_done(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn schedule_and_execute_without_input_data(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();
            let address = contract.account_id;
            let transaction = Transaction {
                callee: Some(address.clone()),
                selector: ink::selector_bytes!("TimelockController::get_min_delay"),
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let id = {
                let _msg = call.hash_operation(transaction.clone(), None, salt);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let schedule_tx = {
                let _msg = call.schedule(transaction.clone(), None, salt, 0);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let is_operation_done = {
                let _msg =call.is_operation_done(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, false);

            let execute_tx = {
                let _msg = call.execute(transaction.clone(), None, salt);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("execute failed")
            }
            .return_value();

            assert_eq!(execute_tx, Ok(()));

            let is_operation_done = {
                let _msg =call.is_operation_done(id);
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(is_operation_done, true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn schedule_and_execute_by_passing_value_into_update_delay_and_update(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let new_min_delay: u64 = 15;
            let address = contract.account_id;

            let transaction = Transaction {
                callee: Some(address.clone()),
                selector: ink::selector_bytes!("TimelockController::update_delay"),
                input: new_min_delay.to_le_bytes().to_vec(),
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = call.schedule(transaction.clone(), None, salt, 0);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let get_min_delay = {
                let _msg = call.get_min_delay();
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_min_delay, 0);

            let execute_tx = {
                let _msg = call.execute(transaction.clone(), None, salt);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("execute failed")
            }
            .return_value();

            assert_eq!(execute_tx, Ok(()));

            let get_min_delay = {
                let _msg = call.get_min_delay();
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(get_min_delay, new_min_delay);

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_schedule_because_signer_is_not_proposal(clientclient: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();
            let address = contract.account_id;

            let transaction = Transaction {
                callee: Some(address.clone()),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = call.schedule(transaction.clone(), None, salt, 0);
                client.call_dry_run(&ink_e2e::charlie(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(schedule_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_execute_because_signer_is_not_executor(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();
            let address = contract.account_id;

            let transaction = Transaction {
                callee: Some(address.clone()),
                selector: [0, 0, 0, 0],
                input: vec![],
                transferred_value: 0,
                gas_limit: 0,
            };

            let salt = [0; 32];

            let schedule_tx = {
                let _msg = call.schedule(transaction.clone(), None, salt, 0);
                client
                    .call(&ink_e2e::bob(), &_msg, 0, None)
                    .await
                    .expect("schedule failed")
            }
            .return_value();

            assert_eq!(schedule_tx, Ok(()));

            let execute_tx = {
                let _msg = call.execute(transaction.clone(), None, salt);
                client.call_dry_run(&ink_e2e::charlie(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(execute_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_update_delay(client: Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(0, vec![address_of!(Bob)], vec![address_of!(Bob)]);
            let contract = client
                .instantiate("my_timelock_controller", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let mut call = contract.call::<Contract>();

            let update_delay_tx = {
                let _msg = call.update_delay(15);
                client.call_dry_run(&ink_e2e::bob(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(update_delay_tx, Err(_)));

            Ok(())
        }
    }
}
