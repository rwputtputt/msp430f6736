#[doc = "Register `UCA1CTLW1` reader"]
pub struct R(crate::R<UCA1CTLW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1CTLW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1CTLW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1CTLW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1CTLW1` writer"]
pub struct W(crate::W<UCA1CTLW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1CTLW1_SPEC>;
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
impl From<crate::W<UCA1CTLW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1CTLW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USCI Deglitch Time Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: USCI Deglitch time: 0"]
    UCGLIT_0 = 0,
    #[doc = "1: USCI Deglitch time: 1"]
    UCGLIT_1 = 1,
    #[doc = "2: USCI Deglitch time: 2"]
    UCGLIT_2 = 2,
    #[doc = "3: USCI Deglitch time: 3"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCGLIT` reader - USCI Deglitch Time Bit 1"]
pub type UCGLIT_R = crate::FieldReader<u8, UCGLIT_A>;
impl UCGLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_3
    }
}
#[doc = "Field `UCGLIT` writer - USCI Deglitch Time Bit 1"]
pub type UCGLIT_W<'a> = crate::FieldWriterSafe<'a, u16, UCA1CTLW1_SPEC, u8, UCGLIT_A, 2, 0>;
impl<'a> UCGLIT_W<'a> {
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W {
        UCGLIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctlw1](index.html) module"]
pub struct UCA1CTLW1_SPEC;
impl crate::RegisterSpec for UCA1CTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca1ctlw1::R](R) reader structure"]
impl crate::Readable for UCA1CTLW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1ctlw1::W](W) writer structure"]
impl crate::Writable for UCA1CTLW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1CTLW1 to value 0"]
impl crate::Resettable for UCA1CTLW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
