#[metis_lang::contract]
pub mod contract {
    use ink_prelude::{string::String, vec::Vec};
    use metis_erc721 as erc721;
    pub use metis_erc721::{Error, Result, TokenId};
    use metis_lang::{import, metis};

    ///721合约， A ERC721 contract.
    #[ink(storage)]
    #[import(erc721)]
    pub struct Erc721 {
        erc721: erc721::Data<Erc721>,
    }

    ///Transfer事件， Emitted when `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    #[metis(erc721)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub token_id: TokenId,
    }

    ///Approval事件， Emitted when `owner` enables `approved` to manage the `token_id` token.
    #[ink(event)]
    #[metis(erc721)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: Option<AccountId>,
        pub token_id: TokenId,
    }

    ///ApprovalForAll事件， Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    #[ink(event)]
    #[metis(erc721)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub operator: AccountId,
        pub approved: bool,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc721::Impl<Erc721> for Erc721 {
        /// Hook that is called before any token transfer. This includes minting
        /// and burning.
        ///
        /// Calling conditions:
        ///
        ///from和to非空时，token_id将被转移。 - When `from` and `to` are both non-zero, `from`'s `token_id` will be
        /// transferred to `to`.
        ///from为空时，将为to铸造token_id。 - When `from` is zero, `token_id` will be minted for `to`.
        ///to为空时，将注销token_id。 - When `to` is zero, `from`'s `token_id` will be burned.
        ///from和to不能同时为空。 - `from` and `to` are never both zero.
        fn _before_token_transfer(
            &mut self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _token_id: &TokenId,
        ) -> Result<()> {
            Ok(())
        }

        /// Base URI for computing `token_url`. If set, the resulting URI for each
        /// token will be the concatenation of the `baseURI` and the `token_id`. Empty
        /// by default, can be overriden in child contracts.
        fn _base_url(&self) -> String {
            String::from("https://test/")
        }
    }

    impl Erc721 {
        ///构造函数， the constructor of the contract
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut instance = Self {
                erc721: erc721::Data::new(),
            };

            erc721::Impl::init(&mut instance, name, symbol);

            // other logic

            instance
        }

        ///铸造token， For test to mint
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, token_id: TokenId) -> Result<()> {
            erc721::Impl::mint(self, &to, &token_id)
        }

        ///注销token For burn
        #[ink(message)]
        pub fn burn(&mut self, token_id: TokenId) -> Result<()> {
            erc721::Impl::burn(self, &token_id)
        }

        ///返回token的名称， Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc721::Impl::name(self)
        }

        ///返回标记的符号（简称）， Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc721::Impl::symbol(self)
        }

        ///返回token_id标识符， Returns the Uniform Resource Identifier (URI) for `token_id` token.
        #[ink(message)]
        pub fn token_url(&self, token_id: TokenId) -> String {
            erc721::Impl::token_url(self, &token_id)
        }

        ///返回所有者帐户中的token令牌数， @dev Returns the number of tokens in ``owner``'s account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            erc721::Impl::balance_of(self, &owner)
        }

        ///返回token的所有者， @dev Returns the owner of the `token_id` token.
        ///
        /// Requirements:
        ///
        /// - `token_id` must exist.
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> AccountId {
            erc721::Impl::owner_of(self, &token_id)
        }

        ///返回为token批准的帐户， @dev Returns the account approved for `token_id` token.
        ///
        /// Requirements:
        ///
        /// - `token_id` must exist.
        #[ink(message)]
        pub fn get_approved(&self, token_id: TokenId) -> Option<AccountId> {
            erc721::Impl::get_approved(self, &token_id)
        }

        ///如果允许operator管理“owner的所有资产，则返回， @dev Returns if the `operator` is allowed to manage all of the assets of `owner`.
        ///
        /// See {setApprovalForAll}
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            erc721::Impl::is_approved_for_all(self, &owner, &operator)
        }

        ///批准token转移， @dev Gives permission to `to` to transfer `token_id` token to another account.
        /// The approval is cleared when the token is transferred.
        ///
        /// Only a single account can be approved at a time, so approving the zero address clears previous approvals.
        ///
        /// Requirements:
        ///
        /// - The caller must own the token or be an approved operator.
        /// - `token_id` must exist.
        ///
        /// Emits an {Approval} event.
        #[ink(message)]
        pub fn approve(&mut self, to: Option<AccountId>, token_id: TokenId) {
            erc721::Impl::approve(self, to, &token_id)
        }

        ///批准或删除operator作为调用者的操作员， @dev Approve or remove `operator` as an operator for the caller.
        /// Operators can call {transferFrom} or {safeTransferFrom} for any token owned by the caller.
        ///
        /// Requirements:
        ///
        /// - The `operator` cannot be the caller.
        ///
        /// Emits an {ApprovalForAll} event.
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            erc721::Impl::set_approval_for_all(self, operator, approved)
        }

        ///转移token方法， @dev Transfers `token_id` token from `from` to `to`.
        ///
        /// WARNING: Usage of this method is discouraged, use {safeTransferFrom} whenever possible.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must be owned by `from`.
        /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
        ) -> Result<()> {
            erc721::Impl::transfer_from(self, from, to, token_id)
        }

        ///安全转移token方法， @dev Safely transfers `token_id` token from `from` to `to`, checking first that contract recipients
        /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must exist and be owned by `from`.
        /// - If the caller is not `from`, it must be have been allowed to move this token by either {approve} or {setApprovalForAll}.
        /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
        ) -> Result<()> {
            erc721::Impl::safe_transfer_from(self, from, to, token_id)
        }

        ///带数据的安全转移token方法， @dev Safely transfers `token_id` token from `from` to `to`.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must exist and be owned by `from`.
        /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
        /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn safe_transfer_from_with_data(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            data: Vec<u8>,
        ) -> Result<()> {
            erc721::Impl::safe_transfer_from_with_data(self, from, to, token_id, data)
        }
    }
}
