#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod marketplace {
    use ink::prelude::string::{String, ToString};
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use openbrush::contracts::{
        traits::{
            psp22::PSP22Ref,
            psp34::{Id, PSP34Ref},
        }
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        ownable : ownable::Data,
        listings : Mapping<u8, Listing>,
        next_id: u8,
    }

    #[ink(event)]
    pub struct  NFTTransfer {
        token_id : u128,
        to : AccountId,
        token_uri : String,
        price : u128
    }

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    #[derive(PartialEq, Eq, scale::Encode, scale::Decode)]
    pub struct Listing{
        contract_address : AccountId,
        token_id : Id,
        price : u128,
        seller : AccountId,
        token_address : AccountId,
        listed : bool
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MarketPlaceError {
        Ownable(ownable::OwnableError),
        Other(String),
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn create_listing(&mut self, _listing : Listing) -> Result<(), MarketPlaceError> {
            let owner = PSP34Ref::owner_of(&_listing.contract_address, _listing.token_id.clone());
            if owner.unwrap() != self.env().caller(){
                return Err(MarketPlaceError::Other("You are not owner of token".to_string()));
            }
            let allowance = PSP34Ref::allowance(&_listing.contract_address, self.env().caller(), self.env().account_id(), Some(_listing.token_id.clone()));
            if !allowance{
                return Err(MarketPlaceError::Other("Don't aproved".to_string()));
            }
            self.listings.insert(&self.next_id ,&_listing);
            self.next_id += 1;
            Ok(())
        }

        #[ink(message, payable)]
        pub fn buy(&mut self, _listing_id : u8) -> Result<(), MarketPlaceError> {
            let wraped_listing = self.listings.get(_listing_id);
            if wraped_listing == None{
                return Err(MarketPlaceError::Other("No listing with such ID".to_string()))
            }
            let listing = wraped_listing.unwrap();
            let allowance_to_pay = PSP22Ref::allowance(&listing.token_address, self.env().caller(), self.env().account_id());
            if allowance_to_pay < listing.price {
                return Err(MarketPlaceError::Other("Not enough money".to_string()));
            }
            PSP22Ref::transfer_from(&listing.token_address, self.env().caller(), listing.seller, listing.price, Vec::<u8>::new());
            PSP34Ref::transfer(&listing.contract_address, self.env().caller(), listing.token_id, Vec::<u8>::new());
            Self::cancel(self, _listing_id);
            Ok(())
        }

        #[ink(message)]
        pub fn cancel(&mut self, listing_id : u8) -> Result<(), MarketPlaceError>{
            let wraped_listing = self.listings.get(listing_id);
            if wraped_listing == None{
                return Err(MarketPlaceError::Other("No listing with such ID".to_string()));
            }
            let mut listing = wraped_listing.unwrap();
            if self.env().caller() != listing.seller
                && self.env().caller() != self.env().account_id(){
                return Err(MarketPlaceError::Other("You have no rights to cansel that listing".to_string()));
            }
            listing.listed = false;
            self.listings.insert(listing_id, &listing);
            Ok(())
        }

        #[ink[message]]
        pub fn check_listings(&mut self, only_listed : bool) -> Vec<(u8, Listing)> {
            let mut result = Vec::new();
            for i in 0..self.next_id {
                let wraped_listing = self.listings.get(i);
                if wraped_listing == None{continue}
                let listing = wraped_listing.unwrap();
                if listing.listed || !only_listed{
                    result.push((i, listing));
                }
            }
            result
        }

    }

}