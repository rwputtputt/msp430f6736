#[doc = "Register `RCCTL0` reader"]
pub struct R(crate::R<RCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCTL0` writer"]
pub struct W(crate::W<RCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCTL0_SPEC>;
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
impl From<crate::W<RCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCRS0OFF` reader - RAM Controller RAM Sector 0 Off"]
pub type RCRS0OFF_R = crate::BitReader<bool>;
#[doc = "Field `RCRS0OFF` writer - RAM Controller RAM Sector 0 Off"]
pub type RCRS0OFF_W<'a> = crate::BitWriter<'a, u16, RCCTL0_SPEC, bool, 0>;
#[doc = "Field `RCRS1OFF` reader - RAM Controller RAM Sector 1 Off"]
pub type RCRS1OFF_R = crate::BitReader<bool>;
#[doc = "Field `RCRS1OFF` writer - RAM Controller RAM Sector 1 Off"]
pub type RCRS1OFF_W<'a> = crate::BitWriter<'a, u16, RCCTL0_SPEC, bool, 1>;
#[doc = "Field `RCRS2OFF` reader - RAM Controller RAM Sector 2 Off"]
pub type RCRS2OFF_R = crate::BitReader<bool>;
#[doc = "Field `RCRS2OFF` writer - RAM Controller RAM Sector 2 Off"]
pub type RCRS2OFF_W<'a> = crate::BitWriter<'a, u16, RCCTL0_SPEC, bool, 2>;
#[doc = "Field `RCRS3OFF` reader - RAM Controller RAM Sector 3 Off"]
pub type RCRS3OFF_R = crate::BitReader<bool>;
#[doc = "Field `RCRS3OFF` writer - RAM Controller RAM Sector 3 Off"]
pub type RCRS3OFF_W<'a> = crate::BitWriter<'a, u16, RCCTL0_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Controller RAM Sector 0 Off"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W {
        RCRS0OFF_W::new(self)
    }
    #[doc = "Bit 1 - RAM Controller RAM Sector 1 Off"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W {
        RCRS1OFF_W::new(self)
    }
    #[doc = "Bit 2 - RAM Controller RAM Sector 2 Off"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W {
        RCRS2OFF_W::new(self)
    }
    #[doc = "Bit 3 - RAM Controller RAM Sector 3 Off"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W {
        RCRS3OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ram Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcctl0](index.html) module"]
pub struct RCCTL0_SPEC;
impl crate::RegisterSpec for RCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rcctl0::R](R) reader structure"]
impl crate::Readable for RCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcctl0::W](W) writer structure"]
impl crate::Writable for RCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCCTL0 to value 0"]
impl crate::Resettable for RCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
