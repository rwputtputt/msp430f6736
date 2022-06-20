#[doc = "Register `UCB0IE_I2C` reader"]
pub struct R(crate::R<UCB0IE_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IE_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IE_I2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IE_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IE_I2C` writer"]
pub struct W(crate::W<UCB0IE_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IE_I2C_SPEC>;
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
impl From<crate::W<UCB0IE_I2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IE_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE0` reader - I2C Receive Interrupt Enable 0"]
pub type UCRXIE0_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE0` writer - I2C Receive Interrupt Enable 0"]
pub type UCRXIE0_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 0>;
#[doc = "Field `UCTXIE0` reader - I2C Transmit Interrupt Enable 0"]
pub type UCTXIE0_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE0` writer - I2C Transmit Interrupt Enable 0"]
pub type UCTXIE0_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 1>;
#[doc = "Field `UCSTTIE` reader - I2C START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTTIE` writer - I2C START Condition interrupt enable"]
pub type UCSTTIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 2>;
#[doc = "Field `UCSTPIE` reader - I2C STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTPIE` writer - I2C STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 3>;
#[doc = "Field `UCALIE` reader - I2C Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader<bool>;
#[doc = "Field `UCALIE` writer - I2C Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 4>;
#[doc = "Field `UCNACKIE` reader - I2C NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader<bool>;
#[doc = "Field `UCNACKIE` writer - I2C NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 5>;
#[doc = "Field `UCBCNTIE` reader - I2C Automatic stop assertion interrupt enable"]
pub type UCBCNTIE_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNTIE` writer - I2C Automatic stop assertion interrupt enable"]
pub type UCBCNTIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 6>;
#[doc = "Field `UCCLTOIE` reader - I2C Clock Low Timeout interrupt enable"]
pub type UCCLTOIE_R = crate::BitReader<bool>;
#[doc = "Field `UCCLTOIE` writer - I2C Clock Low Timeout interrupt enable"]
pub type UCCLTOIE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 7>;
#[doc = "Field `UCRXIE1` reader - I2C Receive Interrupt Enable 1"]
pub type UCRXIE1_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE1` writer - I2C Receive Interrupt Enable 1"]
pub type UCRXIE1_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 8>;
#[doc = "Field `UCTXIE1` reader - I2C Transmit Interrupt Enable 1"]
pub type UCTXIE1_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE1` writer - I2C Transmit Interrupt Enable 1"]
pub type UCTXIE1_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 9>;
#[doc = "Field `UCRXIE2` reader - I2C Receive Interrupt Enable 2"]
pub type UCRXIE2_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE2` writer - I2C Receive Interrupt Enable 2"]
pub type UCRXIE2_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 10>;
#[doc = "Field `UCTXIE2` reader - I2C Transmit Interrupt Enable 2"]
pub type UCTXIE2_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE2` writer - I2C Transmit Interrupt Enable 2"]
pub type UCTXIE2_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 11>;
#[doc = "Field `UCRXIE3` reader - I2C Receive Interrupt Enable 3"]
pub type UCRXIE3_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIE3` writer - I2C Receive Interrupt Enable 3"]
pub type UCRXIE3_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 12>;
#[doc = "Field `UCTXIE3` reader - I2C Transmit Interrupt Enable 3"]
pub type UCTXIE3_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIE3` writer - I2C Transmit Interrupt Enable 3"]
pub type UCTXIE3_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 13>;
#[doc = "Field `UCBIT9IE` reader - I2C Bit 9 Position Interrupt Enable 3"]
pub type UCBIT9IE_R = crate::BitReader<bool>;
#[doc = "Field `UCBIT9IE` writer - I2C Bit 9 Position Interrupt Enable 3"]
pub type UCBIT9IE_W<'a> = crate::BitWriter<'a, u16, UCB0IE_I2C_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W {
        UCRXIE0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> UCTXIE0_W {
        UCTXIE0_W::new(self)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W::new(self)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W::new(self)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W::new(self)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W::new(self)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W {
        UCBCNTIE_W::new(self)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W {
        UCCLTOIE_W::new(self)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W {
        UCRXIE1_W::new(self)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> UCTXIE1_W {
        UCTXIE1_W::new(self)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W {
        UCRXIE2_W::new(self)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> UCTXIE2_W {
        UCTXIE2_W::new(self)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W {
        UCRXIE3_W::new(self)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> UCTXIE3_W {
        UCTXIE3_W::new(self)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W {
        UCBIT9IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ie_i2c](index.html) module"]
pub struct UCB0IE_I2C_SPEC;
impl crate::RegisterSpec for UCB0IE_I2C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ie_i2c::R](R) reader structure"]
impl crate::Readable for UCB0IE_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ie_i2c::W](W) writer structure"]
impl crate::Writable for UCB0IE_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IE_I2C to value 0"]
impl crate::Resettable for UCB0IE_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
