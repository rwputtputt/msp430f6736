#[doc = "Register `P9DS` reader"]
pub struct R(crate::R<P9DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9DS` writer"]
pub struct W(crate::W<P9DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9DS_SPEC>;
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
impl From<crate::W<P9DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9DS0` reader - P9DS0"]
pub type P9DS0_R = crate::BitReader<bool>;
#[doc = "Field `P9DS0` writer - P9DS0"]
pub type P9DS0_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 0>;
#[doc = "Field `P9DS1` reader - P9DS1"]
pub type P9DS1_R = crate::BitReader<bool>;
#[doc = "Field `P9DS1` writer - P9DS1"]
pub type P9DS1_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 1>;
#[doc = "Field `P9DS2` reader - P9DS2"]
pub type P9DS2_R = crate::BitReader<bool>;
#[doc = "Field `P9DS2` writer - P9DS2"]
pub type P9DS2_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 2>;
#[doc = "Field `P9DS3` reader - P9DS3"]
pub type P9DS3_R = crate::BitReader<bool>;
#[doc = "Field `P9DS3` writer - P9DS3"]
pub type P9DS3_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 3>;
#[doc = "Field `P9DS4` reader - P9DS4"]
pub type P9DS4_R = crate::BitReader<bool>;
#[doc = "Field `P9DS4` writer - P9DS4"]
pub type P9DS4_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 4>;
#[doc = "Field `P9DS5` reader - P9DS5"]
pub type P9DS5_R = crate::BitReader<bool>;
#[doc = "Field `P9DS5` writer - P9DS5"]
pub type P9DS5_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 5>;
#[doc = "Field `P9DS6` reader - P9DS6"]
pub type P9DS6_R = crate::BitReader<bool>;
#[doc = "Field `P9DS6` writer - P9DS6"]
pub type P9DS6_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 6>;
#[doc = "Field `P9DS7` reader - P9DS7"]
pub type P9DS7_R = crate::BitReader<bool>;
#[doc = "Field `P9DS7` writer - P9DS7"]
pub type P9DS7_W<'a> = crate::BitWriter<'a, u8, P9DS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P9DS0"]
    #[inline(always)]
    pub fn p9ds0(&self) -> P9DS0_R {
        P9DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P9DS1"]
    #[inline(always)]
    pub fn p9ds1(&self) -> P9DS1_R {
        P9DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P9DS2"]
    #[inline(always)]
    pub fn p9ds2(&self) -> P9DS2_R {
        P9DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P9DS3"]
    #[inline(always)]
    pub fn p9ds3(&self) -> P9DS3_R {
        P9DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P9DS4"]
    #[inline(always)]
    pub fn p9ds4(&self) -> P9DS4_R {
        P9DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P9DS5"]
    #[inline(always)]
    pub fn p9ds5(&self) -> P9DS5_R {
        P9DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P9DS6"]
    #[inline(always)]
    pub fn p9ds6(&self) -> P9DS6_R {
        P9DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P9DS7"]
    #[inline(always)]
    pub fn p9ds7(&self) -> P9DS7_R {
        P9DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9DS0"]
    #[inline(always)]
    pub fn p9ds0(&mut self) -> P9DS0_W {
        P9DS0_W::new(self)
    }
    #[doc = "Bit 1 - P9DS1"]
    #[inline(always)]
    pub fn p9ds1(&mut self) -> P9DS1_W {
        P9DS1_W::new(self)
    }
    #[doc = "Bit 2 - P9DS2"]
    #[inline(always)]
    pub fn p9ds2(&mut self) -> P9DS2_W {
        P9DS2_W::new(self)
    }
    #[doc = "Bit 3 - P9DS3"]
    #[inline(always)]
    pub fn p9ds3(&mut self) -> P9DS3_W {
        P9DS3_W::new(self)
    }
    #[doc = "Bit 4 - P9DS4"]
    #[inline(always)]
    pub fn p9ds4(&mut self) -> P9DS4_W {
        P9DS4_W::new(self)
    }
    #[doc = "Bit 5 - P9DS5"]
    #[inline(always)]
    pub fn p9ds5(&mut self) -> P9DS5_W {
        P9DS5_W::new(self)
    }
    #[doc = "Bit 6 - P9DS6"]
    #[inline(always)]
    pub fn p9ds6(&mut self) -> P9DS6_W {
        P9DS6_W::new(self)
    }
    #[doc = "Bit 7 - P9DS7"]
    #[inline(always)]
    pub fn p9ds7(&mut self) -> P9DS7_W {
        P9DS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9ds](index.html) module"]
pub struct P9DS_SPEC;
impl crate::RegisterSpec for P9DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9ds::R](R) reader structure"]
impl crate::Readable for P9DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9ds::W](W) writer structure"]
impl crate::Writable for P9DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9DS to value 0"]
impl crate::Resettable for P9DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
