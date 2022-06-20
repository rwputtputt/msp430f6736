#[doc = "Register `ADC10IFG` reader"]
pub struct R(crate::R<ADC10IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10IFG` writer"]
pub struct W(crate::W<ADC10IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10IFG_SPEC>;
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
impl From<crate::W<ADC10IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10IFG0` reader - ADC10_A Interrupt Flag"]
pub type ADC10IFG0_R = crate::BitReader<bool>;
#[doc = "Field `ADC10IFG0` writer - ADC10_A Interrupt Flag"]
pub type ADC10IFG0_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 0>;
#[doc = "Field `ADC10INIFG` reader - ADC10_A Interrupt Flag for the inside of window of the Window comparator"]
pub type ADC10INIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADC10INIFG` writer - ADC10_A Interrupt Flag for the inside of window of the Window comparator"]
pub type ADC10INIFG_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 1>;
#[doc = "Field `ADC10LOIFG` reader - ADC10_A Interrupt Flag for lower threshold of the Window comparator"]
pub type ADC10LOIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADC10LOIFG` writer - ADC10_A Interrupt Flag for lower threshold of the Window comparator"]
pub type ADC10LOIFG_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 2>;
#[doc = "Field `ADC10HIIFG` reader - ADC10_A Interrupt Flag for upper threshold of the Window comparator"]
pub type ADC10HIIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADC10HIIFG` writer - ADC10_A Interrupt Flag for upper threshold of the Window comparator"]
pub type ADC10HIIFG_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 3>;
#[doc = "Field `ADC10OVIFG` reader - ADC10_A ADC10MEM overflow Interrupt Flag"]
pub type ADC10OVIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADC10OVIFG` writer - ADC10_A ADC10MEM overflow Interrupt Flag"]
pub type ADC10OVIFG_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 4>;
#[doc = "Field `ADC10TOVIFG` reader - ADC10_A conversion-time-overflow Interrupt Flag"]
pub type ADC10TOVIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADC10TOVIFG` writer - ADC10_A conversion-time-overflow Interrupt Flag"]
pub type ADC10TOVIFG_W<'a> = crate::BitWriter<'a, u16, ADC10IFG_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - ADC10_A Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg0(&self) -> ADC10IFG0_R {
        ADC10IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10_A Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc10inifg(&self) -> ADC10INIFG_R {
        ADC10INIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10_A Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10loifg(&self) -> ADC10LOIFG_R {
        ADC10LOIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10_A Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10hiifg(&self) -> ADC10HIIFG_R {
        ADC10HIIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10_A ADC10MEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ovifg(&self) -> ADC10OVIFG_R {
        ADC10OVIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC10_A conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc10tovifg(&self) -> ADC10TOVIFG_R {
        ADC10TOVIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10_A Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg0(&mut self) -> ADC10IFG0_W {
        ADC10IFG0_W::new(self)
    }
    #[doc = "Bit 1 - ADC10_A Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc10inifg(&mut self) -> ADC10INIFG_W {
        ADC10INIFG_W::new(self)
    }
    #[doc = "Bit 2 - ADC10_A Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10loifg(&mut self) -> ADC10LOIFG_W {
        ADC10LOIFG_W::new(self)
    }
    #[doc = "Bit 3 - ADC10_A Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10hiifg(&mut self) -> ADC10HIIFG_W {
        ADC10HIIFG_W::new(self)
    }
    #[doc = "Bit 4 - ADC10_A ADC10MEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ovifg(&mut self) -> ADC10OVIFG_W {
        ADC10OVIFG_W::new(self)
    }
    #[doc = "Bit 5 - ADC10_A conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc10tovifg(&mut self) -> ADC10TOVIFG_W {
        ADC10TOVIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ifg](index.html) module"]
pub struct ADC10IFG_SPEC;
impl crate::RegisterSpec for ADC10IFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ifg::R](R) reader structure"]
impl crate::Readable for ADC10IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ifg::W](W) writer structure"]
impl crate::Writable for ADC10IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10IFG to value 0"]
impl crate::Resettable for ADC10IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
