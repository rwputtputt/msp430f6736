#[doc = "Register `P2REN` reader"]
pub struct R(crate::R<P2REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2REN` writer"]
pub struct W(crate::W<P2REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2REN_SPEC>;
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
impl From<crate::W<P2REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2REN0` reader - P2REN0"]
pub type P2REN0_R = crate::BitReader<bool>;
#[doc = "Field `P2REN0` writer - P2REN0"]
pub type P2REN0_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 0>;
#[doc = "Field `P2REN1` reader - P2REN1"]
pub type P2REN1_R = crate::BitReader<bool>;
#[doc = "Field `P2REN1` writer - P2REN1"]
pub type P2REN1_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 1>;
#[doc = "Field `P2REN2` reader - P2REN2"]
pub type P2REN2_R = crate::BitReader<bool>;
#[doc = "Field `P2REN2` writer - P2REN2"]
pub type P2REN2_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 2>;
#[doc = "Field `P2REN3` reader - P2REN3"]
pub type P2REN3_R = crate::BitReader<bool>;
#[doc = "Field `P2REN3` writer - P2REN3"]
pub type P2REN3_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 3>;
#[doc = "Field `P2REN4` reader - P2REN4"]
pub type P2REN4_R = crate::BitReader<bool>;
#[doc = "Field `P2REN4` writer - P2REN4"]
pub type P2REN4_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 4>;
#[doc = "Field `P2REN5` reader - P2REN5"]
pub type P2REN5_R = crate::BitReader<bool>;
#[doc = "Field `P2REN5` writer - P2REN5"]
pub type P2REN5_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 5>;
#[doc = "Field `P2REN6` reader - P2REN6"]
pub type P2REN6_R = crate::BitReader<bool>;
#[doc = "Field `P2REN6` writer - P2REN6"]
pub type P2REN6_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 6>;
#[doc = "Field `P2REN7` reader - P2REN7"]
pub type P2REN7_R = crate::BitReader<bool>;
#[doc = "Field `P2REN7` writer - P2REN7"]
pub type P2REN7_W<'a> = crate::BitWriter<'a, u8, P2REN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&self) -> P2REN0_R {
        P2REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&self) -> P2REN1_R {
        P2REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&self) -> P2REN2_R {
        P2REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&self) -> P2REN3_R {
        P2REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&self) -> P2REN4_R {
        P2REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&self) -> P2REN5_R {
        P2REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&self) -> P2REN6_R {
        P2REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&self) -> P2REN7_R {
        P2REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2REN0"]
    #[inline(always)]
    pub fn p2ren0(&mut self) -> P2REN0_W {
        P2REN0_W::new(self)
    }
    #[doc = "Bit 1 - P2REN1"]
    #[inline(always)]
    pub fn p2ren1(&mut self) -> P2REN1_W {
        P2REN1_W::new(self)
    }
    #[doc = "Bit 2 - P2REN2"]
    #[inline(always)]
    pub fn p2ren2(&mut self) -> P2REN2_W {
        P2REN2_W::new(self)
    }
    #[doc = "Bit 3 - P2REN3"]
    #[inline(always)]
    pub fn p2ren3(&mut self) -> P2REN3_W {
        P2REN3_W::new(self)
    }
    #[doc = "Bit 4 - P2REN4"]
    #[inline(always)]
    pub fn p2ren4(&mut self) -> P2REN4_W {
        P2REN4_W::new(self)
    }
    #[doc = "Bit 5 - P2REN5"]
    #[inline(always)]
    pub fn p2ren5(&mut self) -> P2REN5_W {
        P2REN5_W::new(self)
    }
    #[doc = "Bit 6 - P2REN6"]
    #[inline(always)]
    pub fn p2ren6(&mut self) -> P2REN6_W {
        P2REN6_W::new(self)
    }
    #[doc = "Bit 7 - P2REN7"]
    #[inline(always)]
    pub fn p2ren7(&mut self) -> P2REN7_W {
        P2REN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ren](index.html) module"]
pub struct P2REN_SPEC;
impl crate::RegisterSpec for P2REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ren::R](R) reader structure"]
impl crate::Readable for P2REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ren::W](W) writer structure"]
impl crate::Writable for P2REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2REN to value 0"]
impl crate::Resettable for P2REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
