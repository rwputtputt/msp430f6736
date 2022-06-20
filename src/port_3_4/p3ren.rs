#[doc = "Register `P3REN` reader"]
pub struct R(crate::R<P3REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3REN` writer"]
pub struct W(crate::W<P3REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3REN_SPEC>;
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
impl From<crate::W<P3REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3REN0` reader - P3REN0"]
pub type P3REN0_R = crate::BitReader<bool>;
#[doc = "Field `P3REN0` writer - P3REN0"]
pub type P3REN0_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 0>;
#[doc = "Field `P3REN1` reader - P3REN1"]
pub type P3REN1_R = crate::BitReader<bool>;
#[doc = "Field `P3REN1` writer - P3REN1"]
pub type P3REN1_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 1>;
#[doc = "Field `P3REN2` reader - P3REN2"]
pub type P3REN2_R = crate::BitReader<bool>;
#[doc = "Field `P3REN2` writer - P3REN2"]
pub type P3REN2_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 2>;
#[doc = "Field `P3REN3` reader - P3REN3"]
pub type P3REN3_R = crate::BitReader<bool>;
#[doc = "Field `P3REN3` writer - P3REN3"]
pub type P3REN3_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 3>;
#[doc = "Field `P3REN4` reader - P3REN4"]
pub type P3REN4_R = crate::BitReader<bool>;
#[doc = "Field `P3REN4` writer - P3REN4"]
pub type P3REN4_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 4>;
#[doc = "Field `P3REN5` reader - P3REN5"]
pub type P3REN5_R = crate::BitReader<bool>;
#[doc = "Field `P3REN5` writer - P3REN5"]
pub type P3REN5_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 5>;
#[doc = "Field `P3REN6` reader - P3REN6"]
pub type P3REN6_R = crate::BitReader<bool>;
#[doc = "Field `P3REN6` writer - P3REN6"]
pub type P3REN6_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 6>;
#[doc = "Field `P3REN7` reader - P3REN7"]
pub type P3REN7_R = crate::BitReader<bool>;
#[doc = "Field `P3REN7` writer - P3REN7"]
pub type P3REN7_W<'a> = crate::BitWriter<'a, u8, P3REN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&self) -> P3REN0_R {
        P3REN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&self) -> P3REN1_R {
        P3REN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&self) -> P3REN2_R {
        P3REN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&self) -> P3REN3_R {
        P3REN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&self) -> P3REN4_R {
        P3REN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&self) -> P3REN5_R {
        P3REN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&self) -> P3REN6_R {
        P3REN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&self) -> P3REN7_R {
        P3REN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3REN0"]
    #[inline(always)]
    pub fn p3ren0(&mut self) -> P3REN0_W {
        P3REN0_W::new(self)
    }
    #[doc = "Bit 1 - P3REN1"]
    #[inline(always)]
    pub fn p3ren1(&mut self) -> P3REN1_W {
        P3REN1_W::new(self)
    }
    #[doc = "Bit 2 - P3REN2"]
    #[inline(always)]
    pub fn p3ren2(&mut self) -> P3REN2_W {
        P3REN2_W::new(self)
    }
    #[doc = "Bit 3 - P3REN3"]
    #[inline(always)]
    pub fn p3ren3(&mut self) -> P3REN3_W {
        P3REN3_W::new(self)
    }
    #[doc = "Bit 4 - P3REN4"]
    #[inline(always)]
    pub fn p3ren4(&mut self) -> P3REN4_W {
        P3REN4_W::new(self)
    }
    #[doc = "Bit 5 - P3REN5"]
    #[inline(always)]
    pub fn p3ren5(&mut self) -> P3REN5_W {
        P3REN5_W::new(self)
    }
    #[doc = "Bit 6 - P3REN6"]
    #[inline(always)]
    pub fn p3ren6(&mut self) -> P3REN6_W {
        P3REN6_W::new(self)
    }
    #[doc = "Bit 7 - P3REN7"]
    #[inline(always)]
    pub fn p3ren7(&mut self) -> P3REN7_W {
        P3REN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ren](index.html) module"]
pub struct P3REN_SPEC;
impl crate::RegisterSpec for P3REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3ren::R](R) reader structure"]
impl crate::Readable for P3REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3ren::W](W) writer structure"]
impl crate::Writable for P3REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3REN to value 0"]
impl crate::Resettable for P3REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
