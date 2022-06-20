#[doc = "Register `AUXCTL2` reader"]
pub struct R(crate::R<AUXCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCTL2` writer"]
pub struct W(crate::W<AUXCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCTL2_SPEC>;
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
impl From<crate::W<AUXCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DVCC supply threshold level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUX0LVL_A {
    #[doc = "0: DVCC supply threshold level: 0"]
    AUX0LVL_0 = 0,
    #[doc = "1: DVCC supply threshold level: 1"]
    AUX0LVL_1 = 1,
    #[doc = "2: DVCC supply threshold level: 2"]
    AUX0LVL_2 = 2,
    #[doc = "3: DVCC supply threshold level: 3"]
    AUX0LVL_3 = 3,
    #[doc = "4: DVCC supply threshold level: 4"]
    AUX0LVL_4 = 4,
    #[doc = "5: DVCC supply threshold level: 5"]
    AUX0LVL_5 = 5,
    #[doc = "6: DVCC supply threshold level: 6"]
    AUX0LVL_6 = 6,
    #[doc = "7: DVCC supply threshold level: 7"]
    AUX0LVL_7 = 7,
}
impl From<AUX0LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: AUX0LVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUX0LVL` reader - DVCC supply threshold level Bit: 0"]
pub type AUX0LVL_R = crate::FieldReader<u8, AUX0LVL_A>;
impl AUX0LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX0LVL_A {
        match self.bits {
            0 => AUX0LVL_A::AUX0LVL_0,
            1 => AUX0LVL_A::AUX0LVL_1,
            2 => AUX0LVL_A::AUX0LVL_2,
            3 => AUX0LVL_A::AUX0LVL_3,
            4 => AUX0LVL_A::AUX0LVL_4,
            5 => AUX0LVL_A::AUX0LVL_5,
            6 => AUX0LVL_A::AUX0LVL_6,
            7 => AUX0LVL_A::AUX0LVL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_0`"]
    #[inline(always)]
    pub fn is_aux0lvl_0(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_0
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_1`"]
    #[inline(always)]
    pub fn is_aux0lvl_1(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_1
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_2`"]
    #[inline(always)]
    pub fn is_aux0lvl_2(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_2
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_3`"]
    #[inline(always)]
    pub fn is_aux0lvl_3(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_3
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_4`"]
    #[inline(always)]
    pub fn is_aux0lvl_4(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_4
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_5`"]
    #[inline(always)]
    pub fn is_aux0lvl_5(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_5
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_6`"]
    #[inline(always)]
    pub fn is_aux0lvl_6(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_6
    }
    #[doc = "Checks if the value of the field is `AUX0LVL_7`"]
    #[inline(always)]
    pub fn is_aux0lvl_7(&self) -> bool {
        *self == AUX0LVL_A::AUX0LVL_7
    }
}
#[doc = "Field `AUX0LVL` writer - DVCC supply threshold level Bit: 0"]
pub type AUX0LVL_W<'a> = crate::FieldWriterSafe<'a, u16, AUXCTL2_SPEC, u8, AUX0LVL_A, 3, 0>;
impl<'a> AUX0LVL_W<'a> {
    #[doc = "DVCC supply threshold level: 0"]
    #[inline(always)]
    pub fn aux0lvl_0(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_0)
    }
    #[doc = "DVCC supply threshold level: 1"]
    #[inline(always)]
    pub fn aux0lvl_1(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_1)
    }
    #[doc = "DVCC supply threshold level: 2"]
    #[inline(always)]
    pub fn aux0lvl_2(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_2)
    }
    #[doc = "DVCC supply threshold level: 3"]
    #[inline(always)]
    pub fn aux0lvl_3(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_3)
    }
    #[doc = "DVCC supply threshold level: 4"]
    #[inline(always)]
    pub fn aux0lvl_4(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_4)
    }
    #[doc = "DVCC supply threshold level: 5"]
    #[inline(always)]
    pub fn aux0lvl_5(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_5)
    }
    #[doc = "DVCC supply threshold level: 6"]
    #[inline(always)]
    pub fn aux0lvl_6(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_6)
    }
    #[doc = "DVCC supply threshold level: 7"]
    #[inline(always)]
    pub fn aux0lvl_7(self) -> &'a mut W {
        self.variant(AUX0LVL_A::AUX0LVL_7)
    }
}
#[doc = "AUX1 supply threshold level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUX1LVL_A {
    #[doc = "0: AUX1 supply threshold level: 0"]
    AUX1LVL_0 = 0,
    #[doc = "1: AUX1 supply threshold level: 1"]
    AUX1LVL_1 = 1,
    #[doc = "2: AUX1 supply threshold level: 2"]
    AUX1LVL_2 = 2,
    #[doc = "3: AUX1 supply threshold level: 3"]
    AUX1LVL_3 = 3,
    #[doc = "4: AUX1 supply threshold level: 4"]
    AUX1LVL_4 = 4,
    #[doc = "5: AUX1 supply threshold level: 5"]
    AUX1LVL_5 = 5,
    #[doc = "6: AUX1 supply threshold level: 6"]
    AUX1LVL_6 = 6,
    #[doc = "7: AUX1 supply threshold level: 7"]
    AUX1LVL_7 = 7,
}
impl From<AUX1LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: AUX1LVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUX1LVL` reader - AUX1 supply threshold level Bit: 0"]
pub type AUX1LVL_R = crate::FieldReader<u8, AUX1LVL_A>;
impl AUX1LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX1LVL_A {
        match self.bits {
            0 => AUX1LVL_A::AUX1LVL_0,
            1 => AUX1LVL_A::AUX1LVL_1,
            2 => AUX1LVL_A::AUX1LVL_2,
            3 => AUX1LVL_A::AUX1LVL_3,
            4 => AUX1LVL_A::AUX1LVL_4,
            5 => AUX1LVL_A::AUX1LVL_5,
            6 => AUX1LVL_A::AUX1LVL_6,
            7 => AUX1LVL_A::AUX1LVL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_0`"]
    #[inline(always)]
    pub fn is_aux1lvl_0(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_0
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_1`"]
    #[inline(always)]
    pub fn is_aux1lvl_1(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_1
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_2`"]
    #[inline(always)]
    pub fn is_aux1lvl_2(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_2
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_3`"]
    #[inline(always)]
    pub fn is_aux1lvl_3(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_3
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_4`"]
    #[inline(always)]
    pub fn is_aux1lvl_4(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_4
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_5`"]
    #[inline(always)]
    pub fn is_aux1lvl_5(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_5
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_6`"]
    #[inline(always)]
    pub fn is_aux1lvl_6(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_6
    }
    #[doc = "Checks if the value of the field is `AUX1LVL_7`"]
    #[inline(always)]
    pub fn is_aux1lvl_7(&self) -> bool {
        *self == AUX1LVL_A::AUX1LVL_7
    }
}
#[doc = "Field `AUX1LVL` writer - AUX1 supply threshold level Bit: 0"]
pub type AUX1LVL_W<'a> = crate::FieldWriterSafe<'a, u16, AUXCTL2_SPEC, u8, AUX1LVL_A, 3, 4>;
impl<'a> AUX1LVL_W<'a> {
    #[doc = "AUX1 supply threshold level: 0"]
    #[inline(always)]
    pub fn aux1lvl_0(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_0)
    }
    #[doc = "AUX1 supply threshold level: 1"]
    #[inline(always)]
    pub fn aux1lvl_1(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_1)
    }
    #[doc = "AUX1 supply threshold level: 2"]
    #[inline(always)]
    pub fn aux1lvl_2(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_2)
    }
    #[doc = "AUX1 supply threshold level: 3"]
    #[inline(always)]
    pub fn aux1lvl_3(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_3)
    }
    #[doc = "AUX1 supply threshold level: 4"]
    #[inline(always)]
    pub fn aux1lvl_4(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_4)
    }
    #[doc = "AUX1 supply threshold level: 5"]
    #[inline(always)]
    pub fn aux1lvl_5(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_5)
    }
    #[doc = "AUX1 supply threshold level: 6"]
    #[inline(always)]
    pub fn aux1lvl_6(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_6)
    }
    #[doc = "AUX1 supply threshold level: 7"]
    #[inline(always)]
    pub fn aux1lvl_7(self) -> &'a mut W {
        self.variant(AUX1LVL_A::AUX1LVL_7)
    }
}
#[doc = "AUX2 supply threshold level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUX2LVL_A {
    #[doc = "0: AUX2 supply threshold level: 0"]
    AUX2LVL_0 = 0,
    #[doc = "1: AUX2 supply threshold level: 1"]
    AUX2LVL_1 = 1,
    #[doc = "2: AUX2 supply threshold level: 2"]
    AUX2LVL_2 = 2,
    #[doc = "3: AUX2 supply threshold level: 3"]
    AUX2LVL_3 = 3,
    #[doc = "4: AUX2 supply threshold level: 4"]
    AUX2LVL_4 = 4,
    #[doc = "5: AUX2 supply threshold level: 5"]
    AUX2LVL_5 = 5,
    #[doc = "6: AUX2 supply threshold level: 6"]
    AUX2LVL_6 = 6,
    #[doc = "7: AUX2 supply threshold level: 7"]
    AUX2LVL_7 = 7,
}
impl From<AUX2LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: AUX2LVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUX2LVL` reader - AUX2 supply threshold level Bit: 0"]
pub type AUX2LVL_R = crate::FieldReader<u8, AUX2LVL_A>;
impl AUX2LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX2LVL_A {
        match self.bits {
            0 => AUX2LVL_A::AUX2LVL_0,
            1 => AUX2LVL_A::AUX2LVL_1,
            2 => AUX2LVL_A::AUX2LVL_2,
            3 => AUX2LVL_A::AUX2LVL_3,
            4 => AUX2LVL_A::AUX2LVL_4,
            5 => AUX2LVL_A::AUX2LVL_5,
            6 => AUX2LVL_A::AUX2LVL_6,
            7 => AUX2LVL_A::AUX2LVL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_0`"]
    #[inline(always)]
    pub fn is_aux2lvl_0(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_0
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_1`"]
    #[inline(always)]
    pub fn is_aux2lvl_1(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_1
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_2`"]
    #[inline(always)]
    pub fn is_aux2lvl_2(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_2
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_3`"]
    #[inline(always)]
    pub fn is_aux2lvl_3(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_3
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_4`"]
    #[inline(always)]
    pub fn is_aux2lvl_4(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_4
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_5`"]
    #[inline(always)]
    pub fn is_aux2lvl_5(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_5
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_6`"]
    #[inline(always)]
    pub fn is_aux2lvl_6(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_6
    }
    #[doc = "Checks if the value of the field is `AUX2LVL_7`"]
    #[inline(always)]
    pub fn is_aux2lvl_7(&self) -> bool {
        *self == AUX2LVL_A::AUX2LVL_7
    }
}
#[doc = "Field `AUX2LVL` writer - AUX2 supply threshold level Bit: 0"]
pub type AUX2LVL_W<'a> = crate::FieldWriterSafe<'a, u16, AUXCTL2_SPEC, u8, AUX2LVL_A, 3, 8>;
impl<'a> AUX2LVL_W<'a> {
    #[doc = "AUX2 supply threshold level: 0"]
    #[inline(always)]
    pub fn aux2lvl_0(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_0)
    }
    #[doc = "AUX2 supply threshold level: 1"]
    #[inline(always)]
    pub fn aux2lvl_1(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_1)
    }
    #[doc = "AUX2 supply threshold level: 2"]
    #[inline(always)]
    pub fn aux2lvl_2(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_2)
    }
    #[doc = "AUX2 supply threshold level: 3"]
    #[inline(always)]
    pub fn aux2lvl_3(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_3)
    }
    #[doc = "AUX2 supply threshold level: 4"]
    #[inline(always)]
    pub fn aux2lvl_4(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_4)
    }
    #[doc = "AUX2 supply threshold level: 5"]
    #[inline(always)]
    pub fn aux2lvl_5(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_5)
    }
    #[doc = "AUX2 supply threshold level: 6"]
    #[inline(always)]
    pub fn aux2lvl_6(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_6)
    }
    #[doc = "AUX2 supply threshold level: 7"]
    #[inline(always)]
    pub fn aux2lvl_7(self) -> &'a mut W {
        self.variant(AUX2LVL_A::AUX2LVL_7)
    }
}
#[doc = "Auxiliary supply monitoring rate Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXMR_A {
    #[doc = "0: Auxiliary supply monitoring rate: 0"]
    AUXMR_0 = 0,
    #[doc = "1: Auxiliary supply monitoring rate: 1"]
    AUXMR_1 = 1,
    #[doc = "2: Auxiliary supply monitoring rate: 2"]
    AUXMR_2 = 2,
    #[doc = "3: Auxiliary supply monitoring rate: 3"]
    AUXMR_3 = 3,
}
impl From<AUXMR_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXMR` reader - Auxiliary supply monitoring rate Bit: 0"]
pub type AUXMR_R = crate::FieldReader<u8, AUXMR_A>;
impl AUXMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXMR_A {
        match self.bits {
            0 => AUXMR_A::AUXMR_0,
            1 => AUXMR_A::AUXMR_1,
            2 => AUXMR_A::AUXMR_2,
            3 => AUXMR_A::AUXMR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUXMR_0`"]
    #[inline(always)]
    pub fn is_auxmr_0(&self) -> bool {
        *self == AUXMR_A::AUXMR_0
    }
    #[doc = "Checks if the value of the field is `AUXMR_1`"]
    #[inline(always)]
    pub fn is_auxmr_1(&self) -> bool {
        *self == AUXMR_A::AUXMR_1
    }
    #[doc = "Checks if the value of the field is `AUXMR_2`"]
    #[inline(always)]
    pub fn is_auxmr_2(&self) -> bool {
        *self == AUXMR_A::AUXMR_2
    }
    #[doc = "Checks if the value of the field is `AUXMR_3`"]
    #[inline(always)]
    pub fn is_auxmr_3(&self) -> bool {
        *self == AUXMR_A::AUXMR_3
    }
}
#[doc = "Field `AUXMR` writer - Auxiliary supply monitoring rate Bit: 0"]
pub type AUXMR_W<'a> = crate::FieldWriterSafe<'a, u16, AUXCTL2_SPEC, u8, AUXMR_A, 2, 12>;
impl<'a> AUXMR_W<'a> {
    #[doc = "Auxiliary supply monitoring rate: 0"]
    #[inline(always)]
    pub fn auxmr_0(self) -> &'a mut W {
        self.variant(AUXMR_A::AUXMR_0)
    }
    #[doc = "Auxiliary supply monitoring rate: 1"]
    #[inline(always)]
    pub fn auxmr_1(self) -> &'a mut W {
        self.variant(AUXMR_A::AUXMR_1)
    }
    #[doc = "Auxiliary supply monitoring rate: 2"]
    #[inline(always)]
    pub fn auxmr_2(self) -> &'a mut W {
        self.variant(AUXMR_A::AUXMR_2)
    }
    #[doc = "Auxiliary supply monitoring rate: 3"]
    #[inline(always)]
    pub fn auxmr_3(self) -> &'a mut W {
        self.variant(AUXMR_A::AUXMR_3)
    }
}
impl R {
    #[doc = "Bits 0:2 - DVCC supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux0lvl(&self) -> AUX0LVL_R {
        AUX0LVL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AUX1 supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux1lvl(&self) -> AUX1LVL_R {
        AUX1LVL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - AUX2 supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux2lvl(&self) -> AUX2LVL_R {
        AUX2LVL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Auxiliary supply monitoring rate Bit: 0"]
    #[inline(always)]
    pub fn auxmr(&self) -> AUXMR_R {
        AUXMR_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DVCC supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux0lvl(&mut self) -> AUX0LVL_W {
        AUX0LVL_W::new(self)
    }
    #[doc = "Bits 4:6 - AUX1 supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux1lvl(&mut self) -> AUX1LVL_W {
        AUX1LVL_W::new(self)
    }
    #[doc = "Bits 8:10 - AUX2 supply threshold level Bit: 0"]
    #[inline(always)]
    pub fn aux2lvl(&mut self) -> AUX2LVL_W {
        AUX2LVL_W::new(self)
    }
    #[doc = "Bits 12:13 - Auxiliary supply monitoring rate Bit: 0"]
    #[inline(always)]
    pub fn auxmr(&mut self) -> AUXMR_W {
        AUXMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Supply Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctl2](index.html) module"]
pub struct AUXCTL2_SPEC;
impl crate::RegisterSpec for AUXCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxctl2::R](R) reader structure"]
impl crate::Readable for AUXCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxctl2::W](W) writer structure"]
impl crate::Writable for AUXCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXCTL2 to value 0"]
impl crate::Resettable for AUXCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
