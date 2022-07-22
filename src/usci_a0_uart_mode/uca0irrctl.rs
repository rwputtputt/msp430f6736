#[doc = "Register `UCA0IRRCTL` reader"]
pub struct R(crate::R<UCA0IRRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IRRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IRRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IRRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IRRCTL` writer"]
pub struct W(crate::W<UCA0IRRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IRRCTL_SPEC>;
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
impl From<crate::W<UCA0IRRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IRRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIRRXFE` reader - IrDA receive filter enable"]
pub type UCIRRXFE_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXFE` writer - IrDA receive filter enable"]
pub type UCIRRXFE_W<'a> = crate::BitWriter<'a, u8, UCA0IRRCTL_SPEC, bool, 0>;
#[doc = "Field `UCIRRXPL` reader - IrDA receive input UCA0RXD polarity"]
pub type UCIRRXPL_R = crate::BitReader<bool>;
#[doc = "Field `UCIRRXPL` writer - IrDA receive input UCA0RXD polarity"]
pub type UCIRRXPL_W<'a> = crate::BitWriter<'a, u8, UCA0IRRCTL_SPEC, bool, 1>;
#[doc = "Field `UCIRRXFL0` reader - Receive filter length"]
pub type UCIRRXFL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCIRRXFL0` writer - Receive filter length"]
pub type UCIRRXFL0_W<'a> = crate::FieldWriter<'a, u8, UCA0IRRCTL_SPEC, u8, u8, 6, 7>;
impl R {
    #[doc = "Bit 0 - IrDA receive filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA receive input UCA0RXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> UCIRRXFL0_R {
        UCIRRXFL0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA receive filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W {
        UCIRRXFE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA receive input UCA0RXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W {
        UCIRRXPL_W::new(self)
    }
    #[doc = "Bits 7:12 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl0(&mut self) -> UCIRRXFL0_W {
        UCIRRXFL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0irrctl](index.html) module"]
pub struct UCA0IRRCTL_SPEC;
impl crate::RegisterSpec for UCA0IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0irrctl::R](R) reader structure"]
impl crate::Readable for UCA0IRRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0irrctl::W](W) writer structure"]
impl crate::Writable for UCA0IRRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IRRCTL to value 0"]
impl crate::Resettable for UCA0IRRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
