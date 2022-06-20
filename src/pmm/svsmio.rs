#[doc = "Register `SVSMIO` reader"]
pub struct R(crate::R<SVSMIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVSMIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVSMIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVSMIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVSMIO` writer"]
pub struct W(crate::W<SVSMIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVSMIO_SPEC>;
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
impl From<crate::W<SVSMIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVSMIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVMLOE` reader - SVM low side output enable"]
pub type SVMLOE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLOE` writer - SVM low side output enable"]
pub type SVMLOE_W<'a> = crate::BitWriter<'a, u16, SVSMIO_SPEC, bool, 3>;
#[doc = "Field `SVMLVLROE` reader - SVM low side voltage level reached output enable"]
pub type SVMLVLROE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLVLROE` writer - SVM low side voltage level reached output enable"]
pub type SVMLVLROE_W<'a> = crate::BitWriter<'a, u16, SVSMIO_SPEC, bool, 4>;
#[doc = "Field `SVMOUTPOL` reader - SVMOUT pin polarity"]
pub type SVMOUTPOL_R = crate::BitReader<bool>;
#[doc = "Field `SVMOUTPOL` writer - SVMOUT pin polarity"]
pub type SVMOUTPOL_W<'a> = crate::BitWriter<'a, u16, SVSMIO_SPEC, bool, 5>;
#[doc = "Field `SVMHOE` reader - SVM high side output enable"]
pub type SVMHOE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHOE` writer - SVM high side output enable"]
pub type SVMHOE_W<'a> = crate::BitWriter<'a, u16, SVSMIO_SPEC, bool, 11>;
#[doc = "Field `SVMHVLROE` reader - SVM high side voltage level reached output enable"]
pub type SVMHVLROE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHVLROE` writer - SVM high side voltage level reached output enable"]
pub type SVMHVLROE_W<'a> = crate::BitWriter<'a, u16, SVSMIO_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    pub fn svmloe(&self) -> SVMLOE_R {
        SVMLOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmlvlroe(&self) -> SVMLVLROE_R {
        SVMLVLROE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    pub fn svmoutpol(&self) -> SVMOUTPOL_R {
        SVMOUTPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    pub fn svmhoe(&self) -> SVMHOE_R {
        SVMHOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmhvlroe(&self) -> SVMHVLROE_R {
        SVMHVLROE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SVM low side output enable"]
    #[inline(always)]
    pub fn svmloe(&mut self) -> SVMLOE_W {
        SVMLOE_W::new(self)
    }
    #[doc = "Bit 4 - SVM low side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmlvlroe(&mut self) -> SVMLVLROE_W {
        SVMLVLROE_W::new(self)
    }
    #[doc = "Bit 5 - SVMOUT pin polarity"]
    #[inline(always)]
    pub fn svmoutpol(&mut self) -> SVMOUTPOL_W {
        SVMOUTPOL_W::new(self)
    }
    #[doc = "Bit 11 - SVM high side output enable"]
    #[inline(always)]
    pub fn svmhoe(&mut self) -> SVMHOE_W {
        SVMHOE_W::new(self)
    }
    #[doc = "Bit 12 - SVM high side voltage level reached output enable"]
    #[inline(always)]
    pub fn svmhvlroe(&mut self) -> SVMHVLROE_W {
        SVMHVLROE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SVSIN and SVSOUT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmio](index.html) module"]
pub struct SVSMIO_SPEC;
impl crate::RegisterSpec for SVSMIO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [svsmio::R](R) reader structure"]
impl crate::Readable for SVSMIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svsmio::W](W) writer structure"]
impl crate::Writable for SVSMIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SVSMIO to value 0"]
impl crate::Resettable for SVSMIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
