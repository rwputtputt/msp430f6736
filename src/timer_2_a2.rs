#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer2_A2 Control"]
    pub ta2ctl: crate::Reg<ta2ctl::TA2CTL_SPEC>,
    #[doc = "0x02 - Timer2_A2 Capture/Compare Control 0"]
    pub ta2cctl0: crate::Reg<ta2cctl0::TA2CCTL0_SPEC>,
    #[doc = "0x04 - Timer2_A2 Capture/Compare Control 1"]
    pub ta2cctl1: crate::Reg<ta2cctl1::TA2CCTL1_SPEC>,
    _reserved3: [u8; 0x0a],
    #[doc = "0x10 - Timer2_A2"]
    pub ta2r: crate::Reg<ta2r::TA2R_SPEC>,
    #[doc = "0x12 - Timer2_A2 Capture/Compare 0"]
    pub ta2ccr0: crate::Reg<ta2ccr0::TA2CCR0_SPEC>,
    #[doc = "0x14 - Timer2_A2 Capture/Compare 1"]
    pub ta2ccr1: crate::Reg<ta2ccr1::TA2CCR1_SPEC>,
    _reserved6: [u8; 0x0a],
    #[doc = "0x20 - Timer2_A2 Expansion Register 0"]
    pub ta2ex0: crate::Reg<ta2ex0::TA2EX0_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x2e - Timer2_A2 Interrupt Vector Word"]
    pub ta2iv: crate::Reg<ta2iv::TA2IV_SPEC>,
}
#[doc = "TA2CTL register accessor: an alias for `Reg<TA2CTL_SPEC>`"]
pub type TA2CTL = crate::Reg<ta2ctl::TA2CTL_SPEC>;
#[doc = "Timer2_A2 Control"]
pub mod ta2ctl;
#[doc = "TA2CCTL0 register accessor: an alias for `Reg<TA2CCTL0_SPEC>`"]
pub type TA2CCTL0 = crate::Reg<ta2cctl0::TA2CCTL0_SPEC>;
#[doc = "Timer2_A2 Capture/Compare Control 0"]
pub mod ta2cctl0;
#[doc = "TA2CCTL1 register accessor: an alias for `Reg<TA2CCTL1_SPEC>`"]
pub type TA2CCTL1 = crate::Reg<ta2cctl1::TA2CCTL1_SPEC>;
#[doc = "Timer2_A2 Capture/Compare Control 1"]
pub mod ta2cctl1;
#[doc = "TA2R register accessor: an alias for `Reg<TA2R_SPEC>`"]
pub type TA2R = crate::Reg<ta2r::TA2R_SPEC>;
#[doc = "Timer2_A2"]
pub mod ta2r;
#[doc = "TA2CCR0 register accessor: an alias for `Reg<TA2CCR0_SPEC>`"]
pub type TA2CCR0 = crate::Reg<ta2ccr0::TA2CCR0_SPEC>;
#[doc = "Timer2_A2 Capture/Compare 0"]
pub mod ta2ccr0;
#[doc = "TA2CCR1 register accessor: an alias for `Reg<TA2CCR1_SPEC>`"]
pub type TA2CCR1 = crate::Reg<ta2ccr1::TA2CCR1_SPEC>;
#[doc = "Timer2_A2 Capture/Compare 1"]
pub mod ta2ccr1;
#[doc = "TA2EX0 register accessor: an alias for `Reg<TA2EX0_SPEC>`"]
pub type TA2EX0 = crate::Reg<ta2ex0::TA2EX0_SPEC>;
#[doc = "Timer2_A2 Expansion Register 0"]
pub mod ta2ex0;
#[doc = "TA2IV register accessor: an alias for `Reg<TA2IV_SPEC>`"]
pub type TA2IV = crate::Reg<ta2iv::TA2IV_SPEC>;
#[doc = "Timer2_A2 Interrupt Vector Word"]
pub mod ta2iv;
