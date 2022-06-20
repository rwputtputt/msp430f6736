#[doc = "Register `SFRIE1` reader"]
pub struct R(crate::R<SFRIE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRIE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRIE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRIE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRIE1` writer"]
pub struct W(crate::W<SFRIE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRIE1_SPEC>;
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
impl From<crate::W<SFRIE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRIE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIE` reader - WDT Interrupt Enable"]
pub type WDTIE_R = crate::BitReader<bool>;
#[doc = "Field `WDTIE` writer - WDT Interrupt Enable"]
pub type WDTIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 0>;
#[doc = "Field `OFIE` reader - Osc Fault Enable"]
pub type OFIE_R = crate::BitReader<bool>;
#[doc = "Field `OFIE` writer - Osc Fault Enable"]
pub type OFIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 1>;
#[doc = "Field `VMAIE` reader - Vacant Memory Interrupt Enable"]
pub type VMAIE_R = crate::BitReader<bool>;
#[doc = "Field `VMAIE` writer - Vacant Memory Interrupt Enable"]
pub type VMAIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 3>;
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub type NMIIE_R = crate::BitReader<bool>;
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub type NMIIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 4>;
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_R = crate::BitReader<bool>;
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub type ACCVIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 5>;
#[doc = "Field `JMBINIE` reader - JTAG Mail Box input Interrupt Enable"]
pub type JMBINIE_R = crate::BitReader<bool>;
#[doc = "Field `JMBINIE` writer - JTAG Mail Box input Interrupt Enable"]
pub type JMBINIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 6>;
#[doc = "Field `JMBOUTIE` reader - JTAG Mail Box output Interrupt Enable"]
pub type JMBOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `JMBOUTIE` writer - JTAG Mail Box output Interrupt Enable"]
pub type JMBOUTIE_W<'a> = crate::BitWriter<'a, u16, SFRIE1_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W::new(self)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W {
        OFIE_W::new(self)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VMAIE_W {
        VMAIE_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W {
        NMIIE_W::new(self)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> ACCVIE_W {
        ACCVIE_W::new(self)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JMBINIE_W {
        JMBINIE_W::new(self)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W {
        JMBOUTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrie1](index.html) module"]
pub struct SFRIE1_SPEC;
impl crate::RegisterSpec for SFRIE1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrie1::R](R) reader structure"]
impl crate::Readable for SFRIE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrie1::W](W) writer structure"]
impl crate::Writable for SFRIE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for SFRIE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
