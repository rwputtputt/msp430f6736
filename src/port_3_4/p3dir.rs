#[doc = "Register `P3DIR` reader"]
pub struct R(crate::R<P3DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3DIR` writer"]
pub struct W(crate::W<P3DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3DIR_SPEC>;
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
impl From<crate::W<P3DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3DIR0` reader - P3DIR0"]
pub type P3DIR0_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR0` writer - P3DIR0"]
pub type P3DIR0_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 0>;
#[doc = "Field `P3DIR1` reader - P3DIR1"]
pub type P3DIR1_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR1` writer - P3DIR1"]
pub type P3DIR1_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 1>;
#[doc = "Field `P3DIR2` reader - P3DIR2"]
pub type P3DIR2_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR2` writer - P3DIR2"]
pub type P3DIR2_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 2>;
#[doc = "Field `P3DIR3` reader - P3DIR3"]
pub type P3DIR3_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR3` writer - P3DIR3"]
pub type P3DIR3_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 3>;
#[doc = "Field `P3DIR4` reader - P3DIR4"]
pub type P3DIR4_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR4` writer - P3DIR4"]
pub type P3DIR4_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 4>;
#[doc = "Field `P3DIR5` reader - P3DIR5"]
pub type P3DIR5_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR5` writer - P3DIR5"]
pub type P3DIR5_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 5>;
#[doc = "Field `P3DIR6` reader - P3DIR6"]
pub type P3DIR6_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR6` writer - P3DIR6"]
pub type P3DIR6_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 6>;
#[doc = "Field `P3DIR7` reader - P3DIR7"]
pub type P3DIR7_R = crate::BitReader<bool>;
#[doc = "Field `P3DIR7` writer - P3DIR7"]
pub type P3DIR7_W<'a> = crate::BitWriter<'a, u8, P3DIR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&self) -> P3DIR0_R {
        P3DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&self) -> P3DIR1_R {
        P3DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&self) -> P3DIR2_R {
        P3DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&self) -> P3DIR3_R {
        P3DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&self) -> P3DIR4_R {
        P3DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&self) -> P3DIR5_R {
        P3DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&self) -> P3DIR6_R {
        P3DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&self) -> P3DIR7_R {
        P3DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3DIR0"]
    #[inline(always)]
    pub fn p3dir0(&mut self) -> P3DIR0_W {
        P3DIR0_W::new(self)
    }
    #[doc = "Bit 1 - P3DIR1"]
    #[inline(always)]
    pub fn p3dir1(&mut self) -> P3DIR1_W {
        P3DIR1_W::new(self)
    }
    #[doc = "Bit 2 - P3DIR2"]
    #[inline(always)]
    pub fn p3dir2(&mut self) -> P3DIR2_W {
        P3DIR2_W::new(self)
    }
    #[doc = "Bit 3 - P3DIR3"]
    #[inline(always)]
    pub fn p3dir3(&mut self) -> P3DIR3_W {
        P3DIR3_W::new(self)
    }
    #[doc = "Bit 4 - P3DIR4"]
    #[inline(always)]
    pub fn p3dir4(&mut self) -> P3DIR4_W {
        P3DIR4_W::new(self)
    }
    #[doc = "Bit 5 - P3DIR5"]
    #[inline(always)]
    pub fn p3dir5(&mut self) -> P3DIR5_W {
        P3DIR5_W::new(self)
    }
    #[doc = "Bit 6 - P3DIR6"]
    #[inline(always)]
    pub fn p3dir6(&mut self) -> P3DIR6_W {
        P3DIR6_W::new(self)
    }
    #[doc = "Bit 7 - P3DIR7"]
    #[inline(always)]
    pub fn p3dir7(&mut self) -> P3DIR7_W {
        P3DIR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3dir](index.html) module"]
pub struct P3DIR_SPEC;
impl crate::RegisterSpec for P3DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3dir::R](R) reader structure"]
impl crate::Readable for P3DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3dir::W](W) writer structure"]
impl crate::Writable for P3DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3DIR to value 0"]
impl crate::Resettable for P3DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
