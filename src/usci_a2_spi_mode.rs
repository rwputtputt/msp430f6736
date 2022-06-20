#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A2 Control Register 1"]
    pub uca2ctl1_spi: crate::Reg<uca2ctl1_spi::UCA2CTL1_SPI_SPEC>,
    #[doc = "0x01 - USCI A2 Control Register 0"]
    pub uca2ctl0_spi: crate::Reg<uca2ctl0_spi::UCA2CTL0_SPI_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI A2 Baud Rate 0"]
    pub uca2br0_spi: crate::Reg<uca2br0_spi::UCA2BR0_SPI_SPEC>,
    #[doc = "0x07 - USCI A2 Baud Rate 1"]
    pub uca2br1_spi: crate::Reg<uca2br1_spi::UCA2BR1_SPI_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI A2 Status Register"]
    pub uca2statw_spi: crate::Reg<uca2statw_spi::UCA2STATW_SPI_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI A2 Receive Buffer"]
    pub uca2rxbuf_spi: crate::Reg<uca2rxbuf_spi::UCA2RXBUF_SPI_SPEC>,
    #[doc = "0x0e - USCI A2 Transmit Buffer"]
    pub uca2txbuf_spi: crate::Reg<uca2txbuf_spi::UCA2TXBUF_SPI_SPEC>,
    _reserved7: [u8; 0x0a],
    #[doc = "0x1a - USCI A2 Interrupt Enable Register"]
    pub uca2ie_spi: crate::Reg<uca2ie_spi::UCA2IE_SPI_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x1c - USCI A2 Interrupt Flags Register"]
    pub uca2ifg_spi: crate::Reg<uca2ifg_spi::UCA2IFG_SPI_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x1e - USCI A2 Interrupt Vector Register"]
    pub uca2iv_spi: crate::Reg<uca2iv_spi::UCA2IV_SPI_SPEC>,
}
#[doc = "UCA2CTL1_SPI register accessor: an alias for `Reg<UCA2CTL1_SPI_SPEC>`"]
pub type UCA2CTL1_SPI = crate::Reg<uca2ctl1_spi::UCA2CTL1_SPI_SPEC>;
#[doc = "USCI A2 Control Register 1"]
pub mod uca2ctl1_spi;
#[doc = "UCA2CTL0_SPI register accessor: an alias for `Reg<UCA2CTL0_SPI_SPEC>`"]
pub type UCA2CTL0_SPI = crate::Reg<uca2ctl0_spi::UCA2CTL0_SPI_SPEC>;
#[doc = "USCI A2 Control Register 0"]
pub mod uca2ctl0_spi;
#[doc = "UCA2BR0_SPI register accessor: an alias for `Reg<UCA2BR0_SPI_SPEC>`"]
pub type UCA2BR0_SPI = crate::Reg<uca2br0_spi::UCA2BR0_SPI_SPEC>;
#[doc = "USCI A2 Baud Rate 0"]
pub mod uca2br0_spi;
#[doc = "UCA2BR1_SPI register accessor: an alias for `Reg<UCA2BR1_SPI_SPEC>`"]
pub type UCA2BR1_SPI = crate::Reg<uca2br1_spi::UCA2BR1_SPI_SPEC>;
#[doc = "USCI A2 Baud Rate 1"]
pub mod uca2br1_spi;
#[doc = "UCA2STATW_SPI register accessor: an alias for `Reg<UCA2STATW_SPI_SPEC>`"]
pub type UCA2STATW_SPI = crate::Reg<uca2statw_spi::UCA2STATW_SPI_SPEC>;
#[doc = "USCI A2 Status Register"]
pub mod uca2statw_spi;
#[doc = "UCA2IE_SPI register accessor: an alias for `Reg<UCA2IE_SPI_SPEC>`"]
pub type UCA2IE_SPI = crate::Reg<uca2ie_spi::UCA2IE_SPI_SPEC>;
#[doc = "USCI A2 Interrupt Enable Register"]
pub mod uca2ie_spi;
#[doc = "UCA2IFG_SPI register accessor: an alias for `Reg<UCA2IFG_SPI_SPEC>`"]
pub type UCA2IFG_SPI = crate::Reg<uca2ifg_spi::UCA2IFG_SPI_SPEC>;
#[doc = "USCI A2 Interrupt Flags Register"]
pub mod uca2ifg_spi;
#[doc = "UCA2RXBUF_SPI register accessor: an alias for `Reg<UCA2RXBUF_SPI_SPEC>`"]
pub type UCA2RXBUF_SPI = crate::Reg<uca2rxbuf_spi::UCA2RXBUF_SPI_SPEC>;
#[doc = "USCI A2 Receive Buffer"]
pub mod uca2rxbuf_spi;
#[doc = "UCA2TXBUF_SPI register accessor: an alias for `Reg<UCA2TXBUF_SPI_SPEC>`"]
pub type UCA2TXBUF_SPI = crate::Reg<uca2txbuf_spi::UCA2TXBUF_SPI_SPEC>;
#[doc = "USCI A2 Transmit Buffer"]
pub mod uca2txbuf_spi;
#[doc = "UCA2IV_SPI register accessor: an alias for `Reg<UCA2IV_SPI_SPEC>`"]
pub type UCA2IV_SPI = crate::Reg<uca2iv_spi::UCA2IV_SPI_SPEC>;
#[doc = "USCI A2 Interrupt Vector Register"]
pub mod uca2iv_spi;
