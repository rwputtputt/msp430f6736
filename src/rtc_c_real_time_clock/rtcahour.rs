#[doc = "Register `RTCAHOUR` reader"]
pub struct R(crate::R<RTCAHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCAHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCAHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCAHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCAHOUR` writer"]
pub struct W(crate::W<RTCAHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCAHOUR_SPEC>;
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
impl From<crate::W<RTCAHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCAHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOUR0` reader - Real Time Clock Hour Bit: 0"]
pub type HOUR0_R = crate::BitReader<bool>;
#[doc = "Field `HOUR0` writer - Real Time Clock Hour Bit: 0"]
pub type HOUR0_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 0>;
#[doc = "Field `HOUR1` reader - Real Time Clock Hour Bit: 1"]
pub type HOUR1_R = crate::BitReader<bool>;
#[doc = "Field `HOUR1` writer - Real Time Clock Hour Bit: 1"]
pub type HOUR1_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 1>;
#[doc = "Field `HOUR2` reader - Real Time Clock Hour Bit: 2"]
pub type HOUR2_R = crate::BitReader<bool>;
#[doc = "Field `HOUR2` writer - Real Time Clock Hour Bit: 2"]
pub type HOUR2_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 2>;
#[doc = "Field `HOUR3` reader - Real Time Clock Hour Bit: 3"]
pub type HOUR3_R = crate::BitReader<bool>;
#[doc = "Field `HOUR3` writer - Real Time Clock Hour Bit: 3"]
pub type HOUR3_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 3>;
#[doc = "Field `HOUR4` reader - Real Time Clock Hour Bit: 4"]
pub type HOUR4_R = crate::BitReader<bool>;
#[doc = "Field `HOUR4` writer - Real Time Clock Hour Bit: 4"]
pub type HOUR4_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 4>;
#[doc = "Field `HOUR5` reader - Real Time Clock Hour Bit: 5"]
pub type HOUR5_R = crate::BitReader<bool>;
#[doc = "Field `HOUR5` writer - Real Time Clock Hour Bit: 5"]
pub type HOUR5_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 5>;
#[doc = "Field `HOUR6` reader - Real Time Clock Hour Bit: 6"]
pub type HOUR6_R = crate::BitReader<bool>;
#[doc = "Field `HOUR6` writer - Real Time Clock Hour Bit: 6"]
pub type HOUR6_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 6>;
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub type RTCAE_R = crate::BitReader<bool>;
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub type RTCAE_W<'a> = crate::BitWriter<'a, u8, RTCAHOUR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&self) -> HOUR0_R {
        HOUR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&self) -> HOUR1_R {
        HOUR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&self) -> HOUR2_R {
        HOUR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&self) -> HOUR3_R {
        HOUR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&self) -> HOUR4_R {
        HOUR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&self) -> HOUR5_R {
        HOUR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&self) -> HOUR6_R {
        HOUR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Hour Bit: 0"]
    #[inline(always)]
    pub fn hour0(&mut self) -> HOUR0_W {
        HOUR0_W::new(self)
    }
    #[doc = "Bit 1 - Real Time Clock Hour Bit: 1"]
    #[inline(always)]
    pub fn hour1(&mut self) -> HOUR1_W {
        HOUR1_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Clock Hour Bit: 2"]
    #[inline(always)]
    pub fn hour2(&mut self) -> HOUR2_W {
        HOUR2_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Hour Bit: 3"]
    #[inline(always)]
    pub fn hour3(&mut self) -> HOUR3_W {
        HOUR3_W::new(self)
    }
    #[doc = "Bit 4 - Real Time Clock Hour Bit: 4"]
    #[inline(always)]
    pub fn hour4(&mut self) -> HOUR4_W {
        HOUR4_W::new(self)
    }
    #[doc = "Bit 5 - Real Time Clock Hour Bit: 5"]
    #[inline(always)]
    pub fn hour5(&mut self) -> HOUR5_W {
        HOUR5_W::new(self)
    }
    #[doc = "Bit 6 - Real Time Clock Hour Bit: 6"]
    #[inline(always)]
    pub fn hour6(&mut self) -> HOUR6_W {
        HOUR6_W::new(self)
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
#[doc = "Real Time Clock Alarm Hour\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcahour](index.html) module"]
pub struct RTCAHOUR_SPEC;
impl crate::RegisterSpec for RTCAHOUR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcahour::R](R) reader structure"]
impl crate::Readable for RTCAHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcahour::W](W) writer structure"]
impl crate::Writable for RTCAHOUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCAHOUR to value 0"]
impl crate::Resettable for RTCAHOUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
