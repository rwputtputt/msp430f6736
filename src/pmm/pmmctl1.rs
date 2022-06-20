#[doc = "Register `PMMCTL1` reader"]
pub struct R(crate::R<PMMCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL1` writer"]
pub struct W(crate::W<PMMCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL1_SPEC>;
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
impl From<crate::W<PMMCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMREFMD` reader - PMM Reference Mode"]
pub type PMMREFMD_R = crate::BitReader<bool>;
#[doc = "Field `PMMREFMD` writer - PMM Reference Mode"]
pub type PMMREFMD_W<'a> = crate::BitWriter<'a, u16, PMMCTL1_SPEC, bool, 0>;
#[doc = "Field `PMMCMD0` reader - PMM Voltage Regulator Current Mode Bit: 0"]
pub type PMMCMD0_R = crate::BitReader<bool>;
#[doc = "Field `PMMCMD0` writer - PMM Voltage Regulator Current Mode Bit: 0"]
pub type PMMCMD0_W<'a> = crate::BitWriter<'a, u16, PMMCTL1_SPEC, bool, 4>;
#[doc = "Field `PMMCMD1` reader - PMM Voltage Regulator Current Mode Bit: 1"]
pub type PMMCMD1_R = crate::BitReader<bool>;
#[doc = "Field `PMMCMD1` writer - PMM Voltage Regulator Current Mode Bit: 1"]
pub type PMMCMD1_W<'a> = crate::BitWriter<'a, u16, PMMCTL1_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&self) -> PMMREFMD_R {
        PMMREFMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&self) -> PMMCMD0_R {
        PMMCMD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&self) -> PMMCMD1_R {
        PMMCMD1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&mut self) -> PMMREFMD_W {
        PMMREFMD_W::new(self)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&mut self) -> PMMCMD0_W {
        PMMCMD0_W::new(self)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&mut self) -> PMMCMD1_W {
        PMMCMD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl1](index.html) module"]
pub struct PMMCTL1_SPEC;
impl crate::RegisterSpec for PMMCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl1::R](R) reader structure"]
impl crate::Readable for PMMCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl1::W](W) writer structure"]
impl crate::Writable for PMMCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL1 to value 0"]
impl crate::Resettable for PMMCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
