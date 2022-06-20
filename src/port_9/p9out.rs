#[doc = "Register `P9OUT` reader"]
pub struct R(crate::R<P9OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9OUT` writer"]
pub struct W(crate::W<P9OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9OUT_SPEC>;
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
impl From<crate::W<P9OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9OUT0` reader - P9OUT0"]
pub type P9OUT0_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT0` writer - P9OUT0"]
pub type P9OUT0_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 0>;
#[doc = "Field `P9OUT1` reader - P9OUT1"]
pub type P9OUT1_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT1` writer - P9OUT1"]
pub type P9OUT1_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 1>;
#[doc = "Field `P9OUT2` reader - P9OUT2"]
pub type P9OUT2_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT2` writer - P9OUT2"]
pub type P9OUT2_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 2>;
#[doc = "Field `P9OUT3` reader - P9OUT3"]
pub type P9OUT3_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT3` writer - P9OUT3"]
pub type P9OUT3_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 3>;
#[doc = "Field `P9OUT4` reader - P9OUT4"]
pub type P9OUT4_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT4` writer - P9OUT4"]
pub type P9OUT4_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 4>;
#[doc = "Field `P9OUT5` reader - P9OUT5"]
pub type P9OUT5_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT5` writer - P9OUT5"]
pub type P9OUT5_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 5>;
#[doc = "Field `P9OUT6` reader - P9OUT6"]
pub type P9OUT6_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT6` writer - P9OUT6"]
pub type P9OUT6_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 6>;
#[doc = "Field `P9OUT7` reader - P9OUT7"]
pub type P9OUT7_R = crate::BitReader<bool>;
#[doc = "Field `P9OUT7` writer - P9OUT7"]
pub type P9OUT7_W<'a> = crate::BitWriter<'a, u8, P9OUT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&self) -> P9OUT0_R {
        P9OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&self) -> P9OUT1_R {
        P9OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&self) -> P9OUT2_R {
        P9OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&self) -> P9OUT3_R {
        P9OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&self) -> P9OUT4_R {
        P9OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&self) -> P9OUT5_R {
        P9OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&self) -> P9OUT6_R {
        P9OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&self) -> P9OUT7_R {
        P9OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&mut self) -> P9OUT0_W {
        P9OUT0_W::new(self)
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&mut self) -> P9OUT1_W {
        P9OUT1_W::new(self)
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&mut self) -> P9OUT2_W {
        P9OUT2_W::new(self)
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&mut self) -> P9OUT3_W {
        P9OUT3_W::new(self)
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&mut self) -> P9OUT4_W {
        P9OUT4_W::new(self)
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&mut self) -> P9OUT5_W {
        P9OUT5_W::new(self)
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&mut self) -> P9OUT6_W {
        P9OUT6_W::new(self)
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&mut self) -> P9OUT7_W {
        P9OUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9out](index.html) module"]
pub struct P9OUT_SPEC;
impl crate::RegisterSpec for P9OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9out::R](R) reader structure"]
impl crate::Readable for P9OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9out::W](W) writer structure"]
impl crate::Writable for P9OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9OUT to value 0"]
impl crate::Resettable for P9OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
