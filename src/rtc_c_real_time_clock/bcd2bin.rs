#[doc = "Register `BCD2BIN` reader"]
pub struct R(crate::R<BCD2BIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCD2BIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCD2BIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCD2BIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCD2BIN` writer"]
pub struct W(crate::W<BCD2BIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCD2BIN_SPEC>;
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
impl From<crate::W<BCD2BIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCD2BIN_SPEC>) -> Self {
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
#[doc = "Real Time BCD-to-binary conversion register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcd2bin](index.html) module"]
pub struct BCD2BIN_SPEC;
impl crate::RegisterSpec for BCD2BIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bcd2bin::R](R) reader structure"]
impl crate::Readable for BCD2BIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcd2bin::W](W) writer structure"]
impl crate::Writable for BCD2BIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCD2BIN to value 0"]
impl crate::Resettable for BCD2BIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
