#[doc = "Register `AUXCTL0` reader"]
pub struct R(crate::R<AUXCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCTL0` writer"]
pub struct W(crate::W<AUXCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCTL0_SPEC>;
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
impl From<crate::W<AUXCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKAUX` reader - Lock auxiliary supply system."]
pub type LOCKAUX_R = crate::BitReader<bool>;
#[doc = "Field `LOCKAUX` writer - Lock auxiliary supply system."]
pub type LOCKAUX_W<'a> = crate::BitWriter<'a, u16, AUXCTL0_SPEC, bool, 0>;
#[doc = "Field `AUX0SW` reader - DVCC switch state."]
pub type AUX0SW_R = crate::BitReader<bool>;
#[doc = "Field `AUX0SW` writer - DVCC switch state."]
pub type AUX0SW_W<'a> = crate::BitWriter<'a, u16, AUXCTL0_SPEC, bool, 1>;
#[doc = "Field `AUX1SW` reader - AUX1 switch state."]
pub type AUX1SW_R = crate::BitReader<bool>;
#[doc = "Field `AUX1SW` writer - AUX1 switch state."]
pub type AUX1SW_W<'a> = crate::BitWriter<'a, u16, AUXCTL0_SPEC, bool, 2>;
#[doc = "Field `AUX2SW` reader - AUX2 switch state."]
pub type AUX2SW_R = crate::BitReader<bool>;
#[doc = "Field `AUX2SW` writer - AUX2 switch state."]
pub type AUX2SW_W<'a> = crate::BitWriter<'a, u16, AUXCTL0_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Lock auxiliary supply system."]
    #[inline(always)]
    pub fn lockaux(&self) -> LOCKAUX_R {
        LOCKAUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVCC switch state."]
    #[inline(always)]
    pub fn aux0sw(&self) -> AUX0SW_R {
        AUX0SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUX1 switch state."]
    #[inline(always)]
    pub fn aux1sw(&self) -> AUX1SW_R {
        AUX1SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AUX2 switch state."]
    #[inline(always)]
    pub fn aux2sw(&self) -> AUX2SW_R {
        AUX2SW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock auxiliary supply system."]
    #[inline(always)]
    pub fn lockaux(&mut self) -> LOCKAUX_W {
        LOCKAUX_W::new(self)
    }
    #[doc = "Bit 1 - DVCC switch state."]
    #[inline(always)]
    pub fn aux0sw(&mut self) -> AUX0SW_W {
        AUX0SW_W::new(self)
    }
    #[doc = "Bit 2 - AUX1 switch state."]
    #[inline(always)]
    pub fn aux1sw(&mut self) -> AUX1SW_W {
        AUX1SW_W::new(self)
    }
    #[doc = "Bit 3 - AUX2 switch state."]
    #[inline(always)]
    pub fn aux2sw(&mut self) -> AUX2SW_W {
        AUX2SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Supply Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctl0](index.html) module"]
pub struct AUXCTL0_SPEC;
impl crate::RegisterSpec for AUXCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [auxctl0::R](R) reader structure"]
impl crate::Readable for AUXCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxctl0::W](W) writer structure"]
impl crate::Writable for AUXCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXCTL0 to value 0"]
impl crate::Resettable for AUXCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
