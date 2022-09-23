use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen};


#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]

//This is a declaration of the medical record object i.e MedRecord
pub struct MedRecord {
 pub diagnosis: String,
 pub hospital_name: String,
 pub medicine_administered: String,
 pub date_of_admission: String,
 pub date_of_release: String,
 pub allergies_recorded: String,
 pub price: f64,
}

#[near_bindgen]
impl MedRecord {
 pub fn new(diagnosis: String, hospital_name: String, medicine_administered: String, 
  date_of_admission: String, date_of_release: String,
  allergies_recorded: String, price: f64) -> Self {
      Self { 
        diagnosis,
        hospital_name, 
        medicine_administered, 
        date_of_admission, 
        date_of_release, 
        allergies_recorded,
        price
      }
 }
}