#[doc = "Register `SD24BCTL0` reader"]
pub struct R(crate::R<SD24BCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BCTL0` writer"]
pub struct W(crate::W<SD24BCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BCTL0_SPEC>;
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
impl From<crate::W<SD24BCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD24OV32` reader - SD24B Overflow Control"]
pub type SD24OV32_R = crate::BitReader<bool>;
#[doc = "Field `SD24OV32` writer - SD24B Overflow Control"]
pub type SD24OV32_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 1>;
#[doc = "Field `SD24REFS` reader - SD24B Reference Select"]
pub type SD24REFS_R = crate::BitReader<bool>;
#[doc = "Field `SD24REFS` writer - SD24B Reference Select"]
pub type SD24REFS_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 2>;
#[doc = "SD24B Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24SSEL_A {
    #[doc = "0: SD24B Clock Source Select MCLK"]
    SD24SSEL_0 = 0,
    #[doc = "1: SD24B Clock Source Select SMCLK"]
    SD24SSEL_1 = 1,
    #[doc = "2: SD24B Clock Source Select ACLK"]
    SD24SSEL_2 = 2,
    #[doc = "3: SD24B Clock Source Select TACLK"]
    SD24SSEL_3 = 3,
}
impl From<SD24SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24SSEL` reader - SD24B Clock Source Select 0"]
pub type SD24SSEL_R = crate::FieldReader<u8, SD24SSEL_A>;
impl SD24SSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24SSEL_A {
        match self.bits {
            0 => SD24SSEL_A::SD24SSEL_0,
            1 => SD24SSEL_A::SD24SSEL_1,
            2 => SD24SSEL_A::SD24SSEL_2,
            3 => SD24SSEL_A::SD24SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24SSEL_0`"]
    #[inline(always)]
    pub fn is_sd24ssel_0(&self) -> bool {
        *self == SD24SSEL_A::SD24SSEL_0
    }
    #[doc = "Checks if the value of the field is `SD24SSEL_1`"]
    #[inline(always)]
    pub fn is_sd24ssel_1(&self) -> bool {
        *self == SD24SSEL_A::SD24SSEL_1
    }
    #[doc = "Checks if the value of the field is `SD24SSEL_2`"]
    #[inline(always)]
    pub fn is_sd24ssel_2(&self) -> bool {
        *self == SD24SSEL_A::SD24SSEL_2
    }
    #[doc = "Checks if the value of the field is `SD24SSEL_3`"]
    #[inline(always)]
    pub fn is_sd24ssel_3(&self) -> bool {
        *self == SD24SSEL_A::SD24SSEL_3
    }
}
#[doc = "Field `SD24SSEL` writer - SD24B Clock Source Select 0"]
pub type SD24SSEL_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BCTL0_SPEC, u8, SD24SSEL_A, 2, 4>;
impl<'a> SD24SSEL_W<'a> {
    #[doc = "SD24B Clock Source Select MCLK"]
    #[inline(always)]
    pub fn sd24ssel_0(self) -> &'a mut W {
        self.variant(SD24SSEL_A::SD24SSEL_0)
    }
    #[doc = "SD24B Clock Source Select SMCLK"]
    #[inline(always)]
    pub fn sd24ssel_1(self) -> &'a mut W {
        self.variant(SD24SSEL_A::SD24SSEL_1)
    }
    #[doc = "SD24B Clock Source Select ACLK"]
    #[inline(always)]
    pub fn sd24ssel_2(self) -> &'a mut W {
        self.variant(SD24SSEL_A::SD24SSEL_2)
    }
    #[doc = "SD24B Clock Source Select TACLK"]
    #[inline(always)]
    pub fn sd24ssel_3(self) -> &'a mut W {
        self.variant(SD24SSEL_A::SD24SSEL_3)
    }
}
#[doc = "Field `SD24M4` reader - SD24B Modulator clock to Manchester decoder clock ratio"]
pub type SD24M4_R = crate::BitReader<bool>;
#[doc = "Field `SD24M4` writer - SD24B Modulator clock to Manchester decoder clock ratio"]
pub type SD24M4_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 6>;
#[doc = "Field `SD24CLKOS` reader - SD24B Clock Output Select"]
pub type SD24CLKOS_R = crate::BitReader<bool>;
#[doc = "Field `SD24CLKOS` writer - SD24B Clock Output Select"]
pub type SD24CLKOS_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 7>;
#[doc = "SD24B Frequency pre-scaler Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24PDIV_A {
    #[doc = "0: SD24B Frequency pre-scaler /1"]
    SD24PDIV_0 = 0,
    #[doc = "1: SD24B Frequency pre-scaler /2"]
    SD24PDIV_1 = 1,
    #[doc = "2: SD24B Frequency pre-scaler /4"]
    SD24PDIV_2 = 2,
    #[doc = "3: SD24B Frequency pre-scaler /8"]
    SD24PDIV_3 = 3,
    #[doc = "4: SD24B Frequency pre-scaler /16"]
    SD24PDIV_4 = 4,
    #[doc = "5: SD24B Frequency pre-scaler /32"]
    SD24PDIV_5 = 5,
    #[doc = "6: SD24B Frequency pre-scaler /64"]
    SD24PDIV_6 = 6,
    #[doc = "7: SD24B Frequency pre-scaler /128"]
    SD24PDIV_7 = 7,
}
impl From<SD24PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24PDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24PDIV` reader - SD24B Frequency pre-scaler Bit 0"]
pub type SD24PDIV_R = crate::FieldReader<u8, SD24PDIV_A>;
impl SD24PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24PDIV_A {
        match self.bits {
            0 => SD24PDIV_A::SD24PDIV_0,
            1 => SD24PDIV_A::SD24PDIV_1,
            2 => SD24PDIV_A::SD24PDIV_2,
            3 => SD24PDIV_A::SD24PDIV_3,
            4 => SD24PDIV_A::SD24PDIV_4,
            5 => SD24PDIV_A::SD24PDIV_5,
            6 => SD24PDIV_A::SD24PDIV_6,
            7 => SD24PDIV_A::SD24PDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_0`"]
    #[inline(always)]
    pub fn is_sd24pdiv_0(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_0
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_1`"]
    #[inline(always)]
    pub fn is_sd24pdiv_1(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_1
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_2`"]
    #[inline(always)]
    pub fn is_sd24pdiv_2(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_2
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_3`"]
    #[inline(always)]
    pub fn is_sd24pdiv_3(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_3
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_4`"]
    #[inline(always)]
    pub fn is_sd24pdiv_4(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_4
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_5`"]
    #[inline(always)]
    pub fn is_sd24pdiv_5(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_5
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_6`"]
    #[inline(always)]
    pub fn is_sd24pdiv_6(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_6
    }
    #[doc = "Checks if the value of the field is `SD24PDIV_7`"]
    #[inline(always)]
    pub fn is_sd24pdiv_7(&self) -> bool {
        *self == SD24PDIV_A::SD24PDIV_7
    }
}
#[doc = "Field `SD24PDIV` writer - SD24B Frequency pre-scaler Bit 0"]
pub type SD24PDIV_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BCTL0_SPEC, u8, SD24PDIV_A, 3, 8>;
impl<'a> SD24PDIV_W<'a> {
    #[doc = "SD24B Frequency pre-scaler /1"]
    #[inline(always)]
    pub fn sd24pdiv_0(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_0)
    }
    #[doc = "SD24B Frequency pre-scaler /2"]
    #[inline(always)]
    pub fn sd24pdiv_1(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_1)
    }
    #[doc = "SD24B Frequency pre-scaler /4"]
    #[inline(always)]
    pub fn sd24pdiv_2(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_2)
    }
    #[doc = "SD24B Frequency pre-scaler /8"]
    #[inline(always)]
    pub fn sd24pdiv_3(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_3)
    }
    #[doc = "SD24B Frequency pre-scaler /16"]
    #[inline(always)]
    pub fn sd24pdiv_4(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_4)
    }
    #[doc = "SD24B Frequency pre-scaler /32"]
    #[inline(always)]
    pub fn sd24pdiv_5(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_5)
    }
    #[doc = "SD24B Frequency pre-scaler /64"]
    #[inline(always)]
    pub fn sd24pdiv_6(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_6)
    }
    #[doc = "SD24B Frequency pre-scaler /128"]
    #[inline(always)]
    pub fn sd24pdiv_7(self) -> &'a mut W {
        self.variant(SD24PDIV_A::SD24PDIV_7)
    }
}
#[doc = "Field `SD24DIV0` reader - SD24B Frequency Divider Bit 0"]
pub type SD24DIV0_R = crate::BitReader<bool>;
#[doc = "Field `SD24DIV0` writer - SD24B Frequency Divider Bit 0"]
pub type SD24DIV0_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 11>;
#[doc = "Field `SD24DIV1` reader - SD24B Frequency Divider Bit 1"]
pub type SD24DIV1_R = crate::BitReader<bool>;
#[doc = "Field `SD24DIV1` writer - SD24B Frequency Divider Bit 1"]
pub type SD24DIV1_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 12>;
#[doc = "Field `SD24DIV2` reader - SD24B Frequency Divider Bit 2"]
pub type SD24DIV2_R = crate::BitReader<bool>;
#[doc = "Field `SD24DIV2` writer - SD24B Frequency Divider Bit 2"]
pub type SD24DIV2_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 13>;
#[doc = "Field `SD24DIV3` reader - SD24B Frequency Divider Bit 3"]
pub type SD24DIV3_R = crate::BitReader<bool>;
#[doc = "Field `SD24DIV3` writer - SD24B Frequency Divider Bit 3"]
pub type SD24DIV3_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 14>;
#[doc = "Field `SD24DIV4` reader - SD24B Frequency Divider Bit 4"]
pub type SD24DIV4_R = crate::BitReader<bool>;
#[doc = "Field `SD24DIV4` writer - SD24B Frequency Divider Bit 4"]
pub type SD24DIV4_W<'a> = crate::BitWriter<'a, u16, SD24BCTL0_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 1 - SD24B Overflow Control"]
    #[inline(always)]
    pub fn sd24ov32(&self) -> SD24OV32_R {
        SD24OV32_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD24B Reference Select"]
    #[inline(always)]
    pub fn sd24refs(&self) -> SD24REFS_R {
        SD24REFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SD24B Clock Source Select 0"]
    #[inline(always)]
    pub fn sd24ssel(&self) -> SD24SSEL_R {
        SD24SSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SD24B Modulator clock to Manchester decoder clock ratio"]
    #[inline(always)]
    pub fn sd24m4(&self) -> SD24M4_R {
        SD24M4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD24B Clock Output Select"]
    #[inline(always)]
    pub fn sd24clkos(&self) -> SD24CLKOS_R {
        SD24CLKOS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SD24B Frequency pre-scaler Bit 0"]
    #[inline(always)]
    pub fn sd24pdiv(&self) -> SD24PDIV_R {
        SD24PDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - SD24B Frequency Divider Bit 0"]
    #[inline(always)]
    pub fn sd24div0(&self) -> SD24DIV0_R {
        SD24DIV0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SD24B Frequency Divider Bit 1"]
    #[inline(always)]
    pub fn sd24div1(&self) -> SD24DIV1_R {
        SD24DIV1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SD24B Frequency Divider Bit 2"]
    #[inline(always)]
    pub fn sd24div2(&self) -> SD24DIV2_R {
        SD24DIV2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SD24B Frequency Divider Bit 3"]
    #[inline(always)]
    pub fn sd24div3(&self) -> SD24DIV3_R {
        SD24DIV3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SD24B Frequency Divider Bit 4"]
    #[inline(always)]
    pub fn sd24div4(&self) -> SD24DIV4_R {
        SD24DIV4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SD24B Overflow Control"]
    #[inline(always)]
    pub fn sd24ov32(&mut self) -> SD24OV32_W {
        SD24OV32_W::new(self)
    }
    #[doc = "Bit 2 - SD24B Reference Select"]
    #[inline(always)]
    pub fn sd24refs(&mut self) -> SD24REFS_W {
        SD24REFS_W::new(self)
    }
    #[doc = "Bits 4:5 - SD24B Clock Source Select 0"]
    #[inline(always)]
    pub fn sd24ssel(&mut self) -> SD24SSEL_W {
        SD24SSEL_W::new(self)
    }
    #[doc = "Bit 6 - SD24B Modulator clock to Manchester decoder clock ratio"]
    #[inline(always)]
    pub fn sd24m4(&mut self) -> SD24M4_W {
        SD24M4_W::new(self)
    }
    #[doc = "Bit 7 - SD24B Clock Output Select"]
    #[inline(always)]
    pub fn sd24clkos(&mut self) -> SD24CLKOS_W {
        SD24CLKOS_W::new(self)
    }
    #[doc = "Bits 8:10 - SD24B Frequency pre-scaler Bit 0"]
    #[inline(always)]
    pub fn sd24pdiv(&mut self) -> SD24PDIV_W {
        SD24PDIV_W::new(self)
    }
    #[doc = "Bit 11 - SD24B Frequency Divider Bit 0"]
    #[inline(always)]
    pub fn sd24div0(&mut self) -> SD24DIV0_W {
        SD24DIV0_W::new(self)
    }
    #[doc = "Bit 12 - SD24B Frequency Divider Bit 1"]
    #[inline(always)]
    pub fn sd24div1(&mut self) -> SD24DIV1_W {
        SD24DIV1_W::new(self)
    }
    #[doc = "Bit 13 - SD24B Frequency Divider Bit 2"]
    #[inline(always)]
    pub fn sd24div2(&mut self) -> SD24DIV2_W {
        SD24DIV2_W::new(self)
    }
    #[doc = "Bit 14 - SD24B Frequency Divider Bit 3"]
    #[inline(always)]
    pub fn sd24div3(&mut self) -> SD24DIV3_W {
        SD24DIV3_W::new(self)
    }
    #[doc = "Bit 15 - SD24B Frequency Divider Bit 4"]
    #[inline(always)]
    pub fn sd24div4(&mut self) -> SD24DIV4_W {
        SD24DIV4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bctl0](index.html) module"]
pub struct SD24BCTL0_SPEC;
impl crate::RegisterSpec for SD24BCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bctl0::R](R) reader structure"]
impl crate::Readable for SD24BCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bctl0::W](W) writer structure"]
impl crate::Writable for SD24BCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BCTL0 to value 0"]
impl crate::Resettable for SD24BCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
