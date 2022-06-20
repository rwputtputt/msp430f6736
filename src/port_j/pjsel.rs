#[doc = "Register `PJSEL` reader"]
pub struct R(crate::R<PJSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL` writer"]
pub struct W(crate::W<PJSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL_SPEC>;
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
impl From<crate::W<PJSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL0` reader - PJSEL0"]
pub type PJSEL0_R = crate::BitReader<bool>;
#[doc = "Field `PJSEL0` writer - PJSEL0"]
pub type PJSEL0_W<'a> = crate::BitWriter<'a, u16, PJSEL_SPEC, bool, 0>;
#[doc = "Field `PJSEL1` reader - PJSEL1"]
pub type PJSEL1_R = crate::BitReader<bool>;
#[doc = "Field `PJSEL1` writer - PJSEL1"]
pub type PJSEL1_W<'a> = crate::BitWriter<'a, u16, PJSEL_SPEC, bool, 1>;
#[doc = "Field `PJSEL2` reader - PJSEL2"]
pub type PJSEL2_R = crate::BitReader<bool>;
#[doc = "Field `PJSEL2` writer - PJSEL2"]
pub type PJSEL2_W<'a> = crate::BitWriter<'a, u16, PJSEL_SPEC, bool, 2>;
#[doc = "Field `PJSEL3` reader - PJSEL3"]
pub type PJSEL3_R = crate::BitReader<bool>;
#[doc = "Field `PJSEL3` writer - PJSEL3"]
pub type PJSEL3_W<'a> = crate::BitWriter<'a, u16, PJSEL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - PJSEL0"]
    #[inline(always)]
    pub fn pjsel0(&self) -> PJSEL0_R {
        PJSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJSEL1"]
    #[inline(always)]
    pub fn pjsel1(&self) -> PJSEL1_R {
        PJSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJSEL2"]
    #[inline(always)]
    pub fn pjsel2(&self) -> PJSEL2_R {
        PJSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJSEL3"]
    #[inline(always)]
    pub fn pjsel3(&self) -> PJSEL3_R {
        PJSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJSEL0"]
    #[inline(always)]
    pub fn pjsel0(&mut self) -> PJSEL0_W {
        PJSEL0_W::new(self)
    }
    #[doc = "Bit 1 - PJSEL1"]
    #[inline(always)]
    pub fn pjsel1(&mut self) -> PJSEL1_W {
        PJSEL1_W::new(self)
    }
    #[doc = "Bit 2 - PJSEL2"]
    #[inline(always)]
    pub fn pjsel2(&mut self) -> PJSEL2_W {
        PJSEL2_W::new(self)
    }
    #[doc = "Bit 3 - PJSEL3"]
    #[inline(always)]
    pub fn pjsel3(&mut self) -> PJSEL3_W {
        PJSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel](index.html) module"]
pub struct PJSEL_SPEC;
impl crate::RegisterSpec for PJSEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel::R](R) reader structure"]
impl crate::Readable for PJSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel::W](W) writer structure"]
impl crate::Writable for PJSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL to value 0"]
impl crate::Resettable for PJSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
