#[doc = "Register `P9DIR` reader"]
pub struct R(crate::R<P9DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9DIR` writer"]
pub struct W(crate::W<P9DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9DIR_SPEC>;
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
impl From<crate::W<P9DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9DIR0` reader - P9DIR0"]
pub type P9DIR0_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR0` writer - P9DIR0"]
pub type P9DIR0_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 0>;
#[doc = "Field `P9DIR1` reader - P9DIR1"]
pub type P9DIR1_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR1` writer - P9DIR1"]
pub type P9DIR1_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 1>;
#[doc = "Field `P9DIR2` reader - P9DIR2"]
pub type P9DIR2_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR2` writer - P9DIR2"]
pub type P9DIR2_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 2>;
#[doc = "Field `P9DIR3` reader - P9DIR3"]
pub type P9DIR3_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR3` writer - P9DIR3"]
pub type P9DIR3_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 3>;
#[doc = "Field `P9DIR4` reader - P9DIR4"]
pub type P9DIR4_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR4` writer - P9DIR4"]
pub type P9DIR4_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 4>;
#[doc = "Field `P9DIR5` reader - P9DIR5"]
pub type P9DIR5_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR5` writer - P9DIR5"]
pub type P9DIR5_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 5>;
#[doc = "Field `P9DIR6` reader - P9DIR6"]
pub type P9DIR6_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR6` writer - P9DIR6"]
pub type P9DIR6_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 6>;
#[doc = "Field `P9DIR7` reader - P9DIR7"]
pub type P9DIR7_R = crate::BitReader<bool>;
#[doc = "Field `P9DIR7` writer - P9DIR7"]
pub type P9DIR7_W<'a> = crate::BitWriter<'a, u8, P9DIR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9DIR0"]
    #[inline(always)]
    pub fn p9dir0(&self) -> P9DIR0_R {
        P9DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9DIR1"]
    #[inline(always)]
    pub fn p9dir1(&self) -> P9DIR1_R {
        P9DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9DIR2"]
    #[inline(always)]
    pub fn p9dir2(&self) -> P9DIR2_R {
        P9DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9DIR3"]
    #[inline(always)]
    pub fn p9dir3(&self) -> P9DIR3_R {
        P9DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9DIR4"]
    #[inline(always)]
    pub fn p9dir4(&self) -> P9DIR4_R {
        P9DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9DIR5"]
    #[inline(always)]
    pub fn p9dir5(&self) -> P9DIR5_R {
        P9DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9DIR6"]
    #[inline(always)]
    pub fn p9dir6(&self) -> P9DIR6_R {
        P9DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9DIR7"]
    #[inline(always)]
    pub fn p9dir7(&self) -> P9DIR7_R {
        P9DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9DIR0"]
    #[inline(always)]
    pub fn p9dir0(&mut self) -> P9DIR0_W {
        P9DIR0_W::new(self)
    }
    #[doc = "Bit 1 - P9DIR1"]
    #[inline(always)]
    pub fn p9dir1(&mut self) -> P9DIR1_W {
        P9DIR1_W::new(self)
    }
    #[doc = "Bit 2 - P9DIR2"]
    #[inline(always)]
    pub fn p9dir2(&mut self) -> P9DIR2_W {
        P9DIR2_W::new(self)
    }
    #[doc = "Bit 3 - P9DIR3"]
    #[inline(always)]
    pub fn p9dir3(&mut self) -> P9DIR3_W {
        P9DIR3_W::new(self)
    }
    #[doc = "Bit 4 - P9DIR4"]
    #[inline(always)]
    pub fn p9dir4(&mut self) -> P9DIR4_W {
        P9DIR4_W::new(self)
    }
    #[doc = "Bit 5 - P9DIR5"]
    #[inline(always)]
    pub fn p9dir5(&mut self) -> P9DIR5_W {
        P9DIR5_W::new(self)
    }
    #[doc = "Bit 6 - P9DIR6"]
    #[inline(always)]
    pub fn p9dir6(&mut self) -> P9DIR6_W {
        P9DIR6_W::new(self)
    }
    #[doc = "Bit 7 - P9DIR7"]
    #[inline(always)]
    pub fn p9dir7(&mut self) -> P9DIR7_W {
        P9DIR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9dir](index.html) module"]
pub struct P9DIR_SPEC;
impl crate::RegisterSpec for P9DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9dir::R](R) reader structure"]
impl crate::Readable for P9DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9dir::W](W) writer structure"]
impl crate::Writable for P9DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9DIR to value 0"]
impl crate::Resettable for P9DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
