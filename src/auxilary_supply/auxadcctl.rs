#[doc = "Register `AUXADCCTL` reader"]
pub struct R(crate::R<AUXADCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXADCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXADCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXADCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXADCCTL` writer"]
pub struct W(crate::W<AUXADCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXADCCTL_SPEC>;
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
impl From<crate::W<AUXADCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXADCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXADC` reader - Auxiliary supplies to ADC"]
pub type AUXADC_R = crate::BitReader<bool>;
#[doc = "Field `AUXADC` writer - Auxiliary supplies to ADC"]
pub type AUXADC_W<'a> = crate::BitWriter<'a, u16, AUXADCCTL_SPEC, bool, 0>;
#[doc = "Select supply to be measured with ADC Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXADCSEL_A {
    #[doc = "0: Select supply to be measured with ADC: DVCC"]
    AUXADCSEL_0 = 0,
    #[doc = "1: Select supply to be measured with ADC: AUXVCC1"]
    AUXADCSEL_1 = 1,
    #[doc = "2: Select supply to be measured with ADC: AUXVCC2"]
    AUXADCSEL_2 = 2,
    #[doc = "3: Select supply to be measured with ADC: AUXVCC3"]
    AUXADCSEL_3 = 3,
}
impl From<AUXADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXADCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXADCSEL` reader - Select supply to be measured with ADC Bit: 0"]
pub type AUXADCSEL_R = crate::FieldReader<u8, AUXADCSEL_A>;
impl AUXADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXADCSEL_A {
        match self.bits {
            0 => AUXADCSEL_A::AUXADCSEL_0,
            1 => AUXADCSEL_A::AUXADCSEL_1,
            2 => AUXADCSEL_A::AUXADCSEL_2,
            3 => AUXADCSEL_A::AUXADCSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUXADCSEL_0`"]
    #[inline(always)]
    pub fn is_auxadcsel_0(&self) -> bool {
        *self == AUXADCSEL_A::AUXADCSEL_0
    }
    #[doc = "Checks if the value of the field is `AUXADCSEL_1`"]
    #[inline(always)]
    pub fn is_auxadcsel_1(&self) -> bool {
        *self == AUXADCSEL_A::AUXADCSEL_1
    }
    #[doc = "Checks if the value of the field is `AUXADCSEL_2`"]
    #[inline(always)]
    pub fn is_auxadcsel_2(&self) -> bool {
        *self == AUXADCSEL_A::AUXADCSEL_2
    }
    #[doc = "Checks if the value of the field is `AUXADCSEL_3`"]
    #[inline(always)]
    pub fn is_auxadcsel_3(&self) -> bool {
        *self == AUXADCSEL_A::AUXADCSEL_3
    }
}
#[doc = "Field `AUXADCSEL` writer - Select supply to be measured with ADC Bit: 0"]
pub type AUXADCSEL_W<'a> = crate::FieldWriterSafe<'a, u16, AUXADCCTL_SPEC, u8, AUXADCSEL_A, 2, 1>;
impl<'a> AUXADCSEL_W<'a> {
    #[doc = "Select supply to be measured with ADC: DVCC"]
    #[inline(always)]
    pub fn auxadcsel_0(self) -> &'a mut W {
        self.variant(AUXADCSEL_A::AUXADCSEL_0)
    }
    #[doc = "Select supply to be measured with ADC: AUXVCC1"]
    #[inline(always)]
    pub fn auxadcsel_1(self) -> &'a mut W {
        self.variant(AUXADCSEL_A::AUXADCSEL_1)
    }
    #[doc = "Select supply to be measured with ADC: AUXVCC2"]
    #[inline(always)]
    pub fn auxadcsel_2(self) -> &'a mut W {
        self.variant(AUXADCSEL_A::AUXADCSEL_2)
    }
    #[doc = "Select supply to be measured with ADC: AUXVCC3"]
    #[inline(always)]
    pub fn auxadcsel_3(self) -> &'a mut W {
        self.variant(AUXADCSEL_A::AUXADCSEL_3)
    }
}
#[doc = "Load resistance Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXADCR_A {
    #[doc = "0: Load resistance: 0"]
    AUXADCR_0 = 0,
    #[doc = "1: Load resistance: 1"]
    AUXADCR_1 = 1,
    #[doc = "2: Load resistance: 2"]
    AUXADCR_2 = 2,
    #[doc = "3: Load resistance: 3"]
    AUXADCR_3 = 3,
}
impl From<AUXADCR_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXADCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXADCR` reader - Load resistance Bit: 0"]
pub type AUXADCR_R = crate::FieldReader<u8, AUXADCR_A>;
impl AUXADCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXADCR_A {
        match self.bits {
            0 => AUXADCR_A::AUXADCR_0,
            1 => AUXADCR_A::AUXADCR_1,
            2 => AUXADCR_A::AUXADCR_2,
            3 => AUXADCR_A::AUXADCR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUXADCR_0`"]
    #[inline(always)]
    pub fn is_auxadcr_0(&self) -> bool {
        *self == AUXADCR_A::AUXADCR_0
    }
    #[doc = "Checks if the value of the field is `AUXADCR_1`"]
    #[inline(always)]
    pub fn is_auxadcr_1(&self) -> bool {
        *self == AUXADCR_A::AUXADCR_1
    }
    #[doc = "Checks if the value of the field is `AUXADCR_2`"]
    #[inline(always)]
    pub fn is_auxadcr_2(&self) -> bool {
        *self == AUXADCR_A::AUXADCR_2
    }
    #[doc = "Checks if the value of the field is `AUXADCR_3`"]
    #[inline(always)]
    pub fn is_auxadcr_3(&self) -> bool {
        *self == AUXADCR_A::AUXADCR_3
    }
}
#[doc = "Field `AUXADCR` writer - Load resistance Bit: 0"]
pub type AUXADCR_W<'a> = crate::FieldWriterSafe<'a, u16, AUXADCCTL_SPEC, u8, AUXADCR_A, 2, 4>;
impl<'a> AUXADCR_W<'a> {
    #[doc = "Load resistance: 0"]
    #[inline(always)]
    pub fn auxadcr_0(self) -> &'a mut W {
        self.variant(AUXADCR_A::AUXADCR_0)
    }
    #[doc = "Load resistance: 1"]
    #[inline(always)]
    pub fn auxadcr_1(self) -> &'a mut W {
        self.variant(AUXADCR_A::AUXADCR_1)
    }
    #[doc = "Load resistance: 2"]
    #[inline(always)]
    pub fn auxadcr_2(self) -> &'a mut W {
        self.variant(AUXADCR_A::AUXADCR_2)
    }
    #[doc = "Load resistance: 3"]
    #[inline(always)]
    pub fn auxadcr_3(self) -> &'a mut W {
        self.variant(AUXADCR_A::AUXADCR_3)
    }
}
impl R {
    #[doc = "Bit 0 - Auxiliary supplies to ADC"]
    #[inline(always)]
    pub fn auxadc(&self) -> AUXADC_R {
        AUXADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select supply to be measured with ADC Bit: 0"]
    #[inline(always)]
    pub fn auxadcsel(&self) -> AUXADCSEL_R {
        AUXADCSEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Load resistance Bit: 0"]
    #[inline(always)]
    pub fn auxadcr(&self) -> AUXADCR_R {
        AUXADCR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Auxiliary supplies to ADC"]
    #[inline(always)]
    pub fn auxadc(&mut self) -> AUXADC_W {
        AUXADC_W::new(self)
    }
    #[doc = "Bits 1:2 - Select supply to be measured with ADC Bit: 0"]
    #[inline(always)]
    pub fn auxadcsel(&mut self) -> AUXADCSEL_W {
        AUXADCSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Load resistance Bit: 0"]
    #[inline(always)]
    pub fn auxadcr(&mut self) -> AUXADCR_W {
        AUXADCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxadcctl](index.html) module"]
pub struct AUXADCCTL_SPEC;
impl crate::RegisterSpec for AUXADCCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxadcctl::R](R) reader structure"]
impl crate::Readable for AUXADCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxadcctl::W](W) writer structure"]
impl crate::Writable for AUXADCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXADCCTL to value 0"]
impl crate::Resettable for AUXADCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
