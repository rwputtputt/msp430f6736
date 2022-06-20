#[doc = "Register `P4IN` reader"]
pub struct R(crate::R<P4IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4IN` writer"]
pub struct W(crate::W<P4IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4IN_SPEC>;
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
impl From<crate::W<P4IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P4IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4IN0` reader - P4IN0"]
pub type P4IN0_R = crate::BitReader<bool>;
#[doc = "Field `P4IN0` writer - P4IN0"]
pub type P4IN0_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 0>;
#[doc = "Field `P4IN1` reader - P4IN1"]
pub type P4IN1_R = crate::BitReader<bool>;
#[doc = "Field `P4IN1` writer - P4IN1"]
pub type P4IN1_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 1>;
#[doc = "Field `P4IN2` reader - P4IN2"]
pub type P4IN2_R = crate::BitReader<bool>;
#[doc = "Field `P4IN2` writer - P4IN2"]
pub type P4IN2_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 2>;
#[doc = "Field `P4IN3` reader - P4IN3"]
pub type P4IN3_R = crate::BitReader<bool>;
#[doc = "Field `P4IN3` writer - P4IN3"]
pub type P4IN3_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 3>;
#[doc = "Field `P4IN4` reader - P4IN4"]
pub type P4IN4_R = crate::BitReader<bool>;
#[doc = "Field `P4IN4` writer - P4IN4"]
pub type P4IN4_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 4>;
#[doc = "Field `P4IN5` reader - P4IN5"]
pub type P4IN5_R = crate::BitReader<bool>;
#[doc = "Field `P4IN5` writer - P4IN5"]
pub type P4IN5_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 5>;
#[doc = "Field `P4IN6` reader - P4IN6"]
pub type P4IN6_R = crate::BitReader<bool>;
#[doc = "Field `P4IN6` writer - P4IN6"]
pub type P4IN6_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 6>;
#[doc = "Field `P4IN7` reader - P4IN7"]
pub type P4IN7_R = crate::BitReader<bool>;
#[doc = "Field `P4IN7` writer - P4IN7"]
pub type P4IN7_W<'a> = crate::BitWriter<'a, u8, P4IN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&self) -> P4IN0_R {
        P4IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&self) -> P4IN1_R {
        P4IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&self) -> P4IN2_R {
        P4IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&self) -> P4IN3_R {
        P4IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&self) -> P4IN4_R {
        P4IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&self) -> P4IN5_R {
        P4IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&self) -> P4IN6_R {
        P4IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&self) -> P4IN7_R {
        P4IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IN0"]
    #[inline(always)]
    pub fn p4in0(&mut self) -> P4IN0_W {
        P4IN0_W::new(self)
    }
    #[doc = "Bit 1 - P4IN1"]
    #[inline(always)]
    pub fn p4in1(&mut self) -> P4IN1_W {
        P4IN1_W::new(self)
    }
    #[doc = "Bit 2 - P4IN2"]
    #[inline(always)]
    pub fn p4in2(&mut self) -> P4IN2_W {
        P4IN2_W::new(self)
    }
    #[doc = "Bit 3 - P4IN3"]
    #[inline(always)]
    pub fn p4in3(&mut self) -> P4IN3_W {
        P4IN3_W::new(self)
    }
    #[doc = "Bit 4 - P4IN4"]
    #[inline(always)]
    pub fn p4in4(&mut self) -> P4IN4_W {
        P4IN4_W::new(self)
    }
    #[doc = "Bit 5 - P4IN5"]
    #[inline(always)]
    pub fn p4in5(&mut self) -> P4IN5_W {
        P4IN5_W::new(self)
    }
    #[doc = "Bit 6 - P4IN6"]
    #[inline(always)]
    pub fn p4in6(&mut self) -> P4IN6_W {
        P4IN6_W::new(self)
    }
    #[doc = "Bit 7 - P4IN7"]
    #[inline(always)]
    pub fn p4in7(&mut self) -> P4IN7_W {
        P4IN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 4 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4in](index.html) module"]
pub struct P4IN_SPEC;
impl crate::RegisterSpec for P4IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p4in::R](R) reader structure"]
impl crate::Readable for P4IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4in::W](W) writer structure"]
impl crate::Writable for P4IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4IN to value 0"]
impl crate::Resettable for P4IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
