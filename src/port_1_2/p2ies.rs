#[doc = "Register `P2IES` reader"]
pub struct R(crate::R<P2IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IES` writer"]
pub struct W(crate::W<P2IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IES_SPEC>;
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
impl From<crate::W<P2IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IES0` reader - P2IES0"]
pub type P2IES0_R = crate::BitReader<bool>;
#[doc = "Field `P2IES0` writer - P2IES0"]
pub type P2IES0_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 0>;
#[doc = "Field `P2IES1` reader - P2IES1"]
pub type P2IES1_R = crate::BitReader<bool>;
#[doc = "Field `P2IES1` writer - P2IES1"]
pub type P2IES1_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 1>;
#[doc = "Field `P2IES2` reader - P2IES2"]
pub type P2IES2_R = crate::BitReader<bool>;
#[doc = "Field `P2IES2` writer - P2IES2"]
pub type P2IES2_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 2>;
#[doc = "Field `P2IES3` reader - P2IES3"]
pub type P2IES3_R = crate::BitReader<bool>;
#[doc = "Field `P2IES3` writer - P2IES3"]
pub type P2IES3_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 3>;
#[doc = "Field `P2IES4` reader - P2IES4"]
pub type P2IES4_R = crate::BitReader<bool>;
#[doc = "Field `P2IES4` writer - P2IES4"]
pub type P2IES4_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 4>;
#[doc = "Field `P2IES5` reader - P2IES5"]
pub type P2IES5_R = crate::BitReader<bool>;
#[doc = "Field `P2IES5` writer - P2IES5"]
pub type P2IES5_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 5>;
#[doc = "Field `P2IES6` reader - P2IES6"]
pub type P2IES6_R = crate::BitReader<bool>;
#[doc = "Field `P2IES6` writer - P2IES6"]
pub type P2IES6_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 6>;
#[doc = "Field `P2IES7` reader - P2IES7"]
pub type P2IES7_R = crate::BitReader<bool>;
#[doc = "Field `P2IES7` writer - P2IES7"]
pub type P2IES7_W<'a> = crate::BitWriter<'a, u8, P2IES_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&self) -> P2IES0_R {
        P2IES0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&self) -> P2IES1_R {
        P2IES1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&self) -> P2IES2_R {
        P2IES2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&self) -> P2IES3_R {
        P2IES3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&self) -> P2IES4_R {
        P2IES4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&self) -> P2IES5_R {
        P2IES5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&self) -> P2IES6_R {
        P2IES6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&self) -> P2IES7_R {
        P2IES7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IES0"]
    #[inline(always)]
    pub fn p2ies0(&mut self) -> P2IES0_W {
        P2IES0_W::new(self)
    }
    #[doc = "Bit 1 - P2IES1"]
    #[inline(always)]
    pub fn p2ies1(&mut self) -> P2IES1_W {
        P2IES1_W::new(self)
    }
    #[doc = "Bit 2 - P2IES2"]
    #[inline(always)]
    pub fn p2ies2(&mut self) -> P2IES2_W {
        P2IES2_W::new(self)
    }
    #[doc = "Bit 3 - P2IES3"]
    #[inline(always)]
    pub fn p2ies3(&mut self) -> P2IES3_W {
        P2IES3_W::new(self)
    }
    #[doc = "Bit 4 - P2IES4"]
    #[inline(always)]
    pub fn p2ies4(&mut self) -> P2IES4_W {
        P2IES4_W::new(self)
    }
    #[doc = "Bit 5 - P2IES5"]
    #[inline(always)]
    pub fn p2ies5(&mut self) -> P2IES5_W {
        P2IES5_W::new(self)
    }
    #[doc = "Bit 6 - P2IES6"]
    #[inline(always)]
    pub fn p2ies6(&mut self) -> P2IES6_W {
        P2IES6_W::new(self)
    }
    #[doc = "Bit 7 - P2IES7"]
    #[inline(always)]
    pub fn p2ies7(&mut self) -> P2IES7_W {
        P2IES7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ies](index.html) module"]
pub struct P2IES_SPEC;
impl crate::RegisterSpec for P2IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ies::R](R) reader structure"]
impl crate::Readable for P2IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ies::W](W) writer structure"]
impl crate::Writable for P2IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IES to value 0"]
impl crate::Resettable for P2IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
