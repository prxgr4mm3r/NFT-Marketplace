#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22)]
#[openbrush::contract]
mod psp22 {

    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Psp22 {
        #[storage_field]
        psp22 : psp22::Data,
    }

    impl Psp22 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply : Balance) -> Self {
            let mut instance = Self::default();
            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Sould mint");
            instance
        }
    }
}
