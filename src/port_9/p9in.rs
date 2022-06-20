#[doc = "Register `P9IN` reader"]
pub struct R(crate::R<P9IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9IN` writer"]
pub struct W(crate::W<P9IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9IN_SPEC>;
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
impl From<crate::W<P9IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9IN0` reader - P9IN0"]
pub type P9IN0_R = crate::BitReader<bool>;
#[doc = "Field `P9IN0` writer - P9IN0"]
pub type P9IN0_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 0>;
#[doc = "Field `P9IN1` reader - P9IN1"]
pub type P9IN1_R = crate::BitReader<bool>;
#[doc = "Field `P9IN1` writer - P9IN1"]
pub type P9IN1_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 1>;
#[doc = "Field `P9IN2` reader - P9IN2"]
pub type P9IN2_R = crate::BitReader<bool>;
#[doc = "Field `P9IN2` writer - P9IN2"]
pub type P9IN2_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 2>;
#[doc = "Field `P9IN3` reader - P9IN3"]
pub type P9IN3_R = crate::BitReader<bool>;
#[doc = "Field `P9IN3` writer - P9IN3"]
pub type P9IN3_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 3>;
#[doc = "Field `P9IN4` reader - P9IN4"]
pub type P9IN4_R = crate::BitReader<bool>;
#[doc = "Field `P9IN4` writer - P9IN4"]
pub type P9IN4_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 4>;
#[doc = "Field `P9IN5` reader - P9IN5"]
pub type P9IN5_R = crate::BitReader<bool>;
#[doc = "Field `P9IN5` writer - P9IN5"]
pub type P9IN5_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 5>;
#[doc = "Field `P9IN6` reader - P9IN6"]
pub type P9IN6_R = crate::BitReader<bool>;
#[doc = "Field `P9IN6` writer - P9IN6"]
pub type P9IN6_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 6>;
#[doc = "Field `P9IN7` reader - P9IN7"]
pub type P9IN7_R = crate::BitReader<bool>;
#[doc = "Field `P9IN7` writer - P9IN7"]
pub type P9IN7_W<'a> = crate::BitWriter<'a, u8, P9IN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9IN0"]
    #[inline(always)]
    pub fn p9in0(&self) -> P9IN0_R {
        P9IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9IN1"]
    #[inline(always)]
    pub fn p9in1(&self) -> P9IN1_R {
        P9IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9IN2"]
    #[inline(always)]
    pub fn p9in2(&self) -> P9IN2_R {
        P9IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9IN3"]
    #[inline(always)]
    pub fn p9in3(&self) -> P9IN3_R {
        P9IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9IN4"]
    #[inline(always)]
    pub fn p9in4(&self) -> P9IN4_R {
        P9IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9IN5"]
    #[inline(always)]
    pub fn p9in5(&self) -> P9IN5_R {
        P9IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9IN6"]
    #[inline(always)]
    pub fn p9in6(&self) -> P9IN6_R {
        P9IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9IN7"]
    #[inline(always)]
    pub fn p9in7(&self) -> P9IN7_R {
        P9IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9IN0"]
    #[inline(always)]
    pub fn p9in0(&mut self) -> P9IN0_W {
        P9IN0_W::new(self)
    }
    #[doc = "Bit 1 - P9IN1"]
    #[inline(always)]
    pub fn p9in1(&mut self) -> P9IN1_W {
        P9IN1_W::new(self)
    }
    #[doc = "Bit 2 - P9IN2"]
    #[inline(always)]
    pub fn p9in2(&mut self) -> P9IN2_W {
        P9IN2_W::new(self)
    }
    #[doc = "Bit 3 - P9IN3"]
    #[inline(always)]
    pub fn p9in3(&mut self) -> P9IN3_W {
        P9IN3_W::new(self)
    }
    #[doc = "Bit 4 - P9IN4"]
    #[inline(always)]
    pub fn p9in4(&mut self) -> P9IN4_W {
        P9IN4_W::new(self)
    }
    #[doc = "Bit 5 - P9IN5"]
    #[inline(always)]
    pub fn p9in5(&mut self) -> P9IN5_W {
        P9IN5_W::new(self)
    }
    #[doc = "Bit 6 - P9IN6"]
    #[inline(always)]
    pub fn p9in6(&mut self) -> P9IN6_W {
        P9IN6_W::new(self)
    }
    #[doc = "Bit 7 - P9IN7"]
    #[inline(always)]
    pub fn p9in7(&mut self) -> P9IN7_W {
        P9IN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9in](index.html) module"]
pub struct P9IN_SPEC;
impl crate::RegisterSpec for P9IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9in::R](R) reader structure"]
impl crate::Readable for P9IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9in::W](W) writer structure"]
impl crate::Writable for P9IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9IN to value 0"]
impl crate::Resettable for P9IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
