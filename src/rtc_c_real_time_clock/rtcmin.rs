#[doc = "Register `RTCMIN` reader"]
pub struct R(crate::R<RTCMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCMIN` writer"]
pub struct W(crate::W<RTCMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCMIN_SPEC>;
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
impl From<crate::W<RTCMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINUTES0` reader - Real Time Clock Minutes Bit: 0"]
pub type MINUTES0_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES0` writer - Real Time Clock Minutes Bit: 0"]
pub type MINUTES0_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 0>;
#[doc = "Field `MINUTES1` reader - Real Time Clock Minutes Bit: 1"]
pub type MINUTES1_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES1` writer - Real Time Clock Minutes Bit: 1"]
pub type MINUTES1_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 1>;
#[doc = "Field `MINUTES2` reader - Real Time Clock Minutes Bit: 2"]
pub type MINUTES2_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES2` writer - Real Time Clock Minutes Bit: 2"]
pub type MINUTES2_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 2>;
#[doc = "Field `MINUTES3` reader - Real Time Clock Minutes Bit: 3"]
pub type MINUTES3_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES3` writer - Real Time Clock Minutes Bit: 3"]
pub type MINUTES3_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 3>;
#[doc = "Field `MINUTES4` reader - Real Time Clock Minutes Bit: 4"]
pub type MINUTES4_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES4` writer - Real Time Clock Minutes Bit: 4"]
pub type MINUTES4_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 4>;
#[doc = "Field `MINUTES5` reader - Real Time Clock Minutes Bit: 5"]
pub type MINUTES5_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES5` writer - Real Time Clock Minutes Bit: 5"]
pub type MINUTES5_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 5>;
#[doc = "Field `MINUTES6` reader - Real Time Clock Minutes Bit: 6"]
pub type MINUTES6_R = crate::BitReader<bool>;
#[doc = "Field `MINUTES6` writer - Real Time Clock Minutes Bit: 6"]
pub type MINUTES6_W<'a> = crate::BitWriter<'a, u8, RTCMIN_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&self) -> MINUTES0_R {
        MINUTES0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&self) -> MINUTES1_R {
        MINUTES1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&self) -> MINUTES2_R {
        MINUTES2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&self) -> MINUTES3_R {
        MINUTES3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&self) -> MINUTES4_R {
        MINUTES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&self) -> MINUTES5_R {
        MINUTES5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&self) -> MINUTES6_R {
        MINUTES6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Minutes Bit: 0"]
    #[inline(always)]
    pub fn minutes0(&mut self) -> MINUTES0_W {
        MINUTES0_W::new(self)
    }
    #[doc = "Bit 1 - Real Time Clock Minutes Bit: 1"]
    #[inline(always)]
    pub fn minutes1(&mut self) -> MINUTES1_W {
        MINUTES1_W::new(self)
    }
    #[doc = "Bit 2 - Real Time Clock Minutes Bit: 2"]
    #[inline(always)]
    pub fn minutes2(&mut self) -> MINUTES2_W {
        MINUTES2_W::new(self)
    }
    #[doc = "Bit 3 - Real Time Clock Minutes Bit: 3"]
    #[inline(always)]
    pub fn minutes3(&mut self) -> MINUTES3_W {
        MINUTES3_W::new(self)
    }
    #[doc = "Bit 4 - Real Time Clock Minutes Bit: 4"]
    #[inline(always)]
    pub fn minutes4(&mut self) -> MINUTES4_W {
        MINUTES4_W::new(self)
    }
    #[doc = "Bit 5 - Real Time Clock Minutes Bit: 5"]
    #[inline(always)]
    pub fn minutes5(&mut self) -> MINUTES5_W {
        MINUTES5_W::new(self)
    }
    #[doc = "Bit 6 - Real Time Clock Minutes Bit: 6"]
    #[inline(always)]
    pub fn minutes6(&mut self) -> MINUTES6_W {
        MINUTES6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Clock Minutes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcmin](index.html) module"]
pub struct RTCMIN_SPEC;
impl crate::RegisterSpec for RTCMIN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtcmin::R](R) reader structure"]
impl crate::Readable for RTCMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcmin::W](W) writer structure"]
impl crate::Writable for RTCMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCMIN to value 0"]
impl crate::Resettable for RTCMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
