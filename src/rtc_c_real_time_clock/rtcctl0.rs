#[doc = "Register `RTCCTL0` reader"]
pub struct R(crate::R<RTCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL0` writer"]
pub struct W(crate::W<RTCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL0_SPEC>;
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
impl From<crate::W<RTCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCRDYIFG` reader - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDYIFG` writer - RTC Ready Interrupt Flag"]
pub type RTCRDYIFG_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 0>;
#[doc = "Field `RTCAIFG` reader - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCAIFG` writer - RTC Alarm Interrupt Flag"]
pub type RTCAIFG_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 1>;
#[doc = "Field `RTCTEVIFG` reader - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCTEVIFG` writer - RTC Time Event Interrupt Flag"]
pub type RTCTEVIFG_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 2>;
#[doc = "Field `RTCOFIFG` reader - RTC 32kHz cyrstal oscillator fault interrupt flag"]
pub type RTCOFIFG_R = crate::BitReader<bool>;
#[doc = "Field `RTCOFIFG` writer - RTC 32kHz cyrstal oscillator fault interrupt flag"]
pub type RTCOFIFG_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 3>;
#[doc = "Field `RTCRDYIE` reader - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCRDYIE` writer - RTC Ready Interrupt Enable Flag"]
pub type RTCRDYIE_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 4>;
#[doc = "Field `RTCAIE` reader - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCAIE` writer - RTC Alarm Interrupt Enable Flag"]
pub type RTCAIE_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 5>;
#[doc = "Field `RTCTEVIE` reader - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCTEVIE` writer - RTC Time Event Interrupt Enable Flag"]
pub type RTCTEVIE_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 6>;
#[doc = "Field `RTCOFIE` reader - RTC 32kHz cyrstal oscillator fault interrupt enable"]
pub type RTCOFIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCOFIE` writer - RTC 32kHz cyrstal oscillator fault interrupt enable"]
pub type RTCOFIE_W<'a> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC 32kHz cyrstal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RTCOFIFG_R {
        RTCOFIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC 32kHz cyrstal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RTCOFIE_R {
        RTCOFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W {
        RTCRDYIFG_W::new(self)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W {
        RTCAIFG_W::new(self)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W {
        RTCTEVIFG_W::new(self)
    }
    #[doc = "Bit 3 - RTC 32kHz cyrstal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RTCOFIFG_W {
        RTCOFIFG_W::new(self)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W {
        RTCRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RTCAIE_W {
        RTCAIE_W::new(self)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W {
        RTCTEVIE_W::new(self)
    }
    #[doc = "Bit 7 - RTC 32kHz cyrstal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RTCOFIE_W {
        RTCOFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Clock Control 0/Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl0](index.html) module"]
pub struct RTCCTL0_SPEC;
impl crate::RegisterSpec for RTCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl0::R](R) reader structure"]
impl crate::Readable for RTCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl0::W](W) writer structure"]
impl crate::Writable for RTCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL0 to value 0"]
impl crate::Resettable for RTCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
