#[doc = "Register `UCA0IRTCTL` reader"]
pub struct R(crate::R<UCA0IRTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IRTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IRTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IRTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IRTCTL` writer"]
pub struct W(crate::W<UCA0IRTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IRTCTL_SPEC>;
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
impl From<crate::W<UCA0IRTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IRTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIREN` reader - IrDA encoder and decoder enable"]
pub type UCIREN_R = crate::BitReader<bool>;
#[doc = "Field `UCIREN` writer - IrDA encoder and decoder enable"]
pub type UCIREN_W<'a> = crate::BitWriter<'a, u8, UCA0IRTCTL_SPEC, bool, 0>;
#[doc = "Field `UCIRTXCLK` reader - IrDA transmit pulse clock select"]
pub type UCIRTXCLK_R = crate::BitReader<bool>;
#[doc = "Field `UCIRTXCLK` writer - IrDA transmit pulse clock select"]
pub type UCIRTXCLK_W<'a> = crate::BitWriter<'a, u8, UCA0IRTCTL_SPEC, bool, 1>;
#[doc = "Field `UCIRTXPL` reader - Transmit pulse length"]
pub type UCIRTXPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCIRTXPL` writer - Transmit pulse length"]
pub type UCIRTXPL_W<'a> = crate::FieldWriter<'a, u8, UCA0IRTCTL_SPEC, u8, u8, 6, 7>;
impl R {
    #[doc = "Bit 0 - IrDA encoder and decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder and decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W::new(self)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W::new(self)
    }
    #[doc = "Bits 7:12 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W {
        UCIRTXPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0irtctl](index.html) module"]
pub struct UCA0IRTCTL_SPEC;
impl crate::RegisterSpec for UCA0IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0irtctl::R](R) reader structure"]
impl crate::Readable for UCA0IRTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0irtctl::W](W) writer structure"]
impl crate::Writable for UCA0IRTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IRTCTL to value 0"]
impl crate::Resettable for UCA0IRTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
