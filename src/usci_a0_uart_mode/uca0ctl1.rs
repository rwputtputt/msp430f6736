#[doc = "Register `UCA0CTL1` reader"]
pub struct R(crate::R<UCA0CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0CTL1` writer"]
pub struct W(crate::W<UCA0CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0CTL1_SPEC>;
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
impl From<crate::W<UCA0CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UCSWRST_R = crate::BitReader<bool>;
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UCSWRST_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 0>;
#[doc = "Field `UCT0BRK` reader - Transmit break"]
pub type UCT0BRK_R = crate::BitReader<bool>;
#[doc = "Field `UCT0BRK` writer - Transmit break"]
pub type UCT0BRK_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 1>;
#[doc = "Field `UCT0ADDR` reader - Transmit Address"]
pub type UCT0ADDR_R = crate::BitReader<bool>;
#[doc = "Field `UCT0ADDR` writer - Transmit Address"]
pub type UCT0ADDR_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 2>;
#[doc = "Field `UCDORM` reader - Dormant. Puts USCI into sleep mode"]
pub type UCDORM_R = crate::BitReader<bool>;
#[doc = "Field `UCDORM` writer - Dormant. Puts USCI into sleep mode"]
pub type UCDORM_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 3>;
#[doc = "Field `UCBRKIE` reader - Receive break character interrupt enable"]
pub type UCBRKIE_R = crate::BitReader<bool>;
#[doc = "Field `UCBRKIE` writer - Receive break character interrupt enable"]
pub type UCBRKIE_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 4>;
#[doc = "Field `UCRXEIE` reader - Receive erroneour-character interrupt enable"]
pub type UCRXEIE_R = crate::BitReader<bool>;
#[doc = "Field `UCRXEIE` writer - Receive erroneour-character interrupt enable"]
pub type UCRXEIE_W<'a> = crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, 5>;
#[doc = "Field `UCSSEL0` reader - USCI clock source select for BRCLK"]
pub type UCSSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCSSEL0` writer - USCI clock source select for BRCLK"]
pub type UCSSEL0_W<'a> = crate::FieldWriter<'a, u8, UCA0CTL1_SPEC, u8, u8, 2, 7>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uct0brk(&self) -> UCT0BRK_R {
        UCT0BRK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Address"]
    #[inline(always)]
    pub fn uct0addr(&self) -> UCT0ADDR_R {
        UCT0ADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dormant. Puts USCI into sleep mode"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UCDORM_R {
        UCDORM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UCBRKIE_R {
        UCBRKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive erroneour-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UCRXEIE_R {
        UCRXEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - USCI clock source select for BRCLK"]
    #[inline(always)]
    pub fn ucssel0(&self) -> UCSSEL0_R {
        UCSSEL0_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W::new(self)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uct0brk(&mut self) -> UCT0BRK_W {
        UCT0BRK_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Address"]
    #[inline(always)]
    pub fn uct0addr(&mut self) -> UCT0ADDR_W {
        UCT0ADDR_W::new(self)
    }
    #[doc = "Bit 3 - Dormant. Puts USCI into sleep mode"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UCDORM_W {
        UCDORM_W::new(self)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UCBRKIE_W {
        UCBRKIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive erroneour-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UCRXEIE_W {
        UCRXEIE_W::new(self)
    }
    #[doc = "Bits 7:8 - USCI clock source select for BRCLK"]
    #[inline(always)]
    pub fn ucssel0(&mut self) -> UCSSEL0_W {
        UCSSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl1](index.html) module"]
pub struct UCA0CTL1_SPEC;
impl crate::RegisterSpec for UCA0CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ctl1::R](R) reader structure"]
impl crate::Readable for UCA0CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ctl1::W](W) writer structure"]
impl crate::Writable for UCA0CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0CTL1 to value 0"]
impl crate::Resettable for UCA0CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
