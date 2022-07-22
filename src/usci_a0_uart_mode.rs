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
    #[doc = "0x08 - Modulation control register"]
    pub uca0mctl: crate::Reg<uca0mctl::UCA0MCTL_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0stat: crate::Reg<uca0stat::UCA0STAT_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf: crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf: crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x10 - USCI A0 LIN Control"]
    pub uca0abctl: crate::Reg<uca0abctl::UCA0ABCTL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    pub uca0irtctl: crate::Reg<uca0irtctl::UCA0IRTCTL_SPEC>,
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    pub uca0irrctl: crate::Reg<uca0irrctl::UCA0IRRCTL_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x1c - interrupt enable register"]
    pub uca0ie: crate::Reg<uca0ie::UCA0IE_SPEC>,
    _reserved13: [u8; 0x01],
    _reserved_13_uca: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x1e - interrupt flag register"]
    #[inline(always)]
    pub fn uca0ifg(&self) -> &crate::Reg<uca0ifg::UCA0IFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(30usize)
                as *const crate::Reg<uca0ifg::UCA0IFG_SPEC>)
        }
    }
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv(&self) -> &crate::Reg<uca0iv::UCA0IV_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(30usize)
                as *const crate::Reg<uca0iv::UCA0IV_SPEC>)
        }
    }
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
#[doc = "UCA0STAT register accessor: an alias for `Reg<UCA0STAT_SPEC>`"]
pub type UCA0STAT = crate::Reg<uca0stat::UCA0STAT_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
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
#[doc = "UCA0MCTL register accessor: an alias for `Reg<UCA0MCTL_SPEC>`"]
pub type UCA0MCTL = crate::Reg<uca0mctl::UCA0MCTL_SPEC>;
#[doc = "Modulation control register"]
pub mod uca0mctl;
#[doc = "UCA0IFG register accessor: an alias for `Reg<UCA0IFG_SPEC>`"]
pub type UCA0IFG = crate::Reg<uca0ifg::UCA0IFG_SPEC>;
#[doc = "interrupt flag register"]
pub mod uca0ifg;
#[doc = "UCA0IE register accessor: an alias for `Reg<UCA0IE_SPEC>`"]
pub type UCA0IE = crate::Reg<uca0ie::UCA0IE_SPEC>;
#[doc = "interrupt enable register"]
pub mod uca0ie;
