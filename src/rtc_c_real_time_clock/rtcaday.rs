#[doc = "Register `RTCADAY` reader"]
pub struct R(crate::R<RTCADAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADAY` writer"]
pub struct W(crate::W<RTCADAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADAY_SPEC>;
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
impl From<crate::W<RTCADAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY0` reader - Real Time Clock Day Bit: 0"]
pub type DAY0_R = crate::BitReader<bool>;
#[doc = "Field `DAY0` writer - Real Time Clock Day Bit: 0"]
pub type DAY0_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 0>;
#[doc = "Field `DAY1` reader - Real Time Clock Day Bit: 1"]
pub type DAY1_R = crate::BitReader<bool>;
#[doc = "Field `DAY1` writer - Real Time Clock Day Bit: 1"]
pub type DAY1_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 1>;
#[doc = "Field `DAY2` reader - Real Time Clock Day Bit: 2"]
pub type DAY2_R = crate::BitReader<bool>;
#[doc = "Field `DAY2` writer - Real Time Clock Day Bit: 2"]
pub type DAY2_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 2>;
#[doc = "Field `DAY3` reader - Real Time Clock Day Bit: 3"]
pub type DAY3_R = crate::BitReader<bool>;
#[doc = "Field `DAY3` writer - Real Time Clock Day Bit: 3"]
pub type DAY3_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 3>;
#[doc = "Field `DAY4` reader - Real Time Clock Day Bit: 4"]
pub type DAY4_R = crate::BitReader<bool>;
#[doc = "Field `DAY4` writer - Real Time Clock Day Bit: 4"]
pub type DAY4_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 4>;
#[doc = "Field `DAY5` reader - Real Time Clock Day Bit: 5"]
pub type DAY5_R = crate::BitReader<bool>;
#[doc = "Field `DAY5` writer - Real Time Clock Day Bit: 5"]
pub type DAY5_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 5>;
#[doc = "Field `DAY6` reader - Real Time Clock Day Bit: 6"]
pub type DAY6_R = crate::BitReader<bool>;
#[doc = "Field `DAY6` writer - Real Time Clock Day Bit: 6"]
pub type DAY6_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 6>;
#[doc = "Field `RTCAE` reader - Real Time Clock Alarm enable"]
pub type RTCAE_R = crate::BitReader<bool>;
#[doc = "Field `RTCAE` writer - Real Time Clock Alarm enable"]
pub type RTCAE_W<'a> = crate::BitWriter<'a, u8, RTCADAY_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&self) -> DAY0_R {
        DAY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&self) -> DAY1_R {
        DAY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&self) -> DAY2_R {
        DAY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&self) -> DAY3_R {
        DAY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&self) -> DAY4_R {
        DAY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&self) -> DAY5_R {
        DAY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&self) -> DAY6_R {
        DAY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real Time Clock Alarm enable"]
    #[inline(always)]
    pub fn rtcae(&self) -> RTCAE_R {
        RTCAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Day Bit: 0"]
    #[inline(always)]
    pub fn day0(&mut self) -> DAY0_W {
        DAY0_W::new(self)
    }
    #[doc = "Bit 1 - Real Time Clock Day Bit: 1"]
    #[inline(always)]
    pub fn day1(&mut self) -> DAY1_W {
        DAY1_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Clock Day Bit: 2"]
    #[inline(always)]
    pub fn day2(&mut self) -> DAY2_W {
        DAY2_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Day Bit: 3"]
    #[inline(always)]
    pub fn day3(&mut self) -> DAY3_W {
        DAY3_W::new(self)
    }
    #[doc = "Bit 4 - Real Time Clock Day Bit: 4"]
    #[inline(always)]
    pub fn day4(&mut self) -> DAY4_W {
        DAY4_W::new(self)
    }
    #[doc = "Bit 5 - Real Time Clock Day Bit: 5"]
    #[inline(always)]
    pub fn day5(&mut self) -> DAY5_W {
        DAY5_W::new(self)
    }
    #[doc = "Bit 6 - Real Time Clock Day Bit: 6"]
    #[inline(always)]
    pub fn day6(&mut self) -> DAY6_W {
        DAY6_W::new(self)
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
#[doc = "Real Time Clock Alarm Day\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaday](index.html) module"]
pub struct RTCADAY_SPEC;
impl crate::RegisterSpec for RTCADAY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcaday::R](R) reader structure"]
impl crate::Readable for RTCADAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcaday::W](W) writer structure"]
impl crate::Writable for RTCADAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADAY to value 0"]
impl crate::Resettable for RTCADAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
