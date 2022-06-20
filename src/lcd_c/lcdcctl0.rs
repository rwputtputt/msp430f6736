#[doc = "Register `LCDCCTL0` reader"]
pub struct R(crate::R<LCDCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCCTL0` writer"]
pub struct W(crate::W<LCDCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCCTL0_SPEC>;
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
impl From<crate::W<LCDCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDON` reader - LCD_C LCD On"]
pub type LCDON_R = crate::BitReader<bool>;
#[doc = "Field `LCDON` writer - LCD_C LCD On"]
pub type LCDON_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 0>;
#[doc = "Field `LCDLP` reader - LCD_C Low Power Waveform"]
pub type LCDLP_R = crate::BitReader<bool>;
#[doc = "Field `LCDLP` writer - LCD_C Low Power Waveform"]
pub type LCDLP_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 1>;
#[doc = "Field `LCDSON` reader - LCD_C LCD Segments On"]
pub type LCDSON_R = crate::BitReader<bool>;
#[doc = "Field `LCDSON` writer - LCD_C LCD Segments On"]
pub type LCDSON_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 2>;
#[doc = "Field `LCDMX0` reader - LCD_C Mux Rate Bit: 0"]
pub type LCDMX0_R = crate::BitReader<bool>;
#[doc = "Field `LCDMX0` writer - LCD_C Mux Rate Bit: 0"]
pub type LCDMX0_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 3>;
#[doc = "Field `LCDMX1` reader - LCD_C Mux Rate Bit: 1"]
pub type LCDMX1_R = crate::BitReader<bool>;
#[doc = "Field `LCDMX1` writer - LCD_C Mux Rate Bit: 1"]
pub type LCDMX1_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 4>;
#[doc = "Field `LCDMX2` reader - LCD_C Mux Rate Bit: 2"]
pub type LCDMX2_R = crate::BitReader<bool>;
#[doc = "Field `LCDMX2` writer - LCD_C Mux Rate Bit: 2"]
pub type LCDMX2_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 5>;
#[doc = "Field `LCDSSEL` reader - LCD_C Clock Select"]
pub type LCDSSEL_R = crate::BitReader<bool>;
#[doc = "Field `LCDSSEL` writer - LCD_C Clock Select"]
pub type LCDSSEL_W<'a> = crate::BitWriter<'a, u16, LCDCCTL0_SPEC, bool, 7>;
#[doc = "LCD_C LCD frequency pre-scaler Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDPRE_A {
    #[doc = "0: LCD_C LCD frequency pre-scaler: /1"]
    LCDPRE_0 = 0,
    #[doc = "1: LCD_C LCD frequency pre-scaler: /2"]
    LCDPRE_1 = 1,
    #[doc = "2: LCD_C LCD frequency pre-scaler: /4"]
    LCDPRE_2 = 2,
    #[doc = "3: LCD_C LCD frequency pre-scaler: /8"]
    LCDPRE_3 = 3,
    #[doc = "4: LCD_C LCD frequency pre-scaler: /16"]
    LCDPRE_4 = 4,
    #[doc = "5: LCD_C LCD frequency pre-scaler: /32"]
    LCDPRE_5 = 5,
}
impl From<LCDPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCDPRE` reader - LCD_C LCD frequency pre-scaler Bit: 0"]
pub type LCDPRE_R = crate::FieldReader<u8, LCDPRE_A>;
impl LCDPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCDPRE_A> {
        match self.bits {
            0 => Some(LCDPRE_A::LCDPRE_0),
            1 => Some(LCDPRE_A::LCDPRE_1),
            2 => Some(LCDPRE_A::LCDPRE_2),
            3 => Some(LCDPRE_A::LCDPRE_3),
            4 => Some(LCDPRE_A::LCDPRE_4),
            5 => Some(LCDPRE_A::LCDPRE_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LCDPRE_0`"]
    #[inline(always)]
    pub fn is_lcdpre_0(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_0
    }
    #[doc = "Checks if the value of the field is `LCDPRE_1`"]
    #[inline(always)]
    pub fn is_lcdpre_1(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_1
    }
    #[doc = "Checks if the value of the field is `LCDPRE_2`"]
    #[inline(always)]
    pub fn is_lcdpre_2(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_2
    }
    #[doc = "Checks if the value of the field is `LCDPRE_3`"]
    #[inline(always)]
    pub fn is_lcdpre_3(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_3
    }
    #[doc = "Checks if the value of the field is `LCDPRE_4`"]
    #[inline(always)]
    pub fn is_lcdpre_4(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_4
    }
    #[doc = "Checks if the value of the field is `LCDPRE_5`"]
    #[inline(always)]
    pub fn is_lcdpre_5(&self) -> bool {
        *self == LCDPRE_A::LCDPRE_5
    }
}
#[doc = "Field `LCDPRE` writer - LCD_C LCD frequency pre-scaler Bit: 0"]
pub type LCDPRE_W<'a> = crate::FieldWriter<'a, u16, LCDCCTL0_SPEC, u8, LCDPRE_A, 3, 8>;
impl<'a> LCDPRE_W<'a> {
    #[doc = "LCD_C LCD frequency pre-scaler: /1"]
    #[inline(always)]
    pub fn lcdpre_0(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_0)
    }
    #[doc = "LCD_C LCD frequency pre-scaler: /2"]
    #[inline(always)]
    pub fn lcdpre_1(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_1)
    }
    #[doc = "LCD_C LCD frequency pre-scaler: /4"]
    #[inline(always)]
    pub fn lcdpre_2(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_2)
    }
    #[doc = "LCD_C LCD frequency pre-scaler: /8"]
    #[inline(always)]
    pub fn lcdpre_3(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_3)
    }
    #[doc = "LCD_C LCD frequency pre-scaler: /16"]
    #[inline(always)]
    pub fn lcdpre_4(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_4)
    }
    #[doc = "LCD_C LCD frequency pre-scaler: /32"]
    #[inline(always)]
    pub fn lcdpre_5(self) -> &'a mut W {
        self.variant(LCDPRE_A::LCDPRE_5)
    }
}
#[doc = "LCD_C LCD frequency divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDDIV_A {
    #[doc = "0: LCD_C LCD frequency divider: /1"]
    LCDDIV_0 = 0,
    #[doc = "1: LCD_C LCD frequency divider: /2"]
    LCDDIV_1 = 1,
    #[doc = "2: LCD_C LCD frequency divider: /3"]
    LCDDIV_2 = 2,
    #[doc = "3: LCD_C LCD frequency divider: /4"]
    LCDDIV_3 = 3,
    #[doc = "4: LCD_C LCD frequency divider: /5"]
    LCDDIV_4 = 4,
    #[doc = "5: LCD_C LCD frequency divider: /6"]
    LCDDIV_5 = 5,
    #[doc = "6: LCD_C LCD frequency divider: /7"]
    LCDDIV_6 = 6,
    #[doc = "7: LCD_C LCD frequency divider: /8"]
    LCDDIV_7 = 7,
    #[doc = "8: LCD_C LCD frequency divider: /9"]
    LCDDIV_8 = 8,
    #[doc = "9: LCD_C LCD frequency divider: /10"]
    LCDDIV_9 = 9,
    #[doc = "10: LCD_C LCD frequency divider: /11"]
    LCDDIV_10 = 10,
    #[doc = "11: LCD_C LCD frequency divider: /12"]
    LCDDIV_11 = 11,
    #[doc = "12: LCD_C LCD frequency divider: /13"]
    LCDDIV_12 = 12,
    #[doc = "13: LCD_C LCD frequency divider: /14"]
    LCDDIV_13 = 13,
    #[doc = "14: LCD_C LCD frequency divider: /15"]
    LCDDIV_14 = 14,
    #[doc = "15: LCD_C LCD frequency divider: /16"]
    LCDDIV_15 = 15,
    #[doc = "16: LCD_C LCD frequency divider: /17"]
    LCDDIV_16 = 16,
    #[doc = "17: LCD_C LCD frequency divider: /18"]
    LCDDIV_17 = 17,
    #[doc = "18: LCD_C LCD frequency divider: /19"]
    LCDDIV_18 = 18,
    #[doc = "19: LCD_C LCD frequency divider: /20"]
    LCDDIV_19 = 19,
    #[doc = "20: LCD_C LCD frequency divider: /21"]
    LCDDIV_20 = 20,
    #[doc = "21: LCD_C LCD frequency divider: /22"]
    LCDDIV_21 = 21,
    #[doc = "22: LCD_C LCD frequency divider: /23"]
    LCDDIV_22 = 22,
    #[doc = "23: LCD_C LCD frequency divider: /24"]
    LCDDIV_23 = 23,
    #[doc = "24: LCD_C LCD frequency divider: /25"]
    LCDDIV_24 = 24,
    #[doc = "25: LCD_C LCD frequency divider: /26"]
    LCDDIV_25 = 25,
    #[doc = "26: LCD_C LCD frequency divider: /27"]
    LCDDIV_26 = 26,
    #[doc = "27: LCD_C LCD frequency divider: /28"]
    LCDDIV_27 = 27,
    #[doc = "28: LCD_C LCD frequency divider: /29"]
    LCDDIV_28 = 28,
    #[doc = "29: LCD_C LCD frequency divider: /30"]
    LCDDIV_29 = 29,
    #[doc = "30: LCD_C LCD frequency divider: /31"]
    LCDDIV_30 = 30,
    #[doc = "31: LCD_C LCD frequency divider: /32"]
    LCDDIV_31 = 31,
}
impl From<LCDDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCDDIV` reader - LCD_C LCD frequency divider Bit: 0"]
pub type LCDDIV_R = crate::FieldReader<u8, LCDDIV_A>;
impl LCDDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDDIV_A {
        match self.bits {
            0 => LCDDIV_A::LCDDIV_0,
            1 => LCDDIV_A::LCDDIV_1,
            2 => LCDDIV_A::LCDDIV_2,
            3 => LCDDIV_A::LCDDIV_3,
            4 => LCDDIV_A::LCDDIV_4,
            5 => LCDDIV_A::LCDDIV_5,
            6 => LCDDIV_A::LCDDIV_6,
            7 => LCDDIV_A::LCDDIV_7,
            8 => LCDDIV_A::LCDDIV_8,
            9 => LCDDIV_A::LCDDIV_9,
            10 => LCDDIV_A::LCDDIV_10,
            11 => LCDDIV_A::LCDDIV_11,
            12 => LCDDIV_A::LCDDIV_12,
            13 => LCDDIV_A::LCDDIV_13,
            14 => LCDDIV_A::LCDDIV_14,
            15 => LCDDIV_A::LCDDIV_15,
            16 => LCDDIV_A::LCDDIV_16,
            17 => LCDDIV_A::LCDDIV_17,
            18 => LCDDIV_A::LCDDIV_18,
            19 => LCDDIV_A::LCDDIV_19,
            20 => LCDDIV_A::LCDDIV_20,
            21 => LCDDIV_A::LCDDIV_21,
            22 => LCDDIV_A::LCDDIV_22,
            23 => LCDDIV_A::LCDDIV_23,
            24 => LCDDIV_A::LCDDIV_24,
            25 => LCDDIV_A::LCDDIV_25,
            26 => LCDDIV_A::LCDDIV_26,
            27 => LCDDIV_A::LCDDIV_27,
            28 => LCDDIV_A::LCDDIV_28,
            29 => LCDDIV_A::LCDDIV_29,
            30 => LCDDIV_A::LCDDIV_30,
            31 => LCDDIV_A::LCDDIV_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDDIV_0`"]
    #[inline(always)]
    pub fn is_lcddiv_0(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_0
    }
    #[doc = "Checks if the value of the field is `LCDDIV_1`"]
    #[inline(always)]
    pub fn is_lcddiv_1(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_1
    }
    #[doc = "Checks if the value of the field is `LCDDIV_2`"]
    #[inline(always)]
    pub fn is_lcddiv_2(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_2
    }
    #[doc = "Checks if the value of the field is `LCDDIV_3`"]
    #[inline(always)]
    pub fn is_lcddiv_3(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_3
    }
    #[doc = "Checks if the value of the field is `LCDDIV_4`"]
    #[inline(always)]
    pub fn is_lcddiv_4(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_4
    }
    #[doc = "Checks if the value of the field is `LCDDIV_5`"]
    #[inline(always)]
    pub fn is_lcddiv_5(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_5
    }
    #[doc = "Checks if the value of the field is `LCDDIV_6`"]
    #[inline(always)]
    pub fn is_lcddiv_6(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_6
    }
    #[doc = "Checks if the value of the field is `LCDDIV_7`"]
    #[inline(always)]
    pub fn is_lcddiv_7(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_7
    }
    #[doc = "Checks if the value of the field is `LCDDIV_8`"]
    #[inline(always)]
    pub fn is_lcddiv_8(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_8
    }
    #[doc = "Checks if the value of the field is `LCDDIV_9`"]
    #[inline(always)]
    pub fn is_lcddiv_9(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_9
    }
    #[doc = "Checks if the value of the field is `LCDDIV_10`"]
    #[inline(always)]
    pub fn is_lcddiv_10(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_10
    }
    #[doc = "Checks if the value of the field is `LCDDIV_11`"]
    #[inline(always)]
    pub fn is_lcddiv_11(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_11
    }
    #[doc = "Checks if the value of the field is `LCDDIV_12`"]
    #[inline(always)]
    pub fn is_lcddiv_12(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_12
    }
    #[doc = "Checks if the value of the field is `LCDDIV_13`"]
    #[inline(always)]
    pub fn is_lcddiv_13(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_13
    }
    #[doc = "Checks if the value of the field is `LCDDIV_14`"]
    #[inline(always)]
    pub fn is_lcddiv_14(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_14
    }
    #[doc = "Checks if the value of the field is `LCDDIV_15`"]
    #[inline(always)]
    pub fn is_lcddiv_15(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_15
    }
    #[doc = "Checks if the value of the field is `LCDDIV_16`"]
    #[inline(always)]
    pub fn is_lcddiv_16(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_16
    }
    #[doc = "Checks if the value of the field is `LCDDIV_17`"]
    #[inline(always)]
    pub fn is_lcddiv_17(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_17
    }
    #[doc = "Checks if the value of the field is `LCDDIV_18`"]
    #[inline(always)]
    pub fn is_lcddiv_18(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_18
    }
    #[doc = "Checks if the value of the field is `LCDDIV_19`"]
    #[inline(always)]
    pub fn is_lcddiv_19(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_19
    }
    #[doc = "Checks if the value of the field is `LCDDIV_20`"]
    #[inline(always)]
    pub fn is_lcddiv_20(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_20
    }
    #[doc = "Checks if the value of the field is `LCDDIV_21`"]
    #[inline(always)]
    pub fn is_lcddiv_21(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_21
    }
    #[doc = "Checks if the value of the field is `LCDDIV_22`"]
    #[inline(always)]
    pub fn is_lcddiv_22(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_22
    }
    #[doc = "Checks if the value of the field is `LCDDIV_23`"]
    #[inline(always)]
    pub fn is_lcddiv_23(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_23
    }
    #[doc = "Checks if the value of the field is `LCDDIV_24`"]
    #[inline(always)]
    pub fn is_lcddiv_24(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_24
    }
    #[doc = "Checks if the value of the field is `LCDDIV_25`"]
    #[inline(always)]
    pub fn is_lcddiv_25(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_25
    }
    #[doc = "Checks if the value of the field is `LCDDIV_26`"]
    #[inline(always)]
    pub fn is_lcddiv_26(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_26
    }
    #[doc = "Checks if the value of the field is `LCDDIV_27`"]
    #[inline(always)]
    pub fn is_lcddiv_27(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_27
    }
    #[doc = "Checks if the value of the field is `LCDDIV_28`"]
    #[inline(always)]
    pub fn is_lcddiv_28(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_28
    }
    #[doc = "Checks if the value of the field is `LCDDIV_29`"]
    #[inline(always)]
    pub fn is_lcddiv_29(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_29
    }
    #[doc = "Checks if the value of the field is `LCDDIV_30`"]
    #[inline(always)]
    pub fn is_lcddiv_30(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_30
    }
    #[doc = "Checks if the value of the field is `LCDDIV_31`"]
    #[inline(always)]
    pub fn is_lcddiv_31(&self) -> bool {
        *self == LCDDIV_A::LCDDIV_31
    }
}
#[doc = "Field `LCDDIV` writer - LCD_C LCD frequency divider Bit: 0"]
pub type LCDDIV_W<'a> = crate::FieldWriterSafe<'a, u16, LCDCCTL0_SPEC, u8, LCDDIV_A, 5, 11>;
impl<'a> LCDDIV_W<'a> {
    #[doc = "LCD_C LCD frequency divider: /1"]
    #[inline(always)]
    pub fn lcddiv_0(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_0)
    }
    #[doc = "LCD_C LCD frequency divider: /2"]
    #[inline(always)]
    pub fn lcddiv_1(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_1)
    }
    #[doc = "LCD_C LCD frequency divider: /3"]
    #[inline(always)]
    pub fn lcddiv_2(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_2)
    }
    #[doc = "LCD_C LCD frequency divider: /4"]
    #[inline(always)]
    pub fn lcddiv_3(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_3)
    }
    #[doc = "LCD_C LCD frequency divider: /5"]
    #[inline(always)]
    pub fn lcddiv_4(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_4)
    }
    #[doc = "LCD_C LCD frequency divider: /6"]
    #[inline(always)]
    pub fn lcddiv_5(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_5)
    }
    #[doc = "LCD_C LCD frequency divider: /7"]
    #[inline(always)]
    pub fn lcddiv_6(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_6)
    }
    #[doc = "LCD_C LCD frequency divider: /8"]
    #[inline(always)]
    pub fn lcddiv_7(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_7)
    }
    #[doc = "LCD_C LCD frequency divider: /9"]
    #[inline(always)]
    pub fn lcddiv_8(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_8)
    }
    #[doc = "LCD_C LCD frequency divider: /10"]
    #[inline(always)]
    pub fn lcddiv_9(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_9)
    }
    #[doc = "LCD_C LCD frequency divider: /11"]
    #[inline(always)]
    pub fn lcddiv_10(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_10)
    }
    #[doc = "LCD_C LCD frequency divider: /12"]
    #[inline(always)]
    pub fn lcddiv_11(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_11)
    }
    #[doc = "LCD_C LCD frequency divider: /13"]
    #[inline(always)]
    pub fn lcddiv_12(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_12)
    }
    #[doc = "LCD_C LCD frequency divider: /14"]
    #[inline(always)]
    pub fn lcddiv_13(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_13)
    }
    #[doc = "LCD_C LCD frequency divider: /15"]
    #[inline(always)]
    pub fn lcddiv_14(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_14)
    }
    #[doc = "LCD_C LCD frequency divider: /16"]
    #[inline(always)]
    pub fn lcddiv_15(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_15)
    }
    #[doc = "LCD_C LCD frequency divider: /17"]
    #[inline(always)]
    pub fn lcddiv_16(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_16)
    }
    #[doc = "LCD_C LCD frequency divider: /18"]
    #[inline(always)]
    pub fn lcddiv_17(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_17)
    }
    #[doc = "LCD_C LCD frequency divider: /19"]
    #[inline(always)]
    pub fn lcddiv_18(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_18)
    }
    #[doc = "LCD_C LCD frequency divider: /20"]
    #[inline(always)]
    pub fn lcddiv_19(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_19)
    }
    #[doc = "LCD_C LCD frequency divider: /21"]
    #[inline(always)]
    pub fn lcddiv_20(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_20)
    }
    #[doc = "LCD_C LCD frequency divider: /22"]
    #[inline(always)]
    pub fn lcddiv_21(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_21)
    }
    #[doc = "LCD_C LCD frequency divider: /23"]
    #[inline(always)]
    pub fn lcddiv_22(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_22)
    }
    #[doc = "LCD_C LCD frequency divider: /24"]
    #[inline(always)]
    pub fn lcddiv_23(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_23)
    }
    #[doc = "LCD_C LCD frequency divider: /25"]
    #[inline(always)]
    pub fn lcddiv_24(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_24)
    }
    #[doc = "LCD_C LCD frequency divider: /26"]
    #[inline(always)]
    pub fn lcddiv_25(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_25)
    }
    #[doc = "LCD_C LCD frequency divider: /27"]
    #[inline(always)]
    pub fn lcddiv_26(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_26)
    }
    #[doc = "LCD_C LCD frequency divider: /28"]
    #[inline(always)]
    pub fn lcddiv_27(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_27)
    }
    #[doc = "LCD_C LCD frequency divider: /29"]
    #[inline(always)]
    pub fn lcddiv_28(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_28)
    }
    #[doc = "LCD_C LCD frequency divider: /30"]
    #[inline(always)]
    pub fn lcddiv_29(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_29)
    }
    #[doc = "LCD_C LCD frequency divider: /31"]
    #[inline(always)]
    pub fn lcddiv_30(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_30)
    }
    #[doc = "LCD_C LCD frequency divider: /32"]
    #[inline(always)]
    pub fn lcddiv_31(self) -> &'a mut W {
        self.variant(LCDDIV_A::LCDDIV_31)
    }
}
impl R {
    #[doc = "Bit 0 - LCD_C LCD On"]
    #[inline(always)]
    pub fn lcdon(&self) -> LCDON_R {
        LCDON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD_C Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&self) -> LCDLP_R {
        LCDLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD_C LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&self) -> LCDSON_R {
        LCDSON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD_C Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&self) -> LCDMX0_R {
        LCDMX0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD_C Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&self) -> LCDMX1_R {
        LCDMX1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD_C Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&self) -> LCDMX2_R {
        LCDMX2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD_C Clock Select"]
    #[inline(always)]
    pub fn lcdssel(&self) -> LCDSSEL_R {
        LCDSSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - LCD_C LCD frequency pre-scaler Bit: 0"]
    #[inline(always)]
    pub fn lcdpre(&self) -> LCDPRE_R {
        LCDPRE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - LCD_C LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&self) -> LCDDIV_R {
        LCDDIV_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_C LCD On"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LCDON_W {
        LCDON_W::new(self)
    }
    #[doc = "Bit 1 - LCD_C Low Power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LCDLP_W {
        LCDLP_W::new(self)
    }
    #[doc = "Bit 2 - LCD_C LCD Segments On"]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LCDSON_W {
        LCDSON_W::new(self)
    }
    #[doc = "Bit 3 - LCD_C Mux Rate Bit: 0"]
    #[inline(always)]
    pub fn lcdmx0(&mut self) -> LCDMX0_W {
        LCDMX0_W::new(self)
    }
    #[doc = "Bit 4 - LCD_C Mux Rate Bit: 1"]
    #[inline(always)]
    pub fn lcdmx1(&mut self) -> LCDMX1_W {
        LCDMX1_W::new(self)
    }
    #[doc = "Bit 5 - LCD_C Mux Rate Bit: 2"]
    #[inline(always)]
    pub fn lcdmx2(&mut self) -> LCDMX2_W {
        LCDMX2_W::new(self)
    }
    #[doc = "Bit 7 - LCD_C Clock Select"]
    #[inline(always)]
    pub fn lcdssel(&mut self) -> LCDSSEL_W {
        LCDSSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - LCD_C LCD frequency pre-scaler Bit: 0"]
    #[inline(always)]
    pub fn lcdpre(&mut self) -> LCDPRE_W {
        LCDPRE_W::new(self)
    }
    #[doc = "Bits 11:15 - LCD_C LCD frequency divider Bit: 0"]
    #[inline(always)]
    pub fn lcddiv(&mut self) -> LCDDIV_W {
        LCDDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcctl0](index.html) module"]
pub struct LCDCCTL0_SPEC;
impl crate::RegisterSpec for LCDCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcctl0::R](R) reader structure"]
impl crate::Readable for LCDCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcctl0::W](W) writer structure"]
impl crate::Writable for LCDCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCCTL0 to value 0"]
impl crate::Resettable for LCDCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
