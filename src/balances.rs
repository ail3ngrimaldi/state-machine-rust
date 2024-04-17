use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
	
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}
	
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
    pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		/*  TODO: Get the balance of account `caller`	*/
            let balance_from = self.balance(&caller);
		/*	- Get the balance of account `to`.*/
            let balance_to = self.balance(&to);
		/*	- Use safe math to calculate a `new_caller_balance`.*/
            let new_caller_balance = balance_from.checked_sub(amount).ok_or("Not enough funds.")?;
        /*	- Use safe math to calculate a `new_to_balance`.*/
            let new_to_balance = balance_to.checked_add(amount).ok_or("Overflow occurred.")?;
	    /*  - Insert the new balance of `caller`.*/
            self.set_balance(&caller, new_caller_balance);
		/*	- Insert the new balance of `to`. */
            self.set_balance(&to, new_to_balance);


		Ok(())
	}
   
}


#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() { 
        let mut balances = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_funds() {
        let mut balances = super::Pallet::new();
        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), 50);

        // Transfer funds from alice to bob
        assert!(balances.transfer("alice".to_string(), "bob".to_string(), 50).is_ok());

        // Check balances after transfer
        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 100);

        // Try to transfer more funds than alice has
        assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 60), Err("Not enough funds."));
    }
}