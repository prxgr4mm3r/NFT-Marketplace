#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34)]
#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        next_id: u8,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), id)
        }
    }

}