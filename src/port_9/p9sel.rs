#[doc = "Register `P9SEL` reader"]
pub struct R(crate::R<P9SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9SEL` writer"]
pub struct W(crate::W<P9SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9SEL_SPEC>;
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
impl From<crate::W<P9SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9SEL0` reader - P9SEL0"]
pub type P9SEL0_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL0` writer - P9SEL0"]
pub type P9SEL0_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 0>;
#[doc = "Field `P9SEL1` reader - P9SEL1"]
pub type P9SEL1_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL1` writer - P9SEL1"]
pub type P9SEL1_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 1>;
#[doc = "Field `P9SEL2` reader - P9SEL2"]
pub type P9SEL2_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL2` writer - P9SEL2"]
pub type P9SEL2_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 2>;
#[doc = "Field `P9SEL3` reader - P9SEL3"]
pub type P9SEL3_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL3` writer - P9SEL3"]
pub type P9SEL3_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 3>;
#[doc = "Field `P9SEL4` reader - P9SEL4"]
pub type P9SEL4_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL4` writer - P9SEL4"]
pub type P9SEL4_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 4>;
#[doc = "Field `P9SEL5` reader - P9SEL5"]
pub type P9SEL5_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL5` writer - P9SEL5"]
pub type P9SEL5_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 5>;
#[doc = "Field `P9SEL6` reader - P9SEL6"]
pub type P9SEL6_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL6` writer - P9SEL6"]
pub type P9SEL6_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 6>;
#[doc = "Field `P9SEL7` reader - P9SEL7"]
pub type P9SEL7_R = crate::BitReader<bool>;
#[doc = "Field `P9SEL7` writer - P9SEL7"]
pub type P9SEL7_W<'a> = crate::BitWriter<'a, u8, P9SEL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9SEL0"]
    #[inline(always)]
    pub fn p9sel0(&self) -> P9SEL0_R {
        P9SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9SEL1"]
    #[inline(always)]
    pub fn p9sel1(&self) -> P9SEL1_R {
        P9SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9SEL2"]
    #[inline(always)]
    pub fn p9sel2(&self) -> P9SEL2_R {
        P9SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9SEL3"]
    #[inline(always)]
    pub fn p9sel3(&self) -> P9SEL3_R {
        P9SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9SEL4"]
    #[inline(always)]
    pub fn p9sel4(&self) -> P9SEL4_R {
        P9SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9SEL5"]
    #[inline(always)]
    pub fn p9sel5(&self) -> P9SEL5_R {
        P9SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9SEL6"]
    #[inline(always)]
    pub fn p9sel6(&self) -> P9SEL6_R {
        P9SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9SEL7"]
    #[inline(always)]
    pub fn p9sel7(&self) -> P9SEL7_R {
        P9SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9SEL0"]
    #[inline(always)]
    pub fn p9sel0(&mut self) -> P9SEL0_W {
        P9SEL0_W::new(self)
    }
    #[doc = "Bit 1 - P9SEL1"]
    #[inline(always)]
    pub fn p9sel1(&mut self) -> P9SEL1_W {
        P9SEL1_W::new(self)
    }
    #[doc = "Bit 2 - P9SEL2"]
    #[inline(always)]
    pub fn p9sel2(&mut self) -> P9SEL2_W {
        P9SEL2_W::new(self)
    }
    #[doc = "Bit 3 - P9SEL3"]
    #[inline(always)]
    pub fn p9sel3(&mut self) -> P9SEL3_W {
        P9SEL3_W::new(self)
    }
    #[doc = "Bit 4 - P9SEL4"]
    #[inline(always)]
    pub fn p9sel4(&mut self) -> P9SEL4_W {
        P9SEL4_W::new(self)
    }
    #[doc = "Bit 5 - P9SEL5"]
    #[inline(always)]
    pub fn p9sel5(&mut self) -> P9SEL5_W {
        P9SEL5_W::new(self)
    }
    #[doc = "Bit 6 - P9SEL6"]
    #[inline(always)]
    pub fn p9sel6(&mut self) -> P9SEL6_W {
        P9SEL6_W::new(self)
    }
    #[doc = "Bit 7 - P9SEL7"]
    #[inline(always)]
    pub fn p9sel7(&mut self) -> P9SEL7_W {
        P9SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9sel](index.html) module"]
pub struct P9SEL_SPEC;
impl crate::RegisterSpec for P9SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9sel::R](R) reader structure"]
impl crate::Readable for P9SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9sel::W](W) writer structure"]
impl crate::Writable for P9SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9SEL to value 0"]
impl crate::Resettable for P9SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
