#[doc = "Register `AUXIFG` reader"]
pub struct R(crate::R<AUXIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXIFG` writer"]
pub struct W(crate::W<AUXIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXIFG_SPEC>;
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
impl From<crate::W<AUXIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX0SWIFG` reader - Switched to DVCC interrupt flag"]
pub type AUX0SWIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX0SWIFG` writer - Switched to DVCC interrupt flag"]
pub type AUX0SWIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 0>;
#[doc = "Field `AUX1SWIFG` reader - Switched to AUX1 interrupt flag"]
pub type AUX1SWIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX1SWIFG` writer - Switched to AUX1 interrupt flag"]
pub type AUX1SWIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 1>;
#[doc = "Field `AUX2SWIFG` reader - Switched to AUX2 interrupt flag"]
pub type AUX2SWIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX2SWIFG` writer - Switched to AUX2 interrupt flag"]
pub type AUX2SWIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 2>;
#[doc = "Field `AUX0DRPIFG` reader - DVCC dropped below its threshold interrupt flag"]
pub type AUX0DRPIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX0DRPIFG` writer - DVCC dropped below its threshold interrupt flag"]
pub type AUX0DRPIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 4>;
#[doc = "Field `AUX1DRPIFG` reader - AUX1 dropped below its threshold interrupt flag"]
pub type AUX1DRPIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX1DRPIFG` writer - AUX1 dropped below its threshold interrupt flag"]
pub type AUX1DRPIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 5>;
#[doc = "Field `AUX2DRPIFG` reader - AUX2 dropped below its threshold interrupt flag"]
pub type AUX2DRPIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUX2DRPIFG` writer - AUX2 dropped below its threshold interrupt flag"]
pub type AUX2DRPIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 6>;
#[doc = "Field `AUXMONIFG` reader - Supply monitor interrupt flag"]
pub type AUXMONIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUXMONIFG` writer - Supply monitor interrupt flag"]
pub type AUXMONIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 7>;
#[doc = "Field `AUXSWNMIFG` reader - Supplies switched (non-)maskable interrupt flag"]
pub type AUXSWNMIFG_R = crate::BitReader<bool>;
#[doc = "Field `AUXSWNMIFG` writer - Supplies switched (non-)maskable interrupt flag"]
pub type AUXSWNMIFG_W<'a> = crate::BitWriter<'a, u16, AUXIFG_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Switched to DVCC interrupt flag"]
    #[inline(always)]
    pub fn aux0swifg(&self) -> AUX0SWIFG_R {
        AUX0SWIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switched to AUX1 interrupt flag"]
    #[inline(always)]
    pub fn aux1swifg(&self) -> AUX1SWIFG_R {
        AUX1SWIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Switched to AUX2 interrupt flag"]
    #[inline(always)]
    pub fn aux2swifg(&self) -> AUX2SWIFG_R {
        AUX2SWIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DVCC dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux0drpifg(&self) -> AUX0DRPIFG_R {
        AUX0DRPIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUX1 dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux1drpifg(&self) -> AUX1DRPIFG_R {
        AUX1DRPIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUX2 dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux2drpifg(&self) -> AUX2DRPIFG_R {
        AUX2DRPIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Supply monitor interrupt flag"]
    #[inline(always)]
    pub fn auxmonifg(&self) -> AUXMONIFG_R {
        AUXMONIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Supplies switched (non-)maskable interrupt flag"]
    #[inline(always)]
    pub fn auxswnmifg(&self) -> AUXSWNMIFG_R {
        AUXSWNMIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Switched to DVCC interrupt flag"]
    #[inline(always)]
    pub fn aux0swifg(&mut self) -> AUX0SWIFG_W {
        AUX0SWIFG_W::new(self)
    }
    #[doc = "Bit 1 - Switched to AUX1 interrupt flag"]
    #[inline(always)]
    pub fn aux1swifg(&mut self) -> AUX1SWIFG_W {
        AUX1SWIFG_W::new(self)
    }
    #[doc = "Bit 2 - Switched to AUX2 interrupt flag"]
    #[inline(always)]
    pub fn aux2swifg(&mut self) -> AUX2SWIFG_W {
        AUX2SWIFG_W::new(self)
    }
    #[doc = "Bit 4 - DVCC dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux0drpifg(&mut self) -> AUX0DRPIFG_W {
        AUX0DRPIFG_W::new(self)
    }
    #[doc = "Bit 5 - AUX1 dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux1drpifg(&mut self) -> AUX1DRPIFG_W {
        AUX1DRPIFG_W::new(self)
    }
    #[doc = "Bit 6 - AUX2 dropped below its threshold interrupt flag"]
    #[inline(always)]
    pub fn aux2drpifg(&mut self) -> AUX2DRPIFG_W {
        AUX2DRPIFG_W::new(self)
    }
    #[doc = "Bit 7 - Supply monitor interrupt flag"]
    #[inline(always)]
    pub fn auxmonifg(&mut self) -> AUXMONIFG_W {
        AUXMONIFG_W::new(self)
    }
    #[doc = "Bit 8 - Supplies switched (non-)maskable interrupt flag"]
    #[inline(always)]
    pub fn auxswnmifg(&mut self) -> AUXSWNMIFG_W {
        AUXSWNMIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxifg](index.html) module"]
pub struct AUXIFG_SPEC;
impl crate::RegisterSpec for AUXIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxifg::R](R) reader structure"]
impl crate::Readable for AUXIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxifg::W](W) writer structure"]
impl crate::Writable for AUXIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXIFG to value 0"]
impl crate::Resettable for AUXIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
