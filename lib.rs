#![cfg_attr(not(feature = "std"), no_std)]

pub mod data;
pub mod error_code;
pub mod extension;
pub mod model;

use ink_lang as ink;

#[ink::contract]
mod structcontract {
    use crate::{
        data, error_code,
        model::data::{Inner, Outer},
    };
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct StructContract {
        pub res_outer_map: Mapping<String, Outer>,
        pub res_inner_map: Mapping<String, Inner>,
    }

    #[ink(event)]
    pub struct StructCreated {
        pub event_id: String,
        pub struct_id: String,
    }

    impl StructContract {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_: &mut Self| {})
        }

        #[ink(message)]
        pub fn create_inner(
            &mut self,
            inner: Inner,
            event_id: Option<String>,
        ) -> Result<(), String> {
            data::create_inner(self, inner, event_id)
        }

        #[ink(message)]
        pub fn create_outer(
            &mut self,
            outer: Outer,
            event_id: Option<String>,
        ) -> Result<(), String> {
            data::create_outer(self, outer, event_id)
        }

        #[ink(message)]
        pub fn get_inner(&self, struct_id: String) -> Result<Inner, String> {
            // 若未找到则返回 CODE_NOT_FOUND
            let inner = match self.res_inner_map.get(&struct_id) {
                Some(it) => it,
                None => return Err(error_code::CODE_NOT_FOUND.into()),
            };

            // 合约现还不支持范型，故不能指定 lifetime，只能把有所有权的东西传出。
            return Ok(inner.clone());
        }

        #[ink(message)]
        pub fn get_outer(&self, struct_id: String) -> Result<Outer, String> {
            // 若未找到则返回 CODE_NOT_FOUND
            let outer = match self.res_outer_map.get(&struct_id) {
                Some(it) => it,
                None => return Err(error_code::CODE_NOT_FOUND.into()),
            };

            // 合约现还不支持范型，故不能指定 lifetime，只能把有所有权的东西传出。
            return Ok(outer.clone());
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        extern crate alloc;

        use crate::model::data::{Inner, Outer};
        use alloc::collections::BTreeMap;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let c = StructContract::default();
            // No way to count a map in the current version
            // assert_eq!(c.res_inner_map.len(), 0);
            // assert_eq!(c.res_outer_map.len(), 0);
        }

        #[ink::test]
        fn create_inner_works() {
            // Prepare
            let mut c = StructContract::default();
            let struct_id: String = "111".into();
            let inner = Inner {
                id: struct_id.clone(),
                value: "v".into(),
                my_value: "mv".into(),
            };

            // Invoke with sinner and expect the return value to be Ok()
            assert!(c.create_inner(inner.clone(), None).is_ok());

            // Check if the data in map is as expected
            assert_eq!(c.res_inner_map.get(&struct_id), Some(inner));
        }

        #[ink::test]
        fn create_outer_works() {
            // Prepare
            let mut c = StructContract::default();
            let inner_struct_id: String = "111".into();
            let inner = Inner {
                id: inner_struct_id.clone(),
                value: "v".into(),
                my_value: "mv".into(),
            };

            let outer_struct_id: String = "222".into();
            let outer = Outer {
                id: outer_struct_id.clone(),
                inner: inner.clone(),
                my_inner: inner.clone(),
                extensions: BTreeMap::new(),
            };

            // Invoke with inner and expect the return value to be Ok()
            assert!(c.create_inner(inner.clone(), None).is_ok());

            // Invoke with outer and expect the return value to be Ok()
            assert!(c.create_outer(outer.clone(), None).is_ok());

            // Check if the data in map is as expected
            assert_eq!(c.res_inner_map.get(&inner_struct_id), Some(inner));
            assert_eq!(c.res_outer_map.get(&outer_struct_id), Some(outer));
        }
    }
}
