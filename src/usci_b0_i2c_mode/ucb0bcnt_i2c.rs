#[doc = "Register `UCB0BCNT_I2C` reader"]
pub struct R(crate::R<UCB0BCNT_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0BCNT_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0BCNT_I2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0BCNT_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0BCNT_I2C` writer"]
pub struct W(crate::W<UCB0BCNT_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0BCNT_I2C_SPEC>;
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
impl From<crate::W<UCB0BCNT_I2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0BCNT_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBCNT0` reader - USCI Byte Counter Bit 0"]
pub type UCBCNT0_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT0` writer - USCI Byte Counter Bit 0"]
pub type UCBCNT0_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 0>;
#[doc = "Field `UCBCNT1` reader - USCI Byte Counter Bit 1"]
pub type UCBCNT1_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT1` writer - USCI Byte Counter Bit 1"]
pub type UCBCNT1_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 1>;
#[doc = "Field `UCBCNT2` reader - USCI Byte Counter Bit 2"]
pub type UCBCNT2_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT2` writer - USCI Byte Counter Bit 2"]
pub type UCBCNT2_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 2>;
#[doc = "Field `UCBCNT3` reader - USCI Byte Counter Bit 3"]
pub type UCBCNT3_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT3` writer - USCI Byte Counter Bit 3"]
pub type UCBCNT3_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 3>;
#[doc = "Field `UCBCNT4` reader - USCI Byte Counter Bit 4"]
pub type UCBCNT4_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT4` writer - USCI Byte Counter Bit 4"]
pub type UCBCNT4_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 4>;
#[doc = "Field `UCBCNT5` reader - USCI Byte Counter Bit 5"]
pub type UCBCNT5_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT5` writer - USCI Byte Counter Bit 5"]
pub type UCBCNT5_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 5>;
#[doc = "Field `UCBCNT6` reader - USCI Byte Counter Bit 6"]
pub type UCBCNT6_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT6` writer - USCI Byte Counter Bit 6"]
pub type UCBCNT6_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 6>;
#[doc = "Field `UCBCNT7` reader - USCI Byte Counter Bit 7"]
pub type UCBCNT7_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNT7` writer - USCI Byte Counter Bit 7"]
pub type UCBCNT7_W<'a> = crate::BitWriter<'a, u8, UCB0BCNT_I2C_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&self) -> UCBCNT0_R {
        UCBCNT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&self) -> UCBCNT1_R {
        UCBCNT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&self) -> UCBCNT2_R {
        UCBCNT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&self) -> UCBCNT3_R {
        UCBCNT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&self) -> UCBCNT4_R {
        UCBCNT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&self) -> UCBCNT5_R {
        UCBCNT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&self) -> UCBCNT6_R {
        UCBCNT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&self) -> UCBCNT7_R {
        UCBCNT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&mut self) -> UCBCNT0_W {
        UCBCNT0_W::new(self)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&mut self) -> UCBCNT1_W {
        UCBCNT1_W::new(self)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&mut self) -> UCBCNT2_W {
        UCBCNT2_W::new(self)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&mut self) -> UCBCNT3_W {
        UCBCNT3_W::new(self)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&mut self) -> UCBCNT4_W {
        UCBCNT4_W::new(self)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&mut self) -> UCBCNT5_W {
        UCBCNT5_W::new(self)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&mut self) -> UCBCNT6_W {
        UCBCNT6_W::new(self)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&mut self) -> UCBCNT7_W {
        UCBCNT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Byte Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0bcnt_i2c](index.html) module"]
pub struct UCB0BCNT_I2C_SPEC;
impl crate::RegisterSpec for UCB0BCNT_I2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0bcnt_i2c::R](R) reader structure"]
impl crate::Readable for UCB0BCNT_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0bcnt_i2c::W](W) writer structure"]
impl crate::Writable for UCB0BCNT_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0BCNT_I2C to value 0"]
impl crate::Resettable for UCB0BCNT_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
