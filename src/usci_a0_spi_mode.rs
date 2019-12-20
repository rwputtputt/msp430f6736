#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 0"]
    pub uca0ctl0: UCA0CTL0,
    #[doc = "0x01 - USCI A0 Control Register 1"]
    pub uca0ctl1: UCA0CTL1,
    #[doc = "0x02 - USCI A0 Baud Rate 0"]
    pub uca0br0: UCA0BR0,
    #[doc = "0x03 - USCI A0 Baud Rate 1"]
    pub uca0br1: UCA0BR1,
    #[doc = "0x04 - USCI A0 Modulation Control"]
    pub uca0mctl: UCA0MCTL,
    #[doc = "0x05 - USCI A0 Status Register"]
    pub uca0stat: UCA0STAT,
    #[doc = "0x06 - USCI A0 Receive Buffer"]
    pub uca0rxbuf: UCA0RXBUF,
    #[doc = "0x07 - USCI A0 Transmit Buffer"]
    pub uca0txbuf: UCA0TXBUF,
}
#[doc = "USCI A0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ctl0](uca0ctl0) module"]
pub type UCA0CTL0 = crate::Reg<u8, _UCA0CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTL0;
#[doc = "`read()` method returns [uca0ctl0::R](uca0ctl0::R) reader structure"]
impl crate::Readable for UCA0CTL0 {}
#[doc = "`write(|w| ..)` method takes [uca0ctl0::W](uca0ctl0::W) writer structure"]
impl crate::Writable for UCA0CTL0 {}
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "USCI A0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0ctl1](uca0ctl1) module"]
pub type UCA0CTL1 = crate::Reg<u8, _UCA0CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0CTL1;
#[doc = "`read()` method returns [uca0ctl1::R](uca0ctl1::R) reader structure"]
impl crate::Readable for UCA0CTL1 {}
#[doc = "`write(|w| ..)` method takes [uca0ctl1::W](uca0ctl1::W) writer structure"]
impl crate::Writable for UCA0CTL1 {}
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "USCI A0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0br0](uca0br0) module"]
pub type UCA0BR0 = crate::Reg<u8, _UCA0BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BR0;
#[doc = "`read()` method returns [uca0br0::R](uca0br0::R) reader structure"]
impl crate::Readable for UCA0BR0 {}
#[doc = "`write(|w| ..)` method takes [uca0br0::W](uca0br0::W) writer structure"]
impl crate::Writable for UCA0BR0 {}
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "USCI A0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0br1](uca0br1) module"]
pub type UCA0BR1 = crate::Reg<u8, _UCA0BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0BR1;
#[doc = "`read()` method returns [uca0br1::R](uca0br1::R) reader structure"]
impl crate::Readable for UCA0BR1 {}
#[doc = "`write(|w| ..)` method takes [uca0br1::W](uca0br1::W) writer structure"]
impl crate::Writable for UCA0BR1 {}
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "USCI A0 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0mctl](uca0mctl) module"]
pub type UCA0MCTL = crate::Reg<u8, _UCA0MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0MCTL;
#[doc = "`read()` method returns [uca0mctl::R](uca0mctl::R) reader structure"]
impl crate::Readable for UCA0MCTL {}
#[doc = "`write(|w| ..)` method takes [uca0mctl::W](uca0mctl::W) writer structure"]
impl crate::Writable for UCA0MCTL {}
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl;
#[doc = "USCI A0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0stat](uca0stat) module"]
pub type UCA0STAT = crate::Reg<u8, _UCA0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0STAT;
#[doc = "`read()` method returns [uca0stat::R](uca0stat::R) reader structure"]
impl crate::Readable for UCA0STAT {}
#[doc = "`write(|w| ..)` method takes [uca0stat::W](uca0stat::W) writer structure"]
impl crate::Writable for UCA0STAT {}
#[doc = "USCI A0 Status Register"]
pub mod uca0stat;
#[doc = "USCI A0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0rxbuf](uca0rxbuf) module"]
pub type UCA0RXBUF = crate::Reg<u8, _UCA0RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0RXBUF;
#[doc = "`read()` method returns [uca0rxbuf::R](uca0rxbuf::R) reader structure"]
impl crate::Readable for UCA0RXBUF {}
#[doc = "`write(|w| ..)` method takes [uca0rxbuf::W](uca0rxbuf::W) writer structure"]
impl crate::Writable for UCA0RXBUF {}
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "USCI A0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0txbuf](uca0txbuf) module"]
pub type UCA0TXBUF = crate::Reg<u8, _UCA0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0TXBUF;
#[doc = "`read()` method returns [uca0txbuf::R](uca0txbuf::R) reader structure"]
impl crate::Readable for UCA0TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca0txbuf::W](uca0txbuf::W) writer structure"]
impl crate::Writable for UCA0TXBUF {}
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
