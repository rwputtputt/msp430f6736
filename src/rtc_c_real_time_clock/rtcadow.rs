#[doc = "Register `RTCADOW` reader"]
pub struct R(crate::R<RTCADOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADOW` writer"]
pub struct W(crate::W<RTCADOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADOW_SPEC>;
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
impl From<crate::W<RTCADOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOW0` reader - Real Time Clock DOW Bit: 0"]
pub type DOW0_R = crate::BitReader<bool>;
#[doc = "Field `DOW0` writer - Real Time Clock DOW Bit: 0"]
pub type DOW0_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 0>;
#[doc = "Field `DOW1` reader - Real Time Clock DOW Bit: 1"]
pub type DOW1_R = crate::BitReader<bool>;
#[doc = "Field `DOW1` writer - Real Time Clock DOW Bit: 1"]
pub type DOW1_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 1>;
#[doc = "Field `DOW2` reader - Real Time Clock DOW Bit: 2"]
pub type DOW2_R = crate::BitReader<bool>;
#[doc = "Field `DOW2` writer - Real Time Clock DOW Bit: 2"]
pub type DOW2_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 2>;
#[doc = "Field `DOW3` reader - Real Time Clock DOW Bit: 3"]
pub type DOW3_R = crate::BitReader<bool>;
#[doc = "Field `DOW3` writer - Real Time Clock DOW Bit: 3"]
pub type DOW3_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 3>;
#[doc = "Field `DOW4` reader - Real Time Clock DOW Bit: 4"]
pub type DOW4_R = crate::BitReader<bool>;
#[doc = "Field `DOW4` writer - Real Time Clock DOW Bit: 4"]
pub type DOW4_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 4>;
#[doc = "Field `DOW5` reader - Real Time Clock DOW Bit: 5"]
pub type DOW5_R = crate::BitReader<bool>;
#[doc = "Field `DOW5` writer - Real Time Clock DOW Bit: 5"]
pub type DOW5_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 5>;
#[doc = "Field `DOW6` reader - Real Time Clock DOW Bit: 6"]
pub type DOW6_R = crate::BitReader<bool>;
#[doc = "Field `DOW6` writer - Real Time Clock DOW Bit: 6"]
pub type DOW6_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 6>;
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub type RTCAE_R = crate::BitReader<bool>;
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub type RTCAE_W<'a> = crate::BitWriter<'a, u8, RTCADOW_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&self) -> DOW0_R {
        DOW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&self) -> DOW1_R {
        DOW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&self) -> DOW2_R {
        DOW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&self) -> DOW3_R {
        DOW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&self) -> DOW4_R {
        DOW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&self) -> DOW5_R {
        DOW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&self) -> DOW6_R {
        DOW6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&mut self) -> DOW0_W {
        DOW0_W::new(self)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&mut self) -> DOW1_W {
        DOW1_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&mut self) -> DOW2_W {
        DOW2_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&mut self) -> DOW3_W {
        DOW3_W::new(self)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&mut self) -> DOW4_W {
        DOW4_W::new(self)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&mut self) -> DOW5_W {
        DOW5_W::new(self)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&mut self) -> DOW6_W {
        DOW6_W::new(self)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&mut self) -> RTCAE_W {
        RTCAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Alarm Day of week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadow](index.html) module"]
pub struct RTCADOW_SPEC;
impl crate::RegisterSpec for RTCADOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcadow::R](R) reader structure"]
impl crate::Readable for RTCADOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcadow::W](W) writer structure"]
impl crate::Writable for RTCADOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADOW to value 0"]
impl crate::Resettable for RTCADOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
