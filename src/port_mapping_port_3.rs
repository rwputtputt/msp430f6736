#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P3.0 mapping register"]
    pub p3map0: crate::Reg<p3map0::P3MAP0_SPEC>,
    #[doc = "0x01 - Port P3.1 mapping register"]
    pub p3map1: crate::Reg<p3map1::P3MAP1_SPEC>,
    #[doc = "0x02 - Port P3.2 mapping register"]
    pub p3map2: crate::Reg<p3map2::P3MAP2_SPEC>,
    #[doc = "0x03 - Port P3.3 mapping register"]
    pub p3map3: crate::Reg<p3map3::P3MAP3_SPEC>,
    #[doc = "0x04 - Port P3.4 mapping register"]
    pub p3map4: crate::Reg<p3map4::P3MAP4_SPEC>,
    #[doc = "0x05 - Port P3.5 mapping register"]
    pub p3map5: crate::Reg<p3map5::P3MAP5_SPEC>,
    #[doc = "0x06 - Port P3.6 mapping register"]
    pub p3map6: crate::Reg<p3map6::P3MAP6_SPEC>,
    #[doc = "0x07 - Port P3.7 mapping register"]
    pub p3map7: crate::Reg<p3map7::P3MAP7_SPEC>,
}
#[doc = "P3MAP0 register accessor: an alias for `Reg<P3MAP0_SPEC>`"]
pub type P3MAP0 = crate::Reg<p3map0::P3MAP0_SPEC>;
#[doc = "Port P3.0 mapping register"]
pub mod p3map0;
#[doc = "P3MAP1 register accessor: an alias for `Reg<P3MAP1_SPEC>`"]
pub type P3MAP1 = crate::Reg<p3map1::P3MAP1_SPEC>;
#[doc = "Port P3.1 mapping register"]
pub mod p3map1;
#[doc = "P3MAP2 register accessor: an alias for `Reg<P3MAP2_SPEC>`"]
pub type P3MAP2 = crate::Reg<p3map2::P3MAP2_SPEC>;
#[doc = "Port P3.2 mapping register"]
pub mod p3map2;
#[doc = "P3MAP3 register accessor: an alias for `Reg<P3MAP3_SPEC>`"]
pub type P3MAP3 = crate::Reg<p3map3::P3MAP3_SPEC>;
#[doc = "Port P3.3 mapping register"]
pub mod p3map3;
#[doc = "P3MAP4 register accessor: an alias for `Reg<P3MAP4_SPEC>`"]
pub type P3MAP4 = crate::Reg<p3map4::P3MAP4_SPEC>;
#[doc = "Port P3.4 mapping register"]
pub mod p3map4;
#[doc = "P3MAP5 register accessor: an alias for `Reg<P3MAP5_SPEC>`"]
pub type P3MAP5 = crate::Reg<p3map5::P3MAP5_SPEC>;
#[doc = "Port P3.5 mapping register"]
pub mod p3map5;
#[doc = "P3MAP6 register accessor: an alias for `Reg<P3MAP6_SPEC>`"]
pub type P3MAP6 = crate::Reg<p3map6::P3MAP6_SPEC>;
#[doc = "Port P3.6 mapping register"]
pub mod p3map6;
#[doc = "P3MAP7 register accessor: an alias for `Reg<P3MAP7_SPEC>`"]
pub type P3MAP7 = crate::Reg<p3map7::P3MAP7_SPEC>;
#[doc = "Port P3.7 mapping register"]
pub mod p3map7;
