#[doc = "Register `SD24BCTL1` reader"]
pub struct R(crate::R<SD24BCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BCTL1` writer"]
pub struct W(crate::W<SD24BCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BCTL1_SPEC>;
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
impl From<crate::W<SD24BCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD24GRP0SC` reader - SD24B Group 0 Start Conversion"]
pub type SD24GRP0SC_R = crate::BitReader<bool>;
#[doc = "Field `SD24GRP0SC` writer - SD24B Group 0 Start Conversion"]
pub type SD24GRP0SC_W<'a> = crate::BitWriter<'a, u16, SD24BCTL1_SPEC, bool, 0>;
#[doc = "Field `SD24GRP1SC` reader - SD24B Group 1 Start Conversion"]
pub type SD24GRP1SC_R = crate::BitReader<bool>;
#[doc = "Field `SD24GRP1SC` writer - SD24B Group 1 Start Conversion"]
pub type SD24GRP1SC_W<'a> = crate::BitWriter<'a, u16, SD24BCTL1_SPEC, bool, 1>;
#[doc = "Field `SD24GRP2SC` reader - SD24B Group 2 Start Conversion"]
pub type SD24GRP2SC_R = crate::BitReader<bool>;
#[doc = "Field `SD24GRP2SC` writer - SD24B Group 2 Start Conversion"]
pub type SD24GRP2SC_W<'a> = crate::BitWriter<'a, u16, SD24BCTL1_SPEC, bool, 2>;
#[doc = "Field `SD24GRP3SC` reader - SD24B Group 3 Start Conversion"]
pub type SD24GRP3SC_R = crate::BitReader<bool>;
#[doc = "Field `SD24GRP3SC` writer - SD24B Group 3 Start Conversion"]
pub type SD24GRP3SC_W<'a> = crate::BitWriter<'a, u16, SD24BCTL1_SPEC, bool, 3>;
#[doc = "SD24B DMA Trigger Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD24DMA_A {
    #[doc = "0: SD24B DMA Trigger: 0"]
    SD24DMA_0 = 0,
    #[doc = "1: SD24B DMA Trigger: 1"]
    SD24DMA_1 = 1,
    #[doc = "2: SD24B DMA Trigger: 2"]
    SD24DMA_2 = 2,
    #[doc = "3: SD24B DMA Trigger: 3"]
    SD24DMA_3 = 3,
    #[doc = "4: SD24B DMA Trigger: 4"]
    SD24DMA_4 = 4,
    #[doc = "5: SD24B DMA Trigger: 5"]
    SD24DMA_5 = 5,
    #[doc = "6: SD24B DMA Trigger: 6"]
    SD24DMA_6 = 6,
    #[doc = "7: SD24B DMA Trigger: 7"]
    SD24DMA_7 = 7,
    #[doc = "8: SD24B DMA Trigger: 8"]
    SD24DMA_8 = 8,
}
impl From<SD24DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: SD24DMA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SD24DMA` reader - SD24B DMA Trigger Select Bit 0"]
pub type SD24DMA_R = crate::FieldReader<u8, SD24DMA_A>;
impl SD24DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SD24DMA_A> {
        match self.bits {
            0 => Some(SD24DMA_A::SD24DMA_0),
            1 => Some(SD24DMA_A::SD24DMA_1),
            2 => Some(SD24DMA_A::SD24DMA_2),
            3 => Some(SD24DMA_A::SD24DMA_3),
            4 => Some(SD24DMA_A::SD24DMA_4),
            5 => Some(SD24DMA_A::SD24DMA_5),
            6 => Some(SD24DMA_A::SD24DMA_6),
            7 => Some(SD24DMA_A::SD24DMA_7),
            8 => Some(SD24DMA_A::SD24DMA_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SD24DMA_0`"]
    #[inline(always)]
    pub fn is_sd24dma_0(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_0
    }
    #[doc = "Checks if the value of the field is `SD24DMA_1`"]
    #[inline(always)]
    pub fn is_sd24dma_1(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_1
    }
    #[doc = "Checks if the value of the field is `SD24DMA_2`"]
    #[inline(always)]
    pub fn is_sd24dma_2(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_2
    }
    #[doc = "Checks if the value of the field is `SD24DMA_3`"]
    #[inline(always)]
    pub fn is_sd24dma_3(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_3
    }
    #[doc = "Checks if the value of the field is `SD24DMA_4`"]
    #[inline(always)]
    pub fn is_sd24dma_4(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_4
    }
    #[doc = "Checks if the value of the field is `SD24DMA_5`"]
    #[inline(always)]
    pub fn is_sd24dma_5(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_5
    }
    #[doc = "Checks if the value of the field is `SD24DMA_6`"]
    #[inline(always)]
    pub fn is_sd24dma_6(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_6
    }
    #[doc = "Checks if the value of the field is `SD24DMA_7`"]
    #[inline(always)]
    pub fn is_sd24dma_7(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_7
    }
    #[doc = "Checks if the value of the field is `SD24DMA_8`"]
    #[inline(always)]
    pub fn is_sd24dma_8(&self) -> bool {
        *self == SD24DMA_A::SD24DMA_8
    }
}
#[doc = "Field `SD24DMA` writer - SD24B DMA Trigger Select Bit 0"]
pub type SD24DMA_W<'a> = crate::FieldWriter<'a, u16, SD24BCTL1_SPEC, u8, SD24DMA_A, 4, 8>;
impl<'a> SD24DMA_W<'a> {
    #[doc = "SD24B DMA Trigger: 0"]
    #[inline(always)]
    pub fn sd24dma_0(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_0)
    }
    #[doc = "SD24B DMA Trigger: 1"]
    #[inline(always)]
    pub fn sd24dma_1(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_1)
    }
    #[doc = "SD24B DMA Trigger: 2"]
    #[inline(always)]
    pub fn sd24dma_2(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_2)
    }
    #[doc = "SD24B DMA Trigger: 3"]
    #[inline(always)]
    pub fn sd24dma_3(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_3)
    }
    #[doc = "SD24B DMA Trigger: 4"]
    #[inline(always)]
    pub fn sd24dma_4(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_4)
    }
    #[doc = "SD24B DMA Trigger: 5"]
    #[inline(always)]
    pub fn sd24dma_5(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_5)
    }
    #[doc = "SD24B DMA Trigger: 6"]
    #[inline(always)]
    pub fn sd24dma_6(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_6)
    }
    #[doc = "SD24B DMA Trigger: 7"]
    #[inline(always)]
    pub fn sd24dma_7(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_7)
    }
    #[doc = "SD24B DMA Trigger: 8"]
    #[inline(always)]
    pub fn sd24dma_8(self) -> &'a mut W {
        self.variant(SD24DMA_A::SD24DMA_8)
    }
}
impl R {
    #[doc = "Bit 0 - SD24B Group 0 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp0sc(&self) -> SD24GRP0SC_R {
        SD24GRP0SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD24B Group 1 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp1sc(&self) -> SD24GRP1SC_R {
        SD24GRP1SC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD24B Group 2 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp2sc(&self) -> SD24GRP2SC_R {
        SD24GRP2SC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD24B Group 3 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp3sc(&self) -> SD24GRP3SC_R {
        SD24GRP3SC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - SD24B DMA Trigger Select Bit 0"]
    #[inline(always)]
    pub fn sd24dma(&self) -> SD24DMA_R {
        SD24DMA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SD24B Group 0 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp0sc(&mut self) -> SD24GRP0SC_W {
        SD24GRP0SC_W::new(self)
    }
    #[doc = "Bit 1 - SD24B Group 1 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp1sc(&mut self) -> SD24GRP1SC_W {
        SD24GRP1SC_W::new(self)
    }
    #[doc = "Bit 2 - SD24B Group 2 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp2sc(&mut self) -> SD24GRP2SC_W {
        SD24GRP2SC_W::new(self)
    }
    #[doc = "Bit 3 - SD24B Group 3 Start Conversion"]
    #[inline(always)]
    pub fn sd24grp3sc(&mut self) -> SD24GRP3SC_W {
        SD24GRP3SC_W::new(self)
    }
    #[doc = "Bits 8:11 - SD24B DMA Trigger Select Bit 0"]
    #[inline(always)]
    pub fn sd24dma(&mut self) -> SD24DMA_W {
        SD24DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bctl1](index.html) module"]
pub struct SD24BCTL1_SPEC;
impl crate::RegisterSpec for SD24BCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bctl1::R](R) reader structure"]
impl crate::Readable for SD24BCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bctl1::W](W) writer structure"]
impl crate::Writable for SD24BCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BCTL1 to value 0"]
impl crate::Resettable for SD24BCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
