#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SD24B Control Register 0"]
    pub sd24bctl0: crate::Reg<sd24bctl0::SD24BCTL0_SPEC>,
    #[doc = "0x02 - SD24B Control Register 1"]
    pub sd24bctl1: crate::Reg<sd24bctl1::SD24BCTL1_SPEC>,
    _reserved2: [u8; 0x06],
    #[doc = "0x0a - SD24B Interrupt Flag Register"]
    pub sd24bifg: crate::Reg<sd24bifg::SD24BIFG_SPEC>,
    #[doc = "0x0c - SD24B Interrupt Enable Register"]
    pub sd24bie: crate::Reg<sd24bie::SD24BIE_SPEC>,
    #[doc = "0x0e - SD24B Interrupt Vector Register"]
    pub sd24biv: crate::Reg<sd24biv::SD24BIV_SPEC>,
    #[doc = "0x10 - SD24B Channel 0 Control Register"]
    pub sd24bcctl0: crate::Reg<sd24bcctl0::SD24BCCTL0_SPEC>,
    #[doc = "0x12 - SD24B Channel 0 Input Control Register"]
    pub sd24binctl0: crate::Reg<sd24binctl0::SD24BINCTL0_SPEC>,
    #[doc = "0x14 - SD24B Channel 0 OSR Control Register"]
    pub sd24bosr0: crate::Reg<sd24bosr0::SD24BOSR0_SPEC>,
    #[doc = "0x16 - SD24B Channel 0 Preload Register"]
    pub sd24bpre0: crate::Reg<sd24bpre0::SD24BPRE0_SPEC>,
    #[doc = "0x18 - SD24B Channel 1 Control Register"]
    pub sd24bcctl1: crate::Reg<sd24bcctl1::SD24BCCTL1_SPEC>,
    #[doc = "0x1a - SD24B Channel 1 Input Control Register"]
    pub sd24binctl1: crate::Reg<sd24binctl1::SD24BINCTL1_SPEC>,
    #[doc = "0x1c - SD24B Channel 1 OSR Control Register"]
    pub sd24bosr1: crate::Reg<sd24bosr1::SD24BOSR1_SPEC>,
    #[doc = "0x1e - SD24B Channel 1 Preload Register"]
    pub sd24bpre1: crate::Reg<sd24bpre1::SD24BPRE1_SPEC>,
    #[doc = "0x20 - SD24B Channel 2 Control Register"]
    pub sd24bcctl2: crate::Reg<sd24bcctl2::SD24BCCTL2_SPEC>,
    #[doc = "0x22 - SD24B Channel 2 Input Control Register"]
    pub sd24binctl2: crate::Reg<sd24binctl2::SD24BINCTL2_SPEC>,
    #[doc = "0x24 - SD24B Channel 2 OSR Control Register"]
    pub sd24bosr2: crate::Reg<sd24bosr2::SD24BOSR2_SPEC>,
    #[doc = "0x26 - SD24B Channel 2 Preload Register"]
    pub sd24bpre2: crate::Reg<sd24bpre2::SD24BPRE2_SPEC>,
    _reserved17: [u8; 0x28],
    #[doc = "0x50 - SD24B Channel 0 Conversion Memory Low word"]
    pub sd24bmeml0: crate::Reg<sd24bmeml0::SD24BMEML0_SPEC>,
    #[doc = "0x52 - SD24B Channel 0 Conversion Memory High Word"]
    pub sd24bmemh0: crate::Reg<sd24bmemh0::SD24BMEMH0_SPEC>,
    #[doc = "0x54 - SD24B Channel 1 Conversion Memory Low word"]
    pub sd24bmeml1: crate::Reg<sd24bmeml1::SD24BMEML1_SPEC>,
    #[doc = "0x56 - SD24B Channel 1 Conversion Memory High Word"]
    pub sd24bmemh1: crate::Reg<sd24bmemh1::SD24BMEMH1_SPEC>,
    #[doc = "0x58 - SD24B Channel 2 Conversion Memory Low word"]
    pub sd24bmeml2: crate::Reg<sd24bmeml2::SD24BMEML2_SPEC>,
    #[doc = "0x5a - SD24B Channel 2 Conversion Memory High Word"]
    pub sd24bmemh2: crate::Reg<sd24bmemh2::SD24BMEMH2_SPEC>,
}
#[doc = "SD24BCTL0 register accessor: an alias for `Reg<SD24BCTL0_SPEC>`"]
pub type SD24BCTL0 = crate::Reg<sd24bctl0::SD24BCTL0_SPEC>;
#[doc = "SD24B Control Register 0"]
pub mod sd24bctl0;
#[doc = "SD24BCTL1 register accessor: an alias for `Reg<SD24BCTL1_SPEC>`"]
pub type SD24BCTL1 = crate::Reg<sd24bctl1::SD24BCTL1_SPEC>;
#[doc = "SD24B Control Register 1"]
pub mod sd24bctl1;
#[doc = "SD24BIFG register accessor: an alias for `Reg<SD24BIFG_SPEC>`"]
pub type SD24BIFG = crate::Reg<sd24bifg::SD24BIFG_SPEC>;
#[doc = "SD24B Interrupt Flag Register"]
pub mod sd24bifg;
#[doc = "SD24BIE register accessor: an alias for `Reg<SD24BIE_SPEC>`"]
pub type SD24BIE = crate::Reg<sd24bie::SD24BIE_SPEC>;
#[doc = "SD24B Interrupt Enable Register"]
pub mod sd24bie;
#[doc = "SD24BIV register accessor: an alias for `Reg<SD24BIV_SPEC>`"]
pub type SD24BIV = crate::Reg<sd24biv::SD24BIV_SPEC>;
#[doc = "SD24B Interrupt Vector Register"]
pub mod sd24biv;
#[doc = "SD24BCCTL0 register accessor: an alias for `Reg<SD24BCCTL0_SPEC>`"]
pub type SD24BCCTL0 = crate::Reg<sd24bcctl0::SD24BCCTL0_SPEC>;
#[doc = "SD24B Channel 0 Control Register"]
pub mod sd24bcctl0;
#[doc = "SD24BINCTL0 register accessor: an alias for `Reg<SD24BINCTL0_SPEC>`"]
pub type SD24BINCTL0 = crate::Reg<sd24binctl0::SD24BINCTL0_SPEC>;
#[doc = "SD24B Channel 0 Input Control Register"]
pub mod sd24binctl0;
#[doc = "SD24BOSR0 register accessor: an alias for `Reg<SD24BOSR0_SPEC>`"]
pub type SD24BOSR0 = crate::Reg<sd24bosr0::SD24BOSR0_SPEC>;
#[doc = "SD24B Channel 0 OSR Control Register"]
pub mod sd24bosr0;
#[doc = "SD24BPRE0 register accessor: an alias for `Reg<SD24BPRE0_SPEC>`"]
pub type SD24BPRE0 = crate::Reg<sd24bpre0::SD24BPRE0_SPEC>;
#[doc = "SD24B Channel 0 Preload Register"]
pub mod sd24bpre0;
#[doc = "SD24BCCTL1 register accessor: an alias for `Reg<SD24BCCTL1_SPEC>`"]
pub type SD24BCCTL1 = crate::Reg<sd24bcctl1::SD24BCCTL1_SPEC>;
#[doc = "SD24B Channel 1 Control Register"]
pub mod sd24bcctl1;
#[doc = "SD24BINCTL1 register accessor: an alias for `Reg<SD24BINCTL1_SPEC>`"]
pub type SD24BINCTL1 = crate::Reg<sd24binctl1::SD24BINCTL1_SPEC>;
#[doc = "SD24B Channel 1 Input Control Register"]
pub mod sd24binctl1;
#[doc = "SD24BOSR1 register accessor: an alias for `Reg<SD24BOSR1_SPEC>`"]
pub type SD24BOSR1 = crate::Reg<sd24bosr1::SD24BOSR1_SPEC>;
#[doc = "SD24B Channel 1 OSR Control Register"]
pub mod sd24bosr1;
#[doc = "SD24BPRE1 register accessor: an alias for `Reg<SD24BPRE1_SPEC>`"]
pub type SD24BPRE1 = crate::Reg<sd24bpre1::SD24BPRE1_SPEC>;
#[doc = "SD24B Channel 1 Preload Register"]
pub mod sd24bpre1;
#[doc = "SD24BCCTL2 register accessor: an alias for `Reg<SD24BCCTL2_SPEC>`"]
pub type SD24BCCTL2 = crate::Reg<sd24bcctl2::SD24BCCTL2_SPEC>;
#[doc = "SD24B Channel 2 Control Register"]
pub mod sd24bcctl2;
#[doc = "SD24BINCTL2 register accessor: an alias for `Reg<SD24BINCTL2_SPEC>`"]
pub type SD24BINCTL2 = crate::Reg<sd24binctl2::SD24BINCTL2_SPEC>;
#[doc = "SD24B Channel 2 Input Control Register"]
pub mod sd24binctl2;
#[doc = "SD24BOSR2 register accessor: an alias for `Reg<SD24BOSR2_SPEC>`"]
pub type SD24BOSR2 = crate::Reg<sd24bosr2::SD24BOSR2_SPEC>;
#[doc = "SD24B Channel 2 OSR Control Register"]
pub mod sd24bosr2;
#[doc = "SD24BPRE2 register accessor: an alias for `Reg<SD24BPRE2_SPEC>`"]
pub type SD24BPRE2 = crate::Reg<sd24bpre2::SD24BPRE2_SPEC>;
#[doc = "SD24B Channel 2 Preload Register"]
pub mod sd24bpre2;
#[doc = "SD24BMEML0 register accessor: an alias for `Reg<SD24BMEML0_SPEC>`"]
pub type SD24BMEML0 = crate::Reg<sd24bmeml0::SD24BMEML0_SPEC>;
#[doc = "SD24B Channel 0 Conversion Memory Low word"]
pub mod sd24bmeml0;
#[doc = "SD24BMEMH0 register accessor: an alias for `Reg<SD24BMEMH0_SPEC>`"]
pub type SD24BMEMH0 = crate::Reg<sd24bmemh0::SD24BMEMH0_SPEC>;
#[doc = "SD24B Channel 0 Conversion Memory High Word"]
pub mod sd24bmemh0;
#[doc = "SD24BMEML1 register accessor: an alias for `Reg<SD24BMEML1_SPEC>`"]
pub type SD24BMEML1 = crate::Reg<sd24bmeml1::SD24BMEML1_SPEC>;
#[doc = "SD24B Channel 1 Conversion Memory Low word"]
pub mod sd24bmeml1;
#[doc = "SD24BMEMH1 register accessor: an alias for `Reg<SD24BMEMH1_SPEC>`"]
pub type SD24BMEMH1 = crate::Reg<sd24bmemh1::SD24BMEMH1_SPEC>;
#[doc = "SD24B Channel 1 Conversion Memory High Word"]
pub mod sd24bmemh1;
#[doc = "SD24BMEML2 register accessor: an alias for `Reg<SD24BMEML2_SPEC>`"]
pub type SD24BMEML2 = crate::Reg<sd24bmeml2::SD24BMEML2_SPEC>;
#[doc = "SD24B Channel 2 Conversion Memory Low word"]
pub mod sd24bmeml2;
#[doc = "SD24BMEMH2 register accessor: an alias for `Reg<SD24BMEMH2_SPEC>`"]
pub type SD24BMEMH2 = crate::Reg<sd24bmemh2::SD24BMEMH2_SPEC>;
#[doc = "SD24B Channel 2 Conversion Memory High Word"]
pub mod sd24bmemh2;
