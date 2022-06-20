#[doc = "Register `ADC10CTL2` reader"]
pub struct R(crate::R<ADC10CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10CTL2` writer"]
pub struct W(crate::W<ADC10CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10CTL2_SPEC>;
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
impl From<crate::W<ADC10CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10REFBURST` reader - ADC10 Reference Burst"]
pub type ADC10REFBURST_R = crate::BitReader<bool>;
#[doc = "Field `ADC10REFBURST` writer - ADC10 Reference Burst"]
pub type ADC10REFBURST_W<'a> = crate::BitWriter<'a, u16, ADC10CTL2_SPEC, bool, 0>;
#[doc = "Field `ADC10SR` reader - ADC10 Sampling Rate"]
pub type ADC10SR_R = crate::BitReader<bool>;
#[doc = "Field `ADC10SR` writer - ADC10 Sampling Rate"]
pub type ADC10SR_W<'a> = crate::BitWriter<'a, u16, ADC10CTL2_SPEC, bool, 2>;
#[doc = "Field `ADC10DF` reader - ADC10 Data Format"]
pub type ADC10DF_R = crate::BitReader<bool>;
#[doc = "Field `ADC10DF` writer - ADC10 Data Format"]
pub type ADC10DF_W<'a> = crate::BitWriter<'a, u16, ADC10CTL2_SPEC, bool, 3>;
#[doc = "Field `ADC10RES` reader - ADC10 Resolution Bit"]
pub type ADC10RES_R = crate::BitReader<bool>;
#[doc = "Field `ADC10RES` writer - ADC10 Resolution Bit"]
pub type ADC10RES_W<'a> = crate::BitWriter<'a, u16, ADC10CTL2_SPEC, bool, 4>;
#[doc = "ADC10 predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10PDIV_A {
    #[doc = "0: ADC10 predivider /1"]
    ADC10PDIV_0 = 0,
    #[doc = "1: ADC10 predivider /2"]
    ADC10PDIV_1 = 1,
    #[doc = "2: ADC10 predivider /64"]
    ADC10PDIV_2 = 2,
    #[doc = "3: ADC10 predivider reserved"]
    ADC10PDIV_3 = 3,
}
impl From<ADC10PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10PDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10PDIV` reader - ADC10 predivider Bit: 0"]
pub type ADC10PDIV_R = crate::FieldReader<u8, ADC10PDIV_A>;
impl ADC10PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10PDIV_A {
        match self.bits {
            0 => ADC10PDIV_A::ADC10PDIV_0,
            1 => ADC10PDIV_A::ADC10PDIV_1,
            2 => ADC10PDIV_A::ADC10PDIV_2,
            3 => ADC10PDIV_A::ADC10PDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10PDIV_0`"]
    #[inline(always)]
    pub fn is_adc10pdiv_0(&self) -> bool {
        *self == ADC10PDIV_A::ADC10PDIV_0
    }
    #[doc = "Checks if the value of the field is `ADC10PDIV_1`"]
    #[inline(always)]
    pub fn is_adc10pdiv_1(&self) -> bool {
        *self == ADC10PDIV_A::ADC10PDIV_1
    }
    #[doc = "Checks if the value of the field is `ADC10PDIV_2`"]
    #[inline(always)]
    pub fn is_adc10pdiv_2(&self) -> bool {
        *self == ADC10PDIV_A::ADC10PDIV_2
    }
    #[doc = "Checks if the value of the field is `ADC10PDIV_3`"]
    #[inline(always)]
    pub fn is_adc10pdiv_3(&self) -> bool {
        *self == ADC10PDIV_A::ADC10PDIV_3
    }
}
#[doc = "Field `ADC10PDIV` writer - ADC10 predivider Bit: 0"]
pub type ADC10PDIV_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10CTL2_SPEC, u8, ADC10PDIV_A, 2, 8>;
impl<'a> ADC10PDIV_W<'a> {
    #[doc = "ADC10 predivider /1"]
    #[inline(always)]
    pub fn adc10pdiv_0(self) -> &'a mut W {
        self.variant(ADC10PDIV_A::ADC10PDIV_0)
    }
    #[doc = "ADC10 predivider /2"]
    #[inline(always)]
    pub fn adc10pdiv_1(self) -> &'a mut W {
        self.variant(ADC10PDIV_A::ADC10PDIV_1)
    }
    #[doc = "ADC10 predivider /64"]
    #[inline(always)]
    pub fn adc10pdiv_2(self) -> &'a mut W {
        self.variant(ADC10PDIV_A::ADC10PDIV_2)
    }
    #[doc = "ADC10 predivider reserved"]
    #[inline(always)]
    pub fn adc10pdiv_3(self) -> &'a mut W {
        self.variant(ADC10PDIV_A::ADC10PDIV_3)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Reference Burst"]
    #[inline(always)]
    pub fn adc10refburst(&self) -> ADC10REFBURST_R {
        ADC10REFBURST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 Sampling Rate"]
    #[inline(always)]
    pub fn adc10sr(&self) -> ADC10SR_R {
        ADC10SR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 Data Format"]
    #[inline(always)]
    pub fn adc10df(&self) -> ADC10DF_R {
        ADC10DF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10 Resolution Bit"]
    #[inline(always)]
    pub fn adc10res(&self) -> ADC10RES_R {
        ADC10RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ADC10 predivider Bit: 0"]
    #[inline(always)]
    pub fn adc10pdiv(&self) -> ADC10PDIV_R {
        ADC10PDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Reference Burst"]
    #[inline(always)]
    pub fn adc10refburst(&mut self) -> ADC10REFBURST_W {
        ADC10REFBURST_W::new(self)
    }
    #[doc = "Bit 2 - ADC10 Sampling Rate"]
    #[inline(always)]
    pub fn adc10sr(&mut self) -> ADC10SR_W {
        ADC10SR_W::new(self)
    }
    #[doc = "Bit 3 - ADC10 Data Format"]
    #[inline(always)]
    pub fn adc10df(&mut self) -> ADC10DF_W {
        ADC10DF_W::new(self)
    }
    #[doc = "Bit 4 - ADC10 Resolution Bit"]
    #[inline(always)]
    pub fn adc10res(&mut self) -> ADC10RES_W {
        ADC10RES_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC10 predivider Bit: 0"]
    #[inline(always)]
    pub fn adc10pdiv(&mut self) -> ADC10PDIV_W {
        ADC10PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl2](index.html) module"]
pub struct ADC10CTL2_SPEC;
impl crate::RegisterSpec for ADC10CTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ctl2::R](R) reader structure"]
impl crate::Readable for ADC10CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ctl2::W](W) writer structure"]
impl crate::Writable for ADC10CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10CTL2 to value 0"]
impl crate::Resettable for ADC10CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
