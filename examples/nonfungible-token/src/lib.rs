use std::ops::Add;
use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, env, near_bindgen};
use near_sdk::json_types::U128;

pub type TokenId = u128;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NonfungibleToken {
    /// Owner of contract
    owner: AccountId,
    /// Total supply of all Token
    total_supply: Balance,
    /// Current account token list
    tokens: HashMap<TokenId, AccountId>,
    /// owner account to the total amount
    owner_to_tokens: HashMap<AccountId, Balance>,
    /// approval that an Account to send token on behalf of an owner
    approvals: HashMap<TokenId, AccountId>,
}

#[near_bindgen]
impl NonfungibleToken {
    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid");
        let total_supply = total_supply.into();
        assert!(!env::state_exists(), "Already initialized");
        let mut nft = Self {
            owner: owner_id.clone(),
            total_supply: 0 as Balance,
            tokens: HashMap::new(),
            owner_to_tokens: HashMap::new(),
            approvals: HashMap::new(),
        };
        nft.forge(total_supply);
        nft
    }

    /// forge of new Token
    pub fn forge(&mut self, value: U128) {
        let caller_id = env::predecessor_account_id();
        if caller_id != self.owner {
            env::panic(b"Only owner can forge new Token!");
        }
        let amount: u128 = value.into();
        let start_id = self.total_supply.add(1) as u128;
        let end_id = self.total_supply.add(amount) as u128;

        // loop through new tokens being forge
        for token_id in start_id..end_id {
            self.tokens.insert(token_id, caller_id.clone());
        }

        // update total supply of owner
        let from_owner_count = *self.owner_to_tokens.get(&caller_id).unwrap_or(&0);
        let amout: Balance = from_owner_count + amount;
        self.owner_to_tokens.insert(caller_id.clone(), amout);

        // update total supply
        self.total_supply += amount;
    }

    /// Returns the current total_supply
    pub fn total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    /// Returns the balance of the given accountId
    pub fn balance_of(&self, owner_id: &AccountId) -> U128 {
        match self.owner_to_tokens.get(owner_id) {
            Some(value) => {
                U128(*value)
            }
            None => {
                U128(0)
            }
        }
    }

    /// Transfers a token_id to a specified Account from the given AccountId
    pub fn transfer(&mut self, token_id: TokenId, to: AccountId) {
        let owner_id = env::predecessor_account_id();

        self.transfer_from(token_id, owner_id, to)
    }

    /// Transfers a token_id to a specified Account from the caller
    pub fn transfer_from(&mut self, token_id: TokenId, from: AccountId, to: AccountId) {
        if self.is_token_owner(token_id, from.clone()) {
            self.transfer_from_impl(token_id, from, to);
        } else {
            // not owner: check if caller is approved to move the token
            match self.approvals.get(&token_id) {
                Some(approval_id) => {
                    if *approval_id == from {
                        self.transfer_from_impl(token_id, from, to);
                    } else {
                        env::panic(b"Token_id not owned or approvaled the given `account_id`");
                    }
                }
                None => {
                    env::panic(b"Token_id not owned or approvaled the given `account_id`");
                }
            }
        }
    }

    /// Transfers a token_id to a specified Account from the caller
    fn transfer_from_impl(&mut self, token_id: TokenId, from: AccountId, to: AccountId) {
        let origin_id = self.tokens.get(&token_id).unwrap();
        let origin_balance = self.owner_to_tokens.get(origin_id).unwrap();
        if *origin_balance <= 1 {
            self.owner_to_tokens.insert(from, 0);
        } else {
            self.owner_to_tokens.insert(from, origin_balance - 1);
        }

        self.tokens.insert(token_id, to.clone());
        let to_balance = self.owner_to_tokens.get(&to);
        if let None = to_balance {
            self.owner_to_tokens.insert(to, 1);
        } else {
            self.owner_to_tokens.insert(to, *to_balance.unwrap() + 1);
        }
    }

    pub fn is_token_owner(&self, token_id: TokenId, owner_id: AccountId) -> bool {
        let owner = self.tokens.get(&token_id);
        match owner {
            Some(id) => {
                return *id == owner_id;
            }
            None => {
                return false;
            }
        }
    }


    // Grant the access to the given `account_id` for the given `tokenId` .
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    pub fn grant_access(&mut self, token_id: TokenId, account_id: AccountId) {
        let owner = self.tokens.get(&token_id);
        if let None = owner {
            env::panic(b"Only owner can grant access token to another account!");
        }
        let owner = owner.unwrap();
        let caller_id = env::predecessor_account_id().clone();
        if *owner != caller_id {
            env::panic(b"Only owner can grant access token to another account!");
        }
        self.approvals.insert(token_id, account_id);
    }

    // Revokes the access to the given `account_id` for the given `tokenId` .
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    pub fn revoke_access(&mut self, token_id: TokenId) {
        let owner = self.tokens.get(&token_id);
        if let None = owner {
            env::panic(b"Token does not exists for the give `tokenId`!");
        }
        let owner = owner.unwrap();
        let owner_id = env::predecessor_account_id().clone();
        if *owner != owner_id {
            env::panic(b"Only owner can grant access token to another account!");
        }
        self.approvals.remove(&token_id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    fn alice() -> AccountId {
        "alice.near".to_string()
    }

    fn bob() -> AccountId {
        "bob.near".to_string()
    }

    fn carol() -> AccountId {
        "carol.near".to_string()
    }

    fn catch_unwind_silent<F: FnOnce() -> R + std::panic::UnwindSafe, R>(
        f: F,
    ) -> std::thread::Result<R> {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(f);
        std::panic::set_hook(prev_hook);
        result
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_u128;
        let contract = NonfungibleToken::new(carol(), total_supply.into());
        assert_eq!(contract.balance_of(&carol()).0, total_supply);
        assert_eq!(contract.total_supply().0, total_supply);
    }

    #[test]
    fn test_transfer_from() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 1_000_u128;
        let token_id = 1 as TokenId;
        let to = alice();
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.transfer_from(token_id, carol(), to);
        assert_eq!(contract.balance_of(&alice()).0, 1);
    }

    #[test]
    fn test_transfer() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 10_000_u128;
        let token_id = 0 as TokenId;
        let to = alice();
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.transfer(token_id, to);
        assert_eq!(contract.balance_of(&alice()).0, 1);
    }

    #[test]
    fn test_balance_of() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 10_000_u128;
        let token_id = 0 as TokenId;
        let to = alice();
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.transfer(token_id, to);
        assert_eq!(contract.balance_of(&carol()).0, 9999);
    }

    #[test]
    fn test_forge() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 100_u128;
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.forge(10.into());
        assert_eq!(contract.total_supply().0, 110);
    }

    #[test]
    fn test_grant_access() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 100_u128;
        let token_id = 1_u128;
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.grant_access(token_id, bob());
        contract.transfer_from(token_id, bob(), alice());
        assert_eq!(contract.balance_of(&alice()).0, 1);
    }
    #[test]
    fn test_revoke_access() {
        let context = get_context(carol());
        testing_env!(context);
        let total_supply = 100_u128;
        let token_id = 1_u128;
        let mut contract = NonfungibleToken::new(carol(), total_supply.into());
        contract.grant_access(token_id, bob());
        contract.revoke_access(token_id);
        contract.transfer_from(token_id, bob(), alice());
        assert_eq!(contract.balance_of(&alice()).0, 1);
    }

}
