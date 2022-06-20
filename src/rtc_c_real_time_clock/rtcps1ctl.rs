#[doc = "Register `RTCPS1CTL` reader"]
pub struct R(crate::R<RTCPS1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS1CTL` writer"]
pub struct W(crate::W<RTCPS1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS1CTL_SPEC>;
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
impl From<crate::W<RTCPS1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT1PSIFG` reader - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_R = crate::BitReader<bool>;
#[doc = "Field `RT1PSIFG` writer - RTC Prescale Timer 1 Interrupt Flag"]
pub type RT1PSIFG_W<'a> = crate::BitWriter<'a, u16, RTCPS1CTL_SPEC, bool, 0>;
#[doc = "Field `RT1PSIE` reader - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_R = crate::BitReader<bool>;
#[doc = "Field `RT1PSIE` writer - RTC Prescale Timer 1 Interrupt Enable Flag"]
pub type RT1PSIE_W<'a> = crate::BitWriter<'a, u16, RTCPS1CTL_SPEC, bool, 1>;
#[doc = "RTC Prescale Timer 1 Interrupt Interval Bit: 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT1IP_A {
    #[doc = "0: RTC Prescale Timer 1 Interrupt Interval /2"]
    RT1IP_0 = 0,
    #[doc = "1: RTC Prescale Timer 1 Interrupt Interval /4"]
    RT1IP_1 = 1,
    #[doc = "2: RTC Prescale Timer 1 Interrupt Interval /8"]
    RT1IP_2 = 2,
    #[doc = "3: RTC Prescale Timer 1 Interrupt Interval /16"]
    RT1IP_3 = 3,
    #[doc = "4: RTC Prescale Timer 1 Interrupt Interval /32"]
    RT1IP_4 = 4,
    #[doc = "5: RTC Prescale Timer 1 Interrupt Interval /64"]
    RT1IP_5 = 5,
    #[doc = "6: RTC Prescale Timer 1 Interrupt Interval /128"]
    RT1IP_6 = 6,
    #[doc = "7: RTC Prescale Timer 1 Interrupt Interval /256"]
    RT1IP_7 = 7,
}
impl From<RT1IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1IP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RT1IP` reader - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_R = crate::FieldReader<u8, RT1IP_A>;
impl RT1IP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1IP_A {
        match self.bits {
            0 => RT1IP_A::RT1IP_0,
            1 => RT1IP_A::RT1IP_1,
            2 => RT1IP_A::RT1IP_2,
            3 => RT1IP_A::RT1IP_3,
            4 => RT1IP_A::RT1IP_4,
            5 => RT1IP_A::RT1IP_5,
            6 => RT1IP_A::RT1IP_6,
            7 => RT1IP_A::RT1IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1IP_0`"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        *self == RT1IP_A::RT1IP_0
    }
    #[doc = "Checks if the value of the field is `RT1IP_1`"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        *self == RT1IP_A::RT1IP_1
    }
    #[doc = "Checks if the value of the field is `RT1IP_2`"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        *self == RT1IP_A::RT1IP_2
    }
    #[doc = "Checks if the value of the field is `RT1IP_3`"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        *self == RT1IP_A::RT1IP_3
    }
    #[doc = "Checks if the value of the field is `RT1IP_4`"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        *self == RT1IP_A::RT1IP_4
    }
    #[doc = "Checks if the value of the field is `RT1IP_5`"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        *self == RT1IP_A::RT1IP_5
    }
    #[doc = "Checks if the value of the field is `RT1IP_6`"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        *self == RT1IP_A::RT1IP_6
    }
    #[doc = "Checks if the value of the field is `RT1IP_7`"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        *self == RT1IP_A::RT1IP_7
    }
}
#[doc = "Field `RT1IP` writer - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
pub type RT1IP_W<'a> = crate::FieldWriterSafe<'a, u16, RTCPS1CTL_SPEC, u8, RT1IP_A, 3, 2>;
impl<'a> RT1IP_W<'a> {
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_0)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_1)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_2)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_3)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_4)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_5)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_6)
    }
    #[doc = "RTC Prescale Timer 1 Interrupt Interval /256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_7)
    }
}
impl R {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Prescale Timer 1 Interrupt Flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W {
        RT1PSIFG_W::new(self)
    }
    #[doc = "Bit 1 - RTC Prescale Timer 1 Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> RT1PSIE_W {
        RT1PSIE_W::new(self)
    }
    #[doc = "Bits 2:4 - RTC Prescale Timer 1 Interrupt Interval Bit: 2"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> RT1IP_W {
        RT1IP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Prescale Timer 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](index.html) module"]
pub struct RTCPS1CTL_SPEC;
impl crate::RegisterSpec for RTCPS1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps1ctl::R](R) reader structure"]
impl crate::Readable for RTCPS1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](W) writer structure"]
impl crate::Writable for RTCPS1CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for RTCPS1CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
