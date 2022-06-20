#[doc = "Register `SFRIFG1` reader"]
pub struct R(crate::R<SFRIFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRIFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRIFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRIFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRIFG1` writer"]
pub struct W(crate::W<SFRIFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRIFG1_SPEC>;
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
impl From<crate::W<SFRIFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRIFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIFG` reader - WDT Interrupt Flag"]
pub type WDTIFG_R = crate::BitReader<bool>;
#[doc = "Field `WDTIFG` writer - WDT Interrupt Flag"]
pub type WDTIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 0>;
#[doc = "Field `OFIFG` reader - Osc Fault Flag"]
pub type OFIFG_R = crate::BitReader<bool>;
#[doc = "Field `OFIFG` writer - Osc Fault Flag"]
pub type OFIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 1>;
#[doc = "Field `VMAIFG` reader - Vacant Memory Interrupt Flag"]
pub type VMAIFG_R = crate::BitReader<bool>;
#[doc = "Field `VMAIFG` writer - Vacant Memory Interrupt Flag"]
pub type VMAIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 3>;
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub type NMIIFG_R = crate::BitReader<bool>;
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub type NMIIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 4>;
#[doc = "Field `JMBINIFG` reader - JTAG Mail Box input Interrupt Flag"]
pub type JMBINIFG_R = crate::BitReader<bool>;
#[doc = "Field `JMBINIFG` writer - JTAG Mail Box input Interrupt Flag"]
pub type JMBINIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 6>;
#[doc = "Field `JMBOUTIFG` reader - JTAG Mail Box output Interrupt Flag"]
pub type JMBOUTIFG_R = crate::BitReader<bool>;
#[doc = "Field `JMBOUTIFG` writer - JTAG Mail Box output Interrupt Flag"]
pub type JMBOUTIFG_W<'a> = crate::BitWriter<'a, u16, SFRIFG1_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VMAIFG_R {
        VMAIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JMBINIFG_R {
        JMBINIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JMBOUTIFG_R {
        JMBOUTIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W {
        WDTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Osc Fault Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W {
        OFIFG_W::new(self)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VMAIFG_W {
        VMAIFG_W::new(self)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W {
        NMIIFG_W::new(self)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JMBINIFG_W {
        JMBINIFG_W::new(self)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JMBOUTIFG_W {
        JMBOUTIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrifg1](index.html) module"]
pub struct SFRIFG1_SPEC;
impl crate::RegisterSpec for SFRIFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrifg1::R](R) reader structure"]
impl crate::Readable for SFRIFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrifg1::W](W) writer structure"]
impl crate::Writable for SFRIFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRIFG1 to value 0"]
impl crate::Resettable for SFRIFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
