#[doc = "Register `P2SEL` reader"]
pub struct R(crate::R<P2SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2SEL` writer"]
pub struct W(crate::W<P2SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2SEL_SPEC>;
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
impl From<crate::W<P2SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2SEL0` reader - P2SEL0"]
pub type P2SEL0_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0` writer - P2SEL0"]
pub type P2SEL0_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 0>;
#[doc = "Field `P2SEL1` reader - P2SEL1"]
pub type P2SEL1_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL1` writer - P2SEL1"]
pub type P2SEL1_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 1>;
#[doc = "Field `P2SEL2` reader - P2SEL2"]
pub type P2SEL2_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL2` writer - P2SEL2"]
pub type P2SEL2_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 2>;
#[doc = "Field `P2SEL3` reader - P2SEL3"]
pub type P2SEL3_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL3` writer - P2SEL3"]
pub type P2SEL3_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 3>;
#[doc = "Field `P2SEL4` reader - P2SEL4"]
pub type P2SEL4_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL4` writer - P2SEL4"]
pub type P2SEL4_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 4>;
#[doc = "Field `P2SEL5` reader - P2SEL5"]
pub type P2SEL5_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL5` writer - P2SEL5"]
pub type P2SEL5_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 5>;
#[doc = "Field `P2SEL6` reader - P2SEL6"]
pub type P2SEL6_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL6` writer - P2SEL6"]
pub type P2SEL6_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 6>;
#[doc = "Field `P2SEL7` reader - P2SEL7"]
pub type P2SEL7_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL7` writer - P2SEL7"]
pub type P2SEL7_W<'a> = crate::BitWriter<'a, u8, P2SEL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P2SEL0"]
    #[inline(always)]
    pub fn p2sel0(&self) -> P2SEL0_R {
        P2SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&self) -> P2SEL1_R {
        P2SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2SEL2"]
    #[inline(always)]
    pub fn p2sel2(&self) -> P2SEL2_R {
        P2SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2SEL3"]
    #[inline(always)]
    pub fn p2sel3(&self) -> P2SEL3_R {
        P2SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2SEL4"]
    #[inline(always)]
    pub fn p2sel4(&self) -> P2SEL4_R {
        P2SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2SEL5"]
    #[inline(always)]
    pub fn p2sel5(&self) -> P2SEL5_R {
        P2SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2SEL6"]
    #[inline(always)]
    pub fn p2sel6(&self) -> P2SEL6_R {
        P2SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2SEL7"]
    #[inline(always)]
    pub fn p2sel7(&self) -> P2SEL7_R {
        P2SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2SEL0"]
    #[inline(always)]
    pub fn p2sel0(&mut self) -> P2SEL0_W {
        P2SEL0_W::new(self)
    }
    #[doc = "Bit 1 - P2SEL1"]
    #[inline(always)]
    pub fn p2sel1(&mut self) -> P2SEL1_W {
        P2SEL1_W::new(self)
    }
    #[doc = "Bit 2 - P2SEL2"]
    #[inline(always)]
    pub fn p2sel2(&mut self) -> P2SEL2_W {
        P2SEL2_W::new(self)
    }
    #[doc = "Bit 3 - P2SEL3"]
    #[inline(always)]
    pub fn p2sel3(&mut self) -> P2SEL3_W {
        P2SEL3_W::new(self)
    }
    #[doc = "Bit 4 - P2SEL4"]
    #[inline(always)]
    pub fn p2sel4(&mut self) -> P2SEL4_W {
        P2SEL4_W::new(self)
    }
    #[doc = "Bit 5 - P2SEL5"]
    #[inline(always)]
    pub fn p2sel5(&mut self) -> P2SEL5_W {
        P2SEL5_W::new(self)
    }
    #[doc = "Bit 6 - P2SEL6"]
    #[inline(always)]
    pub fn p2sel6(&mut self) -> P2SEL6_W {
        P2SEL6_W::new(self)
    }
    #[doc = "Bit 7 - P2SEL7"]
    #[inline(always)]
    pub fn p2sel7(&mut self) -> P2SEL7_W {
        P2SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2sel](index.html) module"]
pub struct P2SEL_SPEC;
impl crate::RegisterSpec for P2SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2sel::R](R) reader structure"]
impl crate::Readable for P2SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2sel::W](W) writer structure"]
impl crate::Writable for P2SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2SEL to value 0"]
impl crate::Resettable for P2SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
