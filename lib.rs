#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod escrow {
    use ink::contract_ref;
    use ink::storage::Mapping;
    use psp22::PSP22;

    #[ink(storage)]
    pub struct Escrow {
        accounts: Mapping<AccountId, Balance>,
        token: AccountId,
    }

    #[ink(event)]
    pub struct Deposited {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdrawn {
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    impl Escrow {
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
            Self {
                accounts: Mapping::new(),
                token,
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let contract_account_id = self.env().account_id();
            let mut token: contract_ref!(PSP22) = self.token.into();

            token
                .transfer_from(caller, contract_account_id, amount, Vec::new())
                .expect("Transfer failed");

            let balance = self.accounts.get(&caller).unwrap_or_default();
            let new_balance = balance.checked_add(amount).expect("Overflow detected");
            self.accounts.insert(caller, &new_balance);

            self.env().emit_event(Deposited {
                from: caller,
                amount: amount,
            });
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let balance = self.accounts.get(&caller).unwrap_or_default();

            assert!(balance >= amount, "Insufficient balance");

            let new_balance = balance.checked_sub(amount).expect("Underflow detected");
            self.accounts.insert(caller, &new_balance);
            self.env().emit_event(Withdrawn {
                to: caller,
                amount: amount,
            });

            let mut token: contract_ref!(PSP22) = self.token.into();
            token
                .transfer(caller, amount, Vec::new())
                .expect("Transfer failed");
        }

        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            let token: contract_ref!(PSP22) = self.token.into();
            token.balance_of(account)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_new() {
            let token: AccountId = AccountId::from([0x0; 32]);
            let contract = Escrow::new(token);
            assert_eq!(contract.token, token);

            let caller = AccountId::from([0x1; 32]);
            assert_eq!(contract.accounts.get(&caller), None);
        }
    }
}
