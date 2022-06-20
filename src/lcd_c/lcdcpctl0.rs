#[doc = "Register `LCDCPCTL0` reader"]
pub struct R(crate::R<LCDCPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCPCTL0` writer"]
pub struct W(crate::W<LCDCPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCPCTL0_SPEC>;
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
impl From<crate::W<LCDCPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS0` reader - LCD Segment 0 enable."]
pub type LCDS0_R = crate::BitReader<bool>;
#[doc = "Field `LCDS0` writer - LCD Segment 0 enable."]
pub type LCDS0_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 0>;
#[doc = "Field `LCDS1` reader - LCD Segment 1 enable."]
pub type LCDS1_R = crate::BitReader<bool>;
#[doc = "Field `LCDS1` writer - LCD Segment 1 enable."]
pub type LCDS1_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 1>;
#[doc = "Field `LCDS2` reader - LCD Segment 2 enable."]
pub type LCDS2_R = crate::BitReader<bool>;
#[doc = "Field `LCDS2` writer - LCD Segment 2 enable."]
pub type LCDS2_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 2>;
#[doc = "Field `LCDS3` reader - LCD Segment 3 enable."]
pub type LCDS3_R = crate::BitReader<bool>;
#[doc = "Field `LCDS3` writer - LCD Segment 3 enable."]
pub type LCDS3_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 3>;
#[doc = "Field `LCDS4` reader - LCD Segment 4 enable."]
pub type LCDS4_R = crate::BitReader<bool>;
#[doc = "Field `LCDS4` writer - LCD Segment 4 enable."]
pub type LCDS4_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 4>;
#[doc = "Field `LCDS5` reader - LCD Segment 5 enable."]
pub type LCDS5_R = crate::BitReader<bool>;
#[doc = "Field `LCDS5` writer - LCD Segment 5 enable."]
pub type LCDS5_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 5>;
#[doc = "Field `LCDS6` reader - LCD Segment 6 enable."]
pub type LCDS6_R = crate::BitReader<bool>;
#[doc = "Field `LCDS6` writer - LCD Segment 6 enable."]
pub type LCDS6_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 6>;
#[doc = "Field `LCDS7` reader - LCD Segment 7 enable."]
pub type LCDS7_R = crate::BitReader<bool>;
#[doc = "Field `LCDS7` writer - LCD Segment 7 enable."]
pub type LCDS7_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 7>;
#[doc = "Field `LCDS8` reader - LCD Segment 8 enable."]
pub type LCDS8_R = crate::BitReader<bool>;
#[doc = "Field `LCDS8` writer - LCD Segment 8 enable."]
pub type LCDS8_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 8>;
#[doc = "Field `LCDS9` reader - LCD Segment 9 enable."]
pub type LCDS9_R = crate::BitReader<bool>;
#[doc = "Field `LCDS9` writer - LCD Segment 9 enable."]
pub type LCDS9_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 9>;
#[doc = "Field `LCDS10` reader - LCD Segment 10 enable."]
pub type LCDS10_R = crate::BitReader<bool>;
#[doc = "Field `LCDS10` writer - LCD Segment 10 enable."]
pub type LCDS10_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 10>;
#[doc = "Field `LCDS11` reader - LCD Segment 11 enable."]
pub type LCDS11_R = crate::BitReader<bool>;
#[doc = "Field `LCDS11` writer - LCD Segment 11 enable."]
pub type LCDS11_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 11>;
#[doc = "Field `LCDS12` reader - LCD Segment 12 enable."]
pub type LCDS12_R = crate::BitReader<bool>;
#[doc = "Field `LCDS12` writer - LCD Segment 12 enable."]
pub type LCDS12_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 12>;
#[doc = "Field `LCDS13` reader - LCD Segment 13 enable."]
pub type LCDS13_R = crate::BitReader<bool>;
#[doc = "Field `LCDS13` writer - LCD Segment 13 enable."]
pub type LCDS13_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 13>;
#[doc = "Field `LCDS14` reader - LCD Segment 14 enable."]
pub type LCDS14_R = crate::BitReader<bool>;
#[doc = "Field `LCDS14` writer - LCD Segment 14 enable."]
pub type LCDS14_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 14>;
#[doc = "Field `LCDS15` reader - LCD Segment 15 enable."]
pub type LCDS15_R = crate::BitReader<bool>;
#[doc = "Field `LCDS15` writer - LCD Segment 15 enable."]
pub type LCDS15_W<'a> = crate::BitWriter<'a, u16, LCDCPCTL0_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&self) -> LCDS0_R {
        LCDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&self) -> LCDS1_R {
        LCDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&self) -> LCDS2_R {
        LCDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&self) -> LCDS3_R {
        LCDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&self) -> LCDS4_R {
        LCDS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&self) -> LCDS5_R {
        LCDS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&self) -> LCDS6_R {
        LCDS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&self) -> LCDS7_R {
        LCDS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&self) -> LCDS8_R {
        LCDS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&self) -> LCDS9_R {
        LCDS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&self) -> LCDS10_R {
        LCDS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&self) -> LCDS11_R {
        LCDS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&self) -> LCDS12_R {
        LCDS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&self) -> LCDS13_R {
        LCDS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&self) -> LCDS14_R {
        LCDS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&self) -> LCDS15_R {
        LCDS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&mut self) -> LCDS0_W {
        LCDS0_W::new(self)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&mut self) -> LCDS1_W {
        LCDS1_W::new(self)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&mut self) -> LCDS2_W {
        LCDS2_W::new(self)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&mut self) -> LCDS3_W {
        LCDS3_W::new(self)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&mut self) -> LCDS4_W {
        LCDS4_W::new(self)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&mut self) -> LCDS5_W {
        LCDS5_W::new(self)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&mut self) -> LCDS6_W {
        LCDS6_W::new(self)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&mut self) -> LCDS7_W {
        LCDS7_W::new(self)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&mut self) -> LCDS8_W {
        LCDS8_W::new(self)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&mut self) -> LCDS9_W {
        LCDS9_W::new(self)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&mut self) -> LCDS10_W {
        LCDS10_W::new(self)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&mut self) -> LCDS11_W {
        LCDS11_W::new(self)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&mut self) -> LCDS12_W {
        LCDS12_W::new(self)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&mut self) -> LCDS13_W {
        LCDS13_W::new(self)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&mut self) -> LCDS14_W {
        LCDS14_W::new(self)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&mut self) -> LCDS15_W {
        LCDS15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Port Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl0](index.html) module"]
pub struct LCDCPCTL0_SPEC;
impl crate::RegisterSpec for LCDCPCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcpctl0::R](R) reader structure"]
impl crate::Readable for LCDCPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcpctl0::W](W) writer structure"]
impl crate::Writable for LCDCPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCPCTL0 to value 0"]
impl crate::Resettable for LCDCPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
