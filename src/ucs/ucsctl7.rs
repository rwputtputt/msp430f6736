#[doc = "Register `UCSCTL7` reader"]
pub struct R(crate::R<UCSCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL7` writer"]
pub struct W(crate::W<UCSCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL7_SPEC>;
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
impl From<crate::W<UCSCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOFFG` reader - DCO Fault Flag"]
pub type DCOFFG_R = crate::BitReader<bool>;
#[doc = "Field `DCOFFG` writer - DCO Fault Flag"]
pub type DCOFFG_W<'a> = crate::BitWriter<'a, u16, UCSCTL7_SPEC, bool, 0>;
#[doc = "Field `XT1LFOFFG` reader - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1LFOFFG_R = crate::BitReader<bool>;
#[doc = "Field `XT1LFOFFG` writer - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1LFOFFG_W<'a> = crate::BitWriter<'a, u16, UCSCTL7_SPEC, bool, 1>;
#[doc = "Field `XT2OFFG` reader - High Frequency Oscillator 2 Fault Flag"]
pub type XT2OFFG_R = crate::BitReader<bool>;
#[doc = "Field `XT2OFFG` writer - High Frequency Oscillator 2 Fault Flag"]
pub type XT2OFFG_W<'a> = crate::BitWriter<'a, u16, UCSCTL7_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&self) -> XT1LFOFFG_R {
        XT1LFOFFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&self) -> XT2OFFG_R {
        XT2OFFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W {
        DCOFFG_W::new(self)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&mut self) -> XT1LFOFFG_W {
        XT1LFOFFG_W::new(self)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&mut self) -> XT2OFFG_W {
        XT2OFFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl7](index.html) module"]
pub struct UCSCTL7_SPEC;
impl crate::RegisterSpec for UCSCTL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl7::R](R) reader structure"]
impl crate::Readable for UCSCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl7::W](W) writer structure"]
impl crate::Writable for UCSCTL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL7 to value 0"]
impl crate::Resettable for UCSCTL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
