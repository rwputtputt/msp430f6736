#[doc = "Register `P5DS` reader"]
pub struct R(crate::R<P5DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5DS` writer"]
pub struct W(crate::W<P5DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5DS_SPEC>;
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
impl From<crate::W<P5DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5DS0` reader - P5DS0"]
pub type P5DS0_R = crate::BitReader<bool>;
#[doc = "Field `P5DS0` writer - P5DS0"]
pub type P5DS0_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 0>;
#[doc = "Field `P5DS1` reader - P5DS1"]
pub type P5DS1_R = crate::BitReader<bool>;
#[doc = "Field `P5DS1` writer - P5DS1"]
pub type P5DS1_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 1>;
#[doc = "Field `P5DS2` reader - P5DS2"]
pub type P5DS2_R = crate::BitReader<bool>;
#[doc = "Field `P5DS2` writer - P5DS2"]
pub type P5DS2_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 2>;
#[doc = "Field `P5DS3` reader - P5DS3"]
pub type P5DS3_R = crate::BitReader<bool>;
#[doc = "Field `P5DS3` writer - P5DS3"]
pub type P5DS3_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 3>;
#[doc = "Field `P5DS4` reader - P5DS4"]
pub type P5DS4_R = crate::BitReader<bool>;
#[doc = "Field `P5DS4` writer - P5DS4"]
pub type P5DS4_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 4>;
#[doc = "Field `P5DS5` reader - P5DS5"]
pub type P5DS5_R = crate::BitReader<bool>;
#[doc = "Field `P5DS5` writer - P5DS5"]
pub type P5DS5_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 5>;
#[doc = "Field `P5DS6` reader - P5DS6"]
pub type P5DS6_R = crate::BitReader<bool>;
#[doc = "Field `P5DS6` writer - P5DS6"]
pub type P5DS6_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 6>;
#[doc = "Field `P5DS7` reader - P5DS7"]
pub type P5DS7_R = crate::BitReader<bool>;
#[doc = "Field `P5DS7` writer - P5DS7"]
pub type P5DS7_W<'a> = crate::BitWriter<'a, u8, P5DS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    pub fn p5ds0(&self) -> P5DS0_R {
        P5DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    pub fn p5ds1(&self) -> P5DS1_R {
        P5DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    pub fn p5ds2(&self) -> P5DS2_R {
        P5DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    pub fn p5ds3(&self) -> P5DS3_R {
        P5DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    pub fn p5ds4(&self) -> P5DS4_R {
        P5DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    pub fn p5ds5(&self) -> P5DS5_R {
        P5DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    pub fn p5ds6(&self) -> P5DS6_R {
        P5DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    pub fn p5ds7(&self) -> P5DS7_R {
        P5DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5DS0"]
    #[inline(always)]
    pub fn p5ds0(&mut self) -> P5DS0_W {
        P5DS0_W::new(self)
    }
    #[doc = "Bit 1 - P5DS1"]
    #[inline(always)]
    pub fn p5ds1(&mut self) -> P5DS1_W {
        P5DS1_W::new(self)
    }
    #[doc = "Bit 2 - P5DS2"]
    #[inline(always)]
    pub fn p5ds2(&mut self) -> P5DS2_W {
        P5DS2_W::new(self)
    }
    #[doc = "Bit 3 - P5DS3"]
    #[inline(always)]
    pub fn p5ds3(&mut self) -> P5DS3_W {
        P5DS3_W::new(self)
    }
    #[doc = "Bit 4 - P5DS4"]
    #[inline(always)]
    pub fn p5ds4(&mut self) -> P5DS4_W {
        P5DS4_W::new(self)
    }
    #[doc = "Bit 5 - P5DS5"]
    #[inline(always)]
    pub fn p5ds5(&mut self) -> P5DS5_W {
        P5DS5_W::new(self)
    }
    #[doc = "Bit 6 - P5DS6"]
    #[inline(always)]
    pub fn p5ds6(&mut self) -> P5DS6_W {
        P5DS6_W::new(self)
    }
    #[doc = "Bit 7 - P5DS7"]
    #[inline(always)]
    pub fn p5ds7(&mut self) -> P5DS7_W {
        P5DS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5ds](index.html) module"]
pub struct P5DS_SPEC;
impl crate::RegisterSpec for P5DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5ds::R](R) reader structure"]
impl crate::Readable for P5DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5ds::W](W) writer structure"]
impl crate::Writable for P5DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P5DS to value 0"]
impl crate::Resettable for P5DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
