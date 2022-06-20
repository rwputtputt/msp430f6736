#[doc = "Register `PMAPCTL` reader"]
pub struct R(crate::R<PMAPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMAPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMAPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMAPCTL` writer"]
pub struct W(crate::W<PMAPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMAPCTL_SPEC>;
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
impl From<crate::W<PMAPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMAPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAPLOCKED` reader - Port Mapping Lock bit. Read only"]
pub type PMAPLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `PMAPLOCKED` writer - Port Mapping Lock bit. Read only"]
pub type PMAPLOCKED_W<'a> = crate::BitWriter<'a, u16, PMAPCTL_SPEC, bool, 0>;
#[doc = "Field `PMAPRECFG` reader - Port Mapping re-configuration control bit"]
pub type PMAPRECFG_R = crate::BitReader<bool>;
#[doc = "Field `PMAPRECFG` writer - Port Mapping re-configuration control bit"]
pub type PMAPRECFG_W<'a> = crate::BitWriter<'a, u16, PMAPCTL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PMAPLOCKED_R {
        PMAPLOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PMAPRECFG_R {
        PMAPRECFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Mapping Lock bit. Read only"]
    #[inline(always)]
    pub fn pmaplocked(&mut self) -> PMAPLOCKED_W {
        PMAPLOCKED_W::new(self)
    }
    #[doc = "Bit 1 - Port Mapping re-configuration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&mut self) -> PMAPRECFG_W {
        PMAPRECFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Mapping control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapctl](index.html) module"]
pub struct PMAPCTL_SPEC;
impl crate::RegisterSpec for PMAPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmapctl::R](R) reader structure"]
impl crate::Readable for PMAPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmapctl::W](W) writer structure"]
impl crate::Writable for PMAPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMAPCTL to value 0"]
impl crate::Resettable for PMAPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
