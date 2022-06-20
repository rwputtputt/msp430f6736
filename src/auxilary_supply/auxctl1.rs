#[doc = "Register `AUXCTL1` reader"]
pub struct R(crate::R<AUXCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCTL1` writer"]
pub struct W(crate::W<AUXCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCTL1_SPEC>;
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
impl From<crate::W<AUXCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX0OK` reader - DVCC okay flag."]
pub type AUX0OK_R = crate::BitReader<bool>;
#[doc = "Field `AUX0OK` writer - DVCC okay flag."]
pub type AUX0OK_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 0>;
#[doc = "Field `AUX1OK` reader - AUX 1 supply okay flag."]
pub type AUX1OK_R = crate::BitReader<bool>;
#[doc = "Field `AUX1OK` writer - AUX 1 supply okay flag."]
pub type AUX1OK_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 1>;
#[doc = "Field `AUX2OK` reader - AUX 2 supply okay flag."]
pub type AUX2OK_R = crate::BitReader<bool>;
#[doc = "Field `AUX2OK` writer - AUX 2 supply okay flag."]
pub type AUX2OK_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 2>;
#[doc = "Field `AUX2PRIO` reader - Auxiliary supply AUX2 priority."]
pub type AUX2PRIO_R = crate::BitReader<bool>;
#[doc = "Field `AUX2PRIO` writer - Auxiliary supply AUX2 priority."]
pub type AUX2PRIO_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 3>;
#[doc = "Field `AUX0MD` reader - DVCC supply mode."]
pub type AUX0MD_R = crate::BitReader<bool>;
#[doc = "Field `AUX0MD` writer - DVCC supply mode."]
pub type AUX0MD_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 8>;
#[doc = "Field `AUX1MD` reader - AUX1 supply mode."]
pub type AUX1MD_R = crate::BitReader<bool>;
#[doc = "Field `AUX1MD` writer - AUX1 supply mode."]
pub type AUX1MD_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 9>;
#[doc = "Field `AUX2MD` reader - AUX2 supply mode."]
pub type AUX2MD_R = crate::BitReader<bool>;
#[doc = "Field `AUX2MD` writer - AUX2 supply mode."]
pub type AUX2MD_W<'a> = crate::BitWriter<'a, u16, AUXCTL1_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - DVCC okay flag."]
    #[inline(always)]
    pub fn aux0ok(&self) -> AUX0OK_R {
        AUX0OK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AUX 1 supply okay flag."]
    #[inline(always)]
    pub fn aux1ok(&self) -> AUX1OK_R {
        AUX1OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUX 2 supply okay flag."]
    #[inline(always)]
    pub fn aux2ok(&self) -> AUX2OK_R {
        AUX2OK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary supply AUX2 priority."]
    #[inline(always)]
    pub fn aux2prio(&self) -> AUX2PRIO_R {
        AUX2PRIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DVCC supply mode."]
    #[inline(always)]
    pub fn aux0md(&self) -> AUX0MD_R {
        AUX0MD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AUX1 supply mode."]
    #[inline(always)]
    pub fn aux1md(&self) -> AUX1MD_R {
        AUX1MD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AUX2 supply mode."]
    #[inline(always)]
    pub fn aux2md(&self) -> AUX2MD_R {
        AUX2MD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVCC okay flag."]
    #[inline(always)]
    pub fn aux0ok(&mut self) -> AUX0OK_W {
        AUX0OK_W::new(self)
    }
    #[doc = "Bit 1 - AUX 1 supply okay flag."]
    #[inline(always)]
    pub fn aux1ok(&mut self) -> AUX1OK_W {
        AUX1OK_W::new(self)
    }
    #[doc = "Bit 2 - AUX 2 supply okay flag."]
    #[inline(always)]
    pub fn aux2ok(&mut self) -> AUX2OK_W {
        AUX2OK_W::new(self)
    }
    #[doc = "Bit 3 - Auxiliary supply AUX2 priority."]
    #[inline(always)]
    pub fn aux2prio(&mut self) -> AUX2PRIO_W {
        AUX2PRIO_W::new(self)
    }
    #[doc = "Bit 8 - DVCC supply mode."]
    #[inline(always)]
    pub fn aux0md(&mut self) -> AUX0MD_W {
        AUX0MD_W::new(self)
    }
    #[doc = "Bit 9 - AUX1 supply mode."]
    #[inline(always)]
    pub fn aux1md(&mut self) -> AUX1MD_W {
        AUX1MD_W::new(self)
    }
    #[doc = "Bit 10 - AUX2 supply mode."]
    #[inline(always)]
    pub fn aux2md(&mut self) -> AUX2MD_W {
        AUX2MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Supply Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctl1](index.html) module"]
pub struct AUXCTL1_SPEC;
impl crate::RegisterSpec for AUXCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxctl1::R](R) reader structure"]
impl crate::Readable for AUXCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxctl1::W](W) writer structure"]
impl crate::Writable for AUXCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXCTL1 to value 0"]
impl crate::Resettable for AUXCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
