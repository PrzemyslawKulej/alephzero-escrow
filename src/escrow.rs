#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod escrow {
    use ink::contract_ref;
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;
    use psp22::PSP22;

    /// The Escrow contract stores token balances for users and interacts with a PSP22 token contract.
    #[ink(storage)]
    pub struct Escrow {
        /// Mapping from user accounts to their balances in the escrow.
        accounts: Mapping<AccountId, Balance>,
        /// The PSP22 token contract address.
        token: AccountId,
    }

    /// Event emitted when a deposit is made to the escrow.
    #[ink(event)]
    pub struct Deposited {
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    /// Event emitted when a withdrawal is made from the escrow.
    #[ink(event)]
    pub struct Withdrawn {
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    impl Escrow {
        /// Constructor to initialize the escrow with the PSP22 token contract address.
        #[ink(constructor)]
        pub fn new(token: AccountId) -> Self {
            Self {
                accounts: Mapping::new(),
                token,
            }
        }

        /// Approves the escrow contract to spend a specified amount of tokens on behalf of the caller.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: Balance) {
            let mut token: contract_ref!(PSP22) = self.token.into();
            token.approve(spender, amount).expect("Approve failed");
        }

        /// Deposits a specified amount of tokens into the escrow.
        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let contract_account_id = self.env().account_id();
            let mut token: contract_ref!(PSP22) = self.token.into();

            // Interactions: transfer tokens from caller to escrow contract
            token
                .transfer_from(caller, contract_account_id, amount, Vec::new())
                .expect("Transfer failed");

            // Effects: update state after successful transfer
            let balance = self.accounts.get(&caller).unwrap_or_default();
            let new_balance = balance.checked_add(amount).expect("Overflow detected");
            self.accounts.insert(caller, &new_balance);

            // Emit the Withdrawn event(Worked on ink 5.0.0, not working on ink 4.3)
            // self.env().emit_event(Deposited {
            //     from: caller,
            //     amount: amount,
            // });
        }

        /// Withdraws a specified amount of tokens from the escrow.
        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let balance = self.accounts.get(&caller).unwrap_or_default();

            // Checks: ensure sufficient balance
            assert!(balance >= amount, "Insufficient balance");

            // Effects: update state before making external call
            let new_balance = balance.checked_sub(amount).expect("Underflow detected");
            self.accounts.insert(caller, &new_balance);

            // Emit the Withdrawn event(Worked on ink 5.0.0, not working on ink 4.3)
            // self.env().emit_event(Withdrawn {
            //     to: caller,
            //     amount: amount,
            // });

            // Interactions: transfer tokens from escrow contract to caller
            let mut token: contract_ref!(PSP22) = self.token.into();
            token
                .transfer(caller, amount, Vec::new())
                .expect("Transfer failed");
        }

        /// Gets the balance of tokens for a specified account.
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
        fn test_get_balance() {
            let token: AccountId = AccountId::from([0x0; 32]);
            let contract = Escrow::new(token);

            let account = AccountId::from([0x1; 32]);
            let balance = contract.get_balance(account);
            assert_eq!(balance, 0);
        }
    }
}
