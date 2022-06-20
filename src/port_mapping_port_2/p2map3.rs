#[doc = "Register `P2MAP3` reader"]
pub struct R(crate::R<P2MAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2MAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2MAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2MAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2MAP3` writer"]
pub struct W(crate::W<P2MAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2MAP3_SPEC>;
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
impl From<crate::W<P2MAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2MAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAP0` reader - PMAP0"]
pub type PMAP0_R = crate::BitReader<bool>;
#[doc = "Field `PMAP0` writer - PMAP0"]
pub type PMAP0_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 0>;
#[doc = "Field `PMAP1` reader - PMAP1"]
pub type PMAP1_R = crate::BitReader<bool>;
#[doc = "Field `PMAP1` writer - PMAP1"]
pub type PMAP1_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 1>;
#[doc = "Field `PMAP2` reader - PMAP2"]
pub type PMAP2_R = crate::BitReader<bool>;
#[doc = "Field `PMAP2` writer - PMAP2"]
pub type PMAP2_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 2>;
#[doc = "Field `PMAP3` reader - PMAP3"]
pub type PMAP3_R = crate::BitReader<bool>;
#[doc = "Field `PMAP3` writer - PMAP3"]
pub type PMAP3_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 3>;
#[doc = "Field `PMAP4` reader - PMAP4"]
pub type PMAP4_R = crate::BitReader<bool>;
#[doc = "Field `PMAP4` writer - PMAP4"]
pub type PMAP4_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 4>;
#[doc = "Field `PMAP5` reader - PMAP5"]
pub type PMAP5_R = crate::BitReader<bool>;
#[doc = "Field `PMAP5` writer - PMAP5"]
pub type PMAP5_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 5>;
#[doc = "Field `PMAP6` reader - PMAP6"]
pub type PMAP6_R = crate::BitReader<bool>;
#[doc = "Field `PMAP6` writer - PMAP6"]
pub type PMAP6_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 6>;
#[doc = "Field `PMAP7` reader - PMAP7"]
pub type PMAP7_R = crate::BitReader<bool>;
#[doc = "Field `PMAP7` writer - PMAP7"]
pub type PMAP7_W<'a> = crate::BitWriter<'a, u8, P2MAP3_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&self) -> PMAP0_R {
        PMAP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&self) -> PMAP1_R {
        PMAP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&self) -> PMAP2_R {
        PMAP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&self) -> PMAP3_R {
        PMAP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&self) -> PMAP4_R {
        PMAP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&self) -> PMAP5_R {
        PMAP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&self) -> PMAP6_R {
        PMAP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&self) -> PMAP7_R {
        PMAP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&mut self) -> PMAP0_W {
        PMAP0_W::new(self)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&mut self) -> PMAP1_W {
        PMAP1_W::new(self)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&mut self) -> PMAP2_W {
        PMAP2_W::new(self)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&mut self) -> PMAP3_W {
        PMAP3_W::new(self)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&mut self) -> PMAP4_W {
        PMAP4_W::new(self)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&mut self) -> PMAP5_W {
        PMAP5_W::new(self)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&mut self) -> PMAP6_W {
        PMAP6_W::new(self)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&mut self) -> PMAP7_W {
        PMAP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P2.3 mapping register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2map3](index.html) module"]
pub struct P2MAP3_SPEC;
impl crate::RegisterSpec for P2MAP3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2map3::R](R) reader structure"]
impl crate::Readable for P2MAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2map3::W](W) writer structure"]
impl crate::Writable for P2MAP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2MAP3 to value 0"]
impl crate::Resettable for P2MAP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
