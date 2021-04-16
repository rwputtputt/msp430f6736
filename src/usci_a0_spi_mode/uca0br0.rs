#[doc = "Register `UCA0BR0` reader"]
pub struct R(crate::R<UCA0BR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0BR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCA0BR0_SPEC>> for R {
    fn from(reader: crate::R<UCA0BR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0BR0` writer"]
pub struct W(crate::W<UCA0BR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0BR0_SPEC>;
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
impl core::convert::From<crate::W<UCA0BR0_SPEC>> for W {
    fn from(writer: crate::W<UCA0BR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0BR0` reader - USCI A0 Baud Rate 0 register"]
pub struct UCA0BR0_R(crate::FieldReader<u8, u8>);
impl UCA0BR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCA0BR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCA0BR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCA0BR0` writer - USCI A0 Baud Rate 0 register"]
pub struct UCA0BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0BR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&self) -> UCA0BR0_R {
        UCA0BR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 0 register"]
    #[inline(always)]
    pub fn uca0br0(&mut self) -> UCA0BR0_W {
        UCA0BR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0br0](index.html) module"]
pub struct UCA0BR0_SPEC;
impl crate::RegisterSpec for UCA0BR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0br0::R](R) reader structure"]
impl crate::Readable for UCA0BR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0br0::W](W) writer structure"]
impl crate::Writable for UCA0BR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0BR0 to value 0"]
impl crate::Resettable for UCA0BR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}