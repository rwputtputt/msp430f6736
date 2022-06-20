#[doc = "Register `UCB0ADDMASK` reader"]
pub struct R(crate::R<UCB0ADDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0ADDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0ADDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0ADDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0ADDMASK` writer"]
pub struct W(crate::W<UCB0ADDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0ADDMASK_SPEC>;
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
impl From<crate::W<UCB0ADDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0ADDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCADDMASK0` reader - I2C Address Mask Bit 0"]
pub type UCADDMASK0_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK0` writer - I2C Address Mask Bit 0"]
pub type UCADDMASK0_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 0>;
#[doc = "Field `UCADDMASK1` reader - I2C Address Mask Bit 1"]
pub type UCADDMASK1_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK1` writer - I2C Address Mask Bit 1"]
pub type UCADDMASK1_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 1>;
#[doc = "Field `UCADDMASK2` reader - I2C Address Mask Bit 2"]
pub type UCADDMASK2_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK2` writer - I2C Address Mask Bit 2"]
pub type UCADDMASK2_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 2>;
#[doc = "Field `UCADDMASK3` reader - I2C Address Mask Bit 3"]
pub type UCADDMASK3_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK3` writer - I2C Address Mask Bit 3"]
pub type UCADDMASK3_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 3>;
#[doc = "Field `UCADDMASK4` reader - I2C Address Mask Bit 4"]
pub type UCADDMASK4_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK4` writer - I2C Address Mask Bit 4"]
pub type UCADDMASK4_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 4>;
#[doc = "Field `UCADDMASK5` reader - I2C Address Mask Bit 5"]
pub type UCADDMASK5_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK5` writer - I2C Address Mask Bit 5"]
pub type UCADDMASK5_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 5>;
#[doc = "Field `UCADDMASK6` reader - I2C Address Mask Bit 6"]
pub type UCADDMASK6_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK6` writer - I2C Address Mask Bit 6"]
pub type UCADDMASK6_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 6>;
#[doc = "Field `UCADDMASK7` reader - I2C Address Mask Bit 7"]
pub type UCADDMASK7_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK7` writer - I2C Address Mask Bit 7"]
pub type UCADDMASK7_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 7>;
#[doc = "Field `UCADDMASK8` reader - I2C Address Mask Bit 8"]
pub type UCADDMASK8_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK8` writer - I2C Address Mask Bit 8"]
pub type UCADDMASK8_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 8>;
#[doc = "Field `UCADDMASK9` reader - I2C Address Mask Bit 9"]
pub type UCADDMASK9_R = crate::BitReader<bool>;
#[doc = "Field `UCADDMASK9` writer - I2C Address Mask Bit 9"]
pub type UCADDMASK9_W<'a> = crate::BitWriter<'a, u16, UCB0ADDMASK_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&self) -> UCADDMASK0_R {
        UCADDMASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&self) -> UCADDMASK1_R {
        UCADDMASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&self) -> UCADDMASK2_R {
        UCADDMASK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&self) -> UCADDMASK3_R {
        UCADDMASK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&self) -> UCADDMASK4_R {
        UCADDMASK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&self) -> UCADDMASK5_R {
        UCADDMASK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&self) -> UCADDMASK6_R {
        UCADDMASK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&self) -> UCADDMASK7_R {
        UCADDMASK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&self) -> UCADDMASK8_R {
        UCADDMASK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&self) -> UCADDMASK9_R {
        UCADDMASK9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&mut self) -> UCADDMASK0_W {
        UCADDMASK0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&mut self) -> UCADDMASK1_W {
        UCADDMASK1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&mut self) -> UCADDMASK2_W {
        UCADDMASK2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&mut self) -> UCADDMASK3_W {
        UCADDMASK3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&mut self) -> UCADDMASK4_W {
        UCADDMASK4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&mut self) -> UCADDMASK5_W {
        UCADDMASK5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&mut self) -> UCADDMASK6_W {
        UCADDMASK6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&mut self) -> UCADDMASK7_W {
        UCADDMASK7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&mut self) -> UCADDMASK8_W {
        UCADDMASK8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&mut self) -> UCADDMASK9_W {
        UCADDMASK9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addmask](index.html) module"]
pub struct UCB0ADDMASK_SPEC;
impl crate::RegisterSpec for UCB0ADDMASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0addmask::R](R) reader structure"]
impl crate::Readable for UCB0ADDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0addmask::W](W) writer structure"]
impl crate::Writable for UCB0ADDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0ADDMASK to value 0"]
impl crate::Resettable for UCB0ADDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
