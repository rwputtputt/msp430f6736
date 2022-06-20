#[doc = "Register `PMMRIE` reader"]
pub struct R(crate::R<PMMRIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMRIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMRIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMRIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMRIE` writer"]
pub struct W(crate::W<PMMRIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMRIE_SPEC>;
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
impl From<crate::W<PMMRIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMRIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVSMLDLYIE` reader - SVS and SVM low side Delay expired interrupt enable"]
pub type SVSMLDLYIE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMLDLYIE` writer - SVS and SVM low side Delay expired interrupt enable"]
pub type SVSMLDLYIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 0>;
#[doc = "Field `SVMLIE` reader - SVM low side interrupt enable"]
pub type SVMLIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLIE` writer - SVM low side interrupt enable"]
pub type SVMLIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 1>;
#[doc = "Field `SVMLVLRIE` reader - SVM low side Voltage Level Reached interrupt enable"]
pub type SVMLVLRIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLVLRIE` writer - SVM low side Voltage Level Reached interrupt enable"]
pub type SVMLVLRIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 2>;
#[doc = "Field `SVSMHDLYIE` reader - SVS and SVM high side Delay expired interrupt enable"]
pub type SVSMHDLYIE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMHDLYIE` writer - SVS and SVM high side Delay expired interrupt enable"]
pub type SVSMHDLYIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 4>;
#[doc = "Field `SVMHIE` reader - SVM high side interrupt enable"]
pub type SVMHIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHIE` writer - SVM high side interrupt enable"]
pub type SVMHIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 5>;
#[doc = "Field `SVMHVLRIE` reader - SVM high side Voltage Level Reached interrupt enable"]
pub type SVMHVLRIE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHVLRIE` writer - SVM high side Voltage Level Reached interrupt enable"]
pub type SVMHVLRIE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 6>;
#[doc = "Field `SVSLPE` reader - SVS low side POR enable"]
pub type SVSLPE_R = crate::BitReader<bool>;
#[doc = "Field `SVSLPE` writer - SVS low side POR enable"]
pub type SVSLPE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 8>;
#[doc = "Field `SVMLVLRPE` reader - SVM low side Voltage Level reached POR enable"]
pub type SVMLVLRPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLVLRPE` writer - SVM low side Voltage Level reached POR enable"]
pub type SVMLVLRPE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 9>;
#[doc = "Field `SVSHPE` reader - SVS high side POR enable"]
pub type SVSHPE_R = crate::BitReader<bool>;
#[doc = "Field `SVSHPE` writer - SVS high side POR enable"]
pub type SVSHPE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 12>;
#[doc = "Field `SVMHVLRPE` reader - SVM high side Voltage Level reached POR enable"]
pub type SVMHVLRPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHVLRPE` writer - SVM high side Voltage Level reached POR enable"]
pub type SVMHVLRPE_W<'a> = crate::BitWriter<'a, u16, PMMRIE_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&self) -> SVSMLDLYIE_R {
        SVSMLDLYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&self) -> SVMLIE_R {
        SVMLIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&self) -> SVMLVLRIE_R {
        SVMLVLRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&self) -> SVSMHDLYIE_R {
        SVSMHDLYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&self) -> SVMHIE_R {
        SVMHIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&self) -> SVMHVLRIE_R {
        SVMHVLRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&self) -> SVSLPE_R {
        SVSLPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&self) -> SVMLVLRPE_R {
        SVMLVLRPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&self) -> SVSHPE_R {
        SVSHPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&self) -> SVMHVLRPE_R {
        SVMHVLRPE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&mut self) -> SVSMLDLYIE_W {
        SVSMLDLYIE_W::new(self)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&mut self) -> SVMLIE_W {
        SVMLIE_W::new(self)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&mut self) -> SVMLVLRIE_W {
        SVMLVLRIE_W::new(self)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&mut self) -> SVSMHDLYIE_W {
        SVSMHDLYIE_W::new(self)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&mut self) -> SVMHIE_W {
        SVMHIE_W::new(self)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&mut self) -> SVMHVLRIE_W {
        SVMHVLRIE_W::new(self)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&mut self) -> SVSLPE_W {
        SVSLPE_W::new(self)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&mut self) -> SVMLVLRPE_W {
        SVMLVLRPE_W::new(self)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&mut self) -> SVSHPE_W {
        SVSHPE_W::new(self)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&mut self) -> SVMHVLRPE_W {
        SVMHVLRPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM and RESET Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmrie](index.html) module"]
pub struct PMMRIE_SPEC;
impl crate::RegisterSpec for PMMRIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmrie::R](R) reader structure"]
impl crate::Readable for PMMRIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmrie::W](W) writer structure"]
impl crate::Writable for PMMRIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMRIE to value 0"]
impl crate::Resettable for PMMRIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
