#[doc = "Register `UCA0MCTL` reader"]
pub struct R(crate::R<UCA0MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0MCTL` writer"]
pub struct W(crate::W<UCA0MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0MCTL_SPEC>;
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
impl From<crate::W<UCA0MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOS16` reader - Oversampling mode enabled"]
pub type UCOS16_R = crate::BitReader<bool>;
#[doc = "Field `UCOS16` writer - Oversampling mode enabled"]
pub type UCOS16_W<'a> = crate::BitWriter<'a, u8, UCA0MCTL_SPEC, bool, 0>;
#[doc = "Field `UCBRS` reader - Secont stage select"]
pub type UCBRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCBRS` writer - Secont stage select"]
pub type UCBRS_W<'a> = crate::FieldWriter<'a, u8, UCA0MCTL_SPEC, u8, u8, 3, 3>;
#[doc = "Field `UCBRF` reader - First modulation stage select"]
pub type UCBRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCBRF` writer - First modulation stage select"]
pub type UCBRF_W<'a> = crate::FieldWriter<'a, u8, UCA0MCTL_SPEC, u8, u8, 4, 7>;
impl R {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5 - Secont stage select"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 7:10 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W::new(self)
    }
    #[doc = "Bits 3:5 - Secont stage select"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UCBRS_W {
        UCBRS_W::new(self)
    }
    #[doc = "Bits 7:10 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modulation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0mctl](index.html) module"]
pub struct UCA0MCTL_SPEC;
impl crate::RegisterSpec for UCA0MCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0mctl::R](R) reader structure"]
impl crate::Readable for UCA0MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0mctl::W](W) writer structure"]
impl crate::Writable for UCA0MCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0MCTL to value 0"]
impl crate::Resettable for UCA0MCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
