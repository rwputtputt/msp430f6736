#[doc = "Register `P1IN` reader"]
pub struct R(crate::R<P1IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IN` writer"]
pub struct W(crate::W<P1IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IN_SPEC>;
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
impl From<crate::W<P1IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IN0` reader - P1IN0"]
pub type P1IN0_R = crate::BitReader<bool>;
#[doc = "Field `P1IN0` writer - P1IN0"]
pub type P1IN0_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 0>;
#[doc = "Field `P1IN1` reader - P1IN1"]
pub type P1IN1_R = crate::BitReader<bool>;
#[doc = "Field `P1IN1` writer - P1IN1"]
pub type P1IN1_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 1>;
#[doc = "Field `P1IN2` reader - P1IN2"]
pub type P1IN2_R = crate::BitReader<bool>;
#[doc = "Field `P1IN2` writer - P1IN2"]
pub type P1IN2_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 2>;
#[doc = "Field `P1IN3` reader - P1IN3"]
pub type P1IN3_R = crate::BitReader<bool>;
#[doc = "Field `P1IN3` writer - P1IN3"]
pub type P1IN3_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 3>;
#[doc = "Field `P1IN4` reader - P1IN4"]
pub type P1IN4_R = crate::BitReader<bool>;
#[doc = "Field `P1IN4` writer - P1IN4"]
pub type P1IN4_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 4>;
#[doc = "Field `P1IN5` reader - P1IN5"]
pub type P1IN5_R = crate::BitReader<bool>;
#[doc = "Field `P1IN5` writer - P1IN5"]
pub type P1IN5_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 5>;
#[doc = "Field `P1IN6` reader - P1IN6"]
pub type P1IN6_R = crate::BitReader<bool>;
#[doc = "Field `P1IN6` writer - P1IN6"]
pub type P1IN6_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 6>;
#[doc = "Field `P1IN7` reader - P1IN7"]
pub type P1IN7_R = crate::BitReader<bool>;
#[doc = "Field `P1IN7` writer - P1IN7"]
pub type P1IN7_W<'a> = crate::BitWriter<'a, u8, P1IN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&self) -> P1IN0_R {
        P1IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&self) -> P1IN1_R {
        P1IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&self) -> P1IN2_R {
        P1IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&self) -> P1IN3_R {
        P1IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&self) -> P1IN4_R {
        P1IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&self) -> P1IN5_R {
        P1IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&self) -> P1IN6_R {
        P1IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&self) -> P1IN7_R {
        P1IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IN0"]
    #[inline(always)]
    pub fn p1in0(&mut self) -> P1IN0_W {
        P1IN0_W::new(self)
    }
    #[doc = "Bit 1 - P1IN1"]
    #[inline(always)]
    pub fn p1in1(&mut self) -> P1IN1_W {
        P1IN1_W::new(self)
    }
    #[doc = "Bit 2 - P1IN2"]
    #[inline(always)]
    pub fn p1in2(&mut self) -> P1IN2_W {
        P1IN2_W::new(self)
    }
    #[doc = "Bit 3 - P1IN3"]
    #[inline(always)]
    pub fn p1in3(&mut self) -> P1IN3_W {
        P1IN3_W::new(self)
    }
    #[doc = "Bit 4 - P1IN4"]
    #[inline(always)]
    pub fn p1in4(&mut self) -> P1IN4_W {
        P1IN4_W::new(self)
    }
    #[doc = "Bit 5 - P1IN5"]
    #[inline(always)]
    pub fn p1in5(&mut self) -> P1IN5_W {
        P1IN5_W::new(self)
    }
    #[doc = "Bit 6 - P1IN6"]
    #[inline(always)]
    pub fn p1in6(&mut self) -> P1IN6_W {
        P1IN6_W::new(self)
    }
    #[doc = "Bit 7 - P1IN7"]
    #[inline(always)]
    pub fn p1in7(&mut self) -> P1IN7_W {
        P1IN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1in](index.html) module"]
pub struct P1IN_SPEC;
impl crate::RegisterSpec for P1IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1in::R](R) reader structure"]
impl crate::Readable for P1IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1in::W](W) writer structure"]
impl crate::Writable for P1IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IN to value 0"]
impl crate::Resettable for P1IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
