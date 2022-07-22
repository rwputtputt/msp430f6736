#[doc = "Register `UCA0CTL0` reader"]
pub struct R(crate::R<UCA0CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0CTL0` writer"]
pub struct W(crate::W<UCA0CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0CTL0_SPEC>;
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
impl From<crate::W<UCA0CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UCSYNC_R = crate::BitReader<bool>;
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UCSYNC_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 0>;
#[doc = "Field `UCMODE0` reader - USCI mode"]
pub type UCMODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCMODE0` writer - USCI mode"]
pub type UCMODE0_W<'a> = crate::FieldWriter<'a, u8, UCA0CTL0_SPEC, u8, u8, 2, 2>;
#[doc = "Field `UCSPB` reader - Stop bit select. Number of stop bits"]
pub type UCSPB_R = crate::BitReader<bool>;
#[doc = "Field `UCSPB` writer - Stop bit select. Number of stop bits"]
pub type UCSPB_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 3>;
#[doc = "Field `UC7BIT` reader - Character length. Selects 7-bit or 8-bit"]
pub type UC7BIT_R = crate::BitReader<bool>;
#[doc = "Field `UC7BIT` writer - Character length. Selects 7-bit or 8-bit"]
pub type UC7BIT_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 4>;
#[doc = "Field `UCMSB` reader - MSP first select"]
pub type UCMSB_R = crate::BitReader<bool>;
#[doc = "Field `UCMSB` writer - MSP first select"]
pub type UCMSB_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 5>;
#[doc = "Field `UCPAR` reader - Parity select"]
pub type UCPAR_R = crate::BitReader<bool>;
#[doc = "Field `UCPAR` writer - Parity select"]
pub type UCPAR_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 6>;
#[doc = "Field `UCPEN` reader - Parity enable"]
pub type UCPEN_R = crate::BitReader<bool>;
#[doc = "Field `UCPEN` writer - Parity enable"]
pub type UCPEN_W<'a> = crate::BitWriter<'a, u8, UCA0CTL0_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - USCI mode"]
    #[inline(always)]
    pub fn ucmode0(&self) -> UCMODE0_R {
        UCMODE0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 3 - Stop bit select. Number of stop bits"]
    #[inline(always)]
    pub fn ucspb(&self) -> UCSPB_R {
        UCSPB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Character length. Selects 7-bit or 8-bit"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSP first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&self) -> UCPAR_R {
        UCPAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UCPEN_R {
        UCPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W {
        UCSYNC_W::new(self)
    }
    #[doc = "Bits 2:3 - USCI mode"]
    #[inline(always)]
    pub fn ucmode0(&mut self) -> UCMODE0_W {
        UCMODE0_W::new(self)
    }
    #[doc = "Bit 3 - Stop bit select. Number of stop bits"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UCSPB_W {
        UCSPB_W::new(self)
    }
    #[doc = "Bit 4 - Character length. Selects 7-bit or 8-bit"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W {
        UC7BIT_W::new(self)
    }
    #[doc = "Bit 5 - MSP first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W {
        UCMSB_W::new(self)
    }
    #[doc = "Bit 6 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UCPAR_W {
        UCPAR_W::new(self)
    }
    #[doc = "Bit 7 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UCPEN_W {
        UCPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl0](index.html) module"]
pub struct UCA0CTL0_SPEC;
impl crate::RegisterSpec for UCA0CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ctl0::R](R) reader structure"]
impl crate::Readable for UCA0CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ctl0::W](W) writer structure"]
impl crate::Writable for UCA0CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0CTL0 to value 0"]
impl crate::Resettable for UCA0CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
