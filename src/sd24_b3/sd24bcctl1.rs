#[doc = "Register `SD24BCCTL1` reader"]
pub struct R(crate::R<SD24BCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BCCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BCCTL1` writer"]
pub struct W(crate::W<SD24BCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BCCTL1_SPEC>;
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
impl From<crate::W<SD24BCCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD24SC` reader - SD24B Start Conversion"]
pub type SD24SC_R = crate::BitReader<bool>;
#[doc = "Field `SD24SC` writer - SD24B Start Conversion"]
pub type SD24SC_W<'a> = crate::BitWriter<'a, u16, SD24BCCTL1_SPEC, bool, 0>;
#[doc = "SD24B Start Conversion Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24SCS_A {
    #[doc = "0: SD24B Start Conversion Select: 0"]
    SD24SCS_0 = 0,
    #[doc = "1: SD24B Start Conversion Select: 1"]
    SD24SCS_1 = 1,
    #[doc = "2: SD24B Start Conversion Select: 2"]
    SD24SCS_2 = 2,
    #[doc = "3: SD24B Start Conversion Select: 3"]
    SD24SCS_3 = 3,
    #[doc = "4: SD24B Start Conversion Select: 4"]
    SD24SCS_4 = 4,
    #[doc = "5: SD24B Start Conversion Select: 5"]
    SD24SCS_5 = 5,
    #[doc = "6: SD24B Start Conversion Select: 6"]
    SD24SCS_6 = 6,
    #[doc = "7: SD24B Start Conversion Select: 7"]
    SD24SCS_7 = 7,
}
impl From<SD24SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24SCS` reader - SD24B Start Conversion Select Bit 0"]
pub type SD24SCS_R = crate::FieldReader<u8, SD24SCS_A>;
impl SD24SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24SCS_A {
        match self.bits {
            0 => SD24SCS_A::SD24SCS_0,
            1 => SD24SCS_A::SD24SCS_1,
            2 => SD24SCS_A::SD24SCS_2,
            3 => SD24SCS_A::SD24SCS_3,
            4 => SD24SCS_A::SD24SCS_4,
            5 => SD24SCS_A::SD24SCS_5,
            6 => SD24SCS_A::SD24SCS_6,
            7 => SD24SCS_A::SD24SCS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24SCS_0`"]
    #[inline(always)]
    pub fn is_sd24scs_0(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_0
    }
    #[doc = "Checks if the value of the field is `SD24SCS_1`"]
    #[inline(always)]
    pub fn is_sd24scs_1(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_1
    }
    #[doc = "Checks if the value of the field is `SD24SCS_2`"]
    #[inline(always)]
    pub fn is_sd24scs_2(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_2
    }
    #[doc = "Checks if the value of the field is `SD24SCS_3`"]
    #[inline(always)]
    pub fn is_sd24scs_3(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_3
    }
    #[doc = "Checks if the value of the field is `SD24SCS_4`"]
    #[inline(always)]
    pub fn is_sd24scs_4(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_4
    }
    #[doc = "Checks if the value of the field is `SD24SCS_5`"]
    #[inline(always)]
    pub fn is_sd24scs_5(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_5
    }
    #[doc = "Checks if the value of the field is `SD24SCS_6`"]
    #[inline(always)]
    pub fn is_sd24scs_6(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_6
    }
    #[doc = "Checks if the value of the field is `SD24SCS_7`"]
    #[inline(always)]
    pub fn is_sd24scs_7(&self) -> bool {
        *self == SD24SCS_A::SD24SCS_7
    }
}
#[doc = "Field `SD24SCS` writer - SD24B Start Conversion Select Bit 0"]
pub type SD24SCS_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BCCTL1_SPEC, u8, SD24SCS_A, 3, 1>;
impl<'a> SD24SCS_W<'a> {
    #[doc = "SD24B Start Conversion Select: 0"]
    #[inline(always)]
    pub fn sd24scs_0(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_0)
    }
    #[doc = "SD24B Start Conversion Select: 1"]
    #[inline(always)]
    pub fn sd24scs_1(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_1)
    }
    #[doc = "SD24B Start Conversion Select: 2"]
    #[inline(always)]
    pub fn sd24scs_2(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_2)
    }
    #[doc = "SD24B Start Conversion Select: 3"]
    #[inline(always)]
    pub fn sd24scs_3(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_3)
    }
    #[doc = "SD24B Start Conversion Select: 4"]
    #[inline(always)]
    pub fn sd24scs_4(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_4)
    }
    #[doc = "SD24B Start Conversion Select: 5"]
    #[inline(always)]
    pub fn sd24scs_5(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_5)
    }
    #[doc = "SD24B Start Conversion Select: 6"]
    #[inline(always)]
    pub fn sd24scs_6(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_6)
    }
    #[doc = "SD24B Start Conversion Select: 7"]
    #[inline(always)]
    pub fn sd24scs_7(self) -> &'a mut W {
        self.variant(SD24SCS_A::SD24SCS_7)
    }
}
#[doc = "SD24B Data Format Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24DF_A {
    #[doc = "0: SD24B Data Format: Offset Binary"]
    SD24DF_0 = 0,
    #[doc = "1: SD24B Data Format: 2's complement"]
    SD24DF_1 = 1,
}
impl From<SD24DF_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24DF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24DF` reader - SD24B Data Format Bit: 0"]
pub type SD24DF_R = crate::FieldReader<u8, SD24DF_A>;
impl SD24DF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SD24DF_A> {
        match self.bits {
            0 => Some(SD24DF_A::SD24DF_0),
            1 => Some(SD24DF_A::SD24DF_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD24DF_0`"]
    #[inline(always)]
    pub fn is_sd24df_0(&self) -> bool {
        *self == SD24DF_A::SD24DF_0
    }
    #[doc = "Checks if the value of the field is `SD24DF_1`"]
    #[inline(always)]
    pub fn is_sd24df_1(&self) -> bool {
        *self == SD24DF_A::SD24DF_1
    }
}
#[doc = "Field `SD24DF` writer - SD24B Data Format Bit: 0"]
pub type SD24DF_W<'a> = crate::FieldWriter<'a, u16, SD24BCCTL1_SPEC, u8, SD24DF_A, 2, 4>;
impl<'a> SD24DF_W<'a> {
    #[doc = "SD24B Data Format: Offset Binary"]
    #[inline(always)]
    pub fn sd24df_0(self) -> &'a mut W {
        self.variant(SD24DF_A::SD24DF_0)
    }
    #[doc = "SD24B Data Format: 2's complement"]
    #[inline(always)]
    pub fn sd24df_1(self) -> &'a mut W {
        self.variant(SD24DF_A::SD24DF_1)
    }
}
#[doc = "Field `SD24ALGN` reader - SD24B Data Alignment"]
pub type SD24ALGN_R = crate::BitReader<bool>;
#[doc = "Field `SD24ALGN` writer - SD24B Data Alignment"]
pub type SD24ALGN_W<'a> = crate::BitWriter<'a, u16, SD24BCCTL1_SPEC, bool, 6>;
#[doc = "Field `SD24SNGL` reader - SD24B Single Trigger Mode"]
pub type SD24SNGL_R = crate::BitReader<bool>;
#[doc = "Field `SD24SNGL` writer - SD24B Single Trigger Mode"]
pub type SD24SNGL_W<'a> = crate::BitWriter<'a, u16, SD24BCCTL1_SPEC, bool, 8>;
#[doc = "Field `SD24CAL` reader - SD24B Calibration"]
pub type SD24CAL_R = crate::BitReader<bool>;
#[doc = "Field `SD24CAL` writer - SD24B Calibration"]
pub type SD24CAL_W<'a> = crate::BitWriter<'a, u16, SD24BCCTL1_SPEC, bool, 9>;
#[doc = "SD24B Digital Filter Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24DFS_A {
    #[doc = "0: SD24B Digital Filter 0"]
    SD24DFS_0 = 0,
    #[doc = "1: SD24B Digital Filter 1"]
    SD24DFS_1 = 1,
    #[doc = "2: SD24B Digital Filter 2"]
    SD24DFS_2 = 2,
    #[doc = "3: SD24B Digital Filter 3"]
    SD24DFS_3 = 3,
}
impl From<SD24DFS_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24DFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24DFS` reader - SD24B Digital Filter Bit: 0"]
pub type SD24DFS_R = crate::FieldReader<u8, SD24DFS_A>;
impl SD24DFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24DFS_A {
        match self.bits {
            0 => SD24DFS_A::SD24DFS_0,
            1 => SD24DFS_A::SD24DFS_1,
            2 => SD24DFS_A::SD24DFS_2,
            3 => SD24DFS_A::SD24DFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24DFS_0`"]
    #[inline(always)]
    pub fn is_sd24dfs_0(&self) -> bool {
        *self == SD24DFS_A::SD24DFS_0
    }
    #[doc = "Checks if the value of the field is `SD24DFS_1`"]
    #[inline(always)]
    pub fn is_sd24dfs_1(&self) -> bool {
        *self == SD24DFS_A::SD24DFS_1
    }
    #[doc = "Checks if the value of the field is `SD24DFS_2`"]
    #[inline(always)]
    pub fn is_sd24dfs_2(&self) -> bool {
        *self == SD24DFS_A::SD24DFS_2
    }
    #[doc = "Checks if the value of the field is `SD24DFS_3`"]
    #[inline(always)]
    pub fn is_sd24dfs_3(&self) -> bool {
        *self == SD24DFS_A::SD24DFS_3
    }
}
#[doc = "Field `SD24DFS` writer - SD24B Digital Filter Bit: 0"]
pub type SD24DFS_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BCCTL1_SPEC, u8, SD24DFS_A, 2, 10>;
impl<'a> SD24DFS_W<'a> {
    #[doc = "SD24B Digital Filter 0"]
    #[inline(always)]
    pub fn sd24dfs_0(self) -> &'a mut W {
        self.variant(SD24DFS_A::SD24DFS_0)
    }
    #[doc = "SD24B Digital Filter 1"]
    #[inline(always)]
    pub fn sd24dfs_1(self) -> &'a mut W {
        self.variant(SD24DFS_A::SD24DFS_1)
    }
    #[doc = "SD24B Digital Filter 2"]
    #[inline(always)]
    pub fn sd24dfs_2(self) -> &'a mut W {
        self.variant(SD24DFS_A::SD24DFS_2)
    }
    #[doc = "SD24B Digital Filter 3"]
    #[inline(always)]
    pub fn sd24dfs_3(self) -> &'a mut W {
        self.variant(SD24DFS_A::SD24DFS_3)
    }
}
#[doc = "Field `SD24DI` reader - SD24B Digital Bitstream Input"]
pub type SD24DI_R = crate::BitReader<bool>;
#[doc = "Field `SD24DI` writer - SD24B Digital Bitstream Input"]
pub type SD24DI_W<'a> = crate::BitWriter<'a, u16, SD24BCCTL1_SPEC, bool, 12>;
#[doc = "SD24B Manchaster Encoding Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24MC_A {
    #[doc = "0: SD24B Manchaster Encoding 0"]
    SD24MC_0 = 0,
    #[doc = "1: SD24B Manchaster Encoding 1"]
    SD24MC_1 = 1,
    #[doc = "2: SD24B Manchaster Encoding 2"]
    SD24MC_2 = 2,
    #[doc = "3: SD24B Manchaster Encoding 3"]
    SD24MC_3 = 3,
}
impl From<SD24MC_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24MC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24MC` reader - SD24B Manchaster Encoding Bit: 0"]
pub type SD24MC_R = crate::FieldReader<u8, SD24MC_A>;
impl SD24MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD24MC_A {
        match self.bits {
            0 => SD24MC_A::SD24MC_0,
            1 => SD24MC_A::SD24MC_1,
            2 => SD24MC_A::SD24MC_2,
            3 => SD24MC_A::SD24MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD24MC_0`"]
    #[inline(always)]
    pub fn is_sd24mc_0(&self) -> bool {
        *self == SD24MC_A::SD24MC_0
    }
    #[doc = "Checks if the value of the field is `SD24MC_1`"]
    #[inline(always)]
    pub fn is_sd24mc_1(&self) -> bool {
        *self == SD24MC_A::SD24MC_1
    }
    #[doc = "Checks if the value of the field is `SD24MC_2`"]
    #[inline(always)]
    pub fn is_sd24mc_2(&self) -> bool {
        *self == SD24MC_A::SD24MC_2
    }
    #[doc = "Checks if the value of the field is `SD24MC_3`"]
    #[inline(always)]
    pub fn is_sd24mc_3(&self) -> bool {
        *self == SD24MC_A::SD24MC_3
    }
}
#[doc = "Field `SD24MC` writer - SD24B Manchaster Encoding Bit: 0"]
pub type SD24MC_W<'a> = crate::FieldWriterSafe<'a, u16, SD24BCCTL1_SPEC, u8, SD24MC_A, 2, 13>;
impl<'a> SD24MC_W<'a> {
    #[doc = "SD24B Manchaster Encoding 0"]
    #[inline(always)]
    pub fn sd24mc_0(self) -> &'a mut W {
        self.variant(SD24MC_A::SD24MC_0)
    }
    #[doc = "SD24B Manchaster Encoding 1"]
    #[inline(always)]
    pub fn sd24mc_1(self) -> &'a mut W {
        self.variant(SD24MC_A::SD24MC_1)
    }
    #[doc = "SD24B Manchaster Encoding 2"]
    #[inline(always)]
    pub fn sd24mc_2(self) -> &'a mut W {
        self.variant(SD24MC_A::SD24MC_2)
    }
    #[doc = "SD24B Manchaster Encoding 3"]
    #[inline(always)]
    pub fn sd24mc_3(self) -> &'a mut W {
        self.variant(SD24MC_A::SD24MC_3)
    }
}
impl R {
    #[doc = "Bit 0 - SD24B Start Conversion"]
    #[inline(always)]
    pub fn sd24sc(&self) -> SD24SC_R {
        SD24SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD24B Start Conversion Select Bit 0"]
    #[inline(always)]
    pub fn sd24scs(&self) -> SD24SCS_R {
        SD24SCS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - SD24B Data Format Bit: 0"]
    #[inline(always)]
    pub fn sd24df(&self) -> SD24DF_R {
        SD24DF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SD24B Data Alignment"]
    #[inline(always)]
    pub fn sd24algn(&self) -> SD24ALGN_R {
        SD24ALGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SD24B Single Trigger Mode"]
    #[inline(always)]
    pub fn sd24sngl(&self) -> SD24SNGL_R {
        SD24SNGL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD24B Calibration"]
    #[inline(always)]
    pub fn sd24cal(&self) -> SD24CAL_R {
        SD24CAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SD24B Digital Filter Bit: 0"]
    #[inline(always)]
    pub fn sd24dfs(&self) -> SD24DFS_R {
        SD24DFS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SD24B Digital Bitstream Input"]
    #[inline(always)]
    pub fn sd24di(&self) -> SD24DI_R {
        SD24DI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - SD24B Manchaster Encoding Bit: 0"]
    #[inline(always)]
    pub fn sd24mc(&self) -> SD24MC_R {
        SD24MC_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SD24B Start Conversion"]
    #[inline(always)]
    pub fn sd24sc(&mut self) -> SD24SC_W {
        SD24SC_W::new(self)
    }
    #[doc = "Bits 1:3 - SD24B Start Conversion Select Bit 0"]
    #[inline(always)]
    pub fn sd24scs(&mut self) -> SD24SCS_W {
        SD24SCS_W::new(self)
    }
    #[doc = "Bits 4:5 - SD24B Data Format Bit: 0"]
    #[inline(always)]
    pub fn sd24df(&mut self) -> SD24DF_W {
        SD24DF_W::new(self)
    }
    #[doc = "Bit 6 - SD24B Data Alignment"]
    #[inline(always)]
    pub fn sd24algn(&mut self) -> SD24ALGN_W {
        SD24ALGN_W::new(self)
    }
    #[doc = "Bit 8 - SD24B Single Trigger Mode"]
    #[inline(always)]
    pub fn sd24sngl(&mut self) -> SD24SNGL_W {
        SD24SNGL_W::new(self)
    }
    #[doc = "Bit 9 - SD24B Calibration"]
    #[inline(always)]
    pub fn sd24cal(&mut self) -> SD24CAL_W {
        SD24CAL_W::new(self)
    }
    #[doc = "Bits 10:11 - SD24B Digital Filter Bit: 0"]
    #[inline(always)]
    pub fn sd24dfs(&mut self) -> SD24DFS_W {
        SD24DFS_W::new(self)
    }
    #[doc = "Bit 12 - SD24B Digital Bitstream Input"]
    #[inline(always)]
    pub fn sd24di(&mut self) -> SD24DI_W {
        SD24DI_W::new(self)
    }
    #[doc = "Bits 13:14 - SD24B Manchaster Encoding Bit: 0"]
    #[inline(always)]
    pub fn sd24mc(&mut self) -> SD24MC_W {
        SD24MC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Channel 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bcctl1](index.html) module"]
pub struct SD24BCCTL1_SPEC;
impl crate::RegisterSpec for SD24BCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bcctl1::R](R) reader structure"]
impl crate::Readable for SD24BCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bcctl1::W](W) writer structure"]
impl crate::Writable for SD24BCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BCCTL1 to value 0"]
impl crate::Resettable for SD24BCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
