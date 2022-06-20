#[doc = "Register `BIN2BCD` reader"]
pub struct R(crate::R<BIN2BCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIN2BCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIN2BCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIN2BCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIN2BCD` writer"]
pub struct W(crate::W<BIN2BCD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIN2BCD_SPEC>;
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
impl From<crate::W<BIN2BCD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIN2BCD_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Binary-to-BCD conversion register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bin2bcd](index.html) module"]
pub struct BIN2BCD_SPEC;
impl crate::RegisterSpec for BIN2BCD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bin2bcd::R](R) reader structure"]
impl crate::Readable for BIN2BCD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bin2bcd::W](W) writer structure"]
impl crate::Writable for BIN2BCD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIN2BCD to value 0"]
impl crate::Resettable for BIN2BCD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
