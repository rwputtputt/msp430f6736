#[doc = "Register `SVSMHCTL` reader"]
pub struct R(crate::R<SVSMHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVSMHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVSMHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVSMHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVSMHCTL` writer"]
pub struct W(crate::W<SVSMHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVSMHCTL_SPEC>;
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
impl From<crate::W<SVSMHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVSMHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SVS and SVM high side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSMHRRL_A {
    #[doc = "0: SVS and SVM high side Reset Release Voltage Level 0"]
    SVSMHRRL_0 = 0,
    #[doc = "1: SVS and SVM high side Reset Release Voltage Level 1"]
    SVSMHRRL_1 = 1,
    #[doc = "2: SVS and SVM high side Reset Release Voltage Level 2"]
    SVSMHRRL_2 = 2,
    #[doc = "3: SVS and SVM high side Reset Release Voltage Level 3"]
    SVSMHRRL_3 = 3,
    #[doc = "4: SVS and SVM high side Reset Release Voltage Level 4"]
    SVSMHRRL_4 = 4,
    #[doc = "5: SVS and SVM high side Reset Release Voltage Level 5"]
    SVSMHRRL_5 = 5,
    #[doc = "6: SVS and SVM high side Reset Release Voltage Level 6"]
    SVSMHRRL_6 = 6,
    #[doc = "7: SVS and SVM high side Reset Release Voltage Level 7"]
    SVSMHRRL_7 = 7,
}
impl From<SVSMHRRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSMHRRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SVSMHRRL` reader - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
pub type SVSMHRRL_R = crate::FieldReader<u8, SVSMHRRL_A>;
impl SVSMHRRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHRRL_A {
        match self.bits {
            0 => SVSMHRRL_A::SVSMHRRL_0,
            1 => SVSMHRRL_A::SVSMHRRL_1,
            2 => SVSMHRRL_A::SVSMHRRL_2,
            3 => SVSMHRRL_A::SVSMHRRL_3,
            4 => SVSMHRRL_A::SVSMHRRL_4,
            5 => SVSMHRRL_A::SVSMHRRL_5,
            6 => SVSMHRRL_A::SVSMHRRL_6,
            7 => SVSMHRRL_A::SVSMHRRL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_0`"]
    #[inline(always)]
    pub fn is_svsmhrrl_0(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_0
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_1`"]
    #[inline(always)]
    pub fn is_svsmhrrl_1(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_1
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_2`"]
    #[inline(always)]
    pub fn is_svsmhrrl_2(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_2
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_3`"]
    #[inline(always)]
    pub fn is_svsmhrrl_3(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_3
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_4`"]
    #[inline(always)]
    pub fn is_svsmhrrl_4(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_4
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_5`"]
    #[inline(always)]
    pub fn is_svsmhrrl_5(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_5
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_6`"]
    #[inline(always)]
    pub fn is_svsmhrrl_6(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_6
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_7`"]
    #[inline(always)]
    pub fn is_svsmhrrl_7(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_7
    }
}
#[doc = "Field `SVSMHRRL` writer - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
pub type SVSMHRRL_W<'a> = crate::FieldWriterSafe<'a, u16, SVSMHCTL_SPEC, u8, SVSMHRRL_A, 3, 0>;
impl<'a> SVSMHRRL_W<'a> {
    #[doc = "SVS and SVM high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmhrrl_0(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_0)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmhrrl_1(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_1)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmhrrl_2(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_2)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmhrrl_3(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_3)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmhrrl_4(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_4)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmhrrl_5(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_5)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmhrrl_6(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_6)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmhrrl_7(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_7)
    }
}
#[doc = "Field `SVSMHDLYST` reader - SVS and SVM high side delay status"]
pub type SVSMHDLYST_R = crate::BitReader<bool>;
#[doc = "Field `SVSMHDLYST` writer - SVS and SVM high side delay status"]
pub type SVSMHDLYST_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 3>;
#[doc = "Field `SVSHMD` reader - SVS high side mode"]
pub type SVSHMD_R = crate::BitReader<bool>;
#[doc = "Field `SVSHMD` writer - SVS high side mode"]
pub type SVSHMD_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 4>;
#[doc = "Field `SVSMHEVM` reader - SVS and SVM high side event mask"]
pub type SVSMHEVM_R = crate::BitReader<bool>;
#[doc = "Field `SVSMHEVM` writer - SVS and SVM high side event mask"]
pub type SVSMHEVM_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 6>;
#[doc = "Field `SVSMHACE` reader - SVS and SVM high side auto control enable"]
pub type SVSMHACE_R = crate::BitReader<bool>;
#[doc = "Field `SVSMHACE` writer - SVS and SVM high side auto control enable"]
pub type SVSMHACE_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 7>;
#[doc = "SVS high side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSHRVL_A {
    #[doc = "0: SVS high side Reset Release Voltage Level 0"]
    SVSHRVL_0 = 0,
    #[doc = "1: SVS high side Reset Release Voltage Level 1"]
    SVSHRVL_1 = 1,
    #[doc = "2: SVS high side Reset Release Voltage Level 2"]
    SVSHRVL_2 = 2,
    #[doc = "3: SVS high side Reset Release Voltage Level 3"]
    SVSHRVL_3 = 3,
}
impl From<SVSHRVL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSHRVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SVSHRVL` reader - SVS high side reset voltage level Bit: 0"]
pub type SVSHRVL_R = crate::FieldReader<u8, SVSHRVL_A>;
impl SVSHRVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHRVL_A {
        match self.bits {
            0 => SVSHRVL_A::SVSHRVL_0,
            1 => SVSHRVL_A::SVSHRVL_1,
            2 => SVSHRVL_A::SVSHRVL_2,
            3 => SVSHRVL_A::SVSHRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_0`"]
    #[inline(always)]
    pub fn is_svshrvl_0(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_0
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_1`"]
    #[inline(always)]
    pub fn is_svshrvl_1(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_1
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_2`"]
    #[inline(always)]
    pub fn is_svshrvl_2(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_2
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_3`"]
    #[inline(always)]
    pub fn is_svshrvl_3(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_3
    }
}
#[doc = "Field `SVSHRVL` writer - SVS high side reset voltage level Bit: 0"]
pub type SVSHRVL_W<'a> = crate::FieldWriterSafe<'a, u16, SVSMHCTL_SPEC, u8, SVSHRVL_A, 2, 8>;
impl<'a> SVSHRVL_W<'a> {
    #[doc = "SVS high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svshrvl_0(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_0)
    }
    #[doc = "SVS high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svshrvl_1(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_1)
    }
    #[doc = "SVS high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svshrvl_2(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_2)
    }
    #[doc = "SVS high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svshrvl_3(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_3)
    }
}
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SVSHE_R = crate::BitReader<bool>;
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SVSHE_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 10>;
#[doc = "Field `SVSHFP` reader - SVS high side full performace mode"]
pub type SVSHFP_R = crate::BitReader<bool>;
#[doc = "Field `SVSHFP` writer - SVS high side full performace mode"]
pub type SVSHFP_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 11>;
#[doc = "Field `SVMHOVPE` reader - SVM high side over-voltage enable"]
pub type SVMHOVPE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHOVPE` writer - SVM high side over-voltage enable"]
pub type SVMHOVPE_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 12>;
#[doc = "Field `SVMHE` reader - SVM high side enable"]
pub type SVMHE_R = crate::BitReader<bool>;
#[doc = "Field `SVMHE` writer - SVM high side enable"]
pub type SVMHE_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 14>;
#[doc = "Field `SVMHFP` reader - SVM high side full performace mode"]
pub type SVMHFP_R = crate::BitReader<bool>;
#[doc = "Field `SVMHFP` writer - SVM high side full performace mode"]
pub type SVMHFP_W<'a> = crate::BitWriter<'a, u16, SVSMHCTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmhrrl(&self) -> SVSMHRRL_R {
        SVSMHRRL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    pub fn svsmhdlyst(&self) -> SVSMHDLYST_R {
        SVSMHDLYST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    pub fn svshmd(&self) -> SVSHMD_R {
        SVSHMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    pub fn svsmhevm(&self) -> SVSMHEVM_R {
        SVSMHEVM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    pub fn svsmhace(&self) -> SVSMHACE_R {
        SVSMHACE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svshrvl(&self) -> SVSHRVL_R {
        SVSHRVL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    pub fn svshfp(&self) -> SVSHFP_R {
        SVSHFP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    pub fn svmhovpe(&self) -> SVMHOVPE_R {
        SVMHOVPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    pub fn svmhe(&self) -> SVMHE_R {
        SVMHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    pub fn svmhfp(&self) -> SVMHFP_R {
        SVMHFP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmhrrl(&mut self) -> SVSMHRRL_W {
        SVSMHRRL_W::new(self)
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    pub fn svsmhdlyst(&mut self) -> SVSMHDLYST_W {
        SVSMHDLYST_W::new(self)
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    pub fn svshmd(&mut self) -> SVSHMD_W {
        SVSHMD_W::new(self)
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    pub fn svsmhevm(&mut self) -> SVSMHEVM_W {
        SVSMHEVM_W::new(self)
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    pub fn svsmhace(&mut self) -> SVSMHACE_W {
        SVSMHACE_W::new(self)
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svshrvl(&mut self) -> SVSHRVL_W {
        SVSHRVL_W::new(self)
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W {
        SVSHE_W::new(self)
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    pub fn svshfp(&mut self) -> SVSHFP_W {
        SVSHFP_W::new(self)
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    pub fn svmhovpe(&mut self) -> SVMHOVPE_W {
        SVMHOVPE_W::new(self)
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    pub fn svmhe(&mut self) -> SVMHE_W {
        SVMHE_W::new(self)
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    pub fn svmhfp(&mut self) -> SVMHFP_W {
        SVMHFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SVS and SVM high side control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svsmhctl](index.html) module"]
pub struct SVSMHCTL_SPEC;
impl crate::RegisterSpec for SVSMHCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [svsmhctl::R](R) reader structure"]
impl crate::Readable for SVSMHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svsmhctl::W](W) writer structure"]
impl crate::Writable for SVSMHCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SVSMHCTL to value 0"]
impl crate::Resettable for SVSMHCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
