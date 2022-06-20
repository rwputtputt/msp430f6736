#[doc = "Register `LCDCIV` reader"]
pub struct R(crate::R<LCDCIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCIV` writer"]
pub struct W(crate::W<LCDCIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCIV_SPEC>;
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
impl From<crate::W<LCDCIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCIV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdciv](index.html) module"]
pub struct LCDCIV_SPEC;
impl crate::RegisterSpec for LCDCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdciv::R](R) reader structure"]
impl crate::Readable for LCDCIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdciv::W](W) writer structure"]
impl crate::Writable for LCDCIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCIV to value 0"]
impl crate::Resettable for LCDCIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
