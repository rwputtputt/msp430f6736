#[doc = "Register `ADC10HI` reader"]
pub struct R(crate::R<ADC10HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10HI` writer"]
pub struct W(crate::W<ADC10HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10HI_SPEC>;
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
impl From<crate::W<ADC10HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10HI_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Window Comparator High Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10hi](index.html) module"]
pub struct ADC10HI_SPEC;
impl crate::RegisterSpec for ADC10HI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10hi::R](R) reader structure"]
impl crate::Readable for ADC10HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10hi::W](W) writer structure"]
impl crate::Writable for ADC10HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10HI to value 0"]
impl crate::Resettable for ADC10HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
