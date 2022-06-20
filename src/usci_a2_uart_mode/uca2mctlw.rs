#[doc = "Register `UCA2MCTLW` reader"]
pub struct R(crate::R<UCA2MCTLW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA2MCTLW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA2MCTLW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA2MCTLW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA2MCTLW` writer"]
pub struct W(crate::W<UCA2MCTLW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA2MCTLW_SPEC>;
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
impl From<crate::W<UCA2MCTLW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA2MCTLW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOS16` reader - USCI 16-times Oversampling enable"]
pub type UCOS16_R = crate::BitReader<bool>;
#[doc = "Field `UCOS16` writer - USCI 16-times Oversampling enable"]
pub type UCOS16_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 0>;
#[doc = "USCI First Stage Modulation Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCBRF_A {
    #[doc = "0: USCI First Stage Modulation: 0"]
    UCBRF_0 = 0,
    #[doc = "1: USCI First Stage Modulation: 1"]
    UCBRF_1 = 1,
    #[doc = "2: USCI First Stage Modulation: 2"]
    UCBRF_2 = 2,
    #[doc = "3: USCI First Stage Modulation: 3"]
    UCBRF_3 = 3,
    #[doc = "4: USCI First Stage Modulation: 4"]
    UCBRF_4 = 4,
    #[doc = "5: USCI First Stage Modulation: 5"]
    UCBRF_5 = 5,
    #[doc = "6: USCI First Stage Modulation: 6"]
    UCBRF_6 = 6,
    #[doc = "7: USCI First Stage Modulation: 7"]
    UCBRF_7 = 7,
    #[doc = "8: USCI First Stage Modulation: 8"]
    UCBRF_8 = 8,
    #[doc = "9: USCI First Stage Modulation: 9"]
    UCBRF_9 = 9,
    #[doc = "10: USCI First Stage Modulation: A"]
    UCBRF_10 = 10,
    #[doc = "11: USCI First Stage Modulation: B"]
    UCBRF_11 = 11,
    #[doc = "12: USCI First Stage Modulation: C"]
    UCBRF_12 = 12,
    #[doc = "13: USCI First Stage Modulation: D"]
    UCBRF_13 = 13,
    #[doc = "14: USCI First Stage Modulation: E"]
    UCBRF_14 = 14,
    #[doc = "15: USCI First Stage Modulation: F"]
    UCBRF_15 = 15,
}
impl From<UCBRF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCBRF` reader - USCI First Stage Modulation Select 3"]
pub type UCBRF_R = crate::FieldReader<u8, UCBRF_A>;
impl UCBRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRF_A {
        match self.bits {
            0 => UCBRF_A::UCBRF_0,
            1 => UCBRF_A::UCBRF_1,
            2 => UCBRF_A::UCBRF_2,
            3 => UCBRF_A::UCBRF_3,
            4 => UCBRF_A::UCBRF_4,
            5 => UCBRF_A::UCBRF_5,
            6 => UCBRF_A::UCBRF_6,
            7 => UCBRF_A::UCBRF_7,
            8 => UCBRF_A::UCBRF_8,
            9 => UCBRF_A::UCBRF_9,
            10 => UCBRF_A::UCBRF_10,
            11 => UCBRF_A::UCBRF_11,
            12 => UCBRF_A::UCBRF_12,
            13 => UCBRF_A::UCBRF_13,
            14 => UCBRF_A::UCBRF_14,
            15 => UCBRF_A::UCBRF_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCBRF_0`"]
    #[inline(always)]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == UCBRF_A::UCBRF_0
    }
    #[doc = "Checks if the value of the field is `UCBRF_1`"]
    #[inline(always)]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == UCBRF_A::UCBRF_1
    }
    #[doc = "Checks if the value of the field is `UCBRF_2`"]
    #[inline(always)]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == UCBRF_A::UCBRF_2
    }
    #[doc = "Checks if the value of the field is `UCBRF_3`"]
    #[inline(always)]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == UCBRF_A::UCBRF_3
    }
    #[doc = "Checks if the value of the field is `UCBRF_4`"]
    #[inline(always)]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == UCBRF_A::UCBRF_4
    }
    #[doc = "Checks if the value of the field is `UCBRF_5`"]
    #[inline(always)]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == UCBRF_A::UCBRF_5
    }
    #[doc = "Checks if the value of the field is `UCBRF_6`"]
    #[inline(always)]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == UCBRF_A::UCBRF_6
    }
    #[doc = "Checks if the value of the field is `UCBRF_7`"]
    #[inline(always)]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == UCBRF_A::UCBRF_7
    }
    #[doc = "Checks if the value of the field is `UCBRF_8`"]
    #[inline(always)]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == UCBRF_A::UCBRF_8
    }
    #[doc = "Checks if the value of the field is `UCBRF_9`"]
    #[inline(always)]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == UCBRF_A::UCBRF_9
    }
    #[doc = "Checks if the value of the field is `UCBRF_10`"]
    #[inline(always)]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == UCBRF_A::UCBRF_10
    }
    #[doc = "Checks if the value of the field is `UCBRF_11`"]
    #[inline(always)]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == UCBRF_A::UCBRF_11
    }
    #[doc = "Checks if the value of the field is `UCBRF_12`"]
    #[inline(always)]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == UCBRF_A::UCBRF_12
    }
    #[doc = "Checks if the value of the field is `UCBRF_13`"]
    #[inline(always)]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == UCBRF_A::UCBRF_13
    }
    #[doc = "Checks if the value of the field is `UCBRF_14`"]
    #[inline(always)]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == UCBRF_A::UCBRF_14
    }
    #[doc = "Checks if the value of the field is `UCBRF_15`"]
    #[inline(always)]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == UCBRF_A::UCBRF_15
    }
}
#[doc = "Field `UCBRF` writer - USCI First Stage Modulation Select 3"]
pub type UCBRF_W<'a> = crate::FieldWriterSafe<'a, u16, UCA2MCTLW_SPEC, u8, UCBRF_A, 4, 4>;
impl<'a> UCBRF_W<'a> {
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrf_0(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrf_1(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrf_2(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrf_3(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrf_4(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrf_5(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrf_6(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrf_7(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn ucbrf_8(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn ucbrf_9(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn ucbrf_10(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn ucbrf_11(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn ucbrf_12(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn ucbrf_13(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn ucbrf_14(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn ucbrf_15(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_15)
    }
}
#[doc = "Field `UCBRS0` reader - USCI Second Stage Modulation Select 0"]
pub type UCBRS0_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS0` writer - USCI Second Stage Modulation Select 0"]
pub type UCBRS0_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 8>;
#[doc = "Field `UCBRS1` reader - USCI Second Stage Modulation Select 1"]
pub type UCBRS1_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS1` writer - USCI Second Stage Modulation Select 1"]
pub type UCBRS1_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 9>;
#[doc = "Field `UCBRS2` reader - USCI Second Stage Modulation Select 2"]
pub type UCBRS2_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS2` writer - USCI Second Stage Modulation Select 2"]
pub type UCBRS2_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 10>;
#[doc = "Field `UCBRS3` reader - USCI Second Stage Modulation Select 3"]
pub type UCBRS3_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS3` writer - USCI Second Stage Modulation Select 3"]
pub type UCBRS3_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 11>;
#[doc = "Field `UCBRS4` reader - USCI Second Stage Modulation Select 4"]
pub type UCBRS4_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS4` writer - USCI Second Stage Modulation Select 4"]
pub type UCBRS4_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 12>;
#[doc = "Field `UCBRS5` reader - USCI Second Stage Modulation Select 5"]
pub type UCBRS5_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS5` writer - USCI Second Stage Modulation Select 5"]
pub type UCBRS5_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 13>;
#[doc = "Field `UCBRS6` reader - USCI Second Stage Modulation Select 6"]
pub type UCBRS6_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS6` writer - USCI Second Stage Modulation Select 6"]
pub type UCBRS6_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 14>;
#[doc = "Field `UCBRS7` reader - USCI Second Stage Modulation Select 7"]
pub type UCBRS7_R = crate::BitReader<bool>;
#[doc = "Field `UCBRS7` writer - USCI Second Stage Modulation Select 7"]
pub type UCBRS7_W<'a> = crate::BitWriter<'a, u16, UCA2MCTLW_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&self) -> UCBRS0_R {
        UCBRS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&self) -> UCBRS1_R {
        UCBRS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&self) -> UCBRS2_R {
        UCBRS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&self) -> UCBRS3_R {
        UCBRS3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&self) -> UCBRS4_R {
        UCBRS4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&self) -> UCBRS5_R {
        UCBRS5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&self) -> UCBRS6_R {
        UCBRS6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&self) -> UCBRS7_R {
        UCBRS7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W::new(self)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W::new(self)
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline(always)]
    pub fn ucbrs0(&mut self) -> UCBRS0_W {
        UCBRS0_W::new(self)
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline(always)]
    pub fn ucbrs1(&mut self) -> UCBRS1_W {
        UCBRS1_W::new(self)
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs2(&mut self) -> UCBRS2_W {
        UCBRS2_W::new(self)
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrs3(&mut self) -> UCBRS3_W {
        UCBRS3_W::new(self)
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline(always)]
    pub fn ucbrs4(&mut self) -> UCBRS4_W {
        UCBRS4_W::new(self)
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline(always)]
    pub fn ucbrs5(&mut self) -> UCBRS5_W {
        UCBRS5_W::new(self)
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline(always)]
    pub fn ucbrs6(&mut self) -> UCBRS6_W {
        UCBRS6_W::new(self)
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline(always)]
    pub fn ucbrs7(&mut self) -> UCBRS7_W {
        UCBRS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A2 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2mctlw](index.html) module"]
pub struct UCA2MCTLW_SPEC;
impl crate::RegisterSpec for UCA2MCTLW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca2mctlw::R](R) reader structure"]
impl crate::Readable for UCA2MCTLW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca2mctlw::W](W) writer structure"]
impl crate::Writable for UCA2MCTLW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA2MCTLW to value 0"]
impl crate::Resettable for UCA2MCTLW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
