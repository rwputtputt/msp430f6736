#[doc = "Register `DMACTL0` reader"]
pub struct R(crate::R<DMACTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL0` writer"]
pub struct W(crate::W<DMACTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL0_SPEC>;
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
impl From<crate::W<DMACTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA channel 0 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA0TSEL_A {
    #[doc = "0: DMA channel 0 transfer select 0"]
    DMA0TSEL_0 = 0,
    #[doc = "1: DMA channel 0 transfer select 1"]
    DMA0TSEL_1 = 1,
    #[doc = "2: DMA channel 0 transfer select 2"]
    DMA0TSEL_2 = 2,
    #[doc = "3: DMA channel 0 transfer select 3"]
    DMA0TSEL_3 = 3,
    #[doc = "4: DMA channel 0 transfer select 4"]
    DMA0TSEL_4 = 4,
    #[doc = "5: DMA channel 0 transfer select 5"]
    DMA0TSEL_5 = 5,
    #[doc = "6: DMA channel 0 transfer select 6"]
    DMA0TSEL_6 = 6,
    #[doc = "7: DMA channel 0 transfer select 7"]
    DMA0TSEL_7 = 7,
    #[doc = "8: DMA channel 0 transfer select 8"]
    DMA0TSEL_8 = 8,
    #[doc = "9: DMA channel 0 transfer select 9"]
    DMA0TSEL_9 = 9,
    #[doc = "10: DMA channel 0 transfer select 10"]
    DMA0TSEL_10 = 10,
    #[doc = "11: DMA channel 0 transfer select 11"]
    DMA0TSEL_11 = 11,
    #[doc = "12: DMA channel 0 transfer select 12"]
    DMA0TSEL_12 = 12,
    #[doc = "13: DMA channel 0 transfer select 13"]
    DMA0TSEL_13 = 13,
    #[doc = "14: DMA channel 0 transfer select 14"]
    DMA0TSEL_14 = 14,
    #[doc = "15: DMA channel 0 transfer select 15"]
    DMA0TSEL_15 = 15,
    #[doc = "16: DMA channel 0 transfer select 16"]
    DMA0TSEL_16 = 16,
    #[doc = "17: DMA channel 0 transfer select 17"]
    DMA0TSEL_17 = 17,
    #[doc = "18: DMA channel 0 transfer select 18"]
    DMA0TSEL_18 = 18,
    #[doc = "19: DMA channel 0 transfer select 19"]
    DMA0TSEL_19 = 19,
    #[doc = "20: DMA channel 0 transfer select 20"]
    DMA0TSEL_20 = 20,
    #[doc = "21: DMA channel 0 transfer select 21"]
    DMA0TSEL_21 = 21,
    #[doc = "22: DMA channel 0 transfer select 22"]
    DMA0TSEL_22 = 22,
    #[doc = "23: DMA channel 0 transfer select 23"]
    DMA0TSEL_23 = 23,
    #[doc = "24: DMA channel 0 transfer select 24"]
    DMA0TSEL_24 = 24,
    #[doc = "25: DMA channel 0 transfer select 25"]
    DMA0TSEL_25 = 25,
    #[doc = "26: DMA channel 0 transfer select 26"]
    DMA0TSEL_26 = 26,
    #[doc = "27: DMA channel 0 transfer select 27"]
    DMA0TSEL_27 = 27,
    #[doc = "28: DMA channel 0 transfer select 28"]
    DMA0TSEL_28 = 28,
    #[doc = "29: DMA channel 0 transfer select 29"]
    DMA0TSEL_29 = 29,
    #[doc = "30: DMA channel 0 transfer select 30"]
    DMA0TSEL_30 = 30,
    #[doc = "31: DMA channel 0 transfer select 31"]
    DMA0TSEL_31 = 31,
}
impl From<DMA0TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA0TSEL` reader - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_R = crate::FieldReader<u8, DMA0TSEL_A>;
impl DMA0TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0TSEL_A {
        match self.bits {
            0 => DMA0TSEL_A::DMA0TSEL_0,
            1 => DMA0TSEL_A::DMA0TSEL_1,
            2 => DMA0TSEL_A::DMA0TSEL_2,
            3 => DMA0TSEL_A::DMA0TSEL_3,
            4 => DMA0TSEL_A::DMA0TSEL_4,
            5 => DMA0TSEL_A::DMA0TSEL_5,
            6 => DMA0TSEL_A::DMA0TSEL_6,
            7 => DMA0TSEL_A::DMA0TSEL_7,
            8 => DMA0TSEL_A::DMA0TSEL_8,
            9 => DMA0TSEL_A::DMA0TSEL_9,
            10 => DMA0TSEL_A::DMA0TSEL_10,
            11 => DMA0TSEL_A::DMA0TSEL_11,
            12 => DMA0TSEL_A::DMA0TSEL_12,
            13 => DMA0TSEL_A::DMA0TSEL_13,
            14 => DMA0TSEL_A::DMA0TSEL_14,
            15 => DMA0TSEL_A::DMA0TSEL_15,
            16 => DMA0TSEL_A::DMA0TSEL_16,
            17 => DMA0TSEL_A::DMA0TSEL_17,
            18 => DMA0TSEL_A::DMA0TSEL_18,
            19 => DMA0TSEL_A::DMA0TSEL_19,
            20 => DMA0TSEL_A::DMA0TSEL_20,
            21 => DMA0TSEL_A::DMA0TSEL_21,
            22 => DMA0TSEL_A::DMA0TSEL_22,
            23 => DMA0TSEL_A::DMA0TSEL_23,
            24 => DMA0TSEL_A::DMA0TSEL_24,
            25 => DMA0TSEL_A::DMA0TSEL_25,
            26 => DMA0TSEL_A::DMA0TSEL_26,
            27 => DMA0TSEL_A::DMA0TSEL_27,
            28 => DMA0TSEL_A::DMA0TSEL_28,
            29 => DMA0TSEL_A::DMA0TSEL_29,
            30 => DMA0TSEL_A::DMA0TSEL_30,
            31 => DMA0TSEL_A::DMA0TSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_0`"]
    #[inline(always)]
    pub fn is_dma0tsel_0(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_1`"]
    #[inline(always)]
    pub fn is_dma0tsel_1(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_2`"]
    #[inline(always)]
    pub fn is_dma0tsel_2(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_3`"]
    #[inline(always)]
    pub fn is_dma0tsel_3(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_4`"]
    #[inline(always)]
    pub fn is_dma0tsel_4(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_5`"]
    #[inline(always)]
    pub fn is_dma0tsel_5(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_6`"]
    #[inline(always)]
    pub fn is_dma0tsel_6(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_7`"]
    #[inline(always)]
    pub fn is_dma0tsel_7(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_8`"]
    #[inline(always)]
    pub fn is_dma0tsel_8(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_9`"]
    #[inline(always)]
    pub fn is_dma0tsel_9(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_10`"]
    #[inline(always)]
    pub fn is_dma0tsel_10(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_11`"]
    #[inline(always)]
    pub fn is_dma0tsel_11(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_12`"]
    #[inline(always)]
    pub fn is_dma0tsel_12(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_13`"]
    #[inline(always)]
    pub fn is_dma0tsel_13(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_14`"]
    #[inline(always)]
    pub fn is_dma0tsel_14(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_15`"]
    #[inline(always)]
    pub fn is_dma0tsel_15(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_15
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_16`"]
    #[inline(always)]
    pub fn is_dma0tsel_16(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_16
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_17`"]
    #[inline(always)]
    pub fn is_dma0tsel_17(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_17
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_18`"]
    #[inline(always)]
    pub fn is_dma0tsel_18(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_18
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_19`"]
    #[inline(always)]
    pub fn is_dma0tsel_19(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_19
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_20`"]
    #[inline(always)]
    pub fn is_dma0tsel_20(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_20
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_21`"]
    #[inline(always)]
    pub fn is_dma0tsel_21(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_21
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_22`"]
    #[inline(always)]
    pub fn is_dma0tsel_22(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_22
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_23`"]
    #[inline(always)]
    pub fn is_dma0tsel_23(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_23
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_24`"]
    #[inline(always)]
    pub fn is_dma0tsel_24(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_24
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_25`"]
    #[inline(always)]
    pub fn is_dma0tsel_25(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_25
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_26`"]
    #[inline(always)]
    pub fn is_dma0tsel_26(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_26
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_27`"]
    #[inline(always)]
    pub fn is_dma0tsel_27(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_27
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_28`"]
    #[inline(always)]
    pub fn is_dma0tsel_28(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_28
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_29`"]
    #[inline(always)]
    pub fn is_dma0tsel_29(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_29
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_30`"]
    #[inline(always)]
    pub fn is_dma0tsel_30(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_30
    }
    #[doc = "Checks if the value of the field is `DMA0TSEL_31`"]
    #[inline(always)]
    pub fn is_dma0tsel_31(&self) -> bool {
        *self == DMA0TSEL_A::DMA0TSEL_31
    }
}
#[doc = "Field `DMA0TSEL` writer - DMA channel 0 transfer select bit 0"]
pub type DMA0TSEL_W<'a> = crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, DMA0TSEL_A, 5, 0>;
impl<'a> DMA0TSEL_W<'a> {
    #[doc = "DMA channel 0 transfer select 0"]
    #[inline(always)]
    pub fn dma0tsel_0(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_0)
    }
    #[doc = "DMA channel 0 transfer select 1"]
    #[inline(always)]
    pub fn dma0tsel_1(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_1)
    }
    #[doc = "DMA channel 0 transfer select 2"]
    #[inline(always)]
    pub fn dma0tsel_2(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_2)
    }
    #[doc = "DMA channel 0 transfer select 3"]
    #[inline(always)]
    pub fn dma0tsel_3(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_3)
    }
    #[doc = "DMA channel 0 transfer select 4"]
    #[inline(always)]
    pub fn dma0tsel_4(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_4)
    }
    #[doc = "DMA channel 0 transfer select 5"]
    #[inline(always)]
    pub fn dma0tsel_5(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_5)
    }
    #[doc = "DMA channel 0 transfer select 6"]
    #[inline(always)]
    pub fn dma0tsel_6(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_6)
    }
    #[doc = "DMA channel 0 transfer select 7"]
    #[inline(always)]
    pub fn dma0tsel_7(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_7)
    }
    #[doc = "DMA channel 0 transfer select 8"]
    #[inline(always)]
    pub fn dma0tsel_8(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_8)
    }
    #[doc = "DMA channel 0 transfer select 9"]
    #[inline(always)]
    pub fn dma0tsel_9(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_9)
    }
    #[doc = "DMA channel 0 transfer select 10"]
    #[inline(always)]
    pub fn dma0tsel_10(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_10)
    }
    #[doc = "DMA channel 0 transfer select 11"]
    #[inline(always)]
    pub fn dma0tsel_11(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_11)
    }
    #[doc = "DMA channel 0 transfer select 12"]
    #[inline(always)]
    pub fn dma0tsel_12(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_12)
    }
    #[doc = "DMA channel 0 transfer select 13"]
    #[inline(always)]
    pub fn dma0tsel_13(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_13)
    }
    #[doc = "DMA channel 0 transfer select 14"]
    #[inline(always)]
    pub fn dma0tsel_14(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_14)
    }
    #[doc = "DMA channel 0 transfer select 15"]
    #[inline(always)]
    pub fn dma0tsel_15(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_15)
    }
    #[doc = "DMA channel 0 transfer select 16"]
    #[inline(always)]
    pub fn dma0tsel_16(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_16)
    }
    #[doc = "DMA channel 0 transfer select 17"]
    #[inline(always)]
    pub fn dma0tsel_17(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_17)
    }
    #[doc = "DMA channel 0 transfer select 18"]
    #[inline(always)]
    pub fn dma0tsel_18(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_18)
    }
    #[doc = "DMA channel 0 transfer select 19"]
    #[inline(always)]
    pub fn dma0tsel_19(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_19)
    }
    #[doc = "DMA channel 0 transfer select 20"]
    #[inline(always)]
    pub fn dma0tsel_20(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_20)
    }
    #[doc = "DMA channel 0 transfer select 21"]
    #[inline(always)]
    pub fn dma0tsel_21(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_21)
    }
    #[doc = "DMA channel 0 transfer select 22"]
    #[inline(always)]
    pub fn dma0tsel_22(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_22)
    }
    #[doc = "DMA channel 0 transfer select 23"]
    #[inline(always)]
    pub fn dma0tsel_23(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_23)
    }
    #[doc = "DMA channel 0 transfer select 24"]
    #[inline(always)]
    pub fn dma0tsel_24(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_24)
    }
    #[doc = "DMA channel 0 transfer select 25"]
    #[inline(always)]
    pub fn dma0tsel_25(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_25)
    }
    #[doc = "DMA channel 0 transfer select 26"]
    #[inline(always)]
    pub fn dma0tsel_26(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_26)
    }
    #[doc = "DMA channel 0 transfer select 27"]
    #[inline(always)]
    pub fn dma0tsel_27(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_27)
    }
    #[doc = "DMA channel 0 transfer select 28"]
    #[inline(always)]
    pub fn dma0tsel_28(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_28)
    }
    #[doc = "DMA channel 0 transfer select 29"]
    #[inline(always)]
    pub fn dma0tsel_29(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_29)
    }
    #[doc = "DMA channel 0 transfer select 30"]
    #[inline(always)]
    pub fn dma0tsel_30(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_30)
    }
    #[doc = "DMA channel 0 transfer select 31"]
    #[inline(always)]
    pub fn dma0tsel_31(self) -> &'a mut W {
        self.variant(DMA0TSEL_A::DMA0TSEL_31)
    }
}
#[doc = "DMA channel 1 transfer select bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA1TSEL_A {
    #[doc = "0: DMA channel 1 transfer select 0"]
    DMA1TSEL_0 = 0,
    #[doc = "1: DMA channel 1 transfer select 1"]
    DMA1TSEL_1 = 1,
    #[doc = "2: DMA channel 1 transfer select 2"]
    DMA1TSEL_2 = 2,
    #[doc = "3: DMA channel 1 transfer select 3"]
    DMA1TSEL_3 = 3,
    #[doc = "4: DMA channel 1 transfer select 4"]
    DMA1TSEL_4 = 4,
    #[doc = "5: DMA channel 1 transfer select 5"]
    DMA1TSEL_5 = 5,
    #[doc = "6: DMA channel 1 transfer select 6"]
    DMA1TSEL_6 = 6,
    #[doc = "7: DMA channel 1 transfer select 7"]
    DMA1TSEL_7 = 7,
    #[doc = "8: DMA channel 1 transfer select 8"]
    DMA1TSEL_8 = 8,
    #[doc = "9: DMA channel 1 transfer select 9"]
    DMA1TSEL_9 = 9,
    #[doc = "10: DMA channel 1 transfer select 10"]
    DMA1TSEL_10 = 10,
    #[doc = "11: DMA channel 1 transfer select 11"]
    DMA1TSEL_11 = 11,
    #[doc = "12: DMA channel 1 transfer select 12"]
    DMA1TSEL_12 = 12,
    #[doc = "13: DMA channel 1 transfer select 13"]
    DMA1TSEL_13 = 13,
    #[doc = "14: DMA channel 1 transfer select 14"]
    DMA1TSEL_14 = 14,
    #[doc = "15: DMA channel 1 transfer select 15"]
    DMA1TSEL_15 = 15,
    #[doc = "16: DMA channel 1 transfer select 16"]
    DMA1TSEL_16 = 16,
    #[doc = "17: DMA channel 1 transfer select 17"]
    DMA1TSEL_17 = 17,
    #[doc = "18: DMA channel 1 transfer select 18"]
    DMA1TSEL_18 = 18,
    #[doc = "19: DMA channel 1 transfer select 19"]
    DMA1TSEL_19 = 19,
    #[doc = "20: DMA channel 1 transfer select 20"]
    DMA1TSEL_20 = 20,
    #[doc = "21: DMA channel 1 transfer select 21"]
    DMA1TSEL_21 = 21,
    #[doc = "22: DMA channel 1 transfer select 22"]
    DMA1TSEL_22 = 22,
    #[doc = "23: DMA channel 1 transfer select 23"]
    DMA1TSEL_23 = 23,
    #[doc = "24: DMA channel 1 transfer select 24"]
    DMA1TSEL_24 = 24,
    #[doc = "25: DMA channel 1 transfer select 25"]
    DMA1TSEL_25 = 25,
    #[doc = "26: DMA channel 1 transfer select 26"]
    DMA1TSEL_26 = 26,
    #[doc = "27: DMA channel 1 transfer select 27"]
    DMA1TSEL_27 = 27,
    #[doc = "28: DMA channel 1 transfer select 28"]
    DMA1TSEL_28 = 28,
    #[doc = "29: DMA channel 1 transfer select 29"]
    DMA1TSEL_29 = 29,
    #[doc = "30: DMA channel 1 transfer select 30"]
    DMA1TSEL_30 = 30,
    #[doc = "31: DMA channel 1 transfer select 31"]
    DMA1TSEL_31 = 31,
}
impl From<DMA1TSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1TSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA1TSEL` reader - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_R = crate::FieldReader<u8, DMA1TSEL_A>;
impl DMA1TSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1TSEL_A {
        match self.bits {
            0 => DMA1TSEL_A::DMA1TSEL_0,
            1 => DMA1TSEL_A::DMA1TSEL_1,
            2 => DMA1TSEL_A::DMA1TSEL_2,
            3 => DMA1TSEL_A::DMA1TSEL_3,
            4 => DMA1TSEL_A::DMA1TSEL_4,
            5 => DMA1TSEL_A::DMA1TSEL_5,
            6 => DMA1TSEL_A::DMA1TSEL_6,
            7 => DMA1TSEL_A::DMA1TSEL_7,
            8 => DMA1TSEL_A::DMA1TSEL_8,
            9 => DMA1TSEL_A::DMA1TSEL_9,
            10 => DMA1TSEL_A::DMA1TSEL_10,
            11 => DMA1TSEL_A::DMA1TSEL_11,
            12 => DMA1TSEL_A::DMA1TSEL_12,
            13 => DMA1TSEL_A::DMA1TSEL_13,
            14 => DMA1TSEL_A::DMA1TSEL_14,
            15 => DMA1TSEL_A::DMA1TSEL_15,
            16 => DMA1TSEL_A::DMA1TSEL_16,
            17 => DMA1TSEL_A::DMA1TSEL_17,
            18 => DMA1TSEL_A::DMA1TSEL_18,
            19 => DMA1TSEL_A::DMA1TSEL_19,
            20 => DMA1TSEL_A::DMA1TSEL_20,
            21 => DMA1TSEL_A::DMA1TSEL_21,
            22 => DMA1TSEL_A::DMA1TSEL_22,
            23 => DMA1TSEL_A::DMA1TSEL_23,
            24 => DMA1TSEL_A::DMA1TSEL_24,
            25 => DMA1TSEL_A::DMA1TSEL_25,
            26 => DMA1TSEL_A::DMA1TSEL_26,
            27 => DMA1TSEL_A::DMA1TSEL_27,
            28 => DMA1TSEL_A::DMA1TSEL_28,
            29 => DMA1TSEL_A::DMA1TSEL_29,
            30 => DMA1TSEL_A::DMA1TSEL_30,
            31 => DMA1TSEL_A::DMA1TSEL_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_0`"]
    #[inline(always)]
    pub fn is_dma1tsel_0(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_0
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_1`"]
    #[inline(always)]
    pub fn is_dma1tsel_1(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_1
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_2`"]
    #[inline(always)]
    pub fn is_dma1tsel_2(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_2
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_3`"]
    #[inline(always)]
    pub fn is_dma1tsel_3(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_3
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_4`"]
    #[inline(always)]
    pub fn is_dma1tsel_4(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_4
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_5`"]
    #[inline(always)]
    pub fn is_dma1tsel_5(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_5
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_6`"]
    #[inline(always)]
    pub fn is_dma1tsel_6(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_6
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_7`"]
    #[inline(always)]
    pub fn is_dma1tsel_7(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_7
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_8`"]
    #[inline(always)]
    pub fn is_dma1tsel_8(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_8
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_9`"]
    #[inline(always)]
    pub fn is_dma1tsel_9(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_9
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_10`"]
    #[inline(always)]
    pub fn is_dma1tsel_10(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_10
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_11`"]
    #[inline(always)]
    pub fn is_dma1tsel_11(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_11
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_12`"]
    #[inline(always)]
    pub fn is_dma1tsel_12(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_12
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_13`"]
    #[inline(always)]
    pub fn is_dma1tsel_13(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_13
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_14`"]
    #[inline(always)]
    pub fn is_dma1tsel_14(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_14
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_15`"]
    #[inline(always)]
    pub fn is_dma1tsel_15(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_15
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_16`"]
    #[inline(always)]
    pub fn is_dma1tsel_16(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_16
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_17`"]
    #[inline(always)]
    pub fn is_dma1tsel_17(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_17
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_18`"]
    #[inline(always)]
    pub fn is_dma1tsel_18(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_18
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_19`"]
    #[inline(always)]
    pub fn is_dma1tsel_19(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_19
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_20`"]
    #[inline(always)]
    pub fn is_dma1tsel_20(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_20
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_21`"]
    #[inline(always)]
    pub fn is_dma1tsel_21(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_21
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_22`"]
    #[inline(always)]
    pub fn is_dma1tsel_22(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_22
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_23`"]
    #[inline(always)]
    pub fn is_dma1tsel_23(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_23
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_24`"]
    #[inline(always)]
    pub fn is_dma1tsel_24(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_24
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_25`"]
    #[inline(always)]
    pub fn is_dma1tsel_25(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_25
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_26`"]
    #[inline(always)]
    pub fn is_dma1tsel_26(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_26
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_27`"]
    #[inline(always)]
    pub fn is_dma1tsel_27(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_27
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_28`"]
    #[inline(always)]
    pub fn is_dma1tsel_28(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_28
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_29`"]
    #[inline(always)]
    pub fn is_dma1tsel_29(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_29
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_30`"]
    #[inline(always)]
    pub fn is_dma1tsel_30(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_30
    }
    #[doc = "Checks if the value of the field is `DMA1TSEL_31`"]
    #[inline(always)]
    pub fn is_dma1tsel_31(&self) -> bool {
        *self == DMA1TSEL_A::DMA1TSEL_31
    }
}
#[doc = "Field `DMA1TSEL` writer - DMA channel 1 transfer select bit 0"]
pub type DMA1TSEL_W<'a> = crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, DMA1TSEL_A, 5, 8>;
impl<'a> DMA1TSEL_W<'a> {
    #[doc = "DMA channel 1 transfer select 0"]
    #[inline(always)]
    pub fn dma1tsel_0(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_0)
    }
    #[doc = "DMA channel 1 transfer select 1"]
    #[inline(always)]
    pub fn dma1tsel_1(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_1)
    }
    #[doc = "DMA channel 1 transfer select 2"]
    #[inline(always)]
    pub fn dma1tsel_2(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_2)
    }
    #[doc = "DMA channel 1 transfer select 3"]
    #[inline(always)]
    pub fn dma1tsel_3(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_3)
    }
    #[doc = "DMA channel 1 transfer select 4"]
    #[inline(always)]
    pub fn dma1tsel_4(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_4)
    }
    #[doc = "DMA channel 1 transfer select 5"]
    #[inline(always)]
    pub fn dma1tsel_5(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_5)
    }
    #[doc = "DMA channel 1 transfer select 6"]
    #[inline(always)]
    pub fn dma1tsel_6(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_6)
    }
    #[doc = "DMA channel 1 transfer select 7"]
    #[inline(always)]
    pub fn dma1tsel_7(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_7)
    }
    #[doc = "DMA channel 1 transfer select 8"]
    #[inline(always)]
    pub fn dma1tsel_8(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_8)
    }
    #[doc = "DMA channel 1 transfer select 9"]
    #[inline(always)]
    pub fn dma1tsel_9(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_9)
    }
    #[doc = "DMA channel 1 transfer select 10"]
    #[inline(always)]
    pub fn dma1tsel_10(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_10)
    }
    #[doc = "DMA channel 1 transfer select 11"]
    #[inline(always)]
    pub fn dma1tsel_11(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_11)
    }
    #[doc = "DMA channel 1 transfer select 12"]
    #[inline(always)]
    pub fn dma1tsel_12(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_12)
    }
    #[doc = "DMA channel 1 transfer select 13"]
    #[inline(always)]
    pub fn dma1tsel_13(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_13)
    }
    #[doc = "DMA channel 1 transfer select 14"]
    #[inline(always)]
    pub fn dma1tsel_14(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_14)
    }
    #[doc = "DMA channel 1 transfer select 15"]
    #[inline(always)]
    pub fn dma1tsel_15(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_15)
    }
    #[doc = "DMA channel 1 transfer select 16"]
    #[inline(always)]
    pub fn dma1tsel_16(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_16)
    }
    #[doc = "DMA channel 1 transfer select 17"]
    #[inline(always)]
    pub fn dma1tsel_17(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_17)
    }
    #[doc = "DMA channel 1 transfer select 18"]
    #[inline(always)]
    pub fn dma1tsel_18(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_18)
    }
    #[doc = "DMA channel 1 transfer select 19"]
    #[inline(always)]
    pub fn dma1tsel_19(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_19)
    }
    #[doc = "DMA channel 1 transfer select 20"]
    #[inline(always)]
    pub fn dma1tsel_20(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_20)
    }
    #[doc = "DMA channel 1 transfer select 21"]
    #[inline(always)]
    pub fn dma1tsel_21(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_21)
    }
    #[doc = "DMA channel 1 transfer select 22"]
    #[inline(always)]
    pub fn dma1tsel_22(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_22)
    }
    #[doc = "DMA channel 1 transfer select 23"]
    #[inline(always)]
    pub fn dma1tsel_23(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_23)
    }
    #[doc = "DMA channel 1 transfer select 24"]
    #[inline(always)]
    pub fn dma1tsel_24(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_24)
    }
    #[doc = "DMA channel 1 transfer select 25"]
    #[inline(always)]
    pub fn dma1tsel_25(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_25)
    }
    #[doc = "DMA channel 1 transfer select 26"]
    #[inline(always)]
    pub fn dma1tsel_26(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_26)
    }
    #[doc = "DMA channel 1 transfer select 27"]
    #[inline(always)]
    pub fn dma1tsel_27(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_27)
    }
    #[doc = "DMA channel 1 transfer select 28"]
    #[inline(always)]
    pub fn dma1tsel_28(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_28)
    }
    #[doc = "DMA channel 1 transfer select 29"]
    #[inline(always)]
    pub fn dma1tsel_29(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_29)
    }
    #[doc = "DMA channel 1 transfer select 30"]
    #[inline(always)]
    pub fn dma1tsel_30(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_30)
    }
    #[doc = "DMA channel 1 transfer select 31"]
    #[inline(always)]
    pub fn dma1tsel_31(self) -> &'a mut W {
        self.variant(DMA1TSEL_A::DMA1TSEL_31)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&self) -> DMA0TSEL_R {
        DMA0TSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&self) -> DMA1TSEL_R {
        DMA1TSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel 0 transfer select bit 0"]
    #[inline(always)]
    pub fn dma0tsel(&mut self) -> DMA0TSEL_W {
        DMA0TSEL_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA channel 1 transfer select bit 0"]
    #[inline(always)]
    pub fn dma1tsel(&mut self) -> DMA1TSEL_W {
        DMA1TSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](index.html) module"]
pub struct DMACTL0_SPEC;
impl crate::RegisterSpec for DMACTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl0::R](R) reader structure"]
impl crate::Readable for DMACTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](W) writer structure"]
impl crate::Writable for DMACTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for DMACTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
