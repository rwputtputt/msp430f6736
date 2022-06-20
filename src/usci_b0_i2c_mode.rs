#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>,
    #[doc = "0x02 - USCI B0 Control Word Register 1"]
    pub ucb0ctlw1: crate::Reg<ucb0ctlw1::UCB0CTLW1_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: crate::Reg<ucb0br0::UCB0BR0_SPEC>,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: crate::Reg<ucb0br1::UCB0BR1_SPEC>,
    #[doc = "0x08 - USCI B0 Status Register"]
    pub ucb0stat_i2c: crate::Reg<ucb0stat_i2c::UCB0STAT_I2C_SPEC>,
    #[doc = "0x09 - USCI B0 Byte Counter Register"]
    pub ucb0bcnt_i2c: crate::Reg<ucb0bcnt_i2c::UCB0BCNT_I2C_SPEC>,
    #[doc = "0x0a - USCI B0 Byte Counter Threshold Register"]
    pub ucb0tbcnt: crate::Reg<ucb0tbcnt::UCB0TBCNT_SPEC>,
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>,
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x14 - USCI B0 I2C Own Address 0"]
    pub ucb0i2coa0: crate::Reg<ucb0i2coa0::UCB0I2COA0_SPEC>,
    #[doc = "0x16 - USCI B0 I2C Own Address 1"]
    pub ucb0i2coa1: crate::Reg<ucb0i2coa1::UCB0I2COA1_SPEC>,
    #[doc = "0x18 - USCI B0 I2C Own Address 2"]
    pub ucb0i2coa2: crate::Reg<ucb0i2coa2::UCB0I2COA2_SPEC>,
    #[doc = "0x1a - USCI B0 I2C Own Address 3"]
    pub ucb0i2coa3: crate::Reg<ucb0i2coa3::UCB0I2COA3_SPEC>,
    #[doc = "0x1c - USCI B0 Received Address Register"]
    pub ucb0addrx: crate::Reg<ucb0addrx::UCB0ADDRX_SPEC>,
    #[doc = "0x1e - USCI B0 Address Mask Register"]
    pub ucb0addmask: crate::Reg<ucb0addmask::UCB0ADDMASK_SPEC>,
    #[doc = "0x20 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>,
    _reserved17: [u8; 0x08],
    _reserved_17_ucb0: [u8; 0x02],
    _reserved_18_ucb0: [u8; 0x02],
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv: crate::Reg<ucb0iv::UCB0IV_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_i2c(&self) -> &crate::Reg<ucb0ie_i2c::UCB0IE_I2C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<ucb0ie_i2c::UCB0IE_I2C_SPEC>)
        }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie(&self) -> &crate::Reg<ucb0ie::UCB0IE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<ucb0ie::UCB0IE_SPEC>)
        }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg_i2c(&self) -> &crate::Reg<ucb0ifg_i2c::UCB0IFG_I2C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<ucb0ifg_i2c::UCB0IFG_I2C_SPEC>)
        }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb0ifg(&self) -> &crate::Reg<ucb0ifg::UCB0IFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<ucb0ifg::UCB0IFG_SPEC>)
        }
    }
}
#[doc = "UCB0CTL1 register accessor: an alias for `Reg<UCB0CTL1_SPEC>`"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 register accessor: an alias for `Reg<UCB0CTL0_SPEC>`"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 register accessor: an alias for `Reg<UCB0BR0_SPEC>`"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 register accessor: an alias for `Reg<UCB0BR1_SPEC>`"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT_I2C register accessor: an alias for `Reg<UCB0STAT_I2C_SPEC>`"]
pub type UCB0STAT_I2C = crate::Reg<ucb0stat_i2c::UCB0STAT_I2C_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_i2c;
#[doc = "UCB0BCNT_I2C register accessor: an alias for `Reg<UCB0BCNT_I2C_SPEC>`"]
pub type UCB0BCNT_I2C = crate::Reg<ucb0bcnt_i2c::UCB0BCNT_I2C_SPEC>;
#[doc = "USCI B0 Byte Counter Register"]
pub mod ucb0bcnt_i2c;
#[doc = "UCB0CTLW1 register accessor: an alias for `Reg<UCB0CTLW1_SPEC>`"]
pub type UCB0CTLW1 = crate::Reg<ucb0ctlw1::UCB0CTLW1_SPEC>;
#[doc = "USCI B0 Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "UCB0TBCNT register accessor: an alias for `Reg<UCB0TBCNT_SPEC>`"]
pub type UCB0TBCNT = crate::Reg<ucb0tbcnt::UCB0TBCNT_SPEC>;
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "UCB0RXBUF register accessor: an alias for `Reg<UCB0RXBUF_SPEC>`"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF register accessor: an alias for `Reg<UCB0TXBUF_SPEC>`"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0I2COA0 register accessor: an alias for `Reg<UCB0I2COA0_SPEC>`"]
pub type UCB0I2COA0 = crate::Reg<ucb0i2coa0::UCB0I2COA0_SPEC>;
#[doc = "USCI B0 I2C Own Address 0"]
pub mod ucb0i2coa0;
#[doc = "UCB0I2COA1 register accessor: an alias for `Reg<UCB0I2COA1_SPEC>`"]
pub type UCB0I2COA1 = crate::Reg<ucb0i2coa1::UCB0I2COA1_SPEC>;
#[doc = "USCI B0 I2C Own Address 1"]
pub mod ucb0i2coa1;
#[doc = "UCB0I2COA2 register accessor: an alias for `Reg<UCB0I2COA2_SPEC>`"]
pub type UCB0I2COA2 = crate::Reg<ucb0i2coa2::UCB0I2COA2_SPEC>;
#[doc = "USCI B0 I2C Own Address 2"]
pub mod ucb0i2coa2;
#[doc = "UCB0I2COA3 register accessor: an alias for `Reg<UCB0I2COA3_SPEC>`"]
pub type UCB0I2COA3 = crate::Reg<ucb0i2coa3::UCB0I2COA3_SPEC>;
#[doc = "USCI B0 I2C Own Address 3"]
pub mod ucb0i2coa3;
#[doc = "UCB0ADDRX register accessor: an alias for `Reg<UCB0ADDRX_SPEC>`"]
pub type UCB0ADDRX = crate::Reg<ucb0addrx::UCB0ADDRX_SPEC>;
#[doc = "USCI B0 Received Address Register"]
pub mod ucb0addrx;
#[doc = "UCB0ADDMASK register accessor: an alias for `Reg<UCB0ADDMASK_SPEC>`"]
pub type UCB0ADDMASK = crate::Reg<ucb0addmask::UCB0ADDMASK_SPEC>;
#[doc = "USCI B0 Address Mask Register"]
pub mod ucb0addmask;
#[doc = "UCB0I2CSA register accessor: an alias for `Reg<UCB0I2CSA_SPEC>`"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IE register accessor: an alias for `Reg<UCB0IE_SPEC>`"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IE_I2C register accessor: an alias for `Reg<UCB0IE_I2C_SPEC>`"]
pub type UCB0IE_I2C = crate::Reg<ucb0ie_i2c::UCB0IE_I2C_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_i2c;
#[doc = "UCB0IFG register accessor: an alias for `Reg<UCB0IFG_SPEC>`"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0IFG_I2C register accessor: an alias for `Reg<UCB0IFG_I2C_SPEC>`"]
pub type UCB0IFG_I2C = crate::Reg<ucb0ifg_i2c::UCB0IFG_I2C_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_i2c;
#[doc = "UCB0IV register accessor: an alias for `Reg<UCB0IV_SPEC>`"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
