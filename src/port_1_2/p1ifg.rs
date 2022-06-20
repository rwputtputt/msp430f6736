#[doc = "Register `P1IFG` reader"]
pub struct R(crate::R<P1IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IFG` writer"]
pub struct W(crate::W<P1IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IFG_SPEC>;
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
impl From<crate::W<P1IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IFG0` reader - P1IFG0"]
pub type P1IFG0_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG0` writer - P1IFG0"]
pub type P1IFG0_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 0>;
#[doc = "Field `P1IFG1` reader - P1IFG1"]
pub type P1IFG1_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG1` writer - P1IFG1"]
pub type P1IFG1_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 1>;
#[doc = "Field `P1IFG2` reader - P1IFG2"]
pub type P1IFG2_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG2` writer - P1IFG2"]
pub type P1IFG2_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 2>;
#[doc = "Field `P1IFG3` reader - P1IFG3"]
pub type P1IFG3_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG3` writer - P1IFG3"]
pub type P1IFG3_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 3>;
#[doc = "Field `P1IFG4` reader - P1IFG4"]
pub type P1IFG4_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG4` writer - P1IFG4"]
pub type P1IFG4_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 4>;
#[doc = "Field `P1IFG5` reader - P1IFG5"]
pub type P1IFG5_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG5` writer - P1IFG5"]
pub type P1IFG5_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 5>;
#[doc = "Field `P1IFG6` reader - P1IFG6"]
pub type P1IFG6_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG6` writer - P1IFG6"]
pub type P1IFG6_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 6>;
#[doc = "Field `P1IFG7` reader - P1IFG7"]
pub type P1IFG7_R = crate::BitReader<bool>;
#[doc = "Field `P1IFG7` writer - P1IFG7"]
pub type P1IFG7_W<'a> = crate::BitWriter<'a, u8, P1IFG_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P1IFG0"]
    #[inline(always)]
    pub fn p1ifg0(&self) -> P1IFG0_R {
        P1IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1IFG1"]
    #[inline(always)]
    pub fn p1ifg1(&self) -> P1IFG1_R {
        P1IFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1IFG2"]
    #[inline(always)]
    pub fn p1ifg2(&self) -> P1IFG2_R {
        P1IFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1IFG3"]
    #[inline(always)]
    pub fn p1ifg3(&self) -> P1IFG3_R {
        P1IFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1IFG4"]
    #[inline(always)]
    pub fn p1ifg4(&self) -> P1IFG4_R {
        P1IFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1IFG5"]
    #[inline(always)]
    pub fn p1ifg5(&self) -> P1IFG5_R {
        P1IFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1IFG6"]
    #[inline(always)]
    pub fn p1ifg6(&self) -> P1IFG6_R {
        P1IFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1IFG7"]
    #[inline(always)]
    pub fn p1ifg7(&self) -> P1IFG7_R {
        P1IFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IFG0"]
    #[inline(always)]
    pub fn p1ifg0(&mut self) -> P1IFG0_W {
        P1IFG0_W::new(self)
    }
    #[doc = "Bit 1 - P1IFG1"]
    #[inline(always)]
    pub fn p1ifg1(&mut self) -> P1IFG1_W {
        P1IFG1_W::new(self)
    }
    #[doc = "Bit 2 - P1IFG2"]
    #[inline(always)]
    pub fn p1ifg2(&mut self) -> P1IFG2_W {
        P1IFG2_W::new(self)
    }
    #[doc = "Bit 3 - P1IFG3"]
    #[inline(always)]
    pub fn p1ifg3(&mut self) -> P1IFG3_W {
        P1IFG3_W::new(self)
    }
    #[doc = "Bit 4 - P1IFG4"]
    #[inline(always)]
    pub fn p1ifg4(&mut self) -> P1IFG4_W {
        P1IFG4_W::new(self)
    }
    #[doc = "Bit 5 - P1IFG5"]
    #[inline(always)]
    pub fn p1ifg5(&mut self) -> P1IFG5_W {
        P1IFG5_W::new(self)
    }
    #[doc = "Bit 6 - P1IFG6"]
    #[inline(always)]
    pub fn p1ifg6(&mut self) -> P1IFG6_W {
        P1IFG6_W::new(self)
    }
    #[doc = "Bit 7 - P1IFG7"]
    #[inline(always)]
    pub fn p1ifg7(&mut self) -> P1IFG7_W {
        P1IFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ifg](index.html) module"]
pub struct P1IFG_SPEC;
impl crate::RegisterSpec for P1IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ifg::R](R) reader structure"]
impl crate::Readable for P1IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ifg::W](W) writer structure"]
impl crate::Writable for P1IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IFG to value 0"]
impl crate::Resettable for P1IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
