#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod dot_rebase {

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };


    // address public monetaryPolicy;

    // Couldn't find much on modifiers...
    // modifier onlyMonetaryPolicy() {
    //     require(msg.sender == monetaryPolicy);
    //     _; // Does this carry over?
    // }

    // bool private rebasePausedDeprecated;
    // bool private tokenPausedDeprecated;

    // modifier validRecipient(address to){
    //     require(to != address(0x0));
    //     require(to != address(this));
    //     _;
    // }

    

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DotRebase {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        /// Gons per fragment
        gons_per_fragment: Lazy<Balance>,
        /// Mapping from owner to number of owned token.
        balances: StorageHashMap<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        // Allowed fragments mapping
        // allowed_fragments: mapping(address => mapping(address => uint256)),
        DECIMALS: uint256,
        MAX_UINT256: uint256,
        INITIAL_FRAGMENTS_SUPPLY: uint256,
        TOTAL_GONS: uint256,
        MAX_SUPPLY: uint256,
        
    }

    // mapping(address => uint256) private _nonces;

    // function setMonetaryPolicy(address monetaryPolicy_) external {
    //     monetaryPolicy = monetaryPolicy_;

    // }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    impl DotRebase {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, initial_supply);
            let DECIMALS = 9;
            let MAX_UINT256 = uint256::max;
            let INITIAL_FRAGMENTS_SUPPLY = 50 * i32::pow(10, 6) * i32::pow(10, DECIMALS);
            let TOTAL_GONS = MAX_UINT256 - (MAX_UINT256 % INITIAL_FRAGMENTS_SUPPLY);
            let MAX_SUPPLY = MAX_UINT256;
            let instance = Self {
                total_supply: Lazy::new(initial_supply),
                balances,
                allowances: StorageHashMap::new(),
                gons_per_fragment: uint256,
                DECIMALS: 9,
                MAX_UINT256: uint256::max,
                INITIAL_FRAGMENTS_SUPPLY: 50 * i32::pow(10, 6) * i32::pow(10, DECIMALS),
                TOTAL_GONS: MAX_UINT256 - (MAX_UINT256 % INITIAL_FRAGMENTS_SUPPLY),
                MAX_SUPPLY: MAX_UINT256,
            };
        
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            instance
            
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        pub fn set_supply(&mut self, amount:Balance) {
            self.total_supply = Lazy::new(amount)
        }

        #[ink(message)]
        pub fn add_account(&mut self, account_id: AccountId, balance: Balance) {
            self.balances.insert(account_id, balance);
        }

        #[ink(message)]
        pub fn get_account_balance(&self, account_id: AccountId) -> Balance {
            *self.balances.get(&account_id).unwrap_or(&0)
        }


        #[ink(message)]
        pub fn rebase(&mut self, epoch: Balance, supply_delta: i128) -> Balance {

            if supply_delta == 0 {

                return *self.total_supply;
            }

            if supply_delta < 0 {
                *self.total_supply -= (supply_delta * -1) as u128;
            } else {
                *self.total_supply += supply_delta as u128;
            }

            if *self.total_supply > self.MAX_SUPPLY {
                *self.total_supply = self.MAX_SUPPLY;
            }

            *self.gons_per_fragment = self.TOTAL_GONS.div(*self.total_supply);

            //emit LogRebase(epoch, _totalSupply);
            *self.total_supply
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let dot_rebase = DotRebase::default();
            assert_eq!(dot_rebase.total_supply(), 0);
        }
    }
}
