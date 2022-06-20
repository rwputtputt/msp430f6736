#[doc = "Register `UCB0ADDRX` reader"]
pub struct R(crate::R<UCB0ADDRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0ADDRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0ADDRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0ADDRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0ADDRX` writer"]
pub struct W(crate::W<UCB0ADDRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0ADDRX_SPEC>;
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
impl From<crate::W<UCB0ADDRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0ADDRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCADDRX0` reader - I2C Receive Address Bit 0"]
pub type UCADDRX0_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX0` writer - I2C Receive Address Bit 0"]
pub type UCADDRX0_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 0>;
#[doc = "Field `UCADDRX1` reader - I2C Receive Address Bit 1"]
pub type UCADDRX1_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX1` writer - I2C Receive Address Bit 1"]
pub type UCADDRX1_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 1>;
#[doc = "Field `UCADDRX2` reader - I2C Receive Address Bit 2"]
pub type UCADDRX2_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX2` writer - I2C Receive Address Bit 2"]
pub type UCADDRX2_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 2>;
#[doc = "Field `UCADDRX3` reader - I2C Receive Address Bit 3"]
pub type UCADDRX3_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX3` writer - I2C Receive Address Bit 3"]
pub type UCADDRX3_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 3>;
#[doc = "Field `UCADDRX4` reader - I2C Receive Address Bit 4"]
pub type UCADDRX4_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX4` writer - I2C Receive Address Bit 4"]
pub type UCADDRX4_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 4>;
#[doc = "Field `UCADDRX5` reader - I2C Receive Address Bit 5"]
pub type UCADDRX5_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX5` writer - I2C Receive Address Bit 5"]
pub type UCADDRX5_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 5>;
#[doc = "Field `UCADDRX6` reader - I2C Receive Address Bit 6"]
pub type UCADDRX6_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX6` writer - I2C Receive Address Bit 6"]
pub type UCADDRX6_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 6>;
#[doc = "Field `UCADDRX7` reader - I2C Receive Address Bit 7"]
pub type UCADDRX7_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX7` writer - I2C Receive Address Bit 7"]
pub type UCADDRX7_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 7>;
#[doc = "Field `UCADDRX8` reader - I2C Receive Address Bit 8"]
pub type UCADDRX8_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX8` writer - I2C Receive Address Bit 8"]
pub type UCADDRX8_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 8>;
#[doc = "Field `UCADDRX9` reader - I2C Receive Address Bit 9"]
pub type UCADDRX9_R = crate::BitReader<bool>;
#[doc = "Field `UCADDRX9` writer - I2C Receive Address Bit 9"]
pub type UCADDRX9_W<'a> = crate::BitWriter<'a, u16, UCB0ADDRX_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&self) -> UCADDRX0_R {
        UCADDRX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&self) -> UCADDRX1_R {
        UCADDRX1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&self) -> UCADDRX2_R {
        UCADDRX2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&self) -> UCADDRX3_R {
        UCADDRX3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&self) -> UCADDRX4_R {
        UCADDRX4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&self) -> UCADDRX5_R {
        UCADDRX5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&self) -> UCADDRX6_R {
        UCADDRX6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&self) -> UCADDRX7_R {
        UCADDRX7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&self) -> UCADDRX8_R {
        UCADDRX8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&self) -> UCADDRX9_R {
        UCADDRX9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&mut self) -> UCADDRX0_W {
        UCADDRX0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&mut self) -> UCADDRX1_W {
        UCADDRX1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&mut self) -> UCADDRX2_W {
        UCADDRX2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&mut self) -> UCADDRX3_W {
        UCADDRX3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&mut self) -> UCADDRX4_W {
        UCADDRX4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&mut self) -> UCADDRX5_W {
        UCADDRX5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&mut self) -> UCADDRX6_W {
        UCADDRX6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&mut self) -> UCADDRX7_W {
        UCADDRX7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&mut self) -> UCADDRX8_W {
        UCADDRX8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&mut self) -> UCADDRX9_W {
        UCADDRX9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addrx](index.html) module"]
pub struct UCB0ADDRX_SPEC;
impl crate::RegisterSpec for UCB0ADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0addrx::R](R) reader structure"]
impl crate::Readable for UCB0ADDRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0addrx::W](W) writer structure"]
impl crate::Writable for UCB0ADDRX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0ADDRX to value 0"]
impl crate::Resettable for UCB0ADDRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
