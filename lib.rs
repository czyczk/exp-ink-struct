#![cfg_attr(not(feature = "std"), no_std)]

pub mod data;
pub mod error_code;
pub mod extension;
pub mod model;

use ink_lang as ink;

#[ink::contract(dynamic_storage_allocator = true)]
mod blbc {
    use crate::{
        data, error_code,
        model::data::{Inner, Outer},
    };
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct Blbc {
        pub res_outer_map: HashMap<String, Outer>,
        pub res_inner_map: HashMap<String, Inner>,
    }

    #[ink(event)]
    pub struct StructCreated {
        pub event_id: String,
        pub struct_id: String,
    }

    impl Blbc {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                //res_outer_map: Default::default(),
                //res_inner_map: Default::default(),
                res_outer_map: HashMap::new(),
                res_inner_map: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_inner(
            &mut self,
            inner_json_str: String,
            event_id: Option<String>,
        ) -> Result<(), String> {
            data::create_inner(self, inner_json_str, event_id)
        }

        #[ink(message)]
        pub fn create_outer(
            &mut self,
            outer_json_str: String,
            event_id: Option<String>,
        ) -> Result<(), String> {
            data::create_outer(self, outer_json_str, event_id)
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
            let blbc = Blbc::default();
            assert_eq!(blbc.res_inner_map.len(), 0);
            assert_eq!(blbc.res_outer_map.len(), 0);
        }

        #[ink::test]
        fn create_inner_works() {
            // Prepare
            let mut blbc = Blbc::default();
            let struct_id: String = "111".into();
            let inner = Inner {
                id: struct_id.clone(),
                value: "v".into(),
                my_value: "mv".into(),
            };

            // Serialize `inner`
            let inner_json_str = serde_json::to_string(&inner).unwrap();

            // Invoke with sinner and expect the return value to be Ok()
            assert!(blbc.create_inner(inner_json_str, None).is_ok());

            // Check if the data in map is as expected
            assert_eq!(blbc.res_inner_map.get(&struct_id), Some(&inner));
        }

        #[ink::test]
        fn create_outer_works() {
            // Prepare
            let mut blbc = Blbc::default();
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

            // Serialize `inner` and `outer`
            let inner_json_str = serde_json::to_string(&inner).unwrap();
            let outer_json_str = serde_json::to_string(&outer).unwrap();

            // Invoke with inner and expect the return value to be Ok()
            assert!(blbc.create_inner(inner_json_str, None).is_ok());

            // Invoke with outer and expect the return value to be Ok()
            assert!(blbc.create_outer(outer_json_str, None).is_ok());

            // Check if the data in map is as expected
            assert_eq!(blbc.res_inner_map.get(&inner_struct_id), Some(&inner));
            assert_eq!(blbc.res_outer_map.get(&outer_struct_id), Some(&outer));
        }
    }
}
