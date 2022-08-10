use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen};

use crate::med_record::MedRecord;

/**
 * User structure
 */
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Patient {
 patient_record: Vec<MedRecord>,
}

impl Default for Patient {
  fn default() -> Self {
    Self {
      patient_record: vec![]
    }
  }
}

#[near_bindgen]
impl Patient {
  // Initializes a new patient object
 pub fn new_patient() -> Self {
  Self {
    patient_record: vec![]
  }
 }

 /**
  * Adds a medical record object to the patient record
  */
 pub fn add_record(&mut self, diagnosis: String, hospital_name: String, medicine_administered: String,
  date_of_admission: String, date_of_release: String,
  allergies_recorded: String, price: f64) {
    let record: MedRecord = MedRecord::new(diagnosis,hospital_name, medicine_administered,
      date_of_admission,date_of_release, allergies_recorded, price);

        self.patient_record.push(record);
 }

 /**
  * Retrieves patient_record objects from the MedRecords Vec
  */
 pub fn show(&self, start: u32, limit: u32) -> Vec<MedRecord> {
  let result: Vec<MedRecord> = self.patient_record.iter().skip(start as usize).take(limit as usize).cloned().collect();
  result
 }

 /**
  * Deletes a patient_record object from the MedRecord Vec
  */
 pub fn remove(&mut self, index: u64) -> MedRecord {
  let size: u64 = self.patient_record.len() as u64;
  assert!(size > 0 && index < size, "Invalid medical record!");
  self.patient_record.remove(index as usize)
 }
}
