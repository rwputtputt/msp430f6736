#[doc = "Register `UCB0IFG_I2C` reader"]
pub struct R(crate::R<UCB0IFG_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IFG_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IFG_I2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IFG_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IFG_I2C` writer"]
pub struct W(crate::W<UCB0IFG_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IFG_I2C_SPEC>;
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
impl From<crate::W<UCB0IFG_I2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IFG_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG0` reader - I2C Receive Interrupt Flag 0"]
pub type UCRXIFG0_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG0` writer - I2C Receive Interrupt Flag 0"]
pub type UCRXIFG0_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 0>;
#[doc = "Field `UCTXIFG0` reader - I2C Transmit Interrupt Flag 0"]
pub type UCTXIFG0_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG0` writer - I2C Transmit Interrupt Flag 0"]
pub type UCTXIFG0_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 1>;
#[doc = "Field `UCSTTIFG` reader - I2C START Condition interrupt Flag"]
pub type UCSTTIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCSTTIFG` writer - I2C START Condition interrupt Flag"]
pub type UCSTTIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 2>;
#[doc = "Field `UCSTPIFG` reader - I2C STOP Condition interrupt Flag"]
pub type UCSTPIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCSTPIFG` writer - I2C STOP Condition interrupt Flag"]
pub type UCSTPIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 3>;
#[doc = "Field `UCALIFG` reader - I2C Arbitration Lost interrupt Flag"]
pub type UCALIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCALIFG` writer - I2C Arbitration Lost interrupt Flag"]
pub type UCALIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 4>;
#[doc = "Field `UCNACKIFG` reader - I2C NACK Condition interrupt Flag"]
pub type UCNACKIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCNACKIFG` writer - I2C NACK Condition interrupt Flag"]
pub type UCNACKIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 5>;
#[doc = "Field `UCBCNTIFG` reader - I2C Byte counter interrupt flag"]
pub type UCBCNTIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCBCNTIFG` writer - I2C Byte counter interrupt flag"]
pub type UCBCNTIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 6>;
#[doc = "Field `UCCLTOIFG` reader - I2C Clock low Timeout interrupt Flag"]
pub type UCCLTOIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCCLTOIFG` writer - I2C Clock low Timeout interrupt Flag"]
pub type UCCLTOIFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 7>;
#[doc = "Field `UCRXIFG1` reader - I2C Receive Interrupt Flag 1"]
pub type UCRXIFG1_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG1` writer - I2C Receive Interrupt Flag 1"]
pub type UCRXIFG1_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 8>;
#[doc = "Field `UCTXIFG1` reader - I2C Transmit Interrupt Flag 1"]
pub type UCTXIFG1_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG1` writer - I2C Transmit Interrupt Flag 1"]
pub type UCTXIFG1_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 9>;
#[doc = "Field `UCRXIFG2` reader - I2C Receive Interrupt Flag 2"]
pub type UCRXIFG2_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG2` writer - I2C Receive Interrupt Flag 2"]
pub type UCRXIFG2_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 10>;
#[doc = "Field `UCTXIFG2` reader - I2C Transmit Interrupt Flag 2"]
pub type UCTXIFG2_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG2` writer - I2C Transmit Interrupt Flag 2"]
pub type UCTXIFG2_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 11>;
#[doc = "Field `UCRXIFG3` reader - I2C Receive Interrupt Flag 3"]
pub type UCRXIFG3_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG3` writer - I2C Receive Interrupt Flag 3"]
pub type UCRXIFG3_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 12>;
#[doc = "Field `UCTXIFG3` reader - I2C Transmit Interrupt Flag 3"]
pub type UCTXIFG3_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG3` writer - I2C Transmit Interrupt Flag 3"]
pub type UCTXIFG3_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 13>;
#[doc = "Field `UCBIT9IFG` reader - I2C Bit 9 Possition Interrupt Flag 3"]
pub type UCBIT9IFG_R = crate::BitReader<bool>;
#[doc = "Field `UCBIT9IFG` writer - I2C Bit 9 Possition Interrupt Flag 3"]
pub type UCBIT9IFG_W<'a> = crate::BitWriter<'a, u16, UCB0IFG_I2C_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> UCRXIFG0_R {
        UCRXIFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> UCTXIFG0_R {
        UCTXIFG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UCBCNTIFG_R {
        UCBCNTIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UCCLTOIFG_R {
        UCCLTOIFG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> UCRXIFG1_R {
        UCRXIFG1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> UCTXIFG1_R {
        UCTXIFG1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> UCRXIFG2_R {
        UCRXIFG2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> UCTXIFG2_R {
        UCTXIFG2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> UCRXIFG3_R {
        UCRXIFG3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> UCTXIFG3_R {
        UCTXIFG3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> UCBIT9IFG_R {
        UCBIT9IFG_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> UCRXIFG0_W {
        UCRXIFG0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> UCTXIFG0_W {
        UCTXIFG0_W::new(self)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W {
        UCSTTIFG_W::new(self)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W {
        UCSTPIFG_W::new(self)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W {
        UCALIFG_W::new(self)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W {
        UCNACKIFG_W::new(self)
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UCBCNTIFG_W {
        UCBCNTIFG_W::new(self)
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UCCLTOIFG_W {
        UCCLTOIFG_W::new(self)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> UCRXIFG1_W {
        UCRXIFG1_W::new(self)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> UCTXIFG1_W {
        UCTXIFG1_W::new(self)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> UCRXIFG2_W {
        UCRXIFG2_W::new(self)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> UCTXIFG2_W {
        UCTXIFG2_W::new(self)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> UCRXIFG3_W {
        UCRXIFG3_W::new(self)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> UCTXIFG3_W {
        UCTXIFG3_W::new(self)
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> UCBIT9IFG_W {
        UCBIT9IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg_i2c](index.html) module"]
pub struct UCB0IFG_I2C_SPEC;
impl crate::RegisterSpec for UCB0IFG_I2C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0ifg_i2c::R](R) reader structure"]
impl crate::Readable for UCB0IFG_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ifg_i2c::W](W) writer structure"]
impl crate::Writable for UCB0IFG_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IFG_I2C to value 0"]
impl crate::Resettable for UCB0IFG_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
