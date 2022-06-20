#[doc = "Register `P9REN` reader"]
pub struct R(crate::R<P9REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9REN` writer"]
pub struct W(crate::W<P9REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9REN_SPEC>;
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
impl From<crate::W<P9REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9REN0` reader - P9REN0"]
pub type P9REN0_R = crate::BitReader<bool>;
#[doc = "Field `P9REN0` writer - P9REN0"]
pub type P9REN0_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 0>;
#[doc = "Field `P9REN1` reader - P9REN1"]
pub type P9REN1_R = crate::BitReader<bool>;
#[doc = "Field `P9REN1` writer - P9REN1"]
pub type P9REN1_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 1>;
#[doc = "Field `P9REN2` reader - P9REN2"]
pub type P9REN2_R = crate::BitReader<bool>;
#[doc = "Field `P9REN2` writer - P9REN2"]
pub type P9REN2_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 2>;
#[doc = "Field `P9REN3` reader - P9REN3"]
pub type P9REN3_R = crate::BitReader<bool>;
#[doc = "Field `P9REN3` writer - P9REN3"]
pub type P9REN3_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 3>;
#[doc = "Field `P9REN4` reader - P9REN4"]
pub type P9REN4_R = crate::BitReader<bool>;
#[doc = "Field `P9REN4` writer - P9REN4"]
pub type P9REN4_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 4>;
#[doc = "Field `P9REN5` reader - P9REN5"]
pub type P9REN5_R = crate::BitReader<bool>;
#[doc = "Field `P9REN5` writer - P9REN5"]
pub type P9REN5_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 5>;
#[doc = "Field `P9REN6` reader - P9REN6"]
pub type P9REN6_R = crate::BitReader<bool>;
#[doc = "Field `P9REN6` writer - P9REN6"]
pub type P9REN6_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 6>;
#[doc = "Field `P9REN7` reader - P9REN7"]
pub type P9REN7_R = crate::BitReader<bool>;
#[doc = "Field `P9REN7` writer - P9REN7"]
pub type P9REN7_W<'a> = crate::BitWriter<'a, u8, P9REN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&self) -> P9REN0_R {
        P9REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&self) -> P9REN1_R {
        P9REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&self) -> P9REN2_R {
        P9REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&self) -> P9REN3_R {
        P9REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&self) -> P9REN4_R {
        P9REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&self) -> P9REN5_R {
        P9REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&self) -> P9REN6_R {
        P9REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&self) -> P9REN7_R {
        P9REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&mut self) -> P9REN0_W {
        P9REN0_W::new(self)
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&mut self) -> P9REN1_W {
        P9REN1_W::new(self)
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&mut self) -> P9REN2_W {
        P9REN2_W::new(self)
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&mut self) -> P9REN3_W {
        P9REN3_W::new(self)
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&mut self) -> P9REN4_W {
        P9REN4_W::new(self)
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&mut self) -> P9REN5_W {
        P9REN5_W::new(self)
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&mut self) -> P9REN6_W {
        P9REN6_W::new(self)
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&mut self) -> P9REN7_W {
        P9REN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9ren](index.html) module"]
pub struct P9REN_SPEC;
impl crate::RegisterSpec for P9REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9ren::R](R) reader structure"]
impl crate::Readable for P9REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9ren::W](W) writer structure"]
impl crate::Writable for P9REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9REN to value 0"]
impl crate::Resettable for P9REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
