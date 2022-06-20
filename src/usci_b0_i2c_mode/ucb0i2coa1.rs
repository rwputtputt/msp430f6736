#[doc = "Register `UCB0I2COA1` reader"]
pub struct R(crate::R<UCB0I2COA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2COA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2COA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0I2COA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2COA1` writer"]
pub struct W(crate::W<UCB0I2COA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2COA1_SPEC>;
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
impl From<crate::W<UCB0I2COA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2COA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOA0` reader - I2C Own Address Bit 0"]
pub type UCOA0_R = crate::BitReader<bool>;
#[doc = "Field `UCOA0` writer - I2C Own Address Bit 0"]
pub type UCOA0_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 0>;
#[doc = "Field `UCOA1` reader - I2C Own Address Bit 1"]
pub type UCOA1_R = crate::BitReader<bool>;
#[doc = "Field `UCOA1` writer - I2C Own Address Bit 1"]
pub type UCOA1_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 1>;
#[doc = "Field `UCOA2` reader - I2C Own Address Bit 2"]
pub type UCOA2_R = crate::BitReader<bool>;
#[doc = "Field `UCOA2` writer - I2C Own Address Bit 2"]
pub type UCOA2_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 2>;
#[doc = "Field `UCOA3` reader - I2C Own Address Bit 3"]
pub type UCOA3_R = crate::BitReader<bool>;
#[doc = "Field `UCOA3` writer - I2C Own Address Bit 3"]
pub type UCOA3_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 3>;
#[doc = "Field `UCOA4` reader - I2C Own Address Bit 4"]
pub type UCOA4_R = crate::BitReader<bool>;
#[doc = "Field `UCOA4` writer - I2C Own Address Bit 4"]
pub type UCOA4_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 4>;
#[doc = "Field `UCOA5` reader - I2C Own Address Bit 5"]
pub type UCOA5_R = crate::BitReader<bool>;
#[doc = "Field `UCOA5` writer - I2C Own Address Bit 5"]
pub type UCOA5_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 5>;
#[doc = "Field `UCOA6` reader - I2C Own Address Bit 6"]
pub type UCOA6_R = crate::BitReader<bool>;
#[doc = "Field `UCOA6` writer - I2C Own Address Bit 6"]
pub type UCOA6_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 6>;
#[doc = "Field `UCOA7` reader - I2C Own Address Bit 7"]
pub type UCOA7_R = crate::BitReader<bool>;
#[doc = "Field `UCOA7` writer - I2C Own Address Bit 7"]
pub type UCOA7_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 7>;
#[doc = "Field `UCOA8` reader - I2C Own Address Bit 8"]
pub type UCOA8_R = crate::BitReader<bool>;
#[doc = "Field `UCOA8` writer - I2C Own Address Bit 8"]
pub type UCOA8_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 8>;
#[doc = "Field `UCOA9` reader - I2C Own Address Bit 9"]
pub type UCOA9_R = crate::BitReader<bool>;
#[doc = "Field `UCOA9` writer - I2C Own Address Bit 9"]
pub type UCOA9_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 9>;
#[doc = "Field `UCOAEN` reader - I2C Own Address enable"]
pub type UCOAEN_R = crate::BitReader<bool>;
#[doc = "Field `UCOAEN` writer - I2C Own Address enable"]
pub type UCOAEN_W<'a> = crate::BitWriter<'a, u16, UCB0I2COA1_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn ucoa0(&self) -> UCOA0_R {
        UCOA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Own Address Bit 1"]
    #[inline(always)]
    pub fn ucoa1(&self) -> UCOA1_R {
        UCOA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Own Address Bit 2"]
    #[inline(always)]
    pub fn ucoa2(&self) -> UCOA2_R {
        UCOA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Own Address Bit 3"]
    #[inline(always)]
    pub fn ucoa3(&self) -> UCOA3_R {
        UCOA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Own Address Bit 4"]
    #[inline(always)]
    pub fn ucoa4(&self) -> UCOA4_R {
        UCOA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Own Address Bit 5"]
    #[inline(always)]
    pub fn ucoa5(&self) -> UCOA5_R {
        UCOA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Own Address Bit 6"]
    #[inline(always)]
    pub fn ucoa6(&self) -> UCOA6_R {
        UCOA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Own Address Bit 7"]
    #[inline(always)]
    pub fn ucoa7(&self) -> UCOA7_R {
        UCOA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Own Address Bit 8"]
    #[inline(always)]
    pub fn ucoa8(&self) -> UCOA8_R {
        UCOA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Own Address Bit 9"]
    #[inline(always)]
    pub fn ucoa9(&self) -> UCOA9_R {
        UCOA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UCOAEN_R {
        UCOAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Own Address Bit 0"]
    #[inline(always)]
    pub fn ucoa0(&mut self) -> UCOA0_W {
        UCOA0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Own Address Bit 1"]
    #[inline(always)]
    pub fn ucoa1(&mut self) -> UCOA1_W {
        UCOA1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Own Address Bit 2"]
    #[inline(always)]
    pub fn ucoa2(&mut self) -> UCOA2_W {
        UCOA2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Own Address Bit 3"]
    #[inline(always)]
    pub fn ucoa3(&mut self) -> UCOA3_W {
        UCOA3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Own Address Bit 4"]
    #[inline(always)]
    pub fn ucoa4(&mut self) -> UCOA4_W {
        UCOA4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Own Address Bit 5"]
    #[inline(always)]
    pub fn ucoa5(&mut self) -> UCOA5_W {
        UCOA5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Own Address Bit 6"]
    #[inline(always)]
    pub fn ucoa6(&mut self) -> UCOA6_W {
        UCOA6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Own Address Bit 7"]
    #[inline(always)]
    pub fn ucoa7(&mut self) -> UCOA7_W {
        UCOA7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Own Address Bit 8"]
    #[inline(always)]
    pub fn ucoa8(&mut self) -> UCOA8_W {
        UCOA8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Own Address Bit 9"]
    #[inline(always)]
    pub fn ucoa9(&mut self) -> UCOA9_W {
        UCOA9_W::new(self)
    }
    #[doc = "Bit 10 - I2C Own Address enable"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UCOAEN_W {
        UCOAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Own Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa1](index.html) module"]
pub struct UCB0I2COA1_SPEC;
impl crate::RegisterSpec for UCB0I2COA1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2coa1::R](R) reader structure"]
impl crate::Readable for UCB0I2COA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa1::W](W) writer structure"]
impl crate::Writable for UCB0I2COA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2COA1 to value 0"]
impl crate::Resettable for UCB0I2COA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
