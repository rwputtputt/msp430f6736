#[doc = "Register `P5SEL` reader"]
pub struct R(crate::R<P5SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5SEL` writer"]
pub struct W(crate::W<P5SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5SEL_SPEC>;
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
impl From<crate::W<P5SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5SEL0` reader - P5SEL0"]
pub type P5SEL0_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL0` writer - P5SEL0"]
pub type P5SEL0_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 0>;
#[doc = "Field `P5SEL1` reader - P5SEL1"]
pub type P5SEL1_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL1` writer - P5SEL1"]
pub type P5SEL1_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 1>;
#[doc = "Field `P5SEL2` reader - P5SEL2"]
pub type P5SEL2_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL2` writer - P5SEL2"]
pub type P5SEL2_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 2>;
#[doc = "Field `P5SEL3` reader - P5SEL3"]
pub type P5SEL3_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL3` writer - P5SEL3"]
pub type P5SEL3_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 3>;
#[doc = "Field `P5SEL4` reader - P5SEL4"]
pub type P5SEL4_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL4` writer - P5SEL4"]
pub type P5SEL4_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 4>;
#[doc = "Field `P5SEL5` reader - P5SEL5"]
pub type P5SEL5_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL5` writer - P5SEL5"]
pub type P5SEL5_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 5>;
#[doc = "Field `P5SEL6` reader - P5SEL6"]
pub type P5SEL6_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL6` writer - P5SEL6"]
pub type P5SEL6_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 6>;
#[doc = "Field `P5SEL7` reader - P5SEL7"]
pub type P5SEL7_R = crate::BitReader<bool>;
#[doc = "Field `P5SEL7` writer - P5SEL7"]
pub type P5SEL7_W<'a> = crate::BitWriter<'a, u8, P5SEL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P5SEL0"]
    #[inline(always)]
    pub fn p5sel0(&self) -> P5SEL0_R {
        P5SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5SEL1"]
    #[inline(always)]
    pub fn p5sel1(&self) -> P5SEL1_R {
        P5SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5SEL2"]
    #[inline(always)]
    pub fn p5sel2(&self) -> P5SEL2_R {
        P5SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5SEL3"]
    #[inline(always)]
    pub fn p5sel3(&self) -> P5SEL3_R {
        P5SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5SEL4"]
    #[inline(always)]
    pub fn p5sel4(&self) -> P5SEL4_R {
        P5SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5SEL5"]
    #[inline(always)]
    pub fn p5sel5(&self) -> P5SEL5_R {
        P5SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5SEL6"]
    #[inline(always)]
    pub fn p5sel6(&self) -> P5SEL6_R {
        P5SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5SEL7"]
    #[inline(always)]
    pub fn p5sel7(&self) -> P5SEL7_R {
        P5SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5SEL0"]
    #[inline(always)]
    pub fn p5sel0(&mut self) -> P5SEL0_W {
        P5SEL0_W::new(self)
    }
    #[doc = "Bit 1 - P5SEL1"]
    #[inline(always)]
    pub fn p5sel1(&mut self) -> P5SEL1_W {
        P5SEL1_W::new(self)
    }
    #[doc = "Bit 2 - P5SEL2"]
    #[inline(always)]
    pub fn p5sel2(&mut self) -> P5SEL2_W {
        P5SEL2_W::new(self)
    }
    #[doc = "Bit 3 - P5SEL3"]
    #[inline(always)]
    pub fn p5sel3(&mut self) -> P5SEL3_W {
        P5SEL3_W::new(self)
    }
    #[doc = "Bit 4 - P5SEL4"]
    #[inline(always)]
    pub fn p5sel4(&mut self) -> P5SEL4_W {
        P5SEL4_W::new(self)
    }
    #[doc = "Bit 5 - P5SEL5"]
    #[inline(always)]
    pub fn p5sel5(&mut self) -> P5SEL5_W {
        P5SEL5_W::new(self)
    }
    #[doc = "Bit 6 - P5SEL6"]
    #[inline(always)]
    pub fn p5sel6(&mut self) -> P5SEL6_W {
        P5SEL6_W::new(self)
    }
    #[doc = "Bit 7 - P5SEL7"]
    #[inline(always)]
    pub fn p5sel7(&mut self) -> P5SEL7_W {
        P5SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5sel](index.html) module"]
pub struct P5SEL_SPEC;
impl crate::RegisterSpec for P5SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5sel::R](R) reader structure"]
impl crate::Readable for P5SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5sel::W](W) writer structure"]
impl crate::Writable for P5SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P5SEL to value 0"]
impl crate::Resettable for P5SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
