#[doc = "Register `UCSCTL6` reader"]
pub struct R(crate::R<UCSCTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL6` writer"]
pub struct W(crate::W<UCSCTL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL6_SPEC>;
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
impl From<crate::W<UCSCTL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XT1OFF` reader - High Frequency Oscillator 1 (XT1) disable"]
pub type XT1OFF_R = crate::BitReader<bool>;
#[doc = "Field `XT1OFF` writer - High Frequency Oscillator 1 (XT1) disable"]
pub type XT1OFF_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 0>;
#[doc = "Field `SMCLKOFF` reader - SMCLK Off"]
pub type SMCLKOFF_R = crate::BitReader<bool>;
#[doc = "Field `SMCLKOFF` writer - SMCLK Off"]
pub type SMCLKOFF_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 1>;
#[doc = "XIN/XOUT Cap Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XCAP_A {
    #[doc = "0: XIN/XOUT Cap 0"]
    XCAP_0 = 0,
    #[doc = "1: XIN/XOUT Cap 1"]
    XCAP_1 = 1,
    #[doc = "2: XIN/XOUT Cap 2"]
    XCAP_2 = 2,
    #[doc = "3: XIN/XOUT Cap 3"]
    XCAP_3 = 3,
}
impl From<XCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: XCAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XCAP` reader - XIN/XOUT Cap Bit: 0"]
pub type XCAP_R = crate::FieldReader<u8, XCAP_A>;
impl XCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XCAP_A {
        match self.bits {
            0 => XCAP_A::XCAP_0,
            1 => XCAP_A::XCAP_1,
            2 => XCAP_A::XCAP_2,
            3 => XCAP_A::XCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XCAP_0`"]
    #[inline(always)]
    pub fn is_xcap_0(&self) -> bool {
        *self == XCAP_A::XCAP_0
    }
    #[doc = "Checks if the value of the field is `XCAP_1`"]
    #[inline(always)]
    pub fn is_xcap_1(&self) -> bool {
        *self == XCAP_A::XCAP_1
    }
    #[doc = "Checks if the value of the field is `XCAP_2`"]
    #[inline(always)]
    pub fn is_xcap_2(&self) -> bool {
        *self == XCAP_A::XCAP_2
    }
    #[doc = "Checks if the value of the field is `XCAP_3`"]
    #[inline(always)]
    pub fn is_xcap_3(&self) -> bool {
        *self == XCAP_A::XCAP_3
    }
}
#[doc = "Field `XCAP` writer - XIN/XOUT Cap Bit: 0"]
pub type XCAP_W<'a> = crate::FieldWriterSafe<'a, u16, UCSCTL6_SPEC, u8, XCAP_A, 2, 2>;
impl<'a> XCAP_W<'a> {
    #[doc = "XIN/XOUT Cap 0"]
    #[inline(always)]
    pub fn xcap_0(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_0)
    }
    #[doc = "XIN/XOUT Cap 1"]
    #[inline(always)]
    pub fn xcap_1(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_1)
    }
    #[doc = "XIN/XOUT Cap 2"]
    #[inline(always)]
    pub fn xcap_2(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_2)
    }
    #[doc = "XIN/XOUT Cap 3"]
    #[inline(always)]
    pub fn xcap_3(self) -> &'a mut W {
        self.variant(XCAP_A::XCAP_3)
    }
}
#[doc = "Field `XT1BYPASS` reader - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `XT1BYPASS` writer - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 4>;
#[doc = "Field `XTS` reader - 1: Selects high-freq. oscillator"]
pub type XTS_R = crate::BitReader<bool>;
#[doc = "Field `XTS` writer - 1: Selects high-freq. oscillator"]
pub type XTS_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 5>;
#[doc = "XT1 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XT1DRIVE_A {
    #[doc = "0: XT1 Drive Level mode: 0"]
    XT1DRIVE_0 = 0,
    #[doc = "1: XT1 Drive Level mode: 1"]
    XT1DRIVE_1 = 1,
    #[doc = "2: XT1 Drive Level mode: 2"]
    XT1DRIVE_2 = 2,
    #[doc = "3: XT1 Drive Level mode: 3"]
    XT1DRIVE_3 = 3,
}
impl From<XT1DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XT1DRIVE` reader - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_R = crate::FieldReader<u8, XT1DRIVE_A>;
impl XT1DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT1DRIVE_A {
        match self.bits {
            0 => XT1DRIVE_A::XT1DRIVE_0,
            1 => XT1DRIVE_A::XT1DRIVE_1,
            2 => XT1DRIVE_A::XT1DRIVE_2,
            3 => XT1DRIVE_A::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_0`"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_1`"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_2`"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_3`"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_3
    }
}
#[doc = "Field `XT1DRIVE` writer - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_W<'a> = crate::FieldWriterSafe<'a, u16, UCSCTL6_SPEC, u8, XT1DRIVE_A, 2, 6>;
impl<'a> XT1DRIVE_W<'a> {
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut W {
        self.variant(XT1DRIVE_A::XT1DRIVE_3)
    }
}
#[doc = "Field `XT2OFF` reader - High Frequency Oscillator 2 (XT2) disable"]
pub type XT2OFF_R = crate::BitReader<bool>;
#[doc = "Field `XT2OFF` writer - High Frequency Oscillator 2 (XT2) disable"]
pub type XT2OFF_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 8>;
#[doc = "Field `XT2BYPASS` reader - XT2 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT2BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `XT2BYPASS` writer - XT2 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT2BYPASS_W<'a> = crate::BitWriter<'a, u16, UCSCTL6_SPEC, bool, 12>;
#[doc = "XT2 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XT2DRIVE_A {
    #[doc = "0: XT2 Drive Level mode: 0"]
    XT2DRIVE_0 = 0,
    #[doc = "1: XT2 Drive Level mode: 1"]
    XT2DRIVE_1 = 1,
    #[doc = "2: XT2 Drive Level mode: 2"]
    XT2DRIVE_2 = 2,
    #[doc = "3: XT2 Drive Level mode: 3"]
    XT2DRIVE_3 = 3,
}
impl From<XT2DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT2DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XT2DRIVE` reader - XT2 Drive Level mode Bit 0"]
pub type XT2DRIVE_R = crate::FieldReader<u8, XT2DRIVE_A>;
impl XT2DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XT2DRIVE_A {
        match self.bits {
            0 => XT2DRIVE_A::XT2DRIVE_0,
            1 => XT2DRIVE_A::XT2DRIVE_1,
            2 => XT2DRIVE_A::XT2DRIVE_2,
            3 => XT2DRIVE_A::XT2DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_0`"]
    #[inline(always)]
    pub fn is_xt2drive_0(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_1`"]
    #[inline(always)]
    pub fn is_xt2drive_1(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_2`"]
    #[inline(always)]
    pub fn is_xt2drive_2(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT2DRIVE_3`"]
    #[inline(always)]
    pub fn is_xt2drive_3(&self) -> bool {
        *self == XT2DRIVE_A::XT2DRIVE_3
    }
}
#[doc = "Field `XT2DRIVE` writer - XT2 Drive Level mode Bit 0"]
pub type XT2DRIVE_W<'a> = crate::FieldWriterSafe<'a, u16, UCSCTL6_SPEC, u8, XT2DRIVE_A, 2, 14>;
impl<'a> XT2DRIVE_W<'a> {
    #[doc = "XT2 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt2drive_0(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_0)
    }
    #[doc = "XT2 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt2drive_1(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_1)
    }
    #[doc = "XT2 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt2drive_2(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_2)
    }
    #[doc = "XT2 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt2drive_3(self) -> &'a mut W {
        self.variant(XT2DRIVE_A::XT2DRIVE_3)
    }
}
impl R {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    pub fn xt1off(&self) -> XT1OFF_R {
        XT1OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    pub fn xcap(&self) -> XCAP_R {
        XCAP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> XT1BYPASS_R {
        XT1BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&self) -> XT1DRIVE_R {
        XT1DRIVE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt2bypass(&self) -> XT2BYPASS_R {
        XT2BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt2drive(&self) -> XT2DRIVE_R {
        XT2DRIVE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Frequency Oscillator 1 (XT1) disable"]
    #[inline(always)]
    pub fn xt1off(&mut self) -> XT1OFF_W {
        XT1OFF_W::new(self)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W::new(self)
    }
    #[doc = "Bits 2:3 - XIN/XOUT Cap Bit: 0"]
    #[inline(always)]
    pub fn xcap(&mut self) -> XCAP_W {
        XCAP_W::new(self)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&mut self) -> XT1BYPASS_W {
        XT1BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W {
        XTS_W::new(self)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&mut self) -> XT1DRIVE_W {
        XT1DRIVE_W::new(self)
    }
    #[doc = "Bit 8 - High Frequency Oscillator 2 (XT2) disable"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> XT2OFF_W {
        XT2OFF_W::new(self)
    }
    #[doc = "Bit 12 - XT2 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt2bypass(&mut self) -> XT2BYPASS_W {
        XT2BYPASS_W::new(self)
    }
    #[doc = "Bits 14:15 - XT2 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt2drive(&mut self) -> XT2DRIVE_W {
        XT2DRIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl6](index.html) module"]
pub struct UCSCTL6_SPEC;
impl crate::RegisterSpec for UCSCTL6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl6::R](R) reader structure"]
impl crate::Readable for UCSCTL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl6::W](W) writer structure"]
impl crate::Writable for UCSCTL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL6 to value 0"]
impl crate::Resettable for UCSCTL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
