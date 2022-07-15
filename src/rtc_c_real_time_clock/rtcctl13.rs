#[doc = "Register `RTCCTL13` reader"]
pub struct R(crate::R<RTCCTL13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL13` writer"]
pub struct W(crate::W<RTCCTL13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL13_SPEC>;
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
impl From<crate::W<RTCCTL13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Time Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: RTC Time Event: 0 (Min. changed)"]
    RTCTEV_0 = 0,
    #[doc = "1: RTC Time Event: 1 (Hour changed)"]
    RTCTEV_1 = 1,
    #[doc = "2: RTC Time Event: 2 (12:00 changed)"]
    RTCTEV_2 = 2,
    #[doc = "3: RTC Time Event: 3 (00:00 changed)"]
    RTCTEV_3 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCTEV` reader - RTC Time Event 1"]
pub type RTCTEV_R = crate::FieldReader<u8, RTCTEV_A>;
impl RTCTEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::RTCTEV_0,
            1 => RTCTEV_A::RTCTEV_1,
            2 => RTCTEV_A::RTCTEV_2,
            3 => RTCTEV_A::RTCTEV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEV_0`"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_0
    }
    #[doc = "Checks if the value of the field is `RTCTEV_1`"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_1
    }
    #[doc = "Checks if the value of the field is `RTCTEV_2`"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_2
    }
    #[doc = "Checks if the value of the field is `RTCTEV_3`"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_3
    }
}
#[doc = "Field `RTCTEV` writer - RTC Time Event 1"]
pub type RTCTEV_W<'a> = crate::FieldWriterSafe<'a, u16, RTCCTL13_SPEC, u8, RTCTEV_A, 2, 0>;
impl<'a> RTCTEV_W<'a> {
    #[doc = "RTC Time Event: 0 (Min. changed)"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_0)
    }
    #[doc = "RTC Time Event: 1 (Hour changed)"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_1)
    }
    #[doc = "RTC Time Event: 2 (12:00 changed)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_2)
    }
    #[doc = "RTC Time Event: 3 (00:00 changed)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_3)
    }
}
#[doc = "RTC Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: RTC Source Select ACLK"]
    RTCSSEL_0 = 0,
    #[doc = "1: RTC Source Select SMCLK"]
    RTCSSEL_1 = 1,
    #[doc = "2: RTC Source Select RT1PS"]
    RTCSSEL_2 = 2,
    #[doc = "3: RTC Source Select RT1PS"]
    RTCSSEL_3 = 3,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCSSEL` reader - RTC Source Select 1"]
pub type RTCSSEL_R = crate::FieldReader<u8, RTCSSEL_A>;
impl RTCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSSEL_A {
        match self.bits {
            0 => RTCSSEL_A::RTCSSEL_0,
            1 => RTCSSEL_A::RTCSSEL_1,
            2 => RTCSSEL_A::RTCSSEL_2,
            3 => RTCSSEL_A::RTCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_0`"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_0
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_1`"]
    #[inline(always)]
    pub fn is_rtcssel_1(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_1
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_2`"]
    #[inline(always)]
    pub fn is_rtcssel_2(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_2
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_3`"]
    #[inline(always)]
    pub fn is_rtcssel_3(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_3
    }
}
#[doc = "Field `RTCSSEL` writer - RTC Source Select 1"]
pub type RTCSSEL_W<'a> = crate::FieldWriterSafe<'a, u16, RTCCTL13_SPEC, u8, RTCSSEL_A, 2, 2>;
impl<'a> RTCSSEL_W<'a> {
    #[doc = "RTC Source Select ACLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_0)
    }
    #[doc = "RTC Source Select SMCLK"]
    #[inline(always)]
    pub fn rtcssel_1(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_1)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_2(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_2)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_3(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_3)
    }
}
#[doc = "Field `RTCRDY` reader - RTC Ready"]
pub type RTCRDY_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDY` writer - RTC Ready"]
pub type RTCRDY_W<'a> = crate::BitWriter<'a, u16, RTCCTL13_SPEC, bool, 4>;
#[doc = "Field `RTCMODE` reader - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_R = crate::BitReader<bool>;
#[doc = "Field `RTCMODE` writer - RTC Mode 0:Counter / 1: Calendar"]
pub type RTCMODE_W<'a> = crate::BitWriter<'a, u16, RTCCTL13_SPEC, bool, 5>;
#[doc = "Field `RTCHOLD` reader - RTC Hold"]
pub type RTCHOLD_R = crate::BitReader<bool>;
#[doc = "Field `RTCHOLD` writer - RTC Hold"]
pub type RTCHOLD_W<'a> = crate::BitWriter<'a, u16, RTCCTL13_SPEC, bool, 6>;
#[doc = "Field `RTCBCD` reader - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_R = crate::BitReader<bool>;
#[doc = "Field `RTCBCD` writer - RTC BCD 0:Binary / 1:BCD"]
pub type RTCBCD_W<'a> = crate::BitWriter<'a, u16, RTCCTL13_SPEC, bool, 7>;
#[doc = "RTC Calibration Frequency Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: RTC Calibration Frequency: No Output"]
    RTCCALF_0 = 0,
    #[doc = "1: RTC Calibration Frequency: 512 Hz"]
    RTCCALF_1 = 1,
    #[doc = "2: RTC Calibration Frequency: 256 Hz"]
    RTCCALF_2 = 2,
    #[doc = "3: RTC Calibration Frequency: 1 Hz"]
    RTCCALF_3 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCCALF` reader - RTC Calibration Frequency Bit 1"]
pub type RTCCALF_R = crate::FieldReader<u8, RTCCALF_A>;
impl RTCCALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCALF_0`"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "Checks if the value of the field is `RTCCALF_1`"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "Checks if the value of the field is `RTCCALF_2`"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "Checks if the value of the field is `RTCCALF_3`"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_3
    }
}
#[doc = "Field `RTCCALF` writer - RTC Calibration Frequency Bit 1"]
pub type RTCCALF_W<'a> = crate::FieldWriterSafe<'a, u16, RTCCTL13_SPEC, u8, RTCCALF_A, 2, 8>;
impl<'a> RTCCALF_W<'a> {
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RTCTEV_W {
        RTCTEV_W::new(self)
    }
    #[doc = "Bits 2:3 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RTCSSEL_W {
        RTCSSEL_W::new(self)
    }
    #[doc = "Bit 4 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&mut self) -> RTCRDY_W {
        RTCRDY_W::new(self)
    }
    #[doc = "Bit 5 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&mut self) -> RTCMODE_W {
        RTCMODE_W::new(self)
    }
    #[doc = "Bit 6 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RTCHOLD_W {
        RTCHOLD_W::new(self)
    }
    #[doc = "Bit 7 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RTCBCD_W {
        RTCBCD_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Clock Control 1/3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl13](index.html) module"]
pub struct RTCCTL13_SPEC;
impl crate::RegisterSpec for RTCCTL13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl13::R](R) reader structure"]
impl crate::Readable for RTCCTL13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl13::W](W) writer structure"]
impl crate::Writable for RTCCTL13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL13 to value 0"]
impl crate::Resettable for RTCCTL13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
