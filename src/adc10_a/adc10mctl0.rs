#[doc = "Register `ADC10MCTL0` reader"]
pub struct R(crate::R<ADC10MCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10MCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10MCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10MCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10MCTL0` writer"]
pub struct W(crate::W<ADC10MCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10MCTL0_SPEC>;
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
impl From<crate::W<ADC10MCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10MCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC10 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10INCH_A {
    #[doc = "0: ADC10 Input Channel 0"]
    ADC10INCH_0 = 0,
    #[doc = "1: ADC10 Input Channel 1"]
    ADC10INCH_1 = 1,
    #[doc = "2: ADC10 Input Channel 2"]
    ADC10INCH_2 = 2,
    #[doc = "3: ADC10 Input Channel 3"]
    ADC10INCH_3 = 3,
    #[doc = "4: ADC10 Input Channel 4"]
    ADC10INCH_4 = 4,
    #[doc = "5: ADC10 Input Channel 5"]
    ADC10INCH_5 = 5,
    #[doc = "6: ADC10 Input Channel 6"]
    ADC10INCH_6 = 6,
    #[doc = "7: ADC10 Input Channel 7"]
    ADC10INCH_7 = 7,
    #[doc = "8: ADC10 Input Channel 8"]
    ADC10INCH_8 = 8,
    #[doc = "9: ADC10 Input Channel 9"]
    ADC10INCH_9 = 9,
    #[doc = "10: ADC10 Input Channel 10"]
    ADC10INCH_10 = 10,
    #[doc = "11: ADC10 Input Channel 11"]
    ADC10INCH_11 = 11,
    #[doc = "12: ADC10 Input Channel 12"]
    ADC10INCH_12 = 12,
    #[doc = "13: ADC10 Input Channel 13"]
    ADC10INCH_13 = 13,
    #[doc = "14: ADC10 Input Channel 14"]
    ADC10INCH_14 = 14,
    #[doc = "15: ADC10 Input Channel 15"]
    ADC10INCH_15 = 15,
}
impl From<ADC10INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10INCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10INCH` reader - ADC10 Input Channel Select Bit 0"]
pub type ADC10INCH_R = crate::FieldReader<u8, ADC10INCH_A>;
impl ADC10INCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10INCH_A {
        match self.bits {
            0 => ADC10INCH_A::ADC10INCH_0,
            1 => ADC10INCH_A::ADC10INCH_1,
            2 => ADC10INCH_A::ADC10INCH_2,
            3 => ADC10INCH_A::ADC10INCH_3,
            4 => ADC10INCH_A::ADC10INCH_4,
            5 => ADC10INCH_A::ADC10INCH_5,
            6 => ADC10INCH_A::ADC10INCH_6,
            7 => ADC10INCH_A::ADC10INCH_7,
            8 => ADC10INCH_A::ADC10INCH_8,
            9 => ADC10INCH_A::ADC10INCH_9,
            10 => ADC10INCH_A::ADC10INCH_10,
            11 => ADC10INCH_A::ADC10INCH_11,
            12 => ADC10INCH_A::ADC10INCH_12,
            13 => ADC10INCH_A::ADC10INCH_13,
            14 => ADC10INCH_A::ADC10INCH_14,
            15 => ADC10INCH_A::ADC10INCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_0`"]
    #[inline(always)]
    pub fn is_adc10inch_0(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_1`"]
    #[inline(always)]
    pub fn is_adc10inch_1(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_2`"]
    #[inline(always)]
    pub fn is_adc10inch_2(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_3`"]
    #[inline(always)]
    pub fn is_adc10inch_3(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_4`"]
    #[inline(always)]
    pub fn is_adc10inch_4(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_5`"]
    #[inline(always)]
    pub fn is_adc10inch_5(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_6`"]
    #[inline(always)]
    pub fn is_adc10inch_6(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_7`"]
    #[inline(always)]
    pub fn is_adc10inch_7(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_8`"]
    #[inline(always)]
    pub fn is_adc10inch_8(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_9`"]
    #[inline(always)]
    pub fn is_adc10inch_9(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_10`"]
    #[inline(always)]
    pub fn is_adc10inch_10(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_11`"]
    #[inline(always)]
    pub fn is_adc10inch_11(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_12`"]
    #[inline(always)]
    pub fn is_adc10inch_12(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_13`"]
    #[inline(always)]
    pub fn is_adc10inch_13(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_14`"]
    #[inline(always)]
    pub fn is_adc10inch_14(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC10INCH_15`"]
    #[inline(always)]
    pub fn is_adc10inch_15(&self) -> bool {
        *self == ADC10INCH_A::ADC10INCH_15
    }
}
#[doc = "Field `ADC10INCH` writer - ADC10 Input Channel Select Bit 0"]
pub type ADC10INCH_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10MCTL0_SPEC, u8, ADC10INCH_A, 4, 0>;
impl<'a> ADC10INCH_W<'a> {
    #[doc = "ADC10 Input Channel 0"]
    #[inline(always)]
    pub fn adc10inch_0(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_0)
    }
    #[doc = "ADC10 Input Channel 1"]
    #[inline(always)]
    pub fn adc10inch_1(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_1)
    }
    #[doc = "ADC10 Input Channel 2"]
    #[inline(always)]
    pub fn adc10inch_2(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_2)
    }
    #[doc = "ADC10 Input Channel 3"]
    #[inline(always)]
    pub fn adc10inch_3(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_3)
    }
    #[doc = "ADC10 Input Channel 4"]
    #[inline(always)]
    pub fn adc10inch_4(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_4)
    }
    #[doc = "ADC10 Input Channel 5"]
    #[inline(always)]
    pub fn adc10inch_5(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_5)
    }
    #[doc = "ADC10 Input Channel 6"]
    #[inline(always)]
    pub fn adc10inch_6(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_6)
    }
    #[doc = "ADC10 Input Channel 7"]
    #[inline(always)]
    pub fn adc10inch_7(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_7)
    }
    #[doc = "ADC10 Input Channel 8"]
    #[inline(always)]
    pub fn adc10inch_8(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_8)
    }
    #[doc = "ADC10 Input Channel 9"]
    #[inline(always)]
    pub fn adc10inch_9(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_9)
    }
    #[doc = "ADC10 Input Channel 10"]
    #[inline(always)]
    pub fn adc10inch_10(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_10)
    }
    #[doc = "ADC10 Input Channel 11"]
    #[inline(always)]
    pub fn adc10inch_11(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_11)
    }
    #[doc = "ADC10 Input Channel 12"]
    #[inline(always)]
    pub fn adc10inch_12(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_12)
    }
    #[doc = "ADC10 Input Channel 13"]
    #[inline(always)]
    pub fn adc10inch_13(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_13)
    }
    #[doc = "ADC10 Input Channel 14"]
    #[inline(always)]
    pub fn adc10inch_14(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_14)
    }
    #[doc = "ADC10 Input Channel 15"]
    #[inline(always)]
    pub fn adc10inch_15(self) -> &'a mut W {
        self.variant(ADC10INCH_A::ADC10INCH_15)
    }
}
#[doc = "ADC10 Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SREF_A {
    #[doc = "0: ADC10 Select Reference 0"]
    ADC10SREF_0 = 0,
    #[doc = "1: ADC10 Select Reference 1"]
    ADC10SREF_1 = 1,
    #[doc = "2: ADC10 Select Reference 2"]
    ADC10SREF_2 = 2,
    #[doc = "3: ADC10 Select Reference 3"]
    ADC10SREF_3 = 3,
    #[doc = "4: ADC10 Select Reference 4"]
    ADC10SREF_4 = 4,
    #[doc = "5: ADC10 Select Reference 5"]
    ADC10SREF_5 = 5,
    #[doc = "6: ADC10 Select Reference 6"]
    ADC10SREF_6 = 6,
    #[doc = "7: ADC10 Select Reference 7"]
    ADC10SREF_7 = 7,
}
impl From<ADC10SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10SREF` reader - ADC10 Select Reference Bit 0"]
pub type ADC10SREF_R = crate::FieldReader<u8, ADC10SREF_A>;
impl ADC10SREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SREF_A {
        match self.bits {
            0 => ADC10SREF_A::ADC10SREF_0,
            1 => ADC10SREF_A::ADC10SREF_1,
            2 => ADC10SREF_A::ADC10SREF_2,
            3 => ADC10SREF_A::ADC10SREF_3,
            4 => ADC10SREF_A::ADC10SREF_4,
            5 => ADC10SREF_A::ADC10SREF_5,
            6 => ADC10SREF_A::ADC10SREF_6,
            7 => ADC10SREF_A::ADC10SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_0`"]
    #[inline(always)]
    pub fn is_adc10sref_0(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_0
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_1`"]
    #[inline(always)]
    pub fn is_adc10sref_1(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_1
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_2`"]
    #[inline(always)]
    pub fn is_adc10sref_2(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_2
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_3`"]
    #[inline(always)]
    pub fn is_adc10sref_3(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_3
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_4`"]
    #[inline(always)]
    pub fn is_adc10sref_4(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_4
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_5`"]
    #[inline(always)]
    pub fn is_adc10sref_5(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_5
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_6`"]
    #[inline(always)]
    pub fn is_adc10sref_6(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_6
    }
    #[doc = "Checks if the value of the field is `ADC10SREF_7`"]
    #[inline(always)]
    pub fn is_adc10sref_7(&self) -> bool {
        *self == ADC10SREF_A::ADC10SREF_7
    }
}
#[doc = "Field `ADC10SREF` writer - ADC10 Select Reference Bit 0"]
pub type ADC10SREF_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10MCTL0_SPEC, u8, ADC10SREF_A, 3, 4>;
impl<'a> ADC10SREF_W<'a> {
    #[doc = "ADC10 Select Reference 0"]
    #[inline(always)]
    pub fn adc10sref_0(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_0)
    }
    #[doc = "ADC10 Select Reference 1"]
    #[inline(always)]
    pub fn adc10sref_1(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_1)
    }
    #[doc = "ADC10 Select Reference 2"]
    #[inline(always)]
    pub fn adc10sref_2(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_2)
    }
    #[doc = "ADC10 Select Reference 3"]
    #[inline(always)]
    pub fn adc10sref_3(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_3)
    }
    #[doc = "ADC10 Select Reference 4"]
    #[inline(always)]
    pub fn adc10sref_4(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_4)
    }
    #[doc = "ADC10 Select Reference 5"]
    #[inline(always)]
    pub fn adc10sref_5(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_5)
    }
    #[doc = "ADC10 Select Reference 6"]
    #[inline(always)]
    pub fn adc10sref_6(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_6)
    }
    #[doc = "ADC10 Select Reference 7"]
    #[inline(always)]
    pub fn adc10sref_7(self) -> &'a mut W {
        self.variant(ADC10SREF_A::ADC10SREF_7)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC10 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc10inch(&self) -> ADC10INCH_R {
        ADC10INCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ADC10 Select Reference Bit 0"]
    #[inline(always)]
    pub fn adc10sref(&self) -> ADC10SREF_R {
        ADC10SREF_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC10 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc10inch(&mut self) -> ADC10INCH_W {
        ADC10INCH_W::new(self)
    }
    #[doc = "Bits 4:6 - ADC10 Select Reference Bit 0"]
    #[inline(always)]
    pub fn adc10sref(&mut self) -> ADC10SREF_W {
        ADC10SREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Memory Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10mctl0](index.html) module"]
pub struct ADC10MCTL0_SPEC;
impl crate::RegisterSpec for ADC10MCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10mctl0::R](R) reader structure"]
impl crate::Readable for ADC10MCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10mctl0::W](W) writer structure"]
impl crate::Writable for ADC10MCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10MCTL0 to value 0"]
impl crate::Resettable for ADC10MCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
