#[doc = "Register `UCB0IFG_SPI` reader"]
pub struct R(crate::R<UCB0IFG_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IFG_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IFG_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IFG_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IFG_SPI` writer"]
pub struct W(crate::W<UCB0IFG_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IFG_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCB0IFG_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IFG_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - SPI Receive Interrupt Flag"]
pub type UCRXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG` writer - SPI Receive Interrupt Flag"]
pub type UCRXIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_SPI_SPEC, bool, 0>;
#[doc = "Field `UCTXIFG` reader - SPI Transmit Interrupt Flag"]
pub type UCTXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG` writer - SPI Transmit Interrupt Flag"]
pub type UCTXIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_SPI_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W::new(self)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg_spi](index.html) module"]
pub struct UCB0IFG_SPI_SPEC;
impl crate::RegisterSpec for UCB0IFG_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ifg_spi::R](R) reader structure"]
impl crate::Readable for UCB0IFG_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ifg_spi::W](W) writer structure"]
impl crate::Writable for UCB0IFG_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IFG_SPI to value 0"]
impl crate::Resettable for UCB0IFG_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
