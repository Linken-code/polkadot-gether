#[metis_lang::contract]
pub mod contract {
    // use erc20 component
    use metis_erc20 as erc20;
    use metis_lang::{import, metis};

    // use Error and Result for erc20
    pub use erc20::{Error, Result};

    /// ERC-20 contract.
    #[ink(storage)]
    #[import(erc20)]
    pub struct Erc20 {
        erc20: erc20::Data<Erc20>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub value: Balance,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::Impl<Erc20> for Erc20 {
        /// Hook that is called before any transfer of tokens. This includes
        /// minting and burning.
        ///
        /// Calling conditions:
        ///
        /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
        /// will be to transferred to `to`.
        /// - when `from` is zero, `amount` tokens will be minted for `to`.
        /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
        /// - `from` and `to` are never both zero.
        fn _before_token_transfer(
            &mut self,
            _from: &E::AccountId,
            _to: &E::AccountId,
            _amount: &E::Balance,
        ) -> Result<()> {
            // some logic

            Ok(())
        }
    }

    impl Erc20 {
        /// the constructor of the contract
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, decimals: u8, initial_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
            };

            erc20::Impl::init(&mut instance, name, symbol, decimals, initial_supply);

            // do some other logic here

            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc20::Impl::name(self)
        }

        /// Returns the symbol of the token,
        /// usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc20::Impl::symbol(self)
        }

        /// Returns the number of decimals used to
        /// get its user representation.
        /// For example, if `decimals` equals `2`,
        /// a balance of `505` tokens should
        /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
        ///
        /// Tokens usually opt for a value of 18,
        /// imitating the relationship between
        /// Ether and Wei in ETH. This is the value {ERC20} uses,
        /// unless this function is
        /// overridden;
        ///
        /// NOTE: This information is only used for _display_ purposes:
        /// it in no way affects any of the arithmetic of the contract
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            erc20::Impl::decimals(self)
        }

        /// Returns the amount of tokens in existence.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Impl::total_supply(self)
        }

        /// Returns the amount of tokens owned by `account`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Impl::balance_of(self, owner)
        }

        /// Returns the remaining number of tokens that `spender` will be
        /// allowed to spend on behalf of `owner` through {transferFrom}. This is
        /// zero by default.
        ///
        /// This value changes when {approve} or {transferFrom} are called.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Impl::allowance(self, owner, spender)
        }

        /// Moves `amount` tokens from the caller's account to `recipient`.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::transfer(self, to, value)
        }

        /// Sets `amount` as the allowance of `spender` over the caller's tokens.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// IMPORTANT: Beware that changing an allowance with this method brings
        /// the risk that someone may use both the old and the new allowance
        /// by unfortunate transaction ordering. One possible solution to
        /// mitigate this race condition is to first reduce the spender's
        /// allowance to 0 and set the desired value afterwards:
        /// <https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729>
        ///
        /// Emits an {Approval} event.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, spender, value)
        }

        /// Moves `amount` tokens from `sender` to `recipient` using the
        /// allowance mechanism. `amount` is then deducted from the caller's
        /// allowance.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc20::Impl::transfer_from(self, from, to, value)
        }

        fn _mint(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
            let null_account = E::AccountId::default();
            if account == null_account {
                return Err(Error::AccountIsZero);
            }

            self._before_token_transfer(&null_account, &account, &amount)?;

            let total_supply = self.get().total_supply();
            let account_balance = self.get().balance_of(&account);

            self.get_mut().set_total_supply(total_supply + amount);
            self.get_mut()
                .set_balance(account.clone(), account_balance + amount);

            self.emit_event_transfer(None, Some(account), amount);

            Ok(())
        }

        fn _burn(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
            let null_account = E::AccountId::default();

            if account == null_account {
                return Err(Error::AccountIsZero);
            }

            self._before_token_transfer(&account, &null_account, &amount)?;

            let account_balance = self.get().balance_of(&account);
            let total_supply = self.get().total_supply();

            if account_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            self.get_mut()
                .set_balance(account.clone(), account_balance - amount);
            self.get_mut().set_total_supply(total_supply - amount);

            self.emit_event_transfer(Some(account), None, amount);

            Ok(())
        }
    }
}
