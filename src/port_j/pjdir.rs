#[doc = "Register `PJDIR` reader"]
pub struct R(crate::R<PJDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDIR` writer"]
pub struct W(crate::W<PJDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDIR_SPEC>;
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
impl From<crate::W<PJDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDIR0` reader - PJDIR0"]
pub type PJDIR0_R = crate::BitReader<bool>;
#[doc = "Field `PJDIR0` writer - PJDIR0"]
pub type PJDIR0_W<'a> = crate::BitWriter<'a, u16, PJDIR_SPEC, bool, 0>;
#[doc = "Field `PJDIR1` reader - PJDIR1"]
pub type PJDIR1_R = crate::BitReader<bool>;
#[doc = "Field `PJDIR1` writer - PJDIR1"]
pub type PJDIR1_W<'a> = crate::BitWriter<'a, u16, PJDIR_SPEC, bool, 1>;
#[doc = "Field `PJDIR2` reader - PJDIR2"]
pub type PJDIR2_R = crate::BitReader<bool>;
#[doc = "Field `PJDIR2` writer - PJDIR2"]
pub type PJDIR2_W<'a> = crate::BitWriter<'a, u16, PJDIR_SPEC, bool, 2>;
#[doc = "Field `PJDIR3` reader - PJDIR3"]
pub type PJDIR3_R = crate::BitReader<bool>;
#[doc = "Field `PJDIR3` writer - PJDIR3"]
pub type PJDIR3_W<'a> = crate::BitWriter<'a, u16, PJDIR_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&self) -> PJDIR0_R {
        PJDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&self) -> PJDIR1_R {
        PJDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&self) -> PJDIR2_R {
        PJDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&self) -> PJDIR3_R {
        PJDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&mut self) -> PJDIR0_W {
        PJDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&mut self) -> PJDIR1_W {
        PJDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&mut self) -> PJDIR2_W {
        PJDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&mut self) -> PJDIR3_W {
        PJDIR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjdir](index.html) module"]
pub struct PJDIR_SPEC;
impl crate::RegisterSpec for PJDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjdir::R](R) reader structure"]
impl crate::Readable for PJDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjdir::W](W) writer structure"]
impl crate::Writable for PJDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PJDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
