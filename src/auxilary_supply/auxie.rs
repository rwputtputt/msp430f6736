#[doc = "Register `AUXIE` reader"]
pub struct R(crate::R<AUXIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXIE` writer"]
pub struct W(crate::W<AUXIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXIE_SPEC>;
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
impl From<crate::W<AUXIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX0SWIE` reader - Switched to DVCC interrupt enable"]
pub type AUX0SWIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX0SWIE` writer - Switched to DVCC interrupt enable"]
pub type AUX0SWIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 0>;
#[doc = "Field `AUX1SWIE` reader - Switched to AUX1 interrupt enable"]
pub type AUX1SWIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX1SWIE` writer - Switched to AUX1 interrupt enable"]
pub type AUX1SWIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 1>;
#[doc = "Field `AUX2SWIE` reader - Switched to AUX2 interrupt enable"]
pub type AUX2SWIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX2SWIE` writer - Switched to AUX2 interrupt enable"]
pub type AUX2SWIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 2>;
#[doc = "Field `AUXSWGIE` reader - Global supply switched interrupt enable."]
pub type AUXSWGIE_R = crate::BitReader<bool>;
#[doc = "Field `AUXSWGIE` writer - Global supply switched interrupt enable."]
pub type AUXSWGIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 3>;
#[doc = "Field `AUX0DRPIE` reader - DVCC dropped below its threshold interrupt enable"]
pub type AUX0DRPIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX0DRPIE` writer - DVCC dropped below its threshold interrupt enable"]
pub type AUX0DRPIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 4>;
#[doc = "Field `AUX1DRPIE` reader - AUX1 dropped below its threshold interrupt enable"]
pub type AUX1DRPIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX1DRPIE` writer - AUX1 dropped below its threshold interrupt enable"]
pub type AUX1DRPIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 5>;
#[doc = "Field `AUX2DRPIE` reader - AUX2 dropped below its threshold interrupt enable"]
pub type AUX2DRPIE_R = crate::BitReader<bool>;
#[doc = "Field `AUX2DRPIE` writer - AUX2 dropped below its threshold interrupt enable"]
pub type AUX2DRPIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 6>;
#[doc = "Field `AUXMONIE` reader - Supply monitor interrupt enable"]
pub type AUXMONIE_R = crate::BitReader<bool>;
#[doc = "Field `AUXMONIE` writer - Supply monitor interrupt enable"]
pub type AUXMONIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 7>;
#[doc = "Field `AUXSWNMIE` reader - Supplies switched (non-)maskable interrupt enable"]
pub type AUXSWNMIE_R = crate::BitReader<bool>;
#[doc = "Field `AUXSWNMIE` writer - Supplies switched (non-)maskable interrupt enable"]
pub type AUXSWNMIE_W<'a> = crate::BitWriter<'a, u16, AUXIE_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Switched to DVCC interrupt enable"]
    #[inline(always)]
    pub fn aux0swie(&self) -> AUX0SWIE_R {
        AUX0SWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switched to AUX1 interrupt enable"]
    #[inline(always)]
    pub fn aux1swie(&self) -> AUX1SWIE_R {
        AUX1SWIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Switched to AUX2 interrupt enable"]
    #[inline(always)]
    pub fn aux2swie(&self) -> AUX2SWIE_R {
        AUX2SWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global supply switched interrupt enable."]
    #[inline(always)]
    pub fn auxswgie(&self) -> AUXSWGIE_R {
        AUXSWGIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVCC dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux0drpie(&self) -> AUX0DRPIE_R {
        AUX0DRPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AUX1 dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux1drpie(&self) -> AUX1DRPIE_R {
        AUX1DRPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUX2 dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux2drpie(&self) -> AUX2DRPIE_R {
        AUX2DRPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Supply monitor interrupt enable"]
    #[inline(always)]
    pub fn auxmonie(&self) -> AUXMONIE_R {
        AUXMONIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Supplies switched (non-)maskable interrupt enable"]
    #[inline(always)]
    pub fn auxswnmie(&self) -> AUXSWNMIE_R {
        AUXSWNMIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Switched to DVCC interrupt enable"]
    #[inline(always)]
    pub fn aux0swie(&mut self) -> AUX0SWIE_W {
        AUX0SWIE_W::new(self)
    }
    #[doc = "Bit 1 - Switched to AUX1 interrupt enable"]
    #[inline(always)]
    pub fn aux1swie(&mut self) -> AUX1SWIE_W {
        AUX1SWIE_W::new(self)
    }
    #[doc = "Bit 2 - Switched to AUX2 interrupt enable"]
    #[inline(always)]
    pub fn aux2swie(&mut self) -> AUX2SWIE_W {
        AUX2SWIE_W::new(self)
    }
    #[doc = "Bit 3 - Global supply switched interrupt enable."]
    #[inline(always)]
    pub fn auxswgie(&mut self) -> AUXSWGIE_W {
        AUXSWGIE_W::new(self)
    }
    #[doc = "Bit 4 - DVCC dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux0drpie(&mut self) -> AUX0DRPIE_W {
        AUX0DRPIE_W::new(self)
    }
    #[doc = "Bit 5 - AUX1 dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux1drpie(&mut self) -> AUX1DRPIE_W {
        AUX1DRPIE_W::new(self)
    }
    #[doc = "Bit 6 - AUX2 dropped below its threshold interrupt enable"]
    #[inline(always)]
    pub fn aux2drpie(&mut self) -> AUX2DRPIE_W {
        AUX2DRPIE_W::new(self)
    }
    #[doc = "Bit 7 - Supply monitor interrupt enable"]
    #[inline(always)]
    pub fn auxmonie(&mut self) -> AUXMONIE_W {
        AUXMONIE_W::new(self)
    }
    #[doc = "Bit 8 - Supplies switched (non-)maskable interrupt enable"]
    #[inline(always)]
    pub fn auxswnmie(&mut self) -> AUXSWNMIE_W {
        AUXSWNMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxie](index.html) module"]
pub struct AUXIE_SPEC;
impl crate::RegisterSpec for AUXIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxie::R](R) reader structure"]
impl crate::Readable for AUXIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxie::W](W) writer structure"]
impl crate::Writable for AUXIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXIE to value 0"]
impl crate::Resettable for AUXIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
