#[doc = "Register `SD24BIE` reader"]
pub struct R(crate::R<SD24BIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BIE` writer"]
pub struct W(crate::W<SD24BIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BIE_SPEC>;
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
impl From<crate::W<SD24BIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD24IE0` reader - SD24B Channel 0 Interrupt Enable"]
pub type SD24IE0_R = crate::BitReader<bool>;
#[doc = "Field `SD24IE0` writer - SD24B Channel 0 Interrupt Enable"]
pub type SD24IE0_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 0>;
#[doc = "Field `SD24IE1` reader - SD24B Channel 1 Interrupt Enable"]
pub type SD24IE1_R = crate::BitReader<bool>;
#[doc = "Field `SD24IE1` writer - SD24B Channel 1 Interrupt Enable"]
pub type SD24IE1_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 1>;
#[doc = "Field `SD24IE2` reader - SD24B Channel 2 Interrupt Enable"]
pub type SD24IE2_R = crate::BitReader<bool>;
#[doc = "Field `SD24IE2` writer - SD24B Channel 2 Interrupt Enable"]
pub type SD24IE2_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 2>;
#[doc = "Field `SD24OVIE0` reader - SD24B Channel 0 Overflow Interrupt Enable"]
pub type SD24OVIE0_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIE0` writer - SD24B Channel 0 Overflow Interrupt Enable"]
pub type SD24OVIE0_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 8>;
#[doc = "Field `SD24OVIE1` reader - SD24B Channel 1 Overflow Interrupt Enable"]
pub type SD24OVIE1_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIE1` writer - SD24B Channel 1 Overflow Interrupt Enable"]
pub type SD24OVIE1_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 9>;
#[doc = "Field `SD24OVIE2` reader - SD24B Channel 2 Overflow Interrupt Enable"]
pub type SD24OVIE2_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIE2` writer - SD24B Channel 2 Overflow Interrupt Enable"]
pub type SD24OVIE2_W<'a> = crate::BitWriter<'a, u16, SD24BIE_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - SD24B Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie0(&self) -> SD24IE0_R {
        SD24IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD24B Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie1(&self) -> SD24IE1_R {
        SD24IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD24B Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie2(&self) -> SD24IE2_R {
        SD24IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SD24B Channel 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie0(&self) -> SD24OVIE0_R {
        SD24OVIE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD24B Channel 1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie1(&self) -> SD24OVIE1_R {
        SD24OVIE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SD24B Channel 2 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie2(&self) -> SD24OVIE2_R {
        SD24OVIE2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD24B Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie0(&mut self) -> SD24IE0_W {
        SD24IE0_W::new(self)
    }
    #[doc = "Bit 1 - SD24B Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie1(&mut self) -> SD24IE1_W {
        SD24IE1_W::new(self)
    }
    #[doc = "Bit 2 - SD24B Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ie2(&mut self) -> SD24IE2_W {
        SD24IE2_W::new(self)
    }
    #[doc = "Bit 8 - SD24B Channel 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie0(&mut self) -> SD24OVIE0_W {
        SD24OVIE0_W::new(self)
    }
    #[doc = "Bit 9 - SD24B Channel 1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie1(&mut self) -> SD24OVIE1_W {
        SD24OVIE1_W::new(self)
    }
    #[doc = "Bit 10 - SD24B Channel 2 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn sd24ovie2(&mut self) -> SD24OVIE2_W {
        SD24OVIE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bie](index.html) module"]
pub struct SD24BIE_SPEC;
impl crate::RegisterSpec for SD24BIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bie::R](R) reader structure"]
impl crate::Readable for SD24BIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bie::W](W) writer structure"]
impl crate::Writable for SD24BIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BIE to value 0"]
impl crate::Resettable for SD24BIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
