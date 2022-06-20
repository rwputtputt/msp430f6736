#[doc = "Register `SYSCTL` reader"]
pub struct R(crate::R<SYSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTL` writer"]
pub struct W(crate::W<SYSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTL_SPEC>;
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
impl From<crate::W<SYSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRIVECT` reader - SYS - RAM based interrupt vectors"]
pub type SYSRIVECT_R = crate::BitReader<bool>;
#[doc = "Field `SYSRIVECT` writer - SYS - RAM based interrupt vectors"]
pub type SYSRIVECT_W<'a> = crate::BitWriter<'a, u16, SYSCTL_SPEC, bool, 0>;
#[doc = "Field `SYSPMMPE` reader - SYS - PMM access protect"]
pub type SYSPMMPE_R = crate::BitReader<bool>;
#[doc = "Field `SYSPMMPE` writer - SYS - PMM access protect"]
pub type SYSPMMPE_W<'a> = crate::BitWriter<'a, u16, SYSCTL_SPEC, bool, 2>;
#[doc = "Field `SYSBSLIND` reader - SYS - TCK/RST indication detected"]
pub type SYSBSLIND_R = crate::BitReader<bool>;
#[doc = "Field `SYSBSLIND` writer - SYS - TCK/RST indication detected"]
pub type SYSBSLIND_W<'a> = crate::BitWriter<'a, u16, SYSCTL_SPEC, bool, 4>;
#[doc = "Field `SYSJTAGPIN` reader - SYS - Dedicated JTAG pins enabled"]
pub type SYSJTAGPIN_R = crate::BitReader<bool>;
#[doc = "Field `SYSJTAGPIN` writer - SYS - Dedicated JTAG pins enabled"]
pub type SYSJTAGPIN_W<'a> = crate::BitWriter<'a, u16, SYSCTL_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&self) -> SYSRIVECT_R {
        SYSRIVECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&self) -> SYSPMMPE_R {
        SYSPMMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&self) -> SYSBSLIND_R {
        SYSBSLIND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&self) -> SYSJTAGPIN_R {
        SYSJTAGPIN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - RAM based interrupt vectors"]
    #[inline(always)]
    pub fn sysrivect(&mut self) -> SYSRIVECT_W {
        SYSRIVECT_W::new(self)
    }
    #[doc = "Bit 2 - SYS - PMM access protect"]
    #[inline(always)]
    pub fn syspmmpe(&mut self) -> SYSPMMPE_W {
        SYSPMMPE_W::new(self)
    }
    #[doc = "Bit 4 - SYS - TCK/RST indication detected"]
    #[inline(always)]
    pub fn sysbslind(&mut self) -> SYSBSLIND_W {
        SYSBSLIND_W::new(self)
    }
    #[doc = "Bit 5 - SYS - Dedicated JTAG pins enabled"]
    #[inline(always)]
    pub fn sysjtagpin(&mut self) -> SYSJTAGPIN_W {
        SYSJTAGPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](index.html) module"]
pub struct SYSCTL_SPEC;
impl crate::RegisterSpec for SYSCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysctl::R](R) reader structure"]
impl crate::Readable for SYSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctl::W](W) writer structure"]
impl crate::Writable for SYSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTL to value 0"]
impl crate::Resettable for SYSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
