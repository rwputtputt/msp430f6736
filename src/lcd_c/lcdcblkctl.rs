#[doc = "Register `LCDCBLKCTL` reader"]
pub struct R(crate::R<LCDCBLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCBLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCBLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCBLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCBLKCTL` writer"]
pub struct W(crate::W<LCDCBLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCBLKCTL_SPEC>;
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
impl From<crate::W<LCDCBLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCBLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LCD_C Blinking mode Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDBLKMOD_A {
    #[doc = "0: LCD_C Blinking mode: Off"]
    LCDBLKMOD_0 = 0,
    #[doc = "1: LCD_C Blinking mode: Individual"]
    LCDBLKMOD_1 = 1,
    #[doc = "2: LCD_C Blinking mode: All"]
    LCDBLKMOD_2 = 2,
    #[doc = "3: LCD_C Blinking mode: Switching"]
    LCDBLKMOD_3 = 3,
}
impl From<LCDBLKMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCDBLKMOD` reader - LCD_C Blinking mode Bit: 0"]
pub type LCDBLKMOD_R = crate::FieldReader<u8, LCDBLKMOD_A>;
impl LCDBLKMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKMOD_A {
        match self.bits {
            0 => LCDBLKMOD_A::LCDBLKMOD_0,
            1 => LCDBLKMOD_A::LCDBLKMOD_1,
            2 => LCDBLKMOD_A::LCDBLKMOD_2,
            3 => LCDBLKMOD_A::LCDBLKMOD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_0`"]
    #[inline(always)]
    pub fn is_lcdblkmod_0(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_1`"]
    #[inline(always)]
    pub fn is_lcdblkmod_1(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_2`"]
    #[inline(always)]
    pub fn is_lcdblkmod_2(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKMOD_3`"]
    #[inline(always)]
    pub fn is_lcdblkmod_3(&self) -> bool {
        *self == LCDBLKMOD_A::LCDBLKMOD_3
    }
}
#[doc = "Field `LCDBLKMOD` writer - LCD_C Blinking mode Bit: 0"]
pub type LCDBLKMOD_W<'a> = crate::FieldWriterSafe<'a, u16, LCDCBLKCTL_SPEC, u8, LCDBLKMOD_A, 2, 0>;
impl<'a> LCDBLKMOD_W<'a> {
    #[doc = "LCD_C Blinking mode: Off"]
    #[inline(always)]
    pub fn lcdblkmod_0(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_0)
    }
    #[doc = "LCD_C Blinking mode: Individual"]
    #[inline(always)]
    pub fn lcdblkmod_1(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_1)
    }
    #[doc = "LCD_C Blinking mode: All"]
    #[inline(always)]
    pub fn lcdblkmod_2(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_2)
    }
    #[doc = "LCD_C Blinking mode: Switching"]
    #[inline(always)]
    pub fn lcdblkmod_3(self) -> &'a mut W {
        self.variant(LCDBLKMOD_A::LCDBLKMOD_3)
    }
}
#[doc = "LCD_C Clock pre-scaler for blinking frequency Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDBLKPRE_A {
    #[doc = "0: LCD_C Clock pre-scaler for blinking frequency: 0"]
    LCDBLKPRE_0 = 0,
    #[doc = "1: LCD_C Clock pre-scaler for blinking frequency: 1"]
    LCDBLKPRE_1 = 1,
    #[doc = "2: LCD_C Clock pre-scaler for blinking frequency: 2"]
    LCDBLKPRE_2 = 2,
    #[doc = "3: LCD_C Clock pre-scaler for blinking frequency: 3"]
    LCDBLKPRE_3 = 3,
    #[doc = "4: LCD_C Clock pre-scaler for blinking frequency: 4"]
    LCDBLKPRE_4 = 4,
    #[doc = "5: LCD_C Clock pre-scaler for blinking frequency: 5"]
    LCDBLKPRE_5 = 5,
    #[doc = "6: LCD_C Clock pre-scaler for blinking frequency: 6"]
    LCDBLKPRE_6 = 6,
    #[doc = "7: LCD_C Clock pre-scaler for blinking frequency: 7"]
    LCDBLKPRE_7 = 7,
}
impl From<LCDBLKPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCDBLKPRE` reader - LCD_C Clock pre-scaler for blinking frequency Bit: 0"]
pub type LCDBLKPRE_R = crate::FieldReader<u8, LCDBLKPRE_A>;
impl LCDBLKPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKPRE_A {
        match self.bits {
            0 => LCDBLKPRE_A::LCDBLKPRE_0,
            1 => LCDBLKPRE_A::LCDBLKPRE_1,
            2 => LCDBLKPRE_A::LCDBLKPRE_2,
            3 => LCDBLKPRE_A::LCDBLKPRE_3,
            4 => LCDBLKPRE_A::LCDBLKPRE_4,
            5 => LCDBLKPRE_A::LCDBLKPRE_5,
            6 => LCDBLKPRE_A::LCDBLKPRE_6,
            7 => LCDBLKPRE_A::LCDBLKPRE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_0`"]
    #[inline(always)]
    pub fn is_lcdblkpre_0(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_1`"]
    #[inline(always)]
    pub fn is_lcdblkpre_1(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_2`"]
    #[inline(always)]
    pub fn is_lcdblkpre_2(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_3`"]
    #[inline(always)]
    pub fn is_lcdblkpre_3(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_3
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_4`"]
    #[inline(always)]
    pub fn is_lcdblkpre_4(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_4
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_5`"]
    #[inline(always)]
    pub fn is_lcdblkpre_5(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_5
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_6`"]
    #[inline(always)]
    pub fn is_lcdblkpre_6(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_6
    }
    #[doc = "Checks if the value of the field is `LCDBLKPRE_7`"]
    #[inline(always)]
    pub fn is_lcdblkpre_7(&self) -> bool {
        *self == LCDBLKPRE_A::LCDBLKPRE_7
    }
}
#[doc = "Field `LCDBLKPRE` writer - LCD_C Clock pre-scaler for blinking frequency Bit: 0"]
pub type LCDBLKPRE_W<'a> = crate::FieldWriterSafe<'a, u16, LCDCBLKCTL_SPEC, u8, LCDBLKPRE_A, 3, 2>;
impl<'a> LCDBLKPRE_W<'a> {
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 0"]
    #[inline(always)]
    pub fn lcdblkpre_0(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_0)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 1"]
    #[inline(always)]
    pub fn lcdblkpre_1(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_1)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 2"]
    #[inline(always)]
    pub fn lcdblkpre_2(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_2)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 3"]
    #[inline(always)]
    pub fn lcdblkpre_3(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_3)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 4"]
    #[inline(always)]
    pub fn lcdblkpre_4(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_4)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 5"]
    #[inline(always)]
    pub fn lcdblkpre_5(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_5)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 6"]
    #[inline(always)]
    pub fn lcdblkpre_6(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_6)
    }
    #[doc = "LCD_C Clock pre-scaler for blinking frequency: 7"]
    #[inline(always)]
    pub fn lcdblkpre_7(self) -> &'a mut W {
        self.variant(LCDBLKPRE_A::LCDBLKPRE_7)
    }
}
#[doc = "LCD_C Clock divider for blinking frequency Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDBLKDIV_A {
    #[doc = "0: LCD_C Clock divider for blinking frequency: 0"]
    LCDBLKDIV_0 = 0,
    #[doc = "1: LCD_C Clock divider for blinking frequency: 1"]
    LCDBLKDIV_1 = 1,
    #[doc = "2: LCD_C Clock divider for blinking frequency: 2"]
    LCDBLKDIV_2 = 2,
    #[doc = "3: LCD_C Clock divider for blinking frequency: 3"]
    LCDBLKDIV_3 = 3,
    #[doc = "4: LCD_C Clock divider for blinking frequency: 4"]
    LCDBLKDIV_4 = 4,
    #[doc = "5: LCD_C Clock divider for blinking frequency: 5"]
    LCDBLKDIV_5 = 5,
    #[doc = "6: LCD_C Clock divider for blinking frequency: 6"]
    LCDBLKDIV_6 = 6,
    #[doc = "7: LCD_C Clock divider for blinking frequency: 7"]
    LCDBLKDIV_7 = 7,
}
impl From<LCDBLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LCDBLKDIV` reader - LCD_C Clock divider for blinking frequency Bit: 0"]
pub type LCDBLKDIV_R = crate::FieldReader<u8, LCDBLKDIV_A>;
impl LCDBLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKDIV_A {
        match self.bits {
            0 => LCDBLKDIV_A::LCDBLKDIV_0,
            1 => LCDBLKDIV_A::LCDBLKDIV_1,
            2 => LCDBLKDIV_A::LCDBLKDIV_2,
            3 => LCDBLKDIV_A::LCDBLKDIV_3,
            4 => LCDBLKDIV_A::LCDBLKDIV_4,
            5 => LCDBLKDIV_A::LCDBLKDIV_5,
            6 => LCDBLKDIV_A::LCDBLKDIV_6,
            7 => LCDBLKDIV_A::LCDBLKDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_0`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_0(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_1`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_1(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_2`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_2(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_3`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_3(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_3
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_4`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_4(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_4
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_5`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_5(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_5
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_6`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_6(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_6
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIV_7`"]
    #[inline(always)]
    pub fn is_lcdblkdiv_7(&self) -> bool {
        *self == LCDBLKDIV_A::LCDBLKDIV_7
    }
}
#[doc = "Field `LCDBLKDIV` writer - LCD_C Clock divider for blinking frequency Bit: 0"]
pub type LCDBLKDIV_W<'a> = crate::FieldWriterSafe<'a, u16, LCDCBLKCTL_SPEC, u8, LCDBLKDIV_A, 3, 5>;
impl<'a> LCDBLKDIV_W<'a> {
    #[doc = "LCD_C Clock divider for blinking frequency: 0"]
    #[inline(always)]
    pub fn lcdblkdiv_0(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_0)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 1"]
    #[inline(always)]
    pub fn lcdblkdiv_1(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_1)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 2"]
    #[inline(always)]
    pub fn lcdblkdiv_2(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_2)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 3"]
    #[inline(always)]
    pub fn lcdblkdiv_3(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_3)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 4"]
    #[inline(always)]
    pub fn lcdblkdiv_4(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_4)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 5"]
    #[inline(always)]
    pub fn lcdblkdiv_5(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_5)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 6"]
    #[inline(always)]
    pub fn lcdblkdiv_6(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_6)
    }
    #[doc = "LCD_C Clock divider for blinking frequency: 7"]
    #[inline(always)]
    pub fn lcdblkdiv_7(self) -> &'a mut W {
        self.variant(LCDBLKDIV_A::LCDBLKDIV_7)
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD_C Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&self) -> LCDBLKMOD_R {
        LCDBLKMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - LCD_C Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&self) -> LCDBLKPRE_R {
        LCDBLKPRE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - LCD_C Clock divider for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkdiv(&self) -> LCDBLKDIV_R {
        LCDBLKDIV_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD_C Blinking mode Bit: 0"]
    #[inline(always)]
    pub fn lcdblkmod(&mut self) -> LCDBLKMOD_W {
        LCDBLKMOD_W::new(self)
    }
    #[doc = "Bits 2:4 - LCD_C Clock pre-scaler for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkpre(&mut self) -> LCDBLKPRE_W {
        LCDBLKPRE_W::new(self)
    }
    #[doc = "Bits 5:7 - LCD_C Clock divider for blinking frequency Bit: 0"]
    #[inline(always)]
    pub fn lcdblkdiv(&mut self) -> LCDBLKDIV_W {
        LCDBLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C blinking control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcblkctl](index.html) module"]
pub struct LCDCBLKCTL_SPEC;
impl crate::RegisterSpec for LCDCBLKCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcblkctl::R](R) reader structure"]
impl crate::Readable for LCDCBLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcblkctl::W](W) writer structure"]
impl crate::Writable for LCDCBLKCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCBLKCTL to value 0"]
impl crate::Resettable for LCDCBLKCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
