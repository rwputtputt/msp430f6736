#[doc = "Register `LCDM20` reader"]
pub struct R(crate::R<LCDM20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDM20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDM20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDM20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDM20` writer"]
pub struct W(crate::W<LCDM20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDM20_SPEC>;
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
impl From<crate::W<LCDM20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDM20_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Memory 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm20](index.html) module"]
pub struct LCDM20_SPEC;
impl crate::RegisterSpec for LCDM20_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdm20::R](R) reader structure"]
impl crate::Readable for LCDM20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdm20::W](W) writer structure"]
impl crate::Writable for LCDM20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDM20 to value 0"]
impl crate::Resettable for LCDM20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
