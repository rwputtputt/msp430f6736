#[doc = "Register `AUX3CHCTL` reader"]
pub struct R(crate::R<AUX3CHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX3CHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX3CHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX3CHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX3CHCTL` writer"]
pub struct W(crate::W<AUX3CHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX3CHCTL_SPEC>;
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
impl From<crate::W<AUX3CHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX3CHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXCHEN` reader - Lock auxiliary supply system."]
pub type AUXCHEN_R = crate::BitReader<bool>;
#[doc = "Field `AUXCHEN` writer - Lock auxiliary supply system."]
pub type AUXCHEN_W<'a> = crate::BitWriter<'a, u16, AUX3CHCTL_SPEC, bool, 0>;
#[doc = "Charger charge current Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXCHC_A {
    #[doc = "0: Charger charge current: 0"]
    AUXCHC_0 = 0,
    #[doc = "1: Charger charge current: 1"]
    AUXCHC_1 = 1,
    #[doc = "2: Charger charge current: 2"]
    AUXCHC_2 = 2,
    #[doc = "3: Charger charge current: 3"]
    AUXCHC_3 = 3,
}
impl From<AUXCHC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCHC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXCHC` reader - Charger charge current Bit: 0"]
pub type AUXCHC_R = crate::FieldReader<u8, AUXCHC_A>;
impl AUXCHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXCHC_A {
        match self.bits {
            0 => AUXCHC_A::AUXCHC_0,
            1 => AUXCHC_A::AUXCHC_1,
            2 => AUXCHC_A::AUXCHC_2,
            3 => AUXCHC_A::AUXCHC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUXCHC_0`"]
    #[inline(always)]
    pub fn is_auxchc_0(&self) -> bool {
        *self == AUXCHC_A::AUXCHC_0
    }
    #[doc = "Checks if the value of the field is `AUXCHC_1`"]
    #[inline(always)]
    pub fn is_auxchc_1(&self) -> bool {
        *self == AUXCHC_A::AUXCHC_1
    }
    #[doc = "Checks if the value of the field is `AUXCHC_2`"]
    #[inline(always)]
    pub fn is_auxchc_2(&self) -> bool {
        *self == AUXCHC_A::AUXCHC_2
    }
    #[doc = "Checks if the value of the field is `AUXCHC_3`"]
    #[inline(always)]
    pub fn is_auxchc_3(&self) -> bool {
        *self == AUXCHC_A::AUXCHC_3
    }
}
#[doc = "Field `AUXCHC` writer - Charger charge current Bit: 0"]
pub type AUXCHC_W<'a> = crate::FieldWriterSafe<'a, u16, AUX3CHCTL_SPEC, u8, AUXCHC_A, 2, 1>;
impl<'a> AUXCHC_W<'a> {
    #[doc = "Charger charge current: 0"]
    #[inline(always)]
    pub fn auxchc_0(self) -> &'a mut W {
        self.variant(AUXCHC_A::AUXCHC_0)
    }
    #[doc = "Charger charge current: 1"]
    #[inline(always)]
    pub fn auxchc_1(self) -> &'a mut W {
        self.variant(AUXCHC_A::AUXCHC_1)
    }
    #[doc = "Charger charge current: 2"]
    #[inline(always)]
    pub fn auxchc_2(self) -> &'a mut W {
        self.variant(AUXCHC_A::AUXCHC_2)
    }
    #[doc = "Charger charge current: 3"]
    #[inline(always)]
    pub fn auxchc_3(self) -> &'a mut W {
        self.variant(AUXCHC_A::AUXCHC_3)
    }
}
#[doc = "Charger end voltage Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXCHV_A {
    #[doc = "0: Charger end voltage: 0"]
    AUXCHV_0 = 0,
    #[doc = "1: Charger end voltage: 1"]
    AUXCHV_1 = 1,
    #[doc = "2: Charger end voltage: 2"]
    AUXCHV_2 = 2,
    #[doc = "3: Charger end voltage: 3"]
    AUXCHV_3 = 3,
}
impl From<AUXCHV_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXCHV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUXCHV` reader - Charger end voltage Bit: 0"]
pub type AUXCHV_R = crate::FieldReader<u8, AUXCHV_A>;
impl AUXCHV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXCHV_A {
        match self.bits {
            0 => AUXCHV_A::AUXCHV_0,
            1 => AUXCHV_A::AUXCHV_1,
            2 => AUXCHV_A::AUXCHV_2,
            3 => AUXCHV_A::AUXCHV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUXCHV_0`"]
    #[inline(always)]
    pub fn is_auxchv_0(&self) -> bool {
        *self == AUXCHV_A::AUXCHV_0
    }
    #[doc = "Checks if the value of the field is `AUXCHV_1`"]
    #[inline(always)]
    pub fn is_auxchv_1(&self) -> bool {
        *self == AUXCHV_A::AUXCHV_1
    }
    #[doc = "Checks if the value of the field is `AUXCHV_2`"]
    #[inline(always)]
    pub fn is_auxchv_2(&self) -> bool {
        *self == AUXCHV_A::AUXCHV_2
    }
    #[doc = "Checks if the value of the field is `AUXCHV_3`"]
    #[inline(always)]
    pub fn is_auxchv_3(&self) -> bool {
        *self == AUXCHV_A::AUXCHV_3
    }
}
#[doc = "Field `AUXCHV` writer - Charger end voltage Bit: 0"]
pub type AUXCHV_W<'a> = crate::FieldWriterSafe<'a, u16, AUX3CHCTL_SPEC, u8, AUXCHV_A, 2, 4>;
impl<'a> AUXCHV_W<'a> {
    #[doc = "Charger end voltage: 0"]
    #[inline(always)]
    pub fn auxchv_0(self) -> &'a mut W {
        self.variant(AUXCHV_A::AUXCHV_0)
    }
    #[doc = "Charger end voltage: 1"]
    #[inline(always)]
    pub fn auxchv_1(self) -> &'a mut W {
        self.variant(AUXCHV_A::AUXCHV_1)
    }
    #[doc = "Charger end voltage: 2"]
    #[inline(always)]
    pub fn auxchv_2(self) -> &'a mut W {
        self.variant(AUXCHV_A::AUXCHV_2)
    }
    #[doc = "Charger end voltage: 3"]
    #[inline(always)]
    pub fn auxchv_3(self) -> &'a mut W {
        self.variant(AUXCHV_A::AUXCHV_3)
    }
}
impl R {
    #[doc = "Bit 0 - Lock auxiliary supply system."]
    #[inline(always)]
    pub fn auxchen(&self) -> AUXCHEN_R {
        AUXCHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Charger charge current Bit: 0"]
    #[inline(always)]
    pub fn auxchc(&self) -> AUXCHC_R {
        AUXCHC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Charger end voltage Bit: 0"]
    #[inline(always)]
    pub fn auxchv(&self) -> AUXCHV_R {
        AUXCHV_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Lock auxiliary supply system."]
    #[inline(always)]
    pub fn auxchen(&mut self) -> AUXCHEN_W {
        AUXCHEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Charger charge current Bit: 0"]
    #[inline(always)]
    pub fn auxchc(&mut self) -> AUXCHC_W {
        AUXCHC_W::new(self)
    }
    #[doc = "Bits 4:5 - Charger end voltage Bit: 0"]
    #[inline(always)]
    pub fn auxchv(&mut self) -> AUXCHV_W {
        AUXCHV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX3 Charger Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux3chctl](index.html) module"]
pub struct AUX3CHCTL_SPEC;
impl crate::RegisterSpec for AUX3CHCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aux3chctl::R](R) reader structure"]
impl crate::Readable for AUX3CHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux3chctl::W](W) writer structure"]
impl crate::Writable for AUX3CHCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX3CHCTL to value 0"]
impl crate::Resettable for AUX3CHCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
