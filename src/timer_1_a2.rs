#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer1_A2 Control"]
    pub ta1ctl: crate::Reg<ta1ctl::TA1CTL_SPEC>,
    #[doc = "0x02 - Timer1_A2 Capture/Compare Control 0"]
    pub ta1cctl0: crate::Reg<ta1cctl0::TA1CCTL0_SPEC>,
    #[doc = "0x04 - Timer1_A2 Capture/Compare Control 1"]
    pub ta1cctl1: crate::Reg<ta1cctl1::TA1CCTL1_SPEC>,
    _reserved3: [u8; 0x0a],
    #[doc = "0x10 - Timer1_A2"]
    pub ta1r: crate::Reg<ta1r::TA1R_SPEC>,
    #[doc = "0x12 - Timer1_A2 Capture/Compare 0"]
    pub ta1ccr0: crate::Reg<ta1ccr0::TA1CCR0_SPEC>,
    #[doc = "0x14 - Timer1_A2 Capture/Compare 1"]
    pub ta1ccr1: crate::Reg<ta1ccr1::TA1CCR1_SPEC>,
    _reserved6: [u8; 0x0a],
    #[doc = "0x20 - Timer1_A2 Expansion Register 0"]
    pub ta1ex0: crate::Reg<ta1ex0::TA1EX0_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x2e - Timer1_A2 Interrupt Vector Word"]
    pub ta1iv: crate::Reg<ta1iv::TA1IV_SPEC>,
}
#[doc = "TA1CTL register accessor: an alias for `Reg<TA1CTL_SPEC>`"]
pub type TA1CTL = crate::Reg<ta1ctl::TA1CTL_SPEC>;
#[doc = "Timer1_A2 Control"]
pub mod ta1ctl;
#[doc = "TA1CCTL0 register accessor: an alias for `Reg<TA1CCTL0_SPEC>`"]
pub type TA1CCTL0 = crate::Reg<ta1cctl0::TA1CCTL0_SPEC>;
#[doc = "Timer1_A2 Capture/Compare Control 0"]
pub mod ta1cctl0;
#[doc = "TA1CCTL1 register accessor: an alias for `Reg<TA1CCTL1_SPEC>`"]
pub type TA1CCTL1 = crate::Reg<ta1cctl1::TA1CCTL1_SPEC>;
#[doc = "Timer1_A2 Capture/Compare Control 1"]
pub mod ta1cctl1;
#[doc = "TA1R register accessor: an alias for `Reg<TA1R_SPEC>`"]
pub type TA1R = crate::Reg<ta1r::TA1R_SPEC>;
#[doc = "Timer1_A2"]
pub mod ta1r;
#[doc = "TA1CCR0 register accessor: an alias for `Reg<TA1CCR0_SPEC>`"]
pub type TA1CCR0 = crate::Reg<ta1ccr0::TA1CCR0_SPEC>;
#[doc = "Timer1_A2 Capture/Compare 0"]
pub mod ta1ccr0;
#[doc = "TA1CCR1 register accessor: an alias for `Reg<TA1CCR1_SPEC>`"]
pub type TA1CCR1 = crate::Reg<ta1ccr1::TA1CCR1_SPEC>;
#[doc = "Timer1_A2 Capture/Compare 1"]
pub mod ta1ccr1;
#[doc = "TA1EX0 register accessor: an alias for `Reg<TA1EX0_SPEC>`"]
pub type TA1EX0 = crate::Reg<ta1ex0::TA1EX0_SPEC>;
#[doc = "Timer1_A2 Expansion Register 0"]
pub mod ta1ex0;
#[doc = "TA1IV register accessor: an alias for `Reg<TA1IV_SPEC>`"]
pub type TA1IV = crate::Reg<ta1iv::TA1IV_SPEC>;
#[doc = "Timer1_A2 Interrupt Vector Word"]
pub mod ta1iv;
