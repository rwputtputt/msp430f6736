#[doc = "Register `P2IN` reader"]
pub struct R(crate::R<P2IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IN` writer"]
pub struct W(crate::W<P2IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IN_SPEC>;
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
impl From<crate::W<P2IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IN0` reader - P2IN0"]
pub type P2IN0_R = crate::BitReader<bool>;
#[doc = "Field `P2IN0` writer - P2IN0"]
pub type P2IN0_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 0>;
#[doc = "Field `P2IN1` reader - P2IN1"]
pub type P2IN1_R = crate::BitReader<bool>;
#[doc = "Field `P2IN1` writer - P2IN1"]
pub type P2IN1_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 1>;
#[doc = "Field `P2IN2` reader - P2IN2"]
pub type P2IN2_R = crate::BitReader<bool>;
#[doc = "Field `P2IN2` writer - P2IN2"]
pub type P2IN2_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 2>;
#[doc = "Field `P2IN3` reader - P2IN3"]
pub type P2IN3_R = crate::BitReader<bool>;
#[doc = "Field `P2IN3` writer - P2IN3"]
pub type P2IN3_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 3>;
#[doc = "Field `P2IN4` reader - P2IN4"]
pub type P2IN4_R = crate::BitReader<bool>;
#[doc = "Field `P2IN4` writer - P2IN4"]
pub type P2IN4_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 4>;
#[doc = "Field `P2IN5` reader - P2IN5"]
pub type P2IN5_R = crate::BitReader<bool>;
#[doc = "Field `P2IN5` writer - P2IN5"]
pub type P2IN5_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 5>;
#[doc = "Field `P2IN6` reader - P2IN6"]
pub type P2IN6_R = crate::BitReader<bool>;
#[doc = "Field `P2IN6` writer - P2IN6"]
pub type P2IN6_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 6>;
#[doc = "Field `P2IN7` reader - P2IN7"]
pub type P2IN7_R = crate::BitReader<bool>;
#[doc = "Field `P2IN7` writer - P2IN7"]
pub type P2IN7_W<'a> = crate::BitWriter<'a, u8, P2IN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&self) -> P2IN0_R {
        P2IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&self) -> P2IN1_R {
        P2IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&self) -> P2IN2_R {
        P2IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&self) -> P2IN3_R {
        P2IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&self) -> P2IN4_R {
        P2IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&self) -> P2IN5_R {
        P2IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&self) -> P2IN6_R {
        P2IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&self) -> P2IN7_R {
        P2IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IN0"]
    #[inline(always)]
    pub fn p2in0(&mut self) -> P2IN0_W {
        P2IN0_W::new(self)
    }
    #[doc = "Bit 1 - P2IN1"]
    #[inline(always)]
    pub fn p2in1(&mut self) -> P2IN1_W {
        P2IN1_W::new(self)
    }
    #[doc = "Bit 2 - P2IN2"]
    #[inline(always)]
    pub fn p2in2(&mut self) -> P2IN2_W {
        P2IN2_W::new(self)
    }
    #[doc = "Bit 3 - P2IN3"]
    #[inline(always)]
    pub fn p2in3(&mut self) -> P2IN3_W {
        P2IN3_W::new(self)
    }
    #[doc = "Bit 4 - P2IN4"]
    #[inline(always)]
    pub fn p2in4(&mut self) -> P2IN4_W {
        P2IN4_W::new(self)
    }
    #[doc = "Bit 5 - P2IN5"]
    #[inline(always)]
    pub fn p2in5(&mut self) -> P2IN5_W {
        P2IN5_W::new(self)
    }
    #[doc = "Bit 6 - P2IN6"]
    #[inline(always)]
    pub fn p2in6(&mut self) -> P2IN6_W {
        P2IN6_W::new(self)
    }
    #[doc = "Bit 7 - P2IN7"]
    #[inline(always)]
    pub fn p2in7(&mut self) -> P2IN7_W {
        P2IN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2in](index.html) module"]
pub struct P2IN_SPEC;
impl crate::RegisterSpec for P2IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2in::R](R) reader structure"]
impl crate::Readable for P2IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2in::W](W) writer structure"]
impl crate::Writable for P2IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IN to value 0"]
impl crate::Resettable for P2IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
