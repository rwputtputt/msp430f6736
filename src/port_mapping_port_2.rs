#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P2.0 mapping register"]
    pub p2map0: crate::Reg<p2map0::P2MAP0_SPEC>,
    #[doc = "0x01 - Port P2.1 mapping register"]
    pub p2map1: crate::Reg<p2map1::P2MAP1_SPEC>,
    #[doc = "0x02 - Port P2.2 mapping register"]
    pub p2map2: crate::Reg<p2map2::P2MAP2_SPEC>,
    #[doc = "0x03 - Port P2.3 mapping register"]
    pub p2map3: crate::Reg<p2map3::P2MAP3_SPEC>,
    #[doc = "0x04 - Port P2.4 mapping register"]
    pub p2map4: crate::Reg<p2map4::P2MAP4_SPEC>,
    #[doc = "0x05 - Port P2.5 mapping register"]
    pub p2map5: crate::Reg<p2map5::P2MAP5_SPEC>,
    #[doc = "0x06 - Port P2.6 mapping register"]
    pub p2map6: crate::Reg<p2map6::P2MAP6_SPEC>,
    #[doc = "0x07 - Port P2.7 mapping register"]
    pub p2map7: crate::Reg<p2map7::P2MAP7_SPEC>,
}
#[doc = "P2MAP0 register accessor: an alias for `Reg<P2MAP0_SPEC>`"]
pub type P2MAP0 = crate::Reg<p2map0::P2MAP0_SPEC>;
#[doc = "Port P2.0 mapping register"]
pub mod p2map0;
#[doc = "P2MAP1 register accessor: an alias for `Reg<P2MAP1_SPEC>`"]
pub type P2MAP1 = crate::Reg<p2map1::P2MAP1_SPEC>;
#[doc = "Port P2.1 mapping register"]
pub mod p2map1;
#[doc = "P2MAP2 register accessor: an alias for `Reg<P2MAP2_SPEC>`"]
pub type P2MAP2 = crate::Reg<p2map2::P2MAP2_SPEC>;
#[doc = "Port P2.2 mapping register"]
pub mod p2map2;
#[doc = "P2MAP3 register accessor: an alias for `Reg<P2MAP3_SPEC>`"]
pub type P2MAP3 = crate::Reg<p2map3::P2MAP3_SPEC>;
#[doc = "Port P2.3 mapping register"]
pub mod p2map3;
#[doc = "P2MAP4 register accessor: an alias for `Reg<P2MAP4_SPEC>`"]
pub type P2MAP4 = crate::Reg<p2map4::P2MAP4_SPEC>;
#[doc = "Port P2.4 mapping register"]
pub mod p2map4;
#[doc = "P2MAP5 register accessor: an alias for `Reg<P2MAP5_SPEC>`"]
pub type P2MAP5 = crate::Reg<p2map5::P2MAP5_SPEC>;
#[doc = "Port P2.5 mapping register"]
pub mod p2map5;
#[doc = "P2MAP6 register accessor: an alias for `Reg<P2MAP6_SPEC>`"]
pub type P2MAP6 = crate::Reg<p2map6::P2MAP6_SPEC>;
#[doc = "Port P2.6 mapping register"]
pub mod p2map6;
#[doc = "P2MAP7 register accessor: an alias for `Reg<P2MAP7_SPEC>`"]
pub type P2MAP7 = crate::Reg<p2map7::P2MAP7_SPEC>;
#[doc = "Port P2.7 mapping register"]
pub mod p2map7;
