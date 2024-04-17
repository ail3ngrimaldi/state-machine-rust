/* TODO: You might need to update your imports. */

use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	block_number: u32,
    /// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
    nonce: BTreeMap<String, u32>
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self { block_number: 0, nonce: BTreeMap::new() }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
		/* TODO: Return the current block number. */
		*self.block_number.get();
	}
    	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) -> u32{
		/* TODO: Increment the current block number by one. */
		let block_number = self.block_number + 1;
        block_number
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
        let incremented_nonce = self.nonce + 1;
        incremented_nonce
		
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following: */
            let mut system = super::Pallet::new();
		/*	- Increment the current block number. */
            system.inc_block_number();
		/*	- Increment the nonce of `alice`.*/
            system.inc_nonce(&"alice".to_string())
		/*	- Check the block number is what we expect.*/
        assert_eq!(balances.balance(&"alice".to_string()), 100);
		/*	- Check the nonce of `alice` is what we expect.*/
        assert_eq!(balances.balance(&"alice".to_string()), 100);
	}
}