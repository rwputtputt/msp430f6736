#[doc = "Register `SVSMLCTL` reader"]
pub struct R(crate::R<SVSMLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVSMLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVSMLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVSMLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVSMLCTL` writer"]
pub struct W(crate::W<SVSMLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVSMLCTL_SPEC>;
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
impl From<crate::W<SVSMLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVSMLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SVS and SVM low side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSMLRRL_A {
    #[doc = "0: SVS and SVM low side Reset Release Voltage Level 0"]
    SVSMLRRL_0 = 0,
    #[doc = "1: SVS and SVM low side Reset Release Voltage Level 1"]
    SVSMLRRL_1 = 1,
    #[doc = "2: SVS and SVM low side Reset Release Voltage Level 2"]
    SVSMLRRL_2 = 2,
    #[doc = "3: SVS and SVM low side Reset Release Voltage Level 3"]
    SVSMLRRL_3 = 3,
    #[doc = "4: SVS and SVM low side Reset Release Voltage Level 4"]
    SVSMLRRL_4 = 4,
    #[doc = "5: SVS and SVM low side Reset Release Voltage Level 5"]
    SVSMLRRL_5 = 5,
    #[doc = "6: SVS and SVM low side Reset Release Voltage Level 6"]
    SVSMLRRL_6 = 6,
    #[doc = "7: SVS and SVM low side Reset Release Voltage Level 7"]
    SVSMLRRL_7 = 7,
}
impl From<SVSMLRRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSMLRRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SVSMLRRL` reader - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
pub type SVSMLRRL_R = crate::FieldReader<u8, SVSMLRRL_A>;
impl SVSMLRRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMLRRL_A {
        match self.bits {
            0 => SVSMLRRL_A::SVSMLRRL_0,
            1 => SVSMLRRL_A::SVSMLRRL_1,
            2 => SVSMLRRL_A::SVSMLRRL_2,
            3 => SVSMLRRL_A::SVSMLRRL_3,
            4 => SVSMLRRL_A::SVSMLRRL_4,
            5 => SVSMLRRL_A::SVSMLRRL_5,
            6 => SVSMLRRL_A::SVSMLRRL_6,
            7 => SVSMLRRL_A::SVSMLRRL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_0`"]
    #[inline(always)]
    pub fn is_svsmlrrl_0(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_0
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_1`"]
    #[inline(always)]
    pub fn is_svsmlrrl_1(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_1
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_2`"]
    #[inline(always)]
    pub fn is_svsmlrrl_2(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_2
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_3`"]
    #[inline(always)]
    pub fn is_svsmlrrl_3(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_3
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_4`"]
    #[inline(always)]
    pub fn is_svsmlrrl_4(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_4
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_5`"]
    #[inline(always)]
    pub fn is_svsmlrrl_5(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_5
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_6`"]
    #[inline(always)]
    pub fn is_svsmlrrl_6(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_6
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_7`"]
    #[inline(always)]
    pub fn is_svsmlrrl_7(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_7
    }
}
#[doc = "Field `SVSMLRRL` writer - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
pub type SVSMLRRL_W<'a> = crate::FieldWriterSafe<'a, u16, SVSMLCTL_SPEC, u8, SVSMLRRL_A, 3, 0>;
impl<'a> SVSMLRRL_W<'a> {
    #[doc = "SVS and SVM low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmlrrl_0(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_0)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmlrrl_1(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_1)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmlrrl_2(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_2)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmlrrl_3(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_3)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmlrrl_4(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_4)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmlrrl_5(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_5)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmlrrl_6(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_6)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmlrrl_7(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_7)
    }
}
#[doc = "Field `SVSMLDLYST` reader - SVS and SVM low side delay status"]
pub type SVSMLDLYST_R = crate::BitReader<bool>;
#[doc = "Field `SVSMLDLYST` writer - SVS and SVM low side delay status"]
pub type SVSMLDLYST_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 3>;
#[doc = "Field `SVSLMD` reader - SVS low side mode"]
pub type SVSLMD_R = crate::BitReader<bool>;
#[doc = "Field `SVSLMD` writer - SVS low side mode"]
pub type SVSLMD_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 4>;
#[doc = "Field `SVSMLEVM` reader - SVS and SVM low side event mask"]
pub type SVSMLEVM_R = crate::BitReader<bool>;
#[doc = "Field `SVSMLEVM` writer - SVS and SVM low side event mask"]
pub type SVSMLEVM_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 6>;
#[doc = "Field `SVSMLACE` reader - SVS and SVM low side auto control enable"]
pub type SVSMLACE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMLACE` writer - SVS and SVM low side auto control enable"]
pub type SVSMLACE_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 7>;
#[doc = "SVS low side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSLRVL_A {
    #[doc = "0: SVS low side Reset Release Voltage Level 0"]
    SVSLRVL_0 = 0,
    #[doc = "1: SVS low side Reset Release Voltage Level 1"]
    SVSLRVL_1 = 1,
    #[doc = "2: SVS low side Reset Release Voltage Level 2"]
    SVSLRVL_2 = 2,
    #[doc = "3: SVS low side Reset Release Voltage Level 3"]
    SVSLRVL_3 = 3,
}
impl From<SVSLRVL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSLRVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SVSLRVL` reader - SVS low side reset voltage level Bit: 0"]
pub type SVSLRVL_R = crate::FieldReader<u8, SVSLRVL_A>;
impl SVSLRVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSLRVL_A {
        match self.bits {
            0 => SVSLRVL_A::SVSLRVL_0,
            1 => SVSLRVL_A::SVSLRVL_1,
            2 => SVSLRVL_A::SVSLRVL_2,
            3 => SVSLRVL_A::SVSLRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_0`"]
    #[inline(always)]
    pub fn is_svslrvl_0(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_0
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_1`"]
    #[inline(always)]
    pub fn is_svslrvl_1(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_1
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_2`"]
    #[inline(always)]
    pub fn is_svslrvl_2(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_2
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_3`"]
    #[inline(always)]
    pub fn is_svslrvl_3(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_3
    }
}
#[doc = "Field `SVSLRVL` writer - SVS low side reset voltage level Bit: 0"]
pub type SVSLRVL_W<'a> = crate::FieldWriterSafe<'a, u16, SVSMLCTL_SPEC, u8, SVSLRVL_A, 2, 8>;
impl<'a> SVSLRVL_W<'a> {
    #[doc = "SVS low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svslrvl_0(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_0)
    }
    #[doc = "SVS low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svslrvl_1(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_1)
    }
    #[doc = "SVS low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svslrvl_2(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_2)
    }
    #[doc = "SVS low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svslrvl_3(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_3)
    }
}
#[doc = "Field `SVSLE` reader - SVS low side enable"]
pub type SVSLE_R = crate::BitReader<bool>;
#[doc = "Field `SVSLE` writer - SVS low side enable"]
pub type SVSLE_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 10>;
#[doc = "Field `SVSLFP` reader - SVS low side full performace mode"]
pub type SVSLFP_R = crate::BitReader<bool>;
#[doc = "Field `SVSLFP` writer - SVS low side full performace mode"]
pub type SVSLFP_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 11>;
#[doc = "Field `SVMLOVPE` reader - SVM low side over-voltage enable"]
pub type SVMLOVPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLOVPE` writer - SVM low side over-voltage enable"]
pub type SVMLOVPE_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 12>;
#[doc = "Field `SVMLE` reader - SVM low side enable"]
pub type SVMLE_R = crate::BitReader<bool>;
#[doc = "Field `SVMLE` writer - SVM low side enable"]
pub type SVMLE_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 14>;
#[doc = "Field `SVMLFP` reader - SVM low side full performace mode"]
pub type SVMLFP_R = crate::BitReader<bool>;
#[doc = "Field `SVMLFP` writer - SVM low side full performace mode"]
pub type SVMLFP_W<'a> = crate::BitWriter<'a, u16, SVSMLCTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmlrrl(&self) -> SVSMLRRL_R {
        SVSMLRRL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    pub fn svsmldlyst(&self) -> SVSMLDLYST_R {
        SVSMLDLYST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    pub fn svslmd(&self) -> SVSLMD_R {
        SVSLMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    pub fn svsmlevm(&self) -> SVSMLEVM_R {
        SVSMLEVM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    pub fn svsmlace(&self) -> SVSMLACE_R {
        SVSMLACE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svslrvl(&self) -> SVSLRVL_R {
        SVSLRVL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    pub fn svsle(&self) -> SVSLE_R {
        SVSLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    pub fn svslfp(&self) -> SVSLFP_R {
        SVSLFP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    pub fn svmlovpe(&self) -> SVMLOVPE_R {
        SVMLOVPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    pub fn svmle(&self) -> SVMLE_R {
        SVMLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    pub fn svmlfp(&self) -> SVMLFP_R {
        SVMLFP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmlrrl(&mut self) -> SVSMLRRL_W {
        SVSMLRRL_W::new(self)
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    pub fn svsmldlyst(&mut self) -> SVSMLDLYST_W {
        SVSMLDLYST_W::new(self)
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    pub fn svslmd(&mut self) -> SVSLMD_W {
        SVSLMD_W::new(self)
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    pub fn svsmlevm(&mut self) -> SVSMLEVM_W {
        SVSMLEVM_W::new(self)
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    pub fn svsmlace(&mut self) -> SVSMLACE_W {
        SVSMLACE_W::new(self)
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svslrvl(&mut self) -> SVSLRVL_W {
        SVSLRVL_W::new(self)
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    pub fn svsle(&mut self) -> SVSLE_W {
        SVSLE_W::new(self)
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    pub fn svslfp(&mut self) -> SVSLFP_W {
        SVSLFP_W::new(self)
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    pub fn svmlovpe(&mut self) -> SVMLOVPE_W {
        SVMLOVPE_W::new(self)
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    pub fn svmle(&mut self) -> SVMLE_W {
        SVMLE_W::new(self)
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    pub fn svmlfp(&mut self) -> SVMLFP_W {
        SVMLFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SVS and SVM low side control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmlctl](index.html) module"]
pub struct SVSMLCTL_SPEC;
impl crate::RegisterSpec for SVSMLCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [svsmlctl::R](R) reader structure"]
impl crate::Readable for SVSMLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svsmlctl::W](W) writer structure"]
impl crate::Writable for SVSMLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SVSMLCTL to value 0"]
impl crate::Resettable for SVSMLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
