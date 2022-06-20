#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 9 Input"]
    pub p9in: crate::Reg<p9in::P9IN_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Port 9 Output"]
    pub p9out: crate::Reg<p9out::P9OUT_SPEC>,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Port 9 Direction"]
    pub p9dir: crate::Reg<p9dir::P9DIR_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x06 - Port 9 Resistor Enable"]
    pub p9ren: crate::Reg<p9ren::P9REN_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x08 - Port 9 Drive Strenght"]
    pub p9ds: crate::Reg<p9ds::P9DS_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - Port 9 Selection"]
    pub p9sel: crate::Reg<p9sel::P9SEL_SPEC>,
}
#[doc = "P9IN register accessor: an alias for `Reg<P9IN_SPEC>`"]
pub type P9IN = crate::Reg<p9in::P9IN_SPEC>;
#[doc = "Port 9 Input"]
pub mod p9in;
#[doc = "P9OUT register accessor: an alias for `Reg<P9OUT_SPEC>`"]
pub type P9OUT = crate::Reg<p9out::P9OUT_SPEC>;
#[doc = "Port 9 Output"]
pub mod p9out;
#[doc = "P9DIR register accessor: an alias for `Reg<P9DIR_SPEC>`"]
pub type P9DIR = crate::Reg<p9dir::P9DIR_SPEC>;
#[doc = "Port 9 Direction"]
pub mod p9dir;
#[doc = "P9REN register accessor: an alias for `Reg<P9REN_SPEC>`"]
pub type P9REN = crate::Reg<p9ren::P9REN_SPEC>;
#[doc = "Port 9 Resistor Enable"]
pub mod p9ren;
#[doc = "P9DS register accessor: an alias for `Reg<P9DS_SPEC>`"]
pub type P9DS = crate::Reg<p9ds::P9DS_SPEC>;
#[doc = "Port 9 Drive Strenght"]
pub mod p9ds;
#[doc = "P9SEL register accessor: an alias for `Reg<P9SEL_SPEC>`"]
pub type P9SEL = crate::Reg<p9sel::P9SEL_SPEC>;
#[doc = "Port 9 Selection"]
pub mod p9sel;
