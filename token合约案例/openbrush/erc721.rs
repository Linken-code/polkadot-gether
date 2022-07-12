#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    // Imports from ink!
    use ink_storage::traits::SpreadAllocate;

    // Imports from openbrush
    use openbrush::contracts::psp34::extensions::burnable::*;
    use openbrush::contracts::psp34::extensions::mintable::*;
    use openbrush::contracts::psp34::*;

    #[derive(Default, SpreadAllocate, PSP34Storage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
    }

    // Section contains default implementation without any modifications
    impl PSP34 for MyPSP34 {}
    impl PSP34Burnable for MyPSP34 {}
    impl PSP34Mintable for MyPSP34 {}

    impl MyPSP34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
