#!/bin/bash 

SIGNER=aricha.testnet

source ./scripts/setting.conf


 #Add record to patient records
#  near call $SUB_ACCOUNT add_record '{"diagnosis": "Diarrhea", "hospital_name": "CGH", 
#                                      "medicine_administered": "Flagyl", 
#                                      "date_of_admission": "21 April 2022", 
#                                      "date_of_release": "21 May 2022", 
#                                      "allergies_recorded": "Protein Allergies",
#                                      "price": 1000 }' --accountId $SIGNER --amount 1

# Show patient records
near call $SUB_ACCOUNT read_record '{"start": 0, "limit": 10}' --accountId $SIGNER

# Remove record to patient records
#near call $SUB_ACCOUNT delete_record '{"id": 0}' --accountId $SIGNER
