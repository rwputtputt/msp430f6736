#[doc = "Register `UCSCTL0` reader"]
pub struct R(crate::R<UCSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL0` writer"]
pub struct W(crate::W<UCSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL0_SPEC>;
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
impl From<crate::W<UCSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD0` reader - Modulation Bit Counter Bit : 0"]
pub type MOD0_R = crate::BitReader<bool>;
#[doc = "Field `MOD0` writer - Modulation Bit Counter Bit : 0"]
pub type MOD0_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 3>;
#[doc = "Field `MOD1` reader - Modulation Bit Counter Bit : 1"]
pub type MOD1_R = crate::BitReader<bool>;
#[doc = "Field `MOD1` writer - Modulation Bit Counter Bit : 1"]
pub type MOD1_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 4>;
#[doc = "Field `MOD2` reader - Modulation Bit Counter Bit : 2"]
pub type MOD2_R = crate::BitReader<bool>;
#[doc = "Field `MOD2` writer - Modulation Bit Counter Bit : 2"]
pub type MOD2_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 5>;
#[doc = "Field `MOD3` reader - Modulation Bit Counter Bit : 3"]
pub type MOD3_R = crate::BitReader<bool>;
#[doc = "Field `MOD3` writer - Modulation Bit Counter Bit : 3"]
pub type MOD3_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 6>;
#[doc = "Field `MOD4` reader - Modulation Bit Counter Bit : 4"]
pub type MOD4_R = crate::BitReader<bool>;
#[doc = "Field `MOD4` writer - Modulation Bit Counter Bit : 4"]
pub type MOD4_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 7>;
#[doc = "Field `DCO0` reader - DCO TAP Bit : 0"]
pub type DCO0_R = crate::BitReader<bool>;
#[doc = "Field `DCO0` writer - DCO TAP Bit : 0"]
pub type DCO0_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 8>;
#[doc = "Field `DCO1` reader - DCO TAP Bit : 1"]
pub type DCO1_R = crate::BitReader<bool>;
#[doc = "Field `DCO1` writer - DCO TAP Bit : 1"]
pub type DCO1_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 9>;
#[doc = "Field `DCO2` reader - DCO TAP Bit : 2"]
pub type DCO2_R = crate::BitReader<bool>;
#[doc = "Field `DCO2` writer - DCO TAP Bit : 2"]
pub type DCO2_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 10>;
#[doc = "Field `DCO3` reader - DCO TAP Bit : 3"]
pub type DCO3_R = crate::BitReader<bool>;
#[doc = "Field `DCO3` writer - DCO TAP Bit : 3"]
pub type DCO3_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 11>;
#[doc = "Field `DCO4` reader - DCO TAP Bit : 4"]
pub type DCO4_R = crate::BitReader<bool>;
#[doc = "Field `DCO4` writer - DCO TAP Bit : 4"]
pub type DCO4_W<'a> = crate::BitWriter<'a, u16, UCSCTL0_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&self) -> DCO0_R {
        DCO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&self) -> DCO1_R {
        DCO1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&self) -> DCO2_R {
        DCO2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&self) -> DCO3_R {
        DCO3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&self) -> DCO4_R {
        DCO4_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> MOD0_W {
        MOD0_W::new(self)
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> MOD1_W {
        MOD1_W::new(self)
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> MOD2_W {
        MOD2_W::new(self)
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> MOD3_W {
        MOD3_W::new(self)
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> MOD4_W {
        MOD4_W::new(self)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> DCO0_W {
        DCO0_W::new(self)
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> DCO1_W {
        DCO1_W::new(self)
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> DCO2_W {
        DCO2_W::new(self)
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&mut self) -> DCO3_W {
        DCO3_W::new(self)
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&mut self) -> DCO4_W {
        DCO4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl0](index.html) module"]
pub struct UCSCTL0_SPEC;
impl crate::RegisterSpec for UCSCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl0::R](R) reader structure"]
impl crate::Readable for UCSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl0::W](W) writer structure"]
impl crate::Writable for UCSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL0 to value 0"]
impl crate::Resettable for UCSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
