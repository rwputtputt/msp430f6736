#[doc = "Register `RTCOCAL` reader"]
pub struct R(crate::R<RTCOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCOCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOCAL` writer"]
pub struct W(crate::W<RTCOCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOCAL_SPEC>;
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
impl From<crate::W<RTCOCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCOCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCOCAL0` reader - RTC Offset Calibration Bit 0"]
pub type RTCOCAL0_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL0` writer - RTC Offset Calibration Bit 0"]
pub type RTCOCAL0_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 0>;
#[doc = "Field `RTCOCAL1` reader - RTC Offset Calibration Bit 1"]
pub type RTCOCAL1_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL1` writer - RTC Offset Calibration Bit 1"]
pub type RTCOCAL1_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 1>;
#[doc = "Field `RTCOCAL2` reader - RTC Offset Calibration Bit 2"]
pub type RTCOCAL2_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL2` writer - RTC Offset Calibration Bit 2"]
pub type RTCOCAL2_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 2>;
#[doc = "Field `RTCOCAL3` reader - RTC Offset Calibration Bit 3"]
pub type RTCOCAL3_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL3` writer - RTC Offset Calibration Bit 3"]
pub type RTCOCAL3_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 3>;
#[doc = "Field `RTCOCAL4` reader - RTC Offset Calibration Bit 4"]
pub type RTCOCAL4_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL4` writer - RTC Offset Calibration Bit 4"]
pub type RTCOCAL4_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 4>;
#[doc = "Field `RTCOCAL5` reader - RTC Offset Calibration Bit 5"]
pub type RTCOCAL5_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL5` writer - RTC Offset Calibration Bit 5"]
pub type RTCOCAL5_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 5>;
#[doc = "Field `RTCOCAL6` reader - RTC Offset Calibration Bit 6"]
pub type RTCOCAL6_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL6` writer - RTC Offset Calibration Bit 6"]
pub type RTCOCAL6_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 6>;
#[doc = "Field `RTCOCAL7` reader - RTC Offset Calibration Bit 7"]
pub type RTCOCAL7_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCAL7` writer - RTC Offset Calibration Bit 7"]
pub type RTCOCAL7_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 7>;
#[doc = "Field `RTCOCALS` reader - RTC Offset Calibration Sign"]
pub type RTCOCALS_R = crate::BitReader<bool>;
#[doc = "Field `RTCOCALS` writer - RTC Offset Calibration Sign"]
pub type RTCOCALS_W<'a> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&self) -> RTCOCAL0_R {
        RTCOCAL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&self) -> RTCOCAL1_R {
        RTCOCAL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&self) -> RTCOCAL2_R {
        RTCOCAL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&self) -> RTCOCAL3_R {
        RTCOCAL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&self) -> RTCOCAL4_R {
        RTCOCAL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&self) -> RTCOCAL5_R {
        RTCOCAL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&self) -> RTCOCAL6_R {
        RTCOCAL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&self) -> RTCOCAL7_R {
        RTCOCAL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&mut self) -> RTCOCAL0_W {
        RTCOCAL0_W::new(self)
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&mut self) -> RTCOCAL1_W {
        RTCOCAL1_W::new(self)
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&mut self) -> RTCOCAL2_W {
        RTCOCAL2_W::new(self)
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&mut self) -> RTCOCAL3_W {
        RTCOCAL3_W::new(self)
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&mut self) -> RTCOCAL4_W {
        RTCOCAL4_W::new(self)
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&mut self) -> RTCOCAL5_W {
        RTCOCAL5_W::new(self)
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&mut self) -> RTCOCAL6_W {
        RTCOCAL6_W::new(self)
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&mut self) -> RTCOCAL7_W {
        RTCOCAL7_W::new(self)
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W {
        RTCOCALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Clock Offset Calibartion\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](index.html) module"]
pub struct RTCOCAL_SPEC;
impl crate::RegisterSpec for RTCOCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcocal::R](R) reader structure"]
impl crate::Readable for RTCOCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](W) writer structure"]
impl crate::Writable for RTCOCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOCAL to value 0"]
impl crate::Resettable for RTCOCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
