// 使用 `alloc` 是因为似乎不能在嵌套类中使用 `HashMap`（`Encode` trait 不满足）。作为权变，使用 `alloc::collections::BTreeMap`。
extern crate alloc;
use alloc::collections::BTreeMap;
use ink_prelude::string::String;
use ink_storage::traits::{PackedLayout, SpreadLayout};
use scale::{Decode, Encode};

#[derive(Debug, Clone, Eq, PartialEq, Decode, Encode, PackedLayout, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
/// ResMetadata 包含要传入链码的资源的元数据
pub struct Inner {
    pub id: String,
    pub value: String,
    pub my_value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Decode, Encode, PackedLayout, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Outer {
    pub id: String,
    pub inner: Inner,
    pub my_inner: Inner,
    pub extensions: BTreeMap<String, String>,
}
