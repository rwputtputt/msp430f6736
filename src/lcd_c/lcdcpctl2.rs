#[doc = "Register `LCDCPCTL2` reader"]
pub struct R(crate::R<LCDCPCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCPCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCPCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCPCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCPCTL2` writer"]
pub struct W(crate::W<LCDCPCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCPCTL2_SPEC>;
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
impl From<crate::W<LCDCPCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCPCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS32` reader - LCD Segment 32 enable."]
pub type LCDS32_R = crate::BitReader<bool>;
#[doc = "Field `LCDS32` writer - LCD Segment 32 enable."]
pub type LCDS32_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 0>;
#[doc = "Field `LCDS33` reader - LCD Segment 33 enable."]
pub type LCDS33_R = crate::BitReader<bool>;
#[doc = "Field `LCDS33` writer - LCD Segment 33 enable."]
pub type LCDS33_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 1>;
#[doc = "Field `LCDS34` reader - LCD Segment 34 enable."]
pub type LCDS34_R = crate::BitReader<bool>;
#[doc = "Field `LCDS34` writer - LCD Segment 34 enable."]
pub type LCDS34_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 2>;
#[doc = "Field `LCDS35` reader - LCD Segment 35 enable."]
pub type LCDS35_R = crate::BitReader<bool>;
#[doc = "Field `LCDS35` writer - LCD Segment 35 enable."]
pub type LCDS35_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 3>;
#[doc = "Field `LCDS36` reader - LCD Segment 36 enable."]
pub type LCDS36_R = crate::BitReader<bool>;
#[doc = "Field `LCDS36` writer - LCD Segment 36 enable."]
pub type LCDS36_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 4>;
#[doc = "Field `LCDS37` reader - LCD Segment 37 enable."]
pub type LCDS37_R = crate::BitReader<bool>;
#[doc = "Field `LCDS37` writer - LCD Segment 37 enable."]
pub type LCDS37_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 5>;
#[doc = "Field `LCDS38` reader - LCD Segment 38 enable."]
pub type LCDS38_R = crate::BitReader<bool>;
#[doc = "Field `LCDS38` writer - LCD Segment 38 enable."]
pub type LCDS38_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 6>;
#[doc = "Field `LCDS39` reader - LCD Segment 39 enable."]
pub type LCDS39_R = crate::BitReader<bool>;
#[doc = "Field `LCDS39` writer - LCD Segment 39 enable."]
pub type LCDS39_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 7>;
#[doc = "Field `LCDS40` reader - LCD Segment 40 enable."]
pub type LCDS40_R = crate::BitReader<bool>;
#[doc = "Field `LCDS40` writer - LCD Segment 40 enable."]
pub type LCDS40_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 8>;
#[doc = "Field `LCDS41` reader - LCD Segment 41 enable."]
pub type LCDS41_R = crate::BitReader<bool>;
#[doc = "Field `LCDS41` writer - LCD Segment 41 enable."]
pub type LCDS41_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 9>;
#[doc = "Field `LCDS42` reader - LCD Segment 42 enable."]
pub type LCDS42_R = crate::BitReader<bool>;
#[doc = "Field `LCDS42` writer - LCD Segment 42 enable."]
pub type LCDS42_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 10>;
#[doc = "Field `LCDS43` reader - LCD Segment 43 enable."]
pub type LCDS43_R = crate::BitReader<bool>;
#[doc = "Field `LCDS43` writer - LCD Segment 43 enable."]
pub type LCDS43_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 11>;
#[doc = "Field `LCDS44` reader - LCD Segment 44 enable."]
pub type LCDS44_R = crate::BitReader<bool>;
#[doc = "Field `LCDS44` writer - LCD Segment 44 enable."]
pub type LCDS44_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 12>;
#[doc = "Field `LCDS45` reader - LCD Segment 45 enable."]
pub type LCDS45_R = crate::BitReader<bool>;
#[doc = "Field `LCDS45` writer - LCD Segment 45 enable."]
pub type LCDS45_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 13>;
#[doc = "Field `LCDS46` reader - LCD Segment 46 enable."]
pub type LCDS46_R = crate::BitReader<bool>;
#[doc = "Field `LCDS46` writer - LCD Segment 46 enable."]
pub type LCDS46_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 14>;
#[doc = "Field `LCDS47` reader - LCD Segment 47 enable."]
pub type LCDS47_R = crate::BitReader<bool>;
#[doc = "Field `LCDS47` writer - LCD Segment 47 enable."]
pub type LCDS47_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL2_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&self) -> LCDS32_R {
        LCDS32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&self) -> LCDS33_R {
        LCDS33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&self) -> LCDS34_R {
        LCDS34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&self) -> LCDS35_R {
        LCDS35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&self) -> LCDS36_R {
        LCDS36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&self) -> LCDS37_R {
        LCDS37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&self) -> LCDS38_R {
        LCDS38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&self) -> LCDS39_R {
        LCDS39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&self) -> LCDS40_R {
        LCDS40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&self) -> LCDS41_R {
        LCDS41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&self) -> LCDS42_R {
        LCDS42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&self) -> LCDS43_R {
        LCDS43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&self) -> LCDS44_R {
        LCDS44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&self) -> LCDS45_R {
        LCDS45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&self) -> LCDS46_R {
        LCDS46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&self) -> LCDS47_R {
        LCDS47_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&mut self) -> LCDS32_W {
        LCDS32_W::new(self)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&mut self) -> LCDS33_W {
        LCDS33_W::new(self)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&mut self) -> LCDS34_W {
        LCDS34_W::new(self)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&mut self) -> LCDS35_W {
        LCDS35_W::new(self)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&mut self) -> LCDS36_W {
        LCDS36_W::new(self)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&mut self) -> LCDS37_W {
        LCDS37_W::new(self)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&mut self) -> LCDS38_W {
        LCDS38_W::new(self)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&mut self) -> LCDS39_W {
        LCDS39_W::new(self)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&mut self) -> LCDS40_W {
        LCDS40_W::new(self)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&mut self) -> LCDS41_W {
        LCDS41_W::new(self)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&mut self) -> LCDS42_W {
        LCDS42_W::new(self)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&mut self) -> LCDS43_W {
        LCDS43_W::new(self)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&mut self) -> LCDS44_W {
        LCDS44_W::new(self)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&mut self) -> LCDS45_W {
        LCDS45_W::new(self)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&mut self) -> LCDS46_W {
        LCDS46_W::new(self)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&mut self) -> LCDS47_W {
        LCDS47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl2](index.html) module"]
pub struct LCDCPCTL2_SPEC;
impl crate::RegisterSpec for LCDCPCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcpctl2::R](R) reader structure"]
impl crate::Readable for LCDCPCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcpctl2::W](W) writer structure"]
impl crate::Writable for LCDCPCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCPCTL2 to value 0"]
impl crate::Resettable for LCDCPCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
