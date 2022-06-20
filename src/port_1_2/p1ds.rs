#[doc = "Register `P1DS` reader"]
pub struct R(crate::R<P1DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1DS` writer"]
pub struct W(crate::W<P1DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1DS_SPEC>;
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
impl From<crate::W<P1DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DS0` reader - P1DS0"]
pub type P1DS0_R = crate::BitReader<bool>;
#[doc = "Field `P1DS0` writer - P1DS0"]
pub type P1DS0_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 0>;
#[doc = "Field `P1DS1` reader - P1DS1"]
pub type P1DS1_R = crate::BitReader<bool>;
#[doc = "Field `P1DS1` writer - P1DS1"]
pub type P1DS1_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 1>;
#[doc = "Field `P1DS2` reader - P1DS2"]
pub type P1DS2_R = crate::BitReader<bool>;
#[doc = "Field `P1DS2` writer - P1DS2"]
pub type P1DS2_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 2>;
#[doc = "Field `P1DS3` reader - P1DS3"]
pub type P1DS3_R = crate::BitReader<bool>;
#[doc = "Field `P1DS3` writer - P1DS3"]
pub type P1DS3_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 3>;
#[doc = "Field `P1DS4` reader - P1DS4"]
pub type P1DS4_R = crate::BitReader<bool>;
#[doc = "Field `P1DS4` writer - P1DS4"]
pub type P1DS4_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 4>;
#[doc = "Field `P1DS5` reader - P1DS5"]
pub type P1DS5_R = crate::BitReader<bool>;
#[doc = "Field `P1DS5` writer - P1DS5"]
pub type P1DS5_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 5>;
#[doc = "Field `P1DS6` reader - P1DS6"]
pub type P1DS6_R = crate::BitReader<bool>;
#[doc = "Field `P1DS6` writer - P1DS6"]
pub type P1DS6_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 6>;
#[doc = "Field `P1DS7` reader - P1DS7"]
pub type P1DS7_R = crate::BitReader<bool>;
#[doc = "Field `P1DS7` writer - P1DS7"]
pub type P1DS7_W<'a> = crate::BitWriter<'a, u8, P1DS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P1DS0"]
    #[inline(always)]
    pub fn p1ds0(&self) -> P1DS0_R {
        P1DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1DS1"]
    #[inline(always)]
    pub fn p1ds1(&self) -> P1DS1_R {
        P1DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1DS2"]
    #[inline(always)]
    pub fn p1ds2(&self) -> P1DS2_R {
        P1DS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1DS3"]
    #[inline(always)]
    pub fn p1ds3(&self) -> P1DS3_R {
        P1DS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1DS4"]
    #[inline(always)]
    pub fn p1ds4(&self) -> P1DS4_R {
        P1DS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1DS5"]
    #[inline(always)]
    pub fn p1ds5(&self) -> P1DS5_R {
        P1DS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1DS6"]
    #[inline(always)]
    pub fn p1ds6(&self) -> P1DS6_R {
        P1DS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1DS7"]
    #[inline(always)]
    pub fn p1ds7(&self) -> P1DS7_R {
        P1DS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DS0"]
    #[inline(always)]
    pub fn p1ds0(&mut self) -> P1DS0_W {
        P1DS0_W::new(self)
    }
    #[doc = "Bit 1 - P1DS1"]
    #[inline(always)]
    pub fn p1ds1(&mut self) -> P1DS1_W {
        P1DS1_W::new(self)
    }
    #[doc = "Bit 2 - P1DS2"]
    #[inline(always)]
    pub fn p1ds2(&mut self) -> P1DS2_W {
        P1DS2_W::new(self)
    }
    #[doc = "Bit 3 - P1DS3"]
    #[inline(always)]
    pub fn p1ds3(&mut self) -> P1DS3_W {
        P1DS3_W::new(self)
    }
    #[doc = "Bit 4 - P1DS4"]
    #[inline(always)]
    pub fn p1ds4(&mut self) -> P1DS4_W {
        P1DS4_W::new(self)
    }
    #[doc = "Bit 5 - P1DS5"]
    #[inline(always)]
    pub fn p1ds5(&mut self) -> P1DS5_W {
        P1DS5_W::new(self)
    }
    #[doc = "Bit 6 - P1DS6"]
    #[inline(always)]
    pub fn p1ds6(&mut self) -> P1DS6_W {
        P1DS6_W::new(self)
    }
    #[doc = "Bit 7 - P1DS7"]
    #[inline(always)]
    pub fn p1ds7(&mut self) -> P1DS7_W {
        P1DS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ds](index.html) module"]
pub struct P1DS_SPEC;
impl crate::RegisterSpec for P1DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ds::R](R) reader structure"]
impl crate::Readable for P1DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ds::W](W) writer structure"]
impl crate::Writable for P1DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1DS to value 0"]
impl crate::Resettable for P1DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
