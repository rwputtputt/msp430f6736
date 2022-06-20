#[doc = "Register `SD24BINCTL0` reader"]
pub struct R(crate::R<SD24BINCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BINCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BINCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BINCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BINCTL0` writer"]
pub struct W(crate::W<SD24BINCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BINCTL0_SPEC>;
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
impl From<crate::W<SD24BINCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BINCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SD24B Input Pre-Amplifier Gain Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24GAIN_A {
    #[doc = "0: SD24B Input Pre-Amplifier Gain Select *0"]
    SD24GAIN_1 = 0,
    #[doc = "1: SD24B Input Pre-Amplifier Gain Select *1"]
    SD24GAIN_2 = 1,
    #[doc = "2: SD24B Input Pre-Amplifier Gain Select *2"]
    SD24GAIN_4 = 2,
    #[doc = "3: SD24B Input Pre-Amplifier Gain Select *3"]
    SD24GAIN_8 = 3,
    #[doc = "4: SD24B Input Pre-Amplifier Gain Select *4"]
    SD24GAIN_16 = 4,
    #[doc = "5: SD24B Input Pre-Amplifier Gain Select *5"]
    SD24GAIN_32 = 5,
    #[doc = "6: SD24B Input Pre-Amplifier Gain Select *6"]
    SD24GAIN_64 = 6,
    #[doc = "7: SD24B Input Pre-Amplifier Gain Select *7"]
    SD24GAIN_128 = 7,
}
impl From<SD24GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24GAIN` reader - SD24B Input Pre-Amplifier Gain Select 0"]
pub type SD24GAIN_R = crate::FieldReader<u8, SD24GAIN_A>;
impl SD24GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24GAIN_A {
        match self.bits {
            0 => SD24GAIN_A::SD24GAIN_1,
            1 => SD24GAIN_A::SD24GAIN_2,
            2 => SD24GAIN_A::SD24GAIN_4,
            3 => SD24GAIN_A::SD24GAIN_8,
            4 => SD24GAIN_A::SD24GAIN_16,
            5 => SD24GAIN_A::SD24GAIN_32,
            6 => SD24GAIN_A::SD24GAIN_64,
            7 => SD24GAIN_A::SD24GAIN_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_1`"]
    #[inline(always)]
    pub fn is_sd24gain_1(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_1
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_2`"]
    #[inline(always)]
    pub fn is_sd24gain_2(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_2
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_4`"]
    #[inline(always)]
    pub fn is_sd24gain_4(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_4
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_8`"]
    #[inline(always)]
    pub fn is_sd24gain_8(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_8
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_16`"]
    #[inline(always)]
    pub fn is_sd24gain_16(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_16
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_32`"]
    #[inline(always)]
    pub fn is_sd24gain_32(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_32
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_64`"]
    #[inline(always)]
    pub fn is_sd24gain_64(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_64
    }
    #[doc = "Checks if the value of the field is `SD24GAIN_128`"]
    #[inline(always)]
    pub fn is_sd24gain_128(&self) -> bool {
        *self == SD24GAIN_A::SD24GAIN_128
    }
}
#[doc = "Field `SD24GAIN` writer - SD24B Input Pre-Amplifier Gain Select 0"]
pub type SD24GAIN_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BINCTL0_SPEC, u8, SD24GAIN_A, 3, 3>;
impl<'a> SD24GAIN_W<'a> {
    #[doc = "SD24B Input Pre-Amplifier Gain Select *0"]
    #[inline(always)]
    pub fn sd24gain_1(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_1)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *1"]
    #[inline(always)]
    pub fn sd24gain_2(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_2)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *2"]
    #[inline(always)]
    pub fn sd24gain_4(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_4)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *3"]
    #[inline(always)]
    pub fn sd24gain_8(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_8)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *4"]
    #[inline(always)]
    pub fn sd24gain_16(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_16)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *5"]
    #[inline(always)]
    pub fn sd24gain_32(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_32)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *6"]
    #[inline(always)]
    pub fn sd24gain_64(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_64)
    }
    #[doc = "SD24B Input Pre-Amplifier Gain Select *7"]
    #[inline(always)]
    pub fn sd24gain_128(self) -> &'a mut W {
        self.variant(SD24GAIN_A::SD24GAIN_128)
    }
}
#[doc = "SD24B Interrupt Delay after 1.Conversion 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24INTDLY_A {
    #[doc = "0: SD24B Interrupt Delay: Int. after 4.Conversion"]
    SD24INTDLY_0 = 0,
    #[doc = "1: SD24B Interrupt Delay: Int. after 3.Conversion"]
    SD24INTDLY_1 = 1,
    #[doc = "2: SD24B Interrupt Delay: Int. after 2.Conversion"]
    SD24INTDLY_2 = 2,
    #[doc = "3: SD24B Interrupt Delay: Int. after 1.Conversion"]
    SD24INTDLY_3 = 3,
}
impl From<SD24INTDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24INTDLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24INTDLY` reader - SD24B Interrupt Delay after 1.Conversion 0"]
pub type SD24INTDLY_R = crate::FieldReader<u8, SD24INTDLY_A>;
impl SD24INTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24INTDLY_A {
        match self.bits {
            0 => SD24INTDLY_A::SD24INTDLY_0,
            1 => SD24INTDLY_A::SD24INTDLY_1,
            2 => SD24INTDLY_A::SD24INTDLY_2,
            3 => SD24INTDLY_A::SD24INTDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24INTDLY_0`"]
    #[inline(always)]
    pub fn is_sd24intdly_0(&self) -> bool {
        *self == SD24INTDLY_A::SD24INTDLY_0
    }
    #[doc = "Checks if the value of the field is `SD24INTDLY_1`"]
    #[inline(always)]
    pub fn is_sd24intdly_1(&self) -> bool {
        *self == SD24INTDLY_A::SD24INTDLY_1
    }
    #[doc = "Checks if the value of the field is `SD24INTDLY_2`"]
    #[inline(always)]
    pub fn is_sd24intdly_2(&self) -> bool {
        *self == SD24INTDLY_A::SD24INTDLY_2
    }
    #[doc = "Checks if the value of the field is `SD24INTDLY_3`"]
    #[inline(always)]
    pub fn is_sd24intdly_3(&self) -> bool {
        *self == SD24INTDLY_A::SD24INTDLY_3
    }
}
#[doc = "Field `SD24INTDLY` writer - SD24B Interrupt Delay after 1.Conversion 0"]
pub type SD24INTDLY_W<'a> =
    crate::FieldWriterSafe<'a, u16, SD24BINCTL0_SPEC, u8, SD24INTDLY_A, 2, 6>;
impl<'a> SD24INTDLY_W<'a> {
    #[doc = "SD24B Interrupt Delay: Int. after 4.Conversion"]
    #[inline(always)]
    pub fn sd24intdly_0(self) -> &'a mut W {
        self.variant(SD24INTDLY_A::SD24INTDLY_0)
    }
    #[doc = "SD24B Interrupt Delay: Int. after 3.Conversion"]
    #[inline(always)]
    pub fn sd24intdly_1(self) -> &'a mut W {
        self.variant(SD24INTDLY_A::SD24INTDLY_1)
    }
    #[doc = "SD24B Interrupt Delay: Int. after 2.Conversion"]
    #[inline(always)]
    pub fn sd24intdly_2(self) -> &'a mut W {
        self.variant(SD24INTDLY_A::SD24INTDLY_2)
    }
    #[doc = "SD24B Interrupt Delay: Int. after 1.Conversion"]
    #[inline(always)]
    pub fn sd24intdly_3(self) -> &'a mut W {
        self.variant(SD24INTDLY_A::SD24INTDLY_3)
    }
}
impl R {
    #[doc = "Bits 3:5 - SD24B Input Pre-Amplifier Gain Select 0"]
    #[inline(always)]
    pub fn sd24gain(&self) -> SD24GAIN_R {
        SD24GAIN_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - SD24B Interrupt Delay after 1.Conversion 0"]
    #[inline(always)]
    pub fn sd24intdly(&self) -> SD24INTDLY_R {
        SD24INTDLY_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - SD24B Input Pre-Amplifier Gain Select 0"]
    #[inline(always)]
    pub fn sd24gain(&mut self) -> SD24GAIN_W {
        SD24GAIN_W::new(self)
    }
    #[doc = "Bits 6:7 - SD24B Interrupt Delay after 1.Conversion 0"]
    #[inline(always)]
    pub fn sd24intdly(&mut self) -> SD24INTDLY_W {
        SD24INTDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Channel 0 Input Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24binctl0](index.html) module"]
pub struct SD24BINCTL0_SPEC;
impl crate::RegisterSpec for SD24BINCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24binctl0::R](R) reader structure"]
impl crate::Readable for SD24BINCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24binctl0::W](W) writer structure"]
impl crate::Writable for SD24BINCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BINCTL0 to value 0"]
impl crate::Resettable for SD24BINCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
