#[doc = "Register `RTCSEC` reader"]
pub struct R(crate::R<RTCSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSEC` writer"]
pub struct W(crate::W<RTCSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSEC_SPEC>;
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
impl From<crate::W<RTCSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECONDS0` reader - Real Time Clock Seconds Bit: 0"]
pub type SECONDS0_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS0` writer - Real Time Clock Seconds Bit: 0"]
pub type SECONDS0_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 0>;
#[doc = "Field `SECONDS1` reader - Real Time Clock Seconds Bit: 1"]
pub type SECONDS1_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS1` writer - Real Time Clock Seconds Bit: 1"]
pub type SECONDS1_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 1>;
#[doc = "Field `SECONDS2` reader - Real Time Clock Seconds Bit: 2"]
pub type SECONDS2_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS2` writer - Real Time Clock Seconds Bit: 2"]
pub type SECONDS2_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 2>;
#[doc = "Field `SECONDS3` reader - Real Time Clock Seconds Bit: 3"]
pub type SECONDS3_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS3` writer - Real Time Clock Seconds Bit: 3"]
pub type SECONDS3_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 3>;
#[doc = "Field `SECONDS4` reader - Real Time Clock Seconds Bit: 4"]
pub type SECONDS4_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS4` writer - Real Time Clock Seconds Bit: 4"]
pub type SECONDS4_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 4>;
#[doc = "Field `SECONDS5` reader - Real Time Clock Seconds Bit: 5"]
pub type SECONDS5_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS5` writer - Real Time Clock Seconds Bit: 5"]
pub type SECONDS5_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 5>;
#[doc = "Field `SECONDS6` reader - Real Time Clock Seconds Bit: 6"]
pub type SECONDS6_R = crate::BitReader<bool>;
#[doc = "Field `SECONDS6` writer - Real Time Clock Seconds Bit: 6"]
pub type SECONDS6_W<'a> = crate::BitWriter<'a, u8, RTCSEC_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    pub fn seconds0(&self) -> SECONDS0_R {
        SECONDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    pub fn seconds1(&self) -> SECONDS1_R {
        SECONDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    pub fn seconds2(&self) -> SECONDS2_R {
        SECONDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    pub fn seconds3(&self) -> SECONDS3_R {
        SECONDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    pub fn seconds4(&self) -> SECONDS4_R {
        SECONDS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    pub fn seconds5(&self) -> SECONDS5_R {
        SECONDS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    pub fn seconds6(&self) -> SECONDS6_R {
        SECONDS6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Seconds Bit: 0"]
    #[inline(always)]
    pub fn seconds0(&mut self) -> SECONDS0_W {
        SECONDS0_W::new(self)
    }
    #[doc = "Bit 1 - Real Time Clock Seconds Bit: 1"]
    #[inline(always)]
    pub fn seconds1(&mut self) -> SECONDS1_W {
        SECONDS1_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Clock Seconds Bit: 2"]
    #[inline(always)]
    pub fn seconds2(&mut self) -> SECONDS2_W {
        SECONDS2_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Seconds Bit: 3"]
    #[inline(always)]
    pub fn seconds3(&mut self) -> SECONDS3_W {
        SECONDS3_W::new(self)
    }
    #[doc = "Bit 4 - Real Time Clock Seconds Bit: 4"]
    #[inline(always)]
    pub fn seconds4(&mut self) -> SECONDS4_W {
        SECONDS4_W::new(self)
    }
    #[doc = "Bit 5 - Real Time Clock Seconds Bit: 5"]
    #[inline(always)]
    pub fn seconds5(&mut self) -> SECONDS5_W {
        SECONDS5_W::new(self)
    }
    #[doc = "Bit 6 - Real Time Clock Seconds Bit: 6"]
    #[inline(always)]
    pub fn seconds6(&mut self) -> SECONDS6_W {
        SECONDS6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsec](index.html) module"]
pub struct RTCSEC_SPEC;
impl crate::RegisterSpec for RTCSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcsec::R](R) reader structure"]
impl crate::Readable for RTCSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsec::W](W) writer structure"]
impl crate::Writable for RTCSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCSEC to value 0"]
impl crate::Resettable for RTCSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
