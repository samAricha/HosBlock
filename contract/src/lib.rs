use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{near_bindgen, env, AccountId, Promise};

mod patient;
mod med_record;

use patient::Patient;
use med_record::MedRecord;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PatientRecord {
    patients: LookupMap<AccountId, Patient>,
}

impl Default for PatientRecord {
  fn default() -> Self {
    Self {
      patients: LookupMap::new(b"c"),
    }
  }
}

#[near_bindgen]
impl PatientRecord {
    
    // Add a new record object to patients's record
     
    #[payable]
    pub fn add_record(&mut self, diagnosis: String, hospital_name: String, medicine_administered: String,
        date_of_admission: String, date_of_release: String,
        allergies_recorded: String, price: u64) {
        // Get user account id
        let signer = env::predecessor_account_id();

        // get attached deposit
        let deposit = env::attached_deposit();

        // Get initial storage space used
        let initial_storage = env::storage_usage();

        // Checking if the patient already exists 
        if let Some(mut patient) = self.patients.get(&signer) {
            // Update patient object with the record info if patient is present
            patient.add(
                diagnosis,
                hospital_name, 
                medicine_administered,
                date_of_admission,
                date_of_release, 
                allergies_recorded, 
                price as f64
            );
            // Update Patient object on the blockchain
            self.patients.insert(&signer, &patient);

            // Pay storage cost
            self.pay_for_storage(initial_storage, deposit);
        } else {
            // Initialize a new Patient object if patient is not present
            let mut patient = Patient::new_patient();

            // Update patient object with the record info
            patient.add(
                diagnosis,
                hospital_name, 
                medicine_administered,
                date_of_admission,
                date_of_release, 
                allergies_recorded, 
                price as f64
            );

            // Persist patient object on blockchain
            self.patients.insert(&signer, &patient);

            // Settle storage cost
            self.pay_for_storage(initial_storage, deposit);
        }
    }

    
    //Retreive/Get a paginated patient record list.
  
    pub fn read_record(&self, start: u32, limit: u32) -> Option<Vec<MedRecord>> {
        // Get patient account id
        let signer = env::predecessor_account_id();

        // Check if patient record exist in records
        if let Some(patient) = self.patients.get(&signer) {
            // Get a list of record objects in patient records
            let records : Vec<MedRecord> = patient.show(start, limit);
            // Return the list
            Some(records)
        } else {
            // Return empty list
            Some(vec![])
        }
    }

  
    // Remove/Delete a patient object from the patients records given its id (index)
     
    pub fn delete_record(&mut self, id: u64) -> Option<MedRecord> {
        // Get user account id
        let signer = env::predecessor_account_id();

        // Get initial storage space occupied
        let initial_storage = env::storage_usage();

        // Check if user record exist in users storage
        if let Some(mut patient) = self.patients.get(&signer) {
            // Delete the car object from user wishlist
            let removed_record = patient.remove(id);

            // Update user object on blockchain
            self.patients.insert(&signer, &patient);

            // Credit the tokens unlocked after releasing storage space
            self.refund_storage_cost(initial_storage);

            // Return deleted car object
            Some(removed_record)
        } else {
            // Return Null
            None
        }
    }


   
    // Settles storage expenses
   
    fn pay_for_storage(&self, initial_storage: u64, attached_storage_cost: u128) {
        // Get Current Storage
        let current_storage = env::storage_usage();
        
        // Get Storage Used
        let storage_used = current_storage - initial_storage;
        
        // Get Storage cost per byte
        let storage_cost: u128 = env::storage_byte_cost();
        
        // Get payable storage fee
        if let Some(total_storage_cost) = storage_cost.checked_mul(storage_used as u128) {
            // Check if user attached enough tokens to cater for storage
            assert!(attached_storage_cost >= total_storage_cost, "Insufficient funds!");
            
            // Check for balance
            let excess_balance = attached_storage_cost - total_storage_cost;
            if excess_balance > 0 {
                // Return excess tokens to user
                self.return_excess_tokens(excess_balance);
            }
        }
    }
    
    
    // Sends back excess tokens to user
    
    pub fn return_excess_tokens(&self, excess_balance: u128) {
        // Get signer address
        let signer = env::predecessor_account_id();
        
        // Send back excess
        Promise::new(signer).transfer(excess_balance);
    }

    
    // Refunds user on storage release
     
    fn refund_storage_cost(&self, initial_storage: u64) {
        // Get current storage space
        let current_storage = env::storage_usage();

        // Compute storage space released
        let storage_released = initial_storage - current_storage;

        // Get storage unit price (per byte)
        let storage_unit_price = env::storage_byte_cost();

        // Compute total refundable storage cost
        if let Some(refundable_storage_cost) = storage_unit_price.checked_mul(storage_released.into()) {
            // Transfer to user wallet address
            self.return_excess_tokens(refundable_storage_cost);
        } else {
            panic!("Error calculating storage cost");
        }
    }

}





/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;    
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::


    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .attached_deposit(1000000000000000000000000)
            .build()
    }
    
    fn get_params() -> (String, String, String, String, String, String, u64) {
        let diagnosis: String = String::from("Diarrhea");
        let hospital_name: String = String::from("CGH"); 
        let medicine_administered: String = String::from("Flagyl");
        let date_of_admission: String = String::from("21/04/2022");
        let date_of_release: String = String::from("21/04/2022"); 
        let allergies_recorded: String = String::from("Protein Allergies");
        let price: u64 = 1000;

        (diagnosis, hospital_name, medicine_administered, date_of_admission, date_of_release, allergies_recorded,price)
    }

    #[test]
    fn add_to_records() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = PatientRecord::default();
        let params = get_params();

        contract.add_record(params.0, params.1, params.2, params.3, params.4, params.5, params.6);

        if let Some(records) = contract.read_record(0, 3) {
            assert_eq!(1, records.len());
            let test_params = get_params();
            assert_eq!(&records[0].medicine_administered, &test_params.2);
        } else {
            panic!("Error in the code");
        }
        
    }

    #[test]
    fn remove_from_records() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = PatientRecord::default();
        let params = get_params();
        contract.add_record(params.0, params.1, params.2, params.3, params.4, params.5, params.6);

        if let Some(records) = contract.read_record(0, 3) {
            assert_eq!(1, records.len());
        } else {
            panic!("Error reading records");
        }

        // Remove functionality
        contract.delete_record(0);

        if let Some(records) = contract.read_record(0, 3) {
            assert_eq!(0, records.len());
        } else {
            panic!("Error reading records");
        }
    }
}

