#[doc = "Register `SYSBSLC` reader"]
pub struct R(crate::R<SYSBSLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSBSLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSBSLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSBSLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSBSLC` writer"]
pub struct W(crate::W<SYSBSLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSBSLC_SPEC>;
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
impl From<crate::W<SYSBSLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSBSLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSBSLSIZE0` reader - SYS - BSL Protection Size 0"]
pub type SYSBSLSIZE0_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLSIZE0` writer - SYS - BSL Protection Size 0"]
pub type SYSBSLSIZE0_W<'a> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, bool, 0>;
#[doc = "Field `SYSBSLSIZE1` reader - SYS - BSL Protection Size 1"]
pub type SYSBSLSIZE1_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLSIZE1` writer - SYS - BSL Protection Size 1"]
pub type SYSBSLSIZE1_W<'a> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, bool, 1>;
#[doc = "Field `SYSBSLR` reader - SYS - RAM assigned to BSL"]
pub type SYSBSLR_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLR` writer - SYS - RAM assigned to BSL"]
pub type SYSBSLR_W<'a> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, bool, 2>;
#[doc = "Field `SYSBSLOFF` reader - SYS - BSL Memory disabled"]
pub type SYSBSLOFF_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLOFF` writer - SYS - BSL Memory disabled"]
pub type SYSBSLOFF_W<'a> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, bool, 14>;
#[doc = "Field `SYSBSLPE` reader - SYS - BSL Memory protection enabled"]
pub type SYSBSLPE_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLPE` writer - SYS - BSL Memory protection enabled"]
pub type SYSBSLPE_W<'a> = crate::BitWriter<'a, u16, SYSBSLC_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&self) -> SYSBSLSIZE0_R {
        SYSBSLSIZE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&self) -> SYSBSLSIZE1_R {
        SYSBSLSIZE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - BSL Protection Size 0"]
    #[inline(always)]
    pub fn sysbslsize0(&mut self) -> SYSBSLSIZE0_W {
        SYSBSLSIZE0_W::new(self)
    }
    #[doc = "Bit 1 - SYS - BSL Protection Size 1"]
    #[inline(always)]
    pub fn sysbslsize1(&mut self) -> SYSBSLSIZE1_W {
        SYSBSLSIZE1_W::new(self)
    }
    #[doc = "Bit 2 - SYS - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SYSBSLR_W {
        SYSBSLR_W::new(self)
    }
    #[doc = "Bit 14 - SYS - BSL Memory disabled"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W {
        SYSBSLOFF_W::new(self)
    }
    #[doc = "Bit 15 - SYS - BSL Memory protection enabled"]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W {
        SYSBSLPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot strap configuration area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysbslc](index.html) module"]
pub struct SYSBSLC_SPEC;
impl crate::RegisterSpec for SYSBSLC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysbslc::R](R) reader structure"]
impl crate::Readable for SYSBSLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysbslc::W](W) writer structure"]
impl crate::Writable for SYSBSLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SYSBSLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
