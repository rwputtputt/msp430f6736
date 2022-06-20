#[doc = "Register `SYSJMBC` reader"]
pub struct R(crate::R<SYSJMBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSJMBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSJMBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBC` writer"]
pub struct W(crate::W<SYSJMBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBC_SPEC>;
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
impl From<crate::W<SYSJMBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSJMBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JMBIN0FG` reader - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type JMBIN0FG_R = crate::BitReader<bool>;
#[doc = "Field `JMBIN0FG` writer - SYS - Incoming JTAG Mailbox 0 Flag"]
pub type JMBIN0FG_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 0>;
#[doc = "Field `JMBIN1FG` reader - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type JMBIN1FG_R = crate::BitReader<bool>;
#[doc = "Field `JMBIN1FG` writer - SYS - Incoming JTAG Mailbox 1 Flag"]
pub type JMBIN1FG_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 1>;
#[doc = "Field `JMBOUT0FG` reader - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type JMBOUT0FG_R = crate::BitReader<bool>;
#[doc = "Field `JMBOUT0FG` writer - SYS - Outgoing JTAG Mailbox 0 Flag"]
pub type JMBOUT0FG_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 2>;
#[doc = "Field `JMBOUT1FG` reader - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type JMBOUT1FG_R = crate::BitReader<bool>;
#[doc = "Field `JMBOUT1FG` writer - SYS - Outgoing JTAG Mailbox 1 Flag"]
pub type JMBOUT1FG_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 3>;
#[doc = "Field `JMBMODE` reader - SYS - JMB 16/32 Bit Mode"]
pub type JMBMODE_R = crate::BitReader<bool>;
#[doc = "Field `JMBMODE` writer - SYS - JMB 16/32 Bit Mode"]
pub type JMBMODE_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 4>;
#[doc = "Field `JMBCLR0OFF` reader - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type JMBCLR0OFF_R = crate::BitReader<bool>;
#[doc = "Field `JMBCLR0OFF` writer - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
pub type JMBCLR0OFF_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 6>;
#[doc = "Field `JMBCLR1OFF` reader - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type JMBCLR1OFF_R = crate::BitReader<bool>;
#[doc = "Field `JMBCLR1OFF` writer - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
pub type JMBCLR1OFF_W<'a> = crate::BitWriter<'a, u16, SYSJMBC_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&self) -> JMBIN0FG_R {
        JMBIN0FG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&self) -> JMBIN1FG_R {
        JMBIN1FG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&self) -> JMBOUT0FG_R {
        JMBOUT0FG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&self) -> JMBOUT1FG_R {
        JMBOUT1FG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&self) -> JMBMODE_R {
        JMBMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&self) -> JMBCLR0OFF_R {
        JMBCLR0OFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&self) -> JMBCLR1OFF_R {
        JMBCLR1OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbin0fg(&mut self) -> JMBIN0FG_W {
        JMBIN0FG_W::new(self)
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbin1fg(&mut self) -> JMBIN1FG_W {
        JMBIN1FG_W::new(self)
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline(always)]
    pub fn jmbout0fg(&mut self) -> JMBOUT0FG_W {
        JMBOUT0FG_W::new(self)
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline(always)]
    pub fn jmbout1fg(&mut self) -> JMBOUT1FG_W {
        JMBOUT1FG_W::new(self)
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline(always)]
    pub fn jmbmode(&mut self) -> JMBMODE_W {
        JMBMODE_W::new(self)
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr0off(&mut self) -> JMBCLR0OFF_W {
        JMBCLR0OFF_W::new(self)
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline(always)]
    pub fn jmbclr1off(&mut self) -> JMBCLR1OFF_W {
        JMBCLR1OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG mailbox control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbc](index.html) module"]
pub struct SYSJMBC_SPEC;
impl crate::RegisterSpec for SYSJMBC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbc::R](R) reader structure"]
impl crate::Readable for SYSJMBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](W) writer structure"]
impl crate::Writable for SYSJMBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBC to value 0"]
impl crate::Resettable for SYSJMBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
