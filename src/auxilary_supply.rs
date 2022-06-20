#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Auxiliary Supply Control register 0"]
    pub auxctl0: crate::Reg<auxctl0::AUXCTL0_SPEC>,
    #[doc = "0x02 - Auxiliary Supply Control register 1"]
    pub auxctl1: crate::Reg<auxctl1::AUXCTL1_SPEC>,
    #[doc = "0x04 - Auxiliary Supply Control register 2"]
    pub auxctl2: crate::Reg<auxctl2::AUXCTL2_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x12 - AUX2 Charger Control register"]
    pub aux2chctl: crate::Reg<aux2chctl::AUX2CHCTL_SPEC>,
    #[doc = "0x14 - AUX3 Charger Control register"]
    pub aux3chctl: crate::Reg<aux3chctl::AUX3CHCTL_SPEC>,
    #[doc = "0x16 - AUX ADC Control"]
    pub auxadcctl: crate::Reg<auxadcctl::AUXADCCTL_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x1a - AUX Interrupt Flag"]
    pub auxifg: crate::Reg<auxifg::AUXIFG_SPEC>,
    #[doc = "0x1c - AUX Interrupt Enable"]
    pub auxie: crate::Reg<auxie::AUXIE_SPEC>,
    #[doc = "0x1e - AUX Interrupt Vector Word"]
    pub auxiv: crate::Reg<auxiv::AUXIV_SPEC>,
}
#[doc = "AUXCTL0 register accessor: an alias for `Reg<AUXCTL0_SPEC>`"]
pub type AUXCTL0 = crate::Reg<auxctl0::AUXCTL0_SPEC>;
#[doc = "Auxiliary Supply Control register 0"]
pub mod auxctl0;
#[doc = "AUXCTL1 register accessor: an alias for `Reg<AUXCTL1_SPEC>`"]
pub type AUXCTL1 = crate::Reg<auxctl1::AUXCTL1_SPEC>;
#[doc = "Auxiliary Supply Control register 1"]
pub mod auxctl1;
#[doc = "AUXCTL2 register accessor: an alias for `Reg<AUXCTL2_SPEC>`"]
pub type AUXCTL2 = crate::Reg<auxctl2::AUXCTL2_SPEC>;
#[doc = "Auxiliary Supply Control register 2"]
pub mod auxctl2;
#[doc = "AUX2CHCTL register accessor: an alias for `Reg<AUX2CHCTL_SPEC>`"]
pub type AUX2CHCTL = crate::Reg<aux2chctl::AUX2CHCTL_SPEC>;
#[doc = "AUX2 Charger Control register"]
pub mod aux2chctl;
#[doc = "AUX3CHCTL register accessor: an alias for `Reg<AUX3CHCTL_SPEC>`"]
pub type AUX3CHCTL = crate::Reg<aux3chctl::AUX3CHCTL_SPEC>;
#[doc = "AUX3 Charger Control register"]
pub mod aux3chctl;
#[doc = "AUXADCCTL register accessor: an alias for `Reg<AUXADCCTL_SPEC>`"]
pub type AUXADCCTL = crate::Reg<auxadcctl::AUXADCCTL_SPEC>;
#[doc = "AUX ADC Control"]
pub mod auxadcctl;
#[doc = "AUXIFG register accessor: an alias for `Reg<AUXIFG_SPEC>`"]
pub type AUXIFG = crate::Reg<auxifg::AUXIFG_SPEC>;
#[doc = "AUX Interrupt Flag"]
pub mod auxifg;
#[doc = "AUXIE register accessor: an alias for `Reg<AUXIE_SPEC>`"]
pub type AUXIE = crate::Reg<auxie::AUXIE_SPEC>;
#[doc = "AUX Interrupt Enable"]
pub mod auxie;
#[doc = "AUXIV register accessor: an alias for `Reg<AUXIV_SPEC>`"]
pub type AUXIV = crate::Reg<auxiv::AUXIV_SPEC>;
#[doc = "AUX Interrupt Vector Word"]
pub mod auxiv;
