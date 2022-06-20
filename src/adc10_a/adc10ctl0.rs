#[doc = "Register `ADC10CTL0` reader"]
pub struct R(crate::R<ADC10CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10CTL0` writer"]
pub struct W(crate::W<ADC10CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10CTL0_SPEC>;
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
impl From<crate::W<ADC10CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10SC` reader - ADC10 Start Conversion"]
pub type ADC10SC_R = crate::BitReader<bool>;
#[doc = "Field `ADC10SC` writer - ADC10 Start Conversion"]
pub type ADC10SC_W<'a> = crate::BitWriter<'a, u16, ADC10CTL0_SPEC, bool, 0>;
#[doc = "Field `ADC10ENC` reader - ADC10 Enable Conversion"]
pub type ADC10ENC_R = crate::BitReader<bool>;
#[doc = "Field `ADC10ENC` writer - ADC10 Enable Conversion"]
pub type ADC10ENC_W<'a> = crate::BitWriter<'a, u16, ADC10CTL0_SPEC, bool, 1>;
#[doc = "Field `ADC10ON` reader - ADC10 On/enable"]
pub type ADC10ON_R = crate::BitReader<bool>;
#[doc = "Field `ADC10ON` writer - ADC10 On/enable"]
pub type ADC10ON_W<'a> = crate::BitWriter<'a, u16, ADC10CTL0_SPEC, bool, 4>;
#[doc = "Field `ADC10MSC` reader - ADC10 Multiple SampleConversion"]
pub type ADC10MSC_R = crate::BitReader<bool>;
#[doc = "Field `ADC10MSC` writer - ADC10 Multiple SampleConversion"]
pub type ADC10MSC_W<'a> = crate::BitWriter<'a, u16, ADC10CTL0_SPEC, bool, 7>;
#[doc = "ADC10 Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SHT_A {
    #[doc = "0: ADC10 Sample Hold Select 0"]
    ADC10SHT_0 = 0,
    #[doc = "1: ADC10 Sample Hold Select 1"]
    ADC10SHT_1 = 1,
    #[doc = "2: ADC10 Sample Hold Select 2"]
    ADC10SHT_2 = 2,
    #[doc = "3: ADC10 Sample Hold Select 3"]
    ADC10SHT_3 = 3,
    #[doc = "4: ADC10 Sample Hold Select 4"]
    ADC10SHT_4 = 4,
    #[doc = "5: ADC10 Sample Hold Select 5"]
    ADC10SHT_5 = 5,
    #[doc = "6: ADC10 Sample Hold Select 6"]
    ADC10SHT_6 = 6,
    #[doc = "7: ADC10 Sample Hold Select 7"]
    ADC10SHT_7 = 7,
    #[doc = "8: ADC10 Sample Hold Select 8"]
    ADC10SHT_8 = 8,
    #[doc = "9: ADC10 Sample Hold Select 9"]
    ADC10SHT_9 = 9,
    #[doc = "10: ADC10 Sample Hold Select 10"]
    ADC10SHT_10 = 10,
    #[doc = "11: ADC10 Sample Hold Select 11"]
    ADC10SHT_11 = 11,
    #[doc = "12: ADC10 Sample Hold Select 12"]
    ADC10SHT_12 = 12,
    #[doc = "13: ADC10 Sample Hold Select 13"]
    ADC10SHT_13 = 13,
    #[doc = "14: ADC10 Sample Hold Select 14"]
    ADC10SHT_14 = 14,
    #[doc = "15: ADC10 Sample Hold Select 15"]
    ADC10SHT_15 = 15,
}
impl From<ADC10SHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SHT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10SHT` reader - ADC10 Sample Hold Select Bit: 0"]
pub type ADC10SHT_R = crate::FieldReader<u8, ADC10SHT_A>;
impl ADC10SHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SHT_A {
        match self.bits {
            0 => ADC10SHT_A::ADC10SHT_0,
            1 => ADC10SHT_A::ADC10SHT_1,
            2 => ADC10SHT_A::ADC10SHT_2,
            3 => ADC10SHT_A::ADC10SHT_3,
            4 => ADC10SHT_A::ADC10SHT_4,
            5 => ADC10SHT_A::ADC10SHT_5,
            6 => ADC10SHT_A::ADC10SHT_6,
            7 => ADC10SHT_A::ADC10SHT_7,
            8 => ADC10SHT_A::ADC10SHT_8,
            9 => ADC10SHT_A::ADC10SHT_9,
            10 => ADC10SHT_A::ADC10SHT_10,
            11 => ADC10SHT_A::ADC10SHT_11,
            12 => ADC10SHT_A::ADC10SHT_12,
            13 => ADC10SHT_A::ADC10SHT_13,
            14 => ADC10SHT_A::ADC10SHT_14,
            15 => ADC10SHT_A::ADC10SHT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_0`"]
    #[inline(always)]
    pub fn is_adc10sht_0(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_0
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_1`"]
    #[inline(always)]
    pub fn is_adc10sht_1(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_1
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_2`"]
    #[inline(always)]
    pub fn is_adc10sht_2(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_2
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_3`"]
    #[inline(always)]
    pub fn is_adc10sht_3(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_3
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_4`"]
    #[inline(always)]
    pub fn is_adc10sht_4(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_4
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_5`"]
    #[inline(always)]
    pub fn is_adc10sht_5(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_5
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_6`"]
    #[inline(always)]
    pub fn is_adc10sht_6(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_6
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_7`"]
    #[inline(always)]
    pub fn is_adc10sht_7(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_7
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_8`"]
    #[inline(always)]
    pub fn is_adc10sht_8(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_8
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_9`"]
    #[inline(always)]
    pub fn is_adc10sht_9(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_9
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_10`"]
    #[inline(always)]
    pub fn is_adc10sht_10(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_10
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_11`"]
    #[inline(always)]
    pub fn is_adc10sht_11(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_11
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_12`"]
    #[inline(always)]
    pub fn is_adc10sht_12(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_12
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_13`"]
    #[inline(always)]
    pub fn is_adc10sht_13(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_13
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_14`"]
    #[inline(always)]
    pub fn is_adc10sht_14(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_14
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_15`"]
    #[inline(always)]
    pub fn is_adc10sht_15(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_15
    }
}
#[doc = "Field `ADC10SHT` writer - ADC10 Sample Hold Select Bit: 0"]
pub type ADC10SHT_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10CTL0_SPEC, u8, ADC10SHT_A, 4, 8>;
impl<'a> ADC10SHT_W<'a> {
    #[doc = "ADC10 Sample Hold Select 0"]
    #[inline(always)]
    pub fn adc10sht_0(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_0)
    }
    #[doc = "ADC10 Sample Hold Select 1"]
    #[inline(always)]
    pub fn adc10sht_1(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_1)
    }
    #[doc = "ADC10 Sample Hold Select 2"]
    #[inline(always)]
    pub fn adc10sht_2(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_2)
    }
    #[doc = "ADC10 Sample Hold Select 3"]
    #[inline(always)]
    pub fn adc10sht_3(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_3)
    }
    #[doc = "ADC10 Sample Hold Select 4"]
    #[inline(always)]
    pub fn adc10sht_4(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_4)
    }
    #[doc = "ADC10 Sample Hold Select 5"]
    #[inline(always)]
    pub fn adc10sht_5(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_5)
    }
    #[doc = "ADC10 Sample Hold Select 6"]
    #[inline(always)]
    pub fn adc10sht_6(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_6)
    }
    #[doc = "ADC10 Sample Hold Select 7"]
    #[inline(always)]
    pub fn adc10sht_7(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_7)
    }
    #[doc = "ADC10 Sample Hold Select 8"]
    #[inline(always)]
    pub fn adc10sht_8(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_8)
    }
    #[doc = "ADC10 Sample Hold Select 9"]
    #[inline(always)]
    pub fn adc10sht_9(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_9)
    }
    #[doc = "ADC10 Sample Hold Select 10"]
    #[inline(always)]
    pub fn adc10sht_10(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_10)
    }
    #[doc = "ADC10 Sample Hold Select 11"]
    #[inline(always)]
    pub fn adc10sht_11(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_11)
    }
    #[doc = "ADC10 Sample Hold Select 12"]
    #[inline(always)]
    pub fn adc10sht_12(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_12)
    }
    #[doc = "ADC10 Sample Hold Select 13"]
    #[inline(always)]
    pub fn adc10sht_13(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_13)
    }
    #[doc = "ADC10 Sample Hold Select 14"]
    #[inline(always)]
    pub fn adc10sht_14(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_14)
    }
    #[doc = "ADC10 Sample Hold Select 15"]
    #[inline(always)]
    pub fn adc10sht_15(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&self) -> ADC10SC_R {
        ADC10SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn adc10enc(&self) -> ADC10ENC_R {
        ADC10ENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10 On/enable"]
    #[inline(always)]
    pub fn adc10on(&self) -> ADC10ON_R {
        ADC10ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc10msc(&self) -> ADC10MSC_R {
        ADC10MSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&self) -> ADC10SHT_R {
        ADC10SHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&mut self) -> ADC10SC_W {
        ADC10SC_W::new(self)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn adc10enc(&mut self) -> ADC10ENC_W {
        ADC10ENC_W::new(self)
    }
    #[doc = "Bit 4 - ADC10 On/enable"]
    #[inline(always)]
    pub fn adc10on(&mut self) -> ADC10ON_W {
        ADC10ON_W::new(self)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc10msc(&mut self) -> ADC10MSC_W {
        ADC10MSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&mut self) -> ADC10SHT_W {
        ADC10SHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl0](index.html) module"]
pub struct ADC10CTL0_SPEC;
impl crate::RegisterSpec for ADC10CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ctl0::R](R) reader structure"]
impl crate::Readable for ADC10CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ctl0::W](W) writer structure"]
impl crate::Writable for ADC10CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10CTL0 to value 0"]
impl crate::Resettable for ADC10CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
