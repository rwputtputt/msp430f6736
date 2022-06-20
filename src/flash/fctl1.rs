#[doc = "Register `FCTL1` reader"]
pub struct R(crate::R<FCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL1` writer"]
pub struct W(crate::W<FCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL1_SPEC>;
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
impl From<crate::W<FCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASE` reader - Enable bit for Flash segment erase"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - Enable bit for Flash segment erase"]
pub type ERASE_W<'a> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, 1>;
#[doc = "Field `MERAS` reader - Enable bit for Flash mass erase"]
pub type MERAS_R = crate::BitReader<bool>;
#[doc = "Field `MERAS` writer - Enable bit for Flash mass erase"]
pub type MERAS_W<'a> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, 2>;
#[doc = "Field `SWRT` reader - Smart Write enable"]
pub type SWRT_R = crate::BitReader<bool>;
#[doc = "Field `SWRT` writer - Smart Write enable"]
pub type SWRT_W<'a> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, 5>;
#[doc = "Field `WRT` reader - Enable bit for Flash write"]
pub type WRT_R = crate::BitReader<bool>;
#[doc = "Field `WRT` writer - Enable bit for Flash write"]
pub type WRT_W<'a> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, 6>;
#[doc = "Field `BLKWRT` reader - Enable bit for Flash segment write"]
pub type BLKWRT_R = crate::BitReader<bool>;
#[doc = "Field `BLKWRT` writer - Enable bit for Flash segment write"]
pub type BLKWRT_W<'a> = crate::BitWriter<'a, u16, FCTL1_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&self) -> MERAS_R {
        MERAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Smart Write enable"]
    #[inline(always)]
    pub fn swrt(&self) -> SWRT_R {
        SWRT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&self) -> WRT_R {
        WRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&self) -> BLKWRT_R {
        BLKWRT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable bit for Flash segment erase"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Enable bit for Flash mass erase"]
    #[inline(always)]
    pub fn meras(&mut self) -> MERAS_W {
        MERAS_W::new(self)
    }
    #[doc = "Bit 5 - Smart Write enable"]
    #[inline(always)]
    pub fn swrt(&mut self) -> SWRT_W {
        SWRT_W::new(self)
    }
    #[doc = "Bit 6 - Enable bit for Flash write"]
    #[inline(always)]
    pub fn wrt(&mut self) -> WRT_W {
        WRT_W::new(self)
    }
    #[doc = "Bit 7 - Enable bit for Flash segment write"]
    #[inline(always)]
    pub fn blkwrt(&mut self) -> BLKWRT_W {
        BLKWRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl1](index.html) module"]
pub struct FCTL1_SPEC;
impl crate::RegisterSpec for FCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctl1::R](R) reader structure"]
impl crate::Readable for FCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl1::W](W) writer structure"]
impl crate::Writable for FCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTL1 to value 0"]
impl crate::Resettable for FCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
