#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1: crate::Reg<uca0ctl1::UCA0CTL1_SPEC>,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0: crate::Reg<uca0ctl0::UCA0CTL0_SPEC>,
    #[doc = "0x02 - USCI A0 Control Word Register 1"]
    pub uca0ctlw1: crate::Reg<uca0ctlw1::UCA0CTLW1_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0: crate::Reg<uca0br0::UCA0BR0_SPEC>,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1: crate::Reg<uca0br1::UCA0BR1_SPEC>,
    #[doc = "0x08 - USCI A0 Modulation Control"]
    pub uca0mctlw: crate::Reg<uca0mctlw::UCA0MCTLW_SPEC>,
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0statw: crate::Reg<uca0statw::UCA0STATW_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf: crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>,
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf: crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>,
    #[doc = "0x10 - USCI A0 LIN Control"]
    pub uca0abctl: crate::Reg<uca0abctl::UCA0ABCTL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    pub uca0irtctl: crate::Reg<uca0irtctl::UCA0IRTCTL_SPEC>,
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    pub uca0irrctl: crate::Reg<uca0irrctl::UCA0IRRCTL_SPEC>,
    _reserved12: [u8; 0x0a],
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    pub uca0iv: crate::Reg<uca0iv::UCA0IV_SPEC>,
}
#[doc = "UCA0CTL1 register accessor: an alias for `Reg<UCA0CTL1_SPEC>`"]
pub type UCA0CTL1 = crate::Reg<uca0ctl1::UCA0CTL1_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0CTL0 register accessor: an alias for `Reg<UCA0CTL0_SPEC>`"]
pub type UCA0CTL0 = crate::Reg<uca0ctl0::UCA0CTL0_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0BR0 register accessor: an alias for `Reg<UCA0BR0_SPEC>`"]
pub type UCA0BR0 = crate::Reg<uca0br0::UCA0BR0_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 register accessor: an alias for `Reg<UCA0BR1_SPEC>`"]
pub type UCA0BR1 = crate::Reg<uca0br1::UCA0BR1_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0STATW register accessor: an alias for `Reg<UCA0STATW_SPEC>`"]
pub type UCA0STATW = crate::Reg<uca0statw::UCA0STATW_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0statw;
#[doc = "UCA0ABCTL register accessor: an alias for `Reg<UCA0ABCTL_SPEC>`"]
pub type UCA0ABCTL = crate::Reg<uca0abctl::UCA0ABCTL_SPEC>;
#[doc = "USCI A0 LIN Control"]
pub mod uca0abctl;
#[doc = "UCA0IRTCTL register accessor: an alias for `Reg<UCA0IRTCTL_SPEC>`"]
pub type UCA0IRTCTL = crate::Reg<uca0irtctl::UCA0IRTCTL_SPEC>;
#[doc = "USCI A0 IrDA Transmit Control"]
pub mod uca0irtctl;
#[doc = "UCA0IRRCTL register accessor: an alias for `Reg<UCA0IRRCTL_SPEC>`"]
pub type UCA0IRRCTL = crate::Reg<uca0irrctl::UCA0IRRCTL_SPEC>;
#[doc = "USCI A0 IrDA Receive Control"]
pub mod uca0irrctl;
#[doc = "UCA0CTLW1 register accessor: an alias for `Reg<UCA0CTLW1_SPEC>`"]
pub type UCA0CTLW1 = crate::Reg<uca0ctlw1::UCA0CTLW1_SPEC>;
#[doc = "USCI A0 Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "UCA0MCTLW register accessor: an alias for `Reg<UCA0MCTLW_SPEC>`"]
pub type UCA0MCTLW = crate::Reg<uca0mctlw::UCA0MCTLW_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctlw;
#[doc = "UCA0RXBUF register accessor: an alias for `Reg<UCA0RXBUF_SPEC>`"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF register accessor: an alias for `Reg<UCA0TXBUF_SPEC>`"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
#[doc = "UCA0IV register accessor: an alias for `Reg<UCA0IV_SPEC>`"]
pub type UCA0IV = crate::Reg<uca0iv::UCA0IV_SPEC>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv;
