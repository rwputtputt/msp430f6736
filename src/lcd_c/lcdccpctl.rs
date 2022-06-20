#[doc = "Register `LCDCCPCTL` reader"]
pub struct R(crate::R<LCDCCPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCCPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCCPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCCPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCCPCTL` writer"]
pub struct W(crate::W<LCDCCPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCCPCTL_SPEC>;
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
impl From<crate::W<LCDCCPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCCPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCPDIS0` reader - LCD charge pump disable"]
pub type LCDCPDIS0_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS0` writer - LCD charge pump disable"]
pub type LCDCPDIS0_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 0>;
#[doc = "Field `LCDCPDIS1` reader - LCD charge pump disable"]
pub type LCDCPDIS1_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS1` writer - LCD charge pump disable"]
pub type LCDCPDIS1_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 1>;
#[doc = "Field `LCDCPDIS2` reader - LCD charge pump disable"]
pub type LCDCPDIS2_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS2` writer - LCD charge pump disable"]
pub type LCDCPDIS2_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 2>;
#[doc = "Field `LCDCPDIS3` reader - LCD charge pump disable"]
pub type LCDCPDIS3_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS3` writer - LCD charge pump disable"]
pub type LCDCPDIS3_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 3>;
#[doc = "Field `LCDCPDIS4` reader - LCD charge pump disable"]
pub type LCDCPDIS4_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS4` writer - LCD charge pump disable"]
pub type LCDCPDIS4_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 4>;
#[doc = "Field `LCDCPDIS5` reader - LCD charge pump disable"]
pub type LCDCPDIS5_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS5` writer - LCD charge pump disable"]
pub type LCDCPDIS5_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 5>;
#[doc = "Field `LCDCPDIS6` reader - LCD charge pump disable"]
pub type LCDCPDIS6_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS6` writer - LCD charge pump disable"]
pub type LCDCPDIS6_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 6>;
#[doc = "Field `LCDCPDIS7` reader - LCD charge pump disable"]
pub type LCDCPDIS7_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPDIS7` writer - LCD charge pump disable"]
pub type LCDCPDIS7_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 7>;
#[doc = "Field `LCDCPCLKSYNC` reader - LCD charge pump clock synchronization"]
pub type LCDCPCLKSYNC_R = crate::BitReader<bool>;
#[doc = "Field `LCDCPCLKSYNC` writer - LCD charge pump clock synchronization"]
pub type LCDCPCLKSYNC_W<'a> = crate::BitWriter<'a, u16, LCDCCPCTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&self) -> LCDCPDIS0_R {
        LCDCPDIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&self) -> LCDCPDIS1_R {
        LCDCPDIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&self) -> LCDCPDIS2_R {
        LCDCPDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&self) -> LCDCPDIS3_R {
        LCDCPDIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&self) -> LCDCPDIS4_R {
        LCDCPDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&self) -> LCDCPDIS5_R {
        LCDCPDIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&self) -> LCDCPDIS6_R {
        LCDCPDIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&self) -> LCDCPDIS7_R {
        LCDCPDIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&self) -> LCDCPCLKSYNC_R {
        LCDCPCLKSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&mut self) -> LCDCPDIS0_W {
        LCDCPDIS0_W::new(self)
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&mut self) -> LCDCPDIS1_W {
        LCDCPDIS1_W::new(self)
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&mut self) -> LCDCPDIS2_W {
        LCDCPDIS2_W::new(self)
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&mut self) -> LCDCPDIS3_W {
        LCDCPDIS3_W::new(self)
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&mut self) -> LCDCPDIS4_W {
        LCDCPDIS4_W::new(self)
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&mut self) -> LCDCPDIS5_W {
        LCDCPDIS5_W::new(self)
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&mut self) -> LCDCPDIS6_W {
        LCDCPDIS6_W::new(self)
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&mut self) -> LCDCPDIS7_W {
        LCDCPDIS7_W::new(self)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&mut self) -> LCDCPCLKSYNC_W {
        LCDCPCLKSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Charge Pump Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdccpctl](index.html) module"]
pub struct LCDCCPCTL_SPEC;
impl crate::RegisterSpec for LCDCCPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdccpctl::R](R) reader structure"]
impl crate::Readable for LCDCCPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdccpctl::W](W) writer structure"]
impl crate::Writable for LCDCCPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCCPCTL to value 0"]
impl crate::Resettable for LCDCCPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
