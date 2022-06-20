#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P1.0 mapping register"]
    pub p1map0: crate::Reg<p1map0::P1MAP0_SPEC>,
    #[doc = "0x01 - Port P1.1 mapping register"]
    pub p1map1: crate::Reg<p1map1::P1MAP1_SPEC>,
    #[doc = "0x02 - Port P1.2 mapping register"]
    pub p1map2: crate::Reg<p1map2::P1MAP2_SPEC>,
    #[doc = "0x03 - Port P1.3 mapping register"]
    pub p1map3: crate::Reg<p1map3::P1MAP3_SPEC>,
    #[doc = "0x04 - Port P1.4 mapping register"]
    pub p1map4: crate::Reg<p1map4::P1MAP4_SPEC>,
    #[doc = "0x05 - Port P1.5 mapping register"]
    pub p1map5: crate::Reg<p1map5::P1MAP5_SPEC>,
    #[doc = "0x06 - Port P1.6 mapping register"]
    pub p1map6: crate::Reg<p1map6::P1MAP6_SPEC>,
    #[doc = "0x07 - Port P1.7 mapping register"]
    pub p1map7: crate::Reg<p1map7::P1MAP7_SPEC>,
}
#[doc = "P1MAP0 register accessor: an alias for `Reg<P1MAP0_SPEC>`"]
pub type P1MAP0 = crate::Reg<p1map0::P1MAP0_SPEC>;
#[doc = "Port P1.0 mapping register"]
pub mod p1map0;
#[doc = "P1MAP1 register accessor: an alias for `Reg<P1MAP1_SPEC>`"]
pub type P1MAP1 = crate::Reg<p1map1::P1MAP1_SPEC>;
#[doc = "Port P1.1 mapping register"]
pub mod p1map1;
#[doc = "P1MAP2 register accessor: an alias for `Reg<P1MAP2_SPEC>`"]
pub type P1MAP2 = crate::Reg<p1map2::P1MAP2_SPEC>;
#[doc = "Port P1.2 mapping register"]
pub mod p1map2;
#[doc = "P1MAP3 register accessor: an alias for `Reg<P1MAP3_SPEC>`"]
pub type P1MAP3 = crate::Reg<p1map3::P1MAP3_SPEC>;
#[doc = "Port P1.3 mapping register"]
pub mod p1map3;
#[doc = "P1MAP4 register accessor: an alias for `Reg<P1MAP4_SPEC>`"]
pub type P1MAP4 = crate::Reg<p1map4::P1MAP4_SPEC>;
#[doc = "Port P1.4 mapping register"]
pub mod p1map4;
#[doc = "P1MAP5 register accessor: an alias for `Reg<P1MAP5_SPEC>`"]
pub type P1MAP5 = crate::Reg<p1map5::P1MAP5_SPEC>;
#[doc = "Port P1.5 mapping register"]
pub mod p1map5;
#[doc = "P1MAP6 register accessor: an alias for `Reg<P1MAP6_SPEC>`"]
pub type P1MAP6 = crate::Reg<p1map6::P1MAP6_SPEC>;
#[doc = "Port P1.6 mapping register"]
pub mod p1map6;
#[doc = "P1MAP7 register accessor: an alias for `Reg<P1MAP7_SPEC>`"]
pub type P1MAP7 = crate::Reg<p1map7::P1MAP7_SPEC>;
#[doc = "Port P1.7 mapping register"]
pub mod p1map7;
