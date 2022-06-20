#[doc = "Register `P6IN` reader"]
pub struct R(crate::R<P6IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6IN` writer"]
pub struct W(crate::W<P6IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6IN_SPEC>;
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
impl From<crate::W<P6IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6IN0` reader - P6IN0"]
pub type P6IN0_R = crate::BitReader<bool>;
#[doc = "Field `P6IN0` writer - P6IN0"]
pub type P6IN0_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 0>;
#[doc = "Field `P6IN1` reader - P6IN1"]
pub type P6IN1_R = crate::BitReader<bool>;
#[doc = "Field `P6IN1` writer - P6IN1"]
pub type P6IN1_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 1>;
#[doc = "Field `P6IN2` reader - P6IN2"]
pub type P6IN2_R = crate::BitReader<bool>;
#[doc = "Field `P6IN2` writer - P6IN2"]
pub type P6IN2_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 2>;
#[doc = "Field `P6IN3` reader - P6IN3"]
pub type P6IN3_R = crate::BitReader<bool>;
#[doc = "Field `P6IN3` writer - P6IN3"]
pub type P6IN3_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 3>;
#[doc = "Field `P6IN4` reader - P6IN4"]
pub type P6IN4_R = crate::BitReader<bool>;
#[doc = "Field `P6IN4` writer - P6IN4"]
pub type P6IN4_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 4>;
#[doc = "Field `P6IN5` reader - P6IN5"]
pub type P6IN5_R = crate::BitReader<bool>;
#[doc = "Field `P6IN5` writer - P6IN5"]
pub type P6IN5_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 5>;
#[doc = "Field `P6IN6` reader - P6IN6"]
pub type P6IN6_R = crate::BitReader<bool>;
#[doc = "Field `P6IN6` writer - P6IN6"]
pub type P6IN6_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 6>;
#[doc = "Field `P6IN7` reader - P6IN7"]
pub type P6IN7_R = crate::BitReader<bool>;
#[doc = "Field `P6IN7` writer - P6IN7"]
pub type P6IN7_W<'a> = crate::BitWriter<'a, u8, P6IN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&self) -> P6IN0_R {
        P6IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&self) -> P6IN1_R {
        P6IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&self) -> P6IN2_R {
        P6IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&self) -> P6IN3_R {
        P6IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&self) -> P6IN4_R {
        P6IN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&self) -> P6IN5_R {
        P6IN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&self) -> P6IN6_R {
        P6IN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&self) -> P6IN7_R {
        P6IN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6IN0"]
    #[inline(always)]
    pub fn p6in0(&mut self) -> P6IN0_W {
        P6IN0_W::new(self)
    }
    #[doc = "Bit 1 - P6IN1"]
    #[inline(always)]
    pub fn p6in1(&mut self) -> P6IN1_W {
        P6IN1_W::new(self)
    }
    #[doc = "Bit 2 - P6IN2"]
    #[inline(always)]
    pub fn p6in2(&mut self) -> P6IN2_W {
        P6IN2_W::new(self)
    }
    #[doc = "Bit 3 - P6IN3"]
    #[inline(always)]
    pub fn p6in3(&mut self) -> P6IN3_W {
        P6IN3_W::new(self)
    }
    #[doc = "Bit 4 - P6IN4"]
    #[inline(always)]
    pub fn p6in4(&mut self) -> P6IN4_W {
        P6IN4_W::new(self)
    }
    #[doc = "Bit 5 - P6IN5"]
    #[inline(always)]
    pub fn p6in5(&mut self) -> P6IN5_W {
        P6IN5_W::new(self)
    }
    #[doc = "Bit 6 - P6IN6"]
    #[inline(always)]
    pub fn p6in6(&mut self) -> P6IN6_W {
        P6IN6_W::new(self)
    }
    #[doc = "Bit 7 - P6IN7"]
    #[inline(always)]
    pub fn p6in7(&mut self) -> P6IN7_W {
        P6IN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 6 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6in](index.html) module"]
pub struct P6IN_SPEC;
impl crate::RegisterSpec for P6IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6in::R](R) reader structure"]
impl crate::Readable for P6IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6in::W](W) writer structure"]
impl crate::Writable for P6IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P6IN to value 0"]
impl crate::Resettable for P6IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
