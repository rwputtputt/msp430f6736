#[doc = "Register `RTCTCMP` reader"]
pub struct R(crate::R<RTCTCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTCMP` writer"]
pub struct W(crate::W<RTCTCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTCMP_SPEC>;
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
impl From<crate::W<RTCTCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCTCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCTCMP0` reader - RTC Temperature Compensation Bit 0"]
pub type RTCTCMP0_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP0` writer - RTC Temperature Compensation Bit 0"]
pub type RTCTCMP0_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 0>;
#[doc = "Field `RTCTCMP1` reader - RTC Temperature Compensation Bit 1"]
pub type RTCTCMP1_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP1` writer - RTC Temperature Compensation Bit 1"]
pub type RTCTCMP1_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 1>;
#[doc = "Field `RTCTCMP2` reader - RTC Temperature Compensation Bit 2"]
pub type RTCTCMP2_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP2` writer - RTC Temperature Compensation Bit 2"]
pub type RTCTCMP2_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 2>;
#[doc = "Field `RTCTCMP3` reader - RTC Temperature Compensation Bit 3"]
pub type RTCTCMP3_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP3` writer - RTC Temperature Compensation Bit 3"]
pub type RTCTCMP3_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 3>;
#[doc = "Field `RTCTCMP4` reader - RTC Temperature Compensation Bit 4"]
pub type RTCTCMP4_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP4` writer - RTC Temperature Compensation Bit 4"]
pub type RTCTCMP4_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 4>;
#[doc = "Field `RTCTCMP5` reader - RTC Temperature Compensation Bit 5"]
pub type RTCTCMP5_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP5` writer - RTC Temperature Compensation Bit 5"]
pub type RTCTCMP5_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 5>;
#[doc = "Field `RTCTCMP6` reader - RTC Temperature Compensation Bit 6"]
pub type RTCTCMP6_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP6` writer - RTC Temperature Compensation Bit 6"]
pub type RTCTCMP6_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 6>;
#[doc = "Field `RTCTCMP7` reader - RTC Temperature Compensation Bit 7"]
pub type RTCTCMP7_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMP7` writer - RTC Temperature Compensation Bit 7"]
pub type RTCTCMP7_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 7>;
#[doc = "Field `RTCTCOK` reader - RTC Temperature compensation write OK"]
pub type RTCTCOK_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCOK` writer - RTC Temperature compensation write OK"]
pub type RTCTCOK_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 13>;
#[doc = "Field `RTCTCRDY` reader - RTC Temperature compensation ready"]
pub type RTCTCRDY_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCRDY` writer - RTC Temperature compensation ready"]
pub type RTCTCRDY_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 14>;
#[doc = "Field `RTCTCMPS` reader - RTC Temperature Compensation Sign"]
pub type RTCTCMPS_R = crate::BitReader<bool>;
#[doc = "Field `RTCTCMPS` writer - RTC Temperature Compensation Sign"]
pub type RTCTCMPS_W<'a> = crate::BitWriter<'a, u16, RTCTCMP_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&self) -> RTCTCMP0_R {
        RTCTCMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&self) -> RTCTCMP1_R {
        RTCTCMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&self) -> RTCTCMP2_R {
        RTCTCMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&self) -> RTCTCMP3_R {
        RTCTCMP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&self) -> RTCTCMP4_R {
        RTCTCMP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&self) -> RTCTCMP5_R {
        RTCTCMP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&self) -> RTCTCMP6_R {
        RTCTCMP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&self) -> RTCTCMP7_R {
        RTCTCMP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RTCTCOK_R {
        RTCTCOK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RTCTCRDY_R {
        RTCTCRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RTCTCMPS_R {
        RTCTCMPS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&mut self) -> RTCTCMP0_W {
        RTCTCMP0_W::new(self)
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&mut self) -> RTCTCMP1_W {
        RTCTCMP1_W::new(self)
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&mut self) -> RTCTCMP2_W {
        RTCTCMP2_W::new(self)
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&mut self) -> RTCTCMP3_W {
        RTCTCMP3_W::new(self)
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&mut self) -> RTCTCMP4_W {
        RTCTCMP4_W::new(self)
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&mut self) -> RTCTCMP5_W {
        RTCTCMP5_W::new(self)
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&mut self) -> RTCTCMP6_W {
        RTCTCMP6_W::new(self)
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&mut self) -> RTCTCMP7_W {
        RTCTCMP7_W::new(self)
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&mut self) -> RTCTCOK_W {
        RTCTCOK_W::new(self)
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&mut self) -> RTCTCRDY_W {
        RTCTCRDY_W::new(self)
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RTCTCMPS_W {
        RTCTCMPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Temperature Compensation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctcmp](index.html) module"]
pub struct RTCTCMP_SPEC;
impl crate::RegisterSpec for RTCTCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctcmp::R](R) reader structure"]
impl crate::Readable for RTCTCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctcmp::W](W) writer structure"]
impl crate::Writable for RTCTCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTCMP to value 0"]
impl crate::Resettable for RTCTCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
