#[doc = "Register `UCB0STAT_I2C` reader"]
pub struct R(crate::R<UCB0STAT_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0STAT_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0STAT_I2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0STAT_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0STAT_I2C` writer"]
pub struct W(crate::W<UCB0STAT_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0STAT_I2C_SPEC>;
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
impl From<crate::W<UCB0STAT_I2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0STAT_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UCBBUSY_R = crate::BitReader<bool>;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UCBBUSY_W<'a> = crate::BitWriter<'a, u8, UCB0STAT_I2C_SPEC, bool, 4>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UCGC_R = crate::BitReader<bool>;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UCGC_W<'a> = crate::BitWriter<'a, u8, UCB0STAT_I2C_SPEC, bool, 5>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader<bool>;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UCSCLLOW_W<'a> = crate::BitWriter<'a, u8, UCB0STAT_I2C_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W {
        UCBBUSY_W::new(self)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UCGC_W {
        UCGC_W::new(self)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W {
        UCSCLLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat_i2c](index.html) module"]
pub struct UCB0STAT_I2C_SPEC;
impl crate::RegisterSpec for UCB0STAT_I2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0stat_i2c::R](R) reader structure"]
impl crate::Readable for UCB0STAT_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0stat_i2c::W](W) writer structure"]
impl crate::Writable for UCB0STAT_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0STAT_I2C to value 0"]
impl crate::Resettable for UCB0STAT_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
