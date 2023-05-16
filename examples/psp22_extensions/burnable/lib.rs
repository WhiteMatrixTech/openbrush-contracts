#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_burnable {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp22::extensions::burnable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for Contract {}
    impl PSP22Burnable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());

            instance
        }

        #[ink(message)]
        pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                self.burn(account.0, account.1)?;
            }
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::psp22::{
            extensions::burnable::psp22burnable_external::PSP22Burnable,
            psp22_external::PSP22,
        };
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_without_allowance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, bob), 0));
            assert!(matches!(balance_of!(client, address, alice), 100));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client.call(&ink_e2e::bob(), _msg, 0, None).await.expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn decreases_total_supply_after_burning(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 100));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(total_supply.return_value(), 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.burn(address_of!(bob), 10));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from_many(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(charlie), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 10));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn_from_many(vec![(address_of!(bob), 10), (address_of!(charlie), 10)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 0));
            assert!(matches!(balance_of_charlie, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_if_one_of_the_accounts_balance_exceeds_amount_to_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_psp22_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(charlie), 5, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn_from_many(vec![(address_of!(bob), 10), (address_of!(charlie), 10)]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(PSP22Error::InsufficientBalance)));

            let balance_of_bob = balance_of!(client, address, bob);
            let balance_of_charlie = balance_of!(client, address, charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            Ok(())
        }
    }
}
