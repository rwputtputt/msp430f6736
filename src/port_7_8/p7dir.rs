#[doc = "Register `P7DIR` reader"]
pub struct R(crate::R<P7DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P7DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P7DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P7DIR` writer"]
pub struct W(crate::W<P7DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P7DIR_SPEC>;
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
impl From<crate::W<P7DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P7DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7DIR0` reader - P7DIR0"]
pub type P7DIR0_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR0` writer - P7DIR0"]
pub type P7DIR0_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 0>;
#[doc = "Field `P7DIR1` reader - P7DIR1"]
pub type P7DIR1_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR1` writer - P7DIR1"]
pub type P7DIR1_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 1>;
#[doc = "Field `P7DIR2` reader - P7DIR2"]
pub type P7DIR2_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR2` writer - P7DIR2"]
pub type P7DIR2_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 2>;
#[doc = "Field `P7DIR3` reader - P7DIR3"]
pub type P7DIR3_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR3` writer - P7DIR3"]
pub type P7DIR3_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 3>;
#[doc = "Field `P7DIR4` reader - P7DIR4"]
pub type P7DIR4_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR4` writer - P7DIR4"]
pub type P7DIR4_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 4>;
#[doc = "Field `P7DIR5` reader - P7DIR5"]
pub type P7DIR5_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR5` writer - P7DIR5"]
pub type P7DIR5_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 5>;
#[doc = "Field `P7DIR6` reader - P7DIR6"]
pub type P7DIR6_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR6` writer - P7DIR6"]
pub type P7DIR6_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 6>;
#[doc = "Field `P7DIR7` reader - P7DIR7"]
pub type P7DIR7_R = crate::BitReader<bool>;
#[doc = "Field `P7DIR7` writer - P7DIR7"]
pub type P7DIR7_W<'a> = crate::BitWriter<'a, u8, P7DIR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&self) -> P7DIR0_R {
        P7DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&self) -> P7DIR1_R {
        P7DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&self) -> P7DIR2_R {
        P7DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&self) -> P7DIR3_R {
        P7DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&self) -> P7DIR4_R {
        P7DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&self) -> P7DIR5_R {
        P7DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&self) -> P7DIR6_R {
        P7DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&self) -> P7DIR7_R {
        P7DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DIR0"]
    #[inline(always)]
    pub fn p7dir0(&mut self) -> P7DIR0_W {
        P7DIR0_W::new(self)
    }
    #[doc = "Bit 1 - P7DIR1"]
    #[inline(always)]
    pub fn p7dir1(&mut self) -> P7DIR1_W {
        P7DIR1_W::new(self)
    }
    #[doc = "Bit 2 - P7DIR2"]
    #[inline(always)]
    pub fn p7dir2(&mut self) -> P7DIR2_W {
        P7DIR2_W::new(self)
    }
    #[doc = "Bit 3 - P7DIR3"]
    #[inline(always)]
    pub fn p7dir3(&mut self) -> P7DIR3_W {
        P7DIR3_W::new(self)
    }
    #[doc = "Bit 4 - P7DIR4"]
    #[inline(always)]
    pub fn p7dir4(&mut self) -> P7DIR4_W {
        P7DIR4_W::new(self)
    }
    #[doc = "Bit 5 - P7DIR5"]
    #[inline(always)]
    pub fn p7dir5(&mut self) -> P7DIR5_W {
        P7DIR5_W::new(self)
    }
    #[doc = "Bit 6 - P7DIR6"]
    #[inline(always)]
    pub fn p7dir6(&mut self) -> P7DIR6_W {
        P7DIR6_W::new(self)
    }
    #[doc = "Bit 7 - P7DIR7"]
    #[inline(always)]
    pub fn p7dir7(&mut self) -> P7DIR7_W {
        P7DIR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 7 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7dir](index.html) module"]
pub struct P7DIR_SPEC;
impl crate::RegisterSpec for P7DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p7dir::R](R) reader structure"]
impl crate::Readable for P7DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p7dir::W](W) writer structure"]
impl crate::Writable for P7DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P7DIR to value 0"]
impl crate::Resettable for P7DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
