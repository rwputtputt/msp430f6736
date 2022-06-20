#[doc = "Register `MPY32CTL0` reader"]
pub struct R(crate::R<MPY32CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPY32CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPY32CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPY32CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPY32CTL0` writer"]
pub struct W(crate::W<MPY32CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPY32CTL0_SPEC>;
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
impl From<crate::W<MPY32CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPY32CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPYC` reader - Carry of the multiplier"]
pub type MPYC_R = crate::BitReader<bool>;
#[doc = "Field `MPYC` writer - Carry of the multiplier"]
pub type MPYC_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 0>;
#[doc = "Field `MPYFRAC` reader - Fractional mode"]
pub type MPYFRAC_R = crate::BitReader<bool>;
#[doc = "Field `MPYFRAC` writer - Fractional mode"]
pub type MPYFRAC_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 2>;
#[doc = "Field `MPYSAT` reader - Saturation mode"]
pub type MPYSAT_R = crate::BitReader<bool>;
#[doc = "Field `MPYSAT` writer - Saturation mode"]
pub type MPYSAT_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 3>;
#[doc = "Multiplier mode Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPYM_A {
    #[doc = "0: Multiplier mode: MPY"]
    MPYM_0 = 0,
    #[doc = "1: Multiplier mode: MPYS"]
    MPYM_1 = 1,
    #[doc = "2: Multiplier mode: MAC"]
    MPYM_2 = 2,
    #[doc = "3: Multiplier mode: MACS"]
    MPYM_3 = 3,
}
impl From<MPYM_A> for u8 {
    #[inline(always)]
    fn from(variant: MPYM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MPYM` reader - Multiplier mode Bit:0"]
pub type MPYM_R = crate::FieldReader<u8, MPYM_A>;
impl MPYM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPYM_A {
        match self.bits {
            0 => MPYM_A::MPYM_0,
            1 => MPYM_A::MPYM_1,
            2 => MPYM_A::MPYM_2,
            3 => MPYM_A::MPYM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPYM_0`"]
    #[inline(always)]
    pub fn is_mpym_0(&self) -> bool {
        *self == MPYM_A::MPYM_0
    }
    #[doc = "Checks if the value of the field is `MPYM_1`"]
    #[inline(always)]
    pub fn is_mpym_1(&self) -> bool {
        *self == MPYM_A::MPYM_1
    }
    #[doc = "Checks if the value of the field is `MPYM_2`"]
    #[inline(always)]
    pub fn is_mpym_2(&self) -> bool {
        *self == MPYM_A::MPYM_2
    }
    #[doc = "Checks if the value of the field is `MPYM_3`"]
    #[inline(always)]
    pub fn is_mpym_3(&self) -> bool {
        *self == MPYM_A::MPYM_3
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode Bit:0"]
pub type MPYM_W<'a> = crate::FieldWriterSafe<'a, u16, MPY32CTL0_SPEC, u8, MPYM_A, 2, 4>;
impl<'a> MPYM_W<'a> {
    #[doc = "Multiplier mode: MPY"]
    #[inline(always)]
    pub fn mpym_0(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_0)
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline(always)]
    pub fn mpym_1(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_1)
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline(always)]
    pub fn mpym_2(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_2)
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline(always)]
    pub fn mpym_3(self) -> &'a mut W {
        self.variant(MPYM_A::MPYM_3)
    }
}
#[doc = "Field `OP1_32` reader - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type OP1_32_R = crate::BitReader<bool>;
#[doc = "Field `OP1_32` writer - Bit-width of operand 1 0:16Bit / 1:32Bit"]
pub type OP1_32_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 6>;
#[doc = "Field `OP2_32` reader - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type OP2_32_R = crate::BitReader<bool>;
#[doc = "Field `OP2_32` writer - Bit-width of operand 2 0:16Bit / 1:32Bit"]
pub type OP2_32_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 7>;
#[doc = "Field `MPYDLYWRTEN` reader - Delayed write enable"]
pub type MPYDLYWRTEN_R = crate::BitReader<bool>;
#[doc = "Field `MPYDLYWRTEN` writer - Delayed write enable"]
pub type MPYDLYWRTEN_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 8>;
#[doc = "Field `MPYDLY32` reader - Delayed write mode"]
pub type MPYDLY32_R = crate::BitReader<bool>;
#[doc = "Field `MPYDLY32` writer - Delayed write mode"]
pub type MPYDLY32_W<'a> = crate::BitWriter<'a, u16, MPY32CTL0_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MPYC_R {
        MPYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MPYFRAC_R {
        MPYFRAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MPYSAT_R {
        MPYSAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&self) -> MPYM_R {
        MPYM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&self) -> OP1_32_R {
        OP1_32_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&self) -> OP2_32_R {
        OP2_32_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MPYDLYWRTEN_R {
        MPYDLYWRTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&self) -> MPYDLY32_R {
        MPYDLY32_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MPYC_W {
        MPYC_W::new(self)
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MPYFRAC_W {
        MPYFRAC_W::new(self)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MPYSAT_W {
        MPYSAT_W::new(self)
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MPYM_W {
        MPYM_W::new(self)
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op1_32(&mut self) -> OP1_32_W {
        OP1_32_W::new(self)
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline(always)]
    pub fn op2_32(&mut self) -> OP2_32_W {
        OP2_32_W::new(self)
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MPYDLYWRTEN_W {
        MPYDLYWRTEN_W::new(self)
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> MPYDLY32_W {
        MPYDLY32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPY32 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32ctl0](index.html) module"]
pub struct MPY32CTL0_SPEC;
impl crate::RegisterSpec for MPY32CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpy32ctl0::R](R) reader structure"]
impl crate::Readable for MPY32CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpy32ctl0::W](W) writer structure"]
impl crate::Writable for MPY32CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPY32CTL0 to value 0"]
impl crate::Resettable for MPY32CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
