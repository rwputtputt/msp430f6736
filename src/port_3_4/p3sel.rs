#[doc = "Register `P3SEL` reader"]
pub struct R(crate::R<P3SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3SEL` writer"]
pub struct W(crate::W<P3SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3SEL_SPEC>;
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
impl From<crate::W<P3SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3SEL0` reader - P3SEL0"]
pub type P3SEL0_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL0` writer - P3SEL0"]
pub type P3SEL0_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 0>;
#[doc = "Field `P3SEL1` reader - P3SEL1"]
pub type P3SEL1_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL1` writer - P3SEL1"]
pub type P3SEL1_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 1>;
#[doc = "Field `P3SEL2` reader - P3SEL2"]
pub type P3SEL2_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL2` writer - P3SEL2"]
pub type P3SEL2_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 2>;
#[doc = "Field `P3SEL3` reader - P3SEL3"]
pub type P3SEL3_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL3` writer - P3SEL3"]
pub type P3SEL3_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 3>;
#[doc = "Field `P3SEL4` reader - P3SEL4"]
pub type P3SEL4_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL4` writer - P3SEL4"]
pub type P3SEL4_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 4>;
#[doc = "Field `P3SEL5` reader - P3SEL5"]
pub type P3SEL5_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL5` writer - P3SEL5"]
pub type P3SEL5_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 5>;
#[doc = "Field `P3SEL6` reader - P3SEL6"]
pub type P3SEL6_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL6` writer - P3SEL6"]
pub type P3SEL6_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 6>;
#[doc = "Field `P3SEL7` reader - P3SEL7"]
pub type P3SEL7_R = crate::BitReader<bool>;
#[doc = "Field `P3SEL7` writer - P3SEL7"]
pub type P3SEL7_W<'a> = crate::BitWriter<'a, u8, P3SEL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3SEL1_R {
        P3SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&self) -> P3SEL2_R {
        P3SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&self) -> P3SEL3_R {
        P3SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&self) -> P3SEL4_R {
        P3SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&self) -> P3SEL5_R {
        P3SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&self) -> P3SEL6_R {
        P3SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&self) -> P3SEL7_R {
        P3SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3SEL0_W {
        P3SEL0_W::new(self)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3SEL1_W {
        P3SEL1_W::new(self)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&mut self) -> P3SEL2_W {
        P3SEL2_W::new(self)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&mut self) -> P3SEL3_W {
        P3SEL3_W::new(self)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&mut self) -> P3SEL4_W {
        P3SEL4_W::new(self)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&mut self) -> P3SEL5_W {
        P3SEL5_W::new(self)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&mut self) -> P3SEL6_W {
        P3SEL6_W::new(self)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&mut self) -> P3SEL7_W {
        P3SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel](index.html) module"]
pub struct P3SEL_SPEC;
impl crate::RegisterSpec for P3SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3sel::R](R) reader structure"]
impl crate::Readable for P3SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3sel::W](W) writer structure"]
impl crate::Writable for P3SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3SEL to value 0"]
impl crate::Resettable for P3SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
