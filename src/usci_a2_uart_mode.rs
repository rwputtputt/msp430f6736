#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A2 Control Register 1"]
    pub uca2ctl1: crate::Reg<uca2ctl1::UCA2CTL1_SPEC>,
    #[doc = "0x01 - USCI A2 Control Register 0"]
    pub uca2ctl0: crate::Reg<uca2ctl0::UCA2CTL0_SPEC>,
    #[doc = "0x02 - USCI A2 Control Word Register 1"]
    pub uca2ctlw1: crate::Reg<uca2ctlw1::UCA2CTLW1_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x06 - USCI A2 Baud Rate 0"]
    pub uca2br0: crate::Reg<uca2br0::UCA2BR0_SPEC>,
    #[doc = "0x07 - USCI A2 Baud Rate 1"]
    pub uca2br1: crate::Reg<uca2br1::UCA2BR1_SPEC>,
    #[doc = "0x08 - USCI A2 Modulation Control"]
    pub uca2mctlw: crate::Reg<uca2mctlw::UCA2MCTLW_SPEC>,
    #[doc = "0x0a - USCI A2 Status Register"]
    pub uca2statw: crate::Reg<uca2statw::UCA2STATW_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0c - USCI A2 Receive Buffer"]
    pub uca2rxbuf: crate::Reg<uca2rxbuf::UCA2RXBUF_SPEC>,
    #[doc = "0x0e - USCI A2 Transmit Buffer"]
    pub uca2txbuf: crate::Reg<uca2txbuf::UCA2TXBUF_SPEC>,
    #[doc = "0x10 - USCI A2 LIN Control"]
    pub uca2abctl: crate::Reg<uca2abctl::UCA2ABCTL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x12 - USCI A2 IrDA Transmit Control"]
    pub uca2irtctl: crate::Reg<uca2irtctl::UCA2IRTCTL_SPEC>,
    #[doc = "0x13 - USCI A2 IrDA Receive Control"]
    pub uca2irrctl: crate::Reg<uca2irrctl::UCA2IRRCTL_SPEC>,
    _reserved12: [u8; 0x0a],
    #[doc = "0x1e - USCI A2 Interrupt Vector Register"]
    pub uca2iv: crate::Reg<uca2iv::UCA2IV_SPEC>,
}
#[doc = "UCA2CTL1 register accessor: an alias for `Reg<UCA2CTL1_SPEC>`"]
pub type UCA2CTL1 = crate::Reg<uca2ctl1::UCA2CTL1_SPEC>;
#[doc = "USCI A2 Control Register 1"]
pub mod uca2ctl1;
#[doc = "UCA2CTL0 register accessor: an alias for `Reg<UCA2CTL0_SPEC>`"]
pub type UCA2CTL0 = crate::Reg<uca2ctl0::UCA2CTL0_SPEC>;
#[doc = "USCI A2 Control Register 0"]
pub mod uca2ctl0;
#[doc = "UCA2BR0 register accessor: an alias for `Reg<UCA2BR0_SPEC>`"]
pub type UCA2BR0 = crate::Reg<uca2br0::UCA2BR0_SPEC>;
#[doc = "USCI A2 Baud Rate 0"]
pub mod uca2br0;
#[doc = "UCA2BR1 register accessor: an alias for `Reg<UCA2BR1_SPEC>`"]
pub type UCA2BR1 = crate::Reg<uca2br1::UCA2BR1_SPEC>;
#[doc = "USCI A2 Baud Rate 1"]
pub mod uca2br1;
#[doc = "UCA2STATW register accessor: an alias for `Reg<UCA2STATW_SPEC>`"]
pub type UCA2STATW = crate::Reg<uca2statw::UCA2STATW_SPEC>;
#[doc = "USCI A2 Status Register"]
pub mod uca2statw;
#[doc = "UCA2ABCTL register accessor: an alias for `Reg<UCA2ABCTL_SPEC>`"]
pub type UCA2ABCTL = crate::Reg<uca2abctl::UCA2ABCTL_SPEC>;
#[doc = "USCI A2 LIN Control"]
pub mod uca2abctl;
#[doc = "UCA2IRTCTL register accessor: an alias for `Reg<UCA2IRTCTL_SPEC>`"]
pub type UCA2IRTCTL = crate::Reg<uca2irtctl::UCA2IRTCTL_SPEC>;
#[doc = "USCI A2 IrDA Transmit Control"]
pub mod uca2irtctl;
#[doc = "UCA2IRRCTL register accessor: an alias for `Reg<UCA2IRRCTL_SPEC>`"]
pub type UCA2IRRCTL = crate::Reg<uca2irrctl::UCA2IRRCTL_SPEC>;
#[doc = "USCI A2 IrDA Receive Control"]
pub mod uca2irrctl;
#[doc = "UCA2CTLW1 register accessor: an alias for `Reg<UCA2CTLW1_SPEC>`"]
pub type UCA2CTLW1 = crate::Reg<uca2ctlw1::UCA2CTLW1_SPEC>;
#[doc = "USCI A2 Control Word Register 1"]
pub mod uca2ctlw1;
#[doc = "UCA2MCTLW register accessor: an alias for `Reg<UCA2MCTLW_SPEC>`"]
pub type UCA2MCTLW = crate::Reg<uca2mctlw::UCA2MCTLW_SPEC>;
#[doc = "USCI A2 Modulation Control"]
pub mod uca2mctlw;
#[doc = "UCA2RXBUF register accessor: an alias for `Reg<UCA2RXBUF_SPEC>`"]
pub type UCA2RXBUF = crate::Reg<uca2rxbuf::UCA2RXBUF_SPEC>;
#[doc = "USCI A2 Receive Buffer"]
pub mod uca2rxbuf;
#[doc = "UCA2TXBUF register accessor: an alias for `Reg<UCA2TXBUF_SPEC>`"]
pub type UCA2TXBUF = crate::Reg<uca2txbuf::UCA2TXBUF_SPEC>;
#[doc = "USCI A2 Transmit Buffer"]
pub mod uca2txbuf;
#[doc = "UCA2IV register accessor: an alias for `Reg<UCA2IV_SPEC>`"]
pub type UCA2IV = crate::Reg<uca2iv::UCA2IV_SPEC>;
#[doc = "USCI A2 Interrupt Vector Register"]
pub mod uca2iv;
