#[doc = "Register `ADC10CTL1` reader"]
pub struct R(crate::R<ADC10CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10CTL1` writer"]
pub struct W(crate::W<ADC10CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10CTL1_SPEC>;
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
impl From<crate::W<ADC10CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10BUSY` reader - ADC10 Busy"]
pub type ADC10BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ADC10BUSY` writer - ADC10 Busy"]
pub type ADC10BUSY_W<'a> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, 0>;
#[doc = "ADC10 Conversion Sequence Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10CONSEQ_A {
    #[doc = "0: ADC10 Conversion Sequence Select: 0"]
    ADC10CONSEQ_0 = 0,
    #[doc = "1: ADC10 Conversion Sequence Select: 1"]
    ADC10CONSEQ_1 = 1,
    #[doc = "2: ADC10 Conversion Sequence Select: 2"]
    ADC10CONSEQ_2 = 2,
    #[doc = "3: ADC10 Conversion Sequence Select: 3"]
    ADC10CONSEQ_3 = 3,
}
impl From<ADC10CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10CONSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10CONSEQ` reader - ADC10 Conversion Sequence Select 0"]
pub type ADC10CONSEQ_R = crate::FieldReader<u8, ADC10CONSEQ_A>;
impl ADC10CONSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10CONSEQ_A {
        match self.bits {
            0 => ADC10CONSEQ_A::ADC10CONSEQ_0,
            1 => ADC10CONSEQ_A::ADC10CONSEQ_1,
            2 => ADC10CONSEQ_A::ADC10CONSEQ_2,
            3 => ADC10CONSEQ_A::ADC10CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10CONSEQ_0`"]
    #[inline(always)]
    pub fn is_adc10conseq_0(&self) -> bool {
        *self == ADC10CONSEQ_A::ADC10CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADC10CONSEQ_1`"]
    #[inline(always)]
    pub fn is_adc10conseq_1(&self) -> bool {
        *self == ADC10CONSEQ_A::ADC10CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADC10CONSEQ_2`"]
    #[inline(always)]
    pub fn is_adc10conseq_2(&self) -> bool {
        *self == ADC10CONSEQ_A::ADC10CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADC10CONSEQ_3`"]
    #[inline(always)]
    pub fn is_adc10conseq_3(&self) -> bool {
        *self == ADC10CONSEQ_A::ADC10CONSEQ_3
    }
}
#[doc = "Field `ADC10CONSEQ` writer - ADC10 Conversion Sequence Select 0"]
pub type ADC10CONSEQ_W<'a> =
    crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10CONSEQ_A, 2, 1>;
impl<'a> ADC10CONSEQ_W<'a> {
    #[doc = "ADC10 Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn adc10conseq_0(self) -> &'a mut W {
        self.variant(ADC10CONSEQ_A::ADC10CONSEQ_0)
    }
    #[doc = "ADC10 Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn adc10conseq_1(self) -> &'a mut W {
        self.variant(ADC10CONSEQ_A::ADC10CONSEQ_1)
    }
    #[doc = "ADC10 Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn adc10conseq_2(self) -> &'a mut W {
        self.variant(ADC10CONSEQ_A::ADC10CONSEQ_2)
    }
    #[doc = "ADC10 Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn adc10conseq_3(self) -> &'a mut W {
        self.variant(ADC10CONSEQ_A::ADC10CONSEQ_3)
    }
}
#[doc = "ADC10 Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SSEL_A {
    #[doc = "0: ADC10 Clock Source Select: 0"]
    ADC10SSEL_0 = 0,
    #[doc = "1: ADC10 Clock Source Select: 1"]
    ADC10SSEL_1 = 1,
    #[doc = "2: ADC10 Clock Source Select: 2"]
    ADC10SSEL_2 = 2,
    #[doc = "3: ADC10 Clock Source Select: 3"]
    ADC10SSEL_3 = 3,
}
impl From<ADC10SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10SSEL` reader - ADC10 Clock Source Select 0"]
pub type ADC10SSEL_R = crate::FieldReader<u8, ADC10SSEL_A>;
impl ADC10SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SSEL_A {
        match self.bits {
            0 => ADC10SSEL_A::ADC10SSEL_0,
            1 => ADC10SSEL_A::ADC10SSEL_1,
            2 => ADC10SSEL_A::ADC10SSEL_2,
            3 => ADC10SSEL_A::ADC10SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_0`"]
    #[inline(always)]
    pub fn is_adc10ssel_0(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_1`"]
    #[inline(always)]
    pub fn is_adc10ssel_1(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_2`"]
    #[inline(always)]
    pub fn is_adc10ssel_2(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC10SSEL_3`"]
    #[inline(always)]
    pub fn is_adc10ssel_3(&self) -> bool {
        *self == ADC10SSEL_A::ADC10SSEL_3
    }
}
#[doc = "Field `ADC10SSEL` writer - ADC10 Clock Source Select 0"]
pub type ADC10SSEL_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10SSEL_A, 2, 3>;
impl<'a> ADC10SSEL_W<'a> {
    #[doc = "ADC10 Clock Source Select: 0"]
    #[inline(always)]
    pub fn adc10ssel_0(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_0)
    }
    #[doc = "ADC10 Clock Source Select: 1"]
    #[inline(always)]
    pub fn adc10ssel_1(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_1)
    }
    #[doc = "ADC10 Clock Source Select: 2"]
    #[inline(always)]
    pub fn adc10ssel_2(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_2)
    }
    #[doc = "ADC10 Clock Source Select: 3"]
    #[inline(always)]
    pub fn adc10ssel_3(self) -> &'a mut W {
        self.variant(ADC10SSEL_A::ADC10SSEL_3)
    }
}
#[doc = "ADC10 Clock Divider Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10DIV_A {
    #[doc = "0: ADC10 Clock Divider Select: 0"]
    ADC10DIV_0 = 0,
    #[doc = "1: ADC10 Clock Divider Select: 1"]
    ADC10DIV_1 = 1,
    #[doc = "2: ADC10 Clock Divider Select: 2"]
    ADC10DIV_2 = 2,
    #[doc = "3: ADC10 Clock Divider Select: 3"]
    ADC10DIV_3 = 3,
    #[doc = "4: ADC10 Clock Divider Select: 4"]
    ADC10DIV_4 = 4,
    #[doc = "5: ADC10 Clock Divider Select: 5"]
    ADC10DIV_5 = 5,
    #[doc = "6: ADC10 Clock Divider Select: 6"]
    ADC10DIV_6 = 6,
    #[doc = "7: ADC10 Clock Divider Select: 7"]
    ADC10DIV_7 = 7,
}
impl From<ADC10DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10DIV` reader - ADC10 Clock Divider Select 0"]
pub type ADC10DIV_R = crate::FieldReader<u8, ADC10DIV_A>;
impl ADC10DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10DIV_A {
        match self.bits {
            0 => ADC10DIV_A::ADC10DIV_0,
            1 => ADC10DIV_A::ADC10DIV_1,
            2 => ADC10DIV_A::ADC10DIV_2,
            3 => ADC10DIV_A::ADC10DIV_3,
            4 => ADC10DIV_A::ADC10DIV_4,
            5 => ADC10DIV_A::ADC10DIV_5,
            6 => ADC10DIV_A::ADC10DIV_6,
            7 => ADC10DIV_A::ADC10DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_0`"]
    #[inline(always)]
    pub fn is_adc10div_0(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_1`"]
    #[inline(always)]
    pub fn is_adc10div_1(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_2`"]
    #[inline(always)]
    pub fn is_adc10div_2(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_3`"]
    #[inline(always)]
    pub fn is_adc10div_3(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_4`"]
    #[inline(always)]
    pub fn is_adc10div_4(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_5`"]
    #[inline(always)]
    pub fn is_adc10div_5(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_6`"]
    #[inline(always)]
    pub fn is_adc10div_6(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC10DIV_7`"]
    #[inline(always)]
    pub fn is_adc10div_7(&self) -> bool {
        *self == ADC10DIV_A::ADC10DIV_7
    }
}
#[doc = "Field `ADC10DIV` writer - ADC10 Clock Divider Select 0"]
pub type ADC10DIV_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10DIV_A, 3, 5>;
impl<'a> ADC10DIV_W<'a> {
    #[doc = "ADC10 Clock Divider Select: 0"]
    #[inline(always)]
    pub fn adc10div_0(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_0)
    }
    #[doc = "ADC10 Clock Divider Select: 1"]
    #[inline(always)]
    pub fn adc10div_1(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_1)
    }
    #[doc = "ADC10 Clock Divider Select: 2"]
    #[inline(always)]
    pub fn adc10div_2(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_2)
    }
    #[doc = "ADC10 Clock Divider Select: 3"]
    #[inline(always)]
    pub fn adc10div_3(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_3)
    }
    #[doc = "ADC10 Clock Divider Select: 4"]
    #[inline(always)]
    pub fn adc10div_4(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_4)
    }
    #[doc = "ADC10 Clock Divider Select: 5"]
    #[inline(always)]
    pub fn adc10div_5(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_5)
    }
    #[doc = "ADC10 Clock Divider Select: 6"]
    #[inline(always)]
    pub fn adc10div_6(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_6)
    }
    #[doc = "ADC10 Clock Divider Select: 7"]
    #[inline(always)]
    pub fn adc10div_7(self) -> &'a mut W {
        self.variant(ADC10DIV_A::ADC10DIV_7)
    }
}
#[doc = "Field `ADC10ISSH` reader - ADC10 Invert Sample Hold Signal"]
pub type ADC10ISSH_R = crate::BitReader<bool>;
#[doc = "Field `ADC10ISSH` writer - ADC10 Invert Sample Hold Signal"]
pub type ADC10ISSH_W<'a> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, 8>;
#[doc = "Field `ADC10SHP` reader - ADC10 Sample/Hold Pulse Mode"]
pub type ADC10SHP_R = crate::BitReader<bool>;
#[doc = "Field `ADC10SHP` writer - ADC10 Sample/Hold Pulse Mode"]
pub type ADC10SHP_W<'a> = crate::BitWriter<'a, u16, ADC10CTL1_SPEC, bool, 9>;
#[doc = "ADC10 Sample/Hold Source 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SHS_A {
    #[doc = "0: ADC10 Sample/Hold Source: 0"]
    ADC10SHS_0 = 0,
    #[doc = "1: ADC10 Sample/Hold Source: 1"]
    ADC10SHS_1 = 1,
    #[doc = "2: ADC10 Sample/Hold Source: 2"]
    ADC10SHS_2 = 2,
    #[doc = "3: ADC10 Sample/Hold Source: 3"]
    ADC10SHS_3 = 3,
}
impl From<ADC10SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SHS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC10SHS` reader - ADC10 Sample/Hold Source 0"]
pub type ADC10SHS_R = crate::FieldReader<u8, ADC10SHS_A>;
impl ADC10SHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SHS_A {
        match self.bits {
            0 => ADC10SHS_A::ADC10SHS_0,
            1 => ADC10SHS_A::ADC10SHS_1,
            2 => ADC10SHS_A::ADC10SHS_2,
            3 => ADC10SHS_A::ADC10SHS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SHS_0`"]
    #[inline(always)]
    pub fn is_adc10shs_0(&self) -> bool {
        *self == ADC10SHS_A::ADC10SHS_0
    }
    #[doc = "Checks if the value of the field is `ADC10SHS_1`"]
    #[inline(always)]
    pub fn is_adc10shs_1(&self) -> bool {
        *self == ADC10SHS_A::ADC10SHS_1
    }
    #[doc = "Checks if the value of the field is `ADC10SHS_2`"]
    #[inline(always)]
    pub fn is_adc10shs_2(&self) -> bool {
        *self == ADC10SHS_A::ADC10SHS_2
    }
    #[doc = "Checks if the value of the field is `ADC10SHS_3`"]
    #[inline(always)]
    pub fn is_adc10shs_3(&self) -> bool {
        *self == ADC10SHS_A::ADC10SHS_3
    }
}
#[doc = "Field `ADC10SHS` writer - ADC10 Sample/Hold Source 0"]
pub type ADC10SHS_W<'a> = crate::FieldWriterSafe<'a, u16, ADC10CTL1_SPEC, u8, ADC10SHS_A, 2, 10>;
impl<'a> ADC10SHS_W<'a> {
    #[doc = "ADC10 Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn adc10shs_0(self) -> &'a mut W {
        self.variant(ADC10SHS_A::ADC10SHS_0)
    }
    #[doc = "ADC10 Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn adc10shs_1(self) -> &'a mut W {
        self.variant(ADC10SHS_A::ADC10SHS_1)
    }
    #[doc = "ADC10 Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn adc10shs_2(self) -> &'a mut W {
        self.variant(ADC10SHS_A::ADC10SHS_2)
    }
    #[doc = "ADC10 Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn adc10shs_3(self) -> &'a mut W {
        self.variant(ADC10SHS_A::ADC10SHS_3)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Busy"]
    #[inline(always)]
    pub fn adc10busy(&self) -> ADC10BUSY_R {
        ADC10BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn adc10conseq(&self) -> ADC10CONSEQ_R {
        ADC10CONSEQ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select 0"]
    #[inline(always)]
    pub fn adc10ssel(&self) -> ADC10SSEL_R {
        ADC10SSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc10div(&self) -> ADC10DIV_R {
        ADC10DIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adc10issh(&self) -> ADC10ISSH_R {
        ADC10ISSH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC10 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adc10shp(&self) -> ADC10SHP_R {
        ADC10SHP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source 0"]
    #[inline(always)]
    pub fn adc10shs(&self) -> ADC10SHS_R {
        ADC10SHS_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Busy"]
    #[inline(always)]
    pub fn adc10busy(&mut self) -> ADC10BUSY_W {
        ADC10BUSY_W::new(self)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn adc10conseq(&mut self) -> ADC10CONSEQ_W {
        ADC10CONSEQ_W::new(self)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select 0"]
    #[inline(always)]
    pub fn adc10ssel(&mut self) -> ADC10SSEL_W {
        ADC10SSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc10div(&mut self) -> ADC10DIV_W {
        ADC10DIV_W::new(self)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adc10issh(&mut self) -> ADC10ISSH_W {
        ADC10ISSH_W::new(self)
    }
    #[doc = "Bit 9 - ADC10 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adc10shp(&mut self) -> ADC10SHP_W {
        ADC10SHP_W::new(self)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source 0"]
    #[inline(always)]
    pub fn adc10shs(&mut self) -> ADC10SHS_W {
        ADC10SHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10ctl1](index.html) module"]
pub struct ADC10CTL1_SPEC;
impl crate::RegisterSpec for ADC10CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10ctl1::R](R) reader structure"]
impl crate::Readable for ADC10CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10ctl1::W](W) writer structure"]
impl crate::Writable for ADC10CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10CTL1 to value 0"]
impl crate::Resettable for ADC10CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
