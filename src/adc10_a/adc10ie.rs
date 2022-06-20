#[doc = "Register `ADC10IE` reader"]
pub struct R(crate::R<ADC10IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10IE` writer"]
pub struct W(crate::W<ADC10IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10IE_SPEC>;
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
impl From<crate::W<ADC10IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10IE0` reader - ADC10_A Interrupt enable"]
pub type ADC10IE0_R = crate::BitReader<bool>;
#[doc = "Field `ADC10IE0` writer - ADC10_A Interrupt enable"]
pub type ADC10IE0_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 0>;
#[doc = "Field `ADC10INIE` reader - ADC10_A Interrupt enable for the inside of window of the Window comparator"]
pub type ADC10INIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC10INIE` writer - ADC10_A Interrupt enable for the inside of window of the Window comparator"]
pub type ADC10INIE_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 1>;
#[doc = "Field `ADC10LOIE` reader - ADC10_A Interrupt enable for lower threshold of the Window comparator"]
pub type ADC10LOIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC10LOIE` writer - ADC10_A Interrupt enable for lower threshold of the Window comparator"]
pub type ADC10LOIE_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 2>;
#[doc = "Field `ADC10HIIE` reader - ADC10_A Interrupt enable for upper threshold of the Window comparator"]
pub type ADC10HIIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC10HIIE` writer - ADC10_A Interrupt enable for upper threshold of the Window comparator"]
pub type ADC10HIIE_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 3>;
#[doc = "Field `ADC10OVIE` reader - ADC10_A ADC10MEM overflow Interrupt enable"]
pub type ADC10OVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC10OVIE` writer - ADC10_A ADC10MEM overflow Interrupt enable"]
pub type ADC10OVIE_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 4>;
#[doc = "Field `ADC10TOVIE` reader - ADC10_A conversion-time-overflow Interrupt enable"]
pub type ADC10TOVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC10TOVIE` writer - ADC10_A conversion-time-overflow Interrupt enable"]
pub type ADC10TOVIE_W<'a> = crate::BitWriter<'a, u16, ADC10IE_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - ADC10_A Interrupt enable"]
    #[inline(always)]
    pub fn adc10ie0(&self) -> ADC10IE0_R {
        ADC10IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10_A Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc10inie(&self) -> ADC10INIE_R {
        ADC10INIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10_A Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10loie(&self) -> ADC10LOIE_R {
        ADC10LOIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10_A Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10hiie(&self) -> ADC10HIIE_R {
        ADC10HIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10_A ADC10MEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adc10ovie(&self) -> ADC10OVIE_R {
        ADC10OVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC10_A conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adc10tovie(&self) -> ADC10TOVIE_R {
        ADC10TOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10_A Interrupt enable"]
    #[inline(always)]
    pub fn adc10ie0(&mut self) -> ADC10IE0_W {
        ADC10IE0_W::new(self)
    }
    #[doc = "Bit 1 - ADC10_A Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc10inie(&mut self) -> ADC10INIE_W {
        ADC10INIE_W::new(self)
    }
    #[doc = "Bit 2 - ADC10_A Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10loie(&mut self) -> ADC10LOIE_W {
        ADC10LOIE_W::new(self)
    }
    #[doc = "Bit 3 - ADC10_A Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc10hiie(&mut self) -> ADC10HIIE_W {
        ADC10HIIE_W::new(self)
    }
    #[doc = "Bit 4 - ADC10_A ADC10MEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adc10ovie(&mut self) -> ADC10OVIE_W {
        ADC10OVIE_W::new(self)
    }
    #[doc = "Bit 5 - ADC10_A conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adc10tovie(&mut self) -> ADC10TOVIE_W {
        ADC10TOVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ie](index.html) module"]
pub struct ADC10IE_SPEC;
impl crate::RegisterSpec for ADC10IE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ie::R](R) reader structure"]
impl crate::Readable for ADC10IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ie::W](W) writer structure"]
impl crate::Writable for ADC10IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10IE to value 0"]
impl crate::Resettable for ADC10IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
