#[doc = "Register `SD24BOSR2` reader"]
pub struct R(crate::R<SD24BOSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BOSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BOSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BOSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BOSR2` writer"]
pub struct W(crate::W<SD24BOSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BOSR2_SPEC>;
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
impl From<crate::W<SD24BOSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BOSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSR0` reader - SD24B Oversampling Rate Bit: 0"]
pub type OSR0_R = crate::BitReader<bool>;
#[doc = "Field `OSR0` writer - SD24B Oversampling Rate Bit: 0"]
pub type OSR0_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 0>;
#[doc = "Field `OSR1` reader - SD24B Oversampling Rate Bit: 1"]
pub type OSR1_R = crate::BitReader<bool>;
#[doc = "Field `OSR1` writer - SD24B Oversampling Rate Bit: 1"]
pub type OSR1_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 1>;
#[doc = "Field `OSR2` reader - SD24B Oversampling Rate Bit: 2"]
pub type OSR2_R = crate::BitReader<bool>;
#[doc = "Field `OSR2` writer - SD24B Oversampling Rate Bit: 2"]
pub type OSR2_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 2>;
#[doc = "Field `OSR3` reader - SD24B Oversampling Rate Bit: 3"]
pub type OSR3_R = crate::BitReader<bool>;
#[doc = "Field `OSR3` writer - SD24B Oversampling Rate Bit: 3"]
pub type OSR3_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 3>;
#[doc = "Field `OSR4` reader - SD24B Oversampling Rate Bit: 4"]
pub type OSR4_R = crate::BitReader<bool>;
#[doc = "Field `OSR4` writer - SD24B Oversampling Rate Bit: 4"]
pub type OSR4_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 4>;
#[doc = "Field `OSR5` reader - SD24B Oversampling Rate Bit: 5"]
pub type OSR5_R = crate::BitReader<bool>;
#[doc = "Field `OSR5` writer - SD24B Oversampling Rate Bit: 5"]
pub type OSR5_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 5>;
#[doc = "Field `OSR6` reader - SD24B Oversampling Rate Bit: 6"]
pub type OSR6_R = crate::BitReader<bool>;
#[doc = "Field `OSR6` writer - SD24B Oversampling Rate Bit: 6"]
pub type OSR6_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 6>;
#[doc = "Field `OSR7` reader - SD24B Oversampling Rate Bit: 7"]
pub type OSR7_R = crate::BitReader<bool>;
#[doc = "Field `OSR7` writer - SD24B Oversampling Rate Bit: 7"]
pub type OSR7_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 7>;
#[doc = "Field `OSR8` reader - SD24B Oversampling Rate Bit: 8"]
pub type OSR8_R = crate::BitReader<bool>;
#[doc = "Field `OSR8` writer - SD24B Oversampling Rate Bit: 8"]
pub type OSR8_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 8>;
#[doc = "Field `OSR9` reader - SD24B Oversampling Rate Bit: 9"]
pub type OSR9_R = crate::BitReader<bool>;
#[doc = "Field `OSR9` writer - SD24B Oversampling Rate Bit: 9"]
pub type OSR9_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 9>;
#[doc = "Field `OSR10` reader - SD24B Oversampling Rate Bit: 10"]
pub type OSR10_R = crate::BitReader<bool>;
#[doc = "Field `OSR10` writer - SD24B Oversampling Rate Bit: 10"]
pub type OSR10_W<'a> = crate::BitWriter<'a, u16, SD24BOSR2_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - SD24B Oversampling Rate Bit: 0"]
    #[inline(always)]
    pub fn osr0(&self) -> OSR0_R {
        OSR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD24B Oversampling Rate Bit: 1"]
    #[inline(always)]
    pub fn osr1(&self) -> OSR1_R {
        OSR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD24B Oversampling Rate Bit: 2"]
    #[inline(always)]
    pub fn osr2(&self) -> OSR2_R {
        OSR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD24B Oversampling Rate Bit: 3"]
    #[inline(always)]
    pub fn osr3(&self) -> OSR3_R {
        OSR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SD24B Oversampling Rate Bit: 4"]
    #[inline(always)]
    pub fn osr4(&self) -> OSR4_R {
        OSR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD24B Oversampling Rate Bit: 5"]
    #[inline(always)]
    pub fn osr5(&self) -> OSR5_R {
        OSR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SD24B Oversampling Rate Bit: 6"]
    #[inline(always)]
    pub fn osr6(&self) -> OSR6_R {
        OSR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD24B Oversampling Rate Bit: 7"]
    #[inline(always)]
    pub fn osr7(&self) -> OSR7_R {
        OSR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SD24B Oversampling Rate Bit: 8"]
    #[inline(always)]
    pub fn osr8(&self) -> OSR8_R {
        OSR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD24B Oversampling Rate Bit: 9"]
    #[inline(always)]
    pub fn osr9(&self) -> OSR9_R {
        OSR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SD24B Oversampling Rate Bit: 10"]
    #[inline(always)]
    pub fn osr10(&self) -> OSR10_R {
        OSR10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD24B Oversampling Rate Bit: 0"]
    #[inline(always)]
    pub fn osr0(&mut self) -> OSR0_W {
        OSR0_W::new(self)
    }
    #[doc = "Bit 1 - SD24B Oversampling Rate Bit: 1"]
    #[inline(always)]
    pub fn osr1(&mut self) -> OSR1_W {
        OSR1_W::new(self)
    }
    #[doc = "Bit 2 - SD24B Oversampling Rate Bit: 2"]
    #[inline(always)]
    pub fn osr2(&mut self) -> OSR2_W {
        OSR2_W::new(self)
    }
    #[doc = "Bit 3 - SD24B Oversampling Rate Bit: 3"]
    #[inline(always)]
    pub fn osr3(&mut self) -> OSR3_W {
        OSR3_W::new(self)
    }
    #[doc = "Bit 4 - SD24B Oversampling Rate Bit: 4"]
    #[inline(always)]
    pub fn osr4(&mut self) -> OSR4_W {
        OSR4_W::new(self)
    }
    #[doc = "Bit 5 - SD24B Oversampling Rate Bit: 5"]
    #[inline(always)]
    pub fn osr5(&mut self) -> OSR5_W {
        OSR5_W::new(self)
    }
    #[doc = "Bit 6 - SD24B Oversampling Rate Bit: 6"]
    #[inline(always)]
    pub fn osr6(&mut self) -> OSR6_W {
        OSR6_W::new(self)
    }
    #[doc = "Bit 7 - SD24B Oversampling Rate Bit: 7"]
    #[inline(always)]
    pub fn osr7(&mut self) -> OSR7_W {
        OSR7_W::new(self)
    }
    #[doc = "Bit 8 - SD24B Oversampling Rate Bit: 8"]
    #[inline(always)]
    pub fn osr8(&mut self) -> OSR8_W {
        OSR8_W::new(self)
    }
    #[doc = "Bit 9 - SD24B Oversampling Rate Bit: 9"]
    #[inline(always)]
    pub fn osr9(&mut self) -> OSR9_W {
        OSR9_W::new(self)
    }
    #[doc = "Bit 10 - SD24B Oversampling Rate Bit: 10"]
    #[inline(always)]
    pub fn osr10(&mut self) -> OSR10_W {
        OSR10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Channel 2 OSR Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bosr2](index.html) module"]
pub struct SD24BOSR2_SPEC;
impl crate::RegisterSpec for SD24BOSR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bosr2::R](R) reader structure"]
impl crate::Readable for SD24BOSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bosr2::W](W) writer structure"]
impl crate::Writable for SD24BOSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BOSR2 to value 0"]
impl crate::Resettable for SD24BOSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
