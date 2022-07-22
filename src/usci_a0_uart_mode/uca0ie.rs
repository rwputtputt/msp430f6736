#[doc = "Register `UCA0IE` reader"]
pub struct R(crate::R<UCA0IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IE` writer"]
pub struct W(crate::W<UCA0IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IE_SPEC>;
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
impl From<crate::W<UCA0IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE` reader - Receive interrupt enable"]
pub type UCRXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE` writer - Receive interrupt enable"]
pub type UCRXIE_W<'a> = crate::BitWriter<'a, u8, UCA0IE_SPEC, bool, 0>;
#[doc = "Field `UCTXIE` reader - Transmit interrupt enable"]
pub type UCTXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE` writer - Transmit interrupt enable"]
pub type UCTXIE_W<'a> = crate::BitWriter<'a, u8, UCA0IE_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W {
        UCRXIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W {
        UCTXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ie](index.html) module"]
pub struct UCA0IE_SPEC;
impl crate::RegisterSpec for UCA0IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ie::R](R) reader structure"]
impl crate::Readable for UCA0IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ie::W](W) writer structure"]
impl crate::Writable for UCA0IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IE to value 0"]
impl crate::Resettable for UCA0IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
