use ink_lang as ink;

#[ink::contract]
mod token {
    #[ink(storage)]
    pub struct Token {
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        total_supply: Balance,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, initial_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            Self {
                balances,
                total_supply: initial_supply,
                owner: caller,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            let sender_balance = self.balance_of(caller);
            if sender_balance < value {
                return false;
            }

            self.balances.insert(caller, sender_balance - value);
            let receiver_balance = self.balance_of(to);
            self.balances.insert(to, receiver_balance + value);
            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(to),
                value,
            });
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_contract_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut contract = Token::new(1_000);
            assert_eq!(contract.total_supply(), 1_000);
            assert_eq!(contract.balance_of(accounts.alice), 1_000);
            assert_eq!(contract.balance_of(accounts.bob), 0);

            assert!(contract.transfer(accounts.bob, 100));
            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }
    }
}
