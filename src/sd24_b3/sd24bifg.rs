#[doc = "Register `SD24BIFG` reader"]
pub struct R(crate::R<SD24BIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BIFG` writer"]
pub struct W(crate::W<SD24BIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BIFG_SPEC>;
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
impl From<crate::W<SD24BIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD24IFG0` reader - SD24B Channel 0 Interrupt Flag"]
pub type SD24IFG0_R = crate::BitReader<bool>;
#[doc = "Field `SD24IFG0` writer - SD24B Channel 0 Interrupt Flag"]
pub type SD24IFG0_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 0>;
#[doc = "Field `SD24IFG1` reader - SD24B Channel 1 Interrupt Flag"]
pub type SD24IFG1_R = crate::BitReader<bool>;
#[doc = "Field `SD24IFG1` writer - SD24B Channel 1 Interrupt Flag"]
pub type SD24IFG1_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 1>;
#[doc = "Field `SD24IFG2` reader - SD24B Channel 2 Interrupt Flag"]
pub type SD24IFG2_R = crate::BitReader<bool>;
#[doc = "Field `SD24IFG2` writer - SD24B Channel 2 Interrupt Flag"]
pub type SD24IFG2_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 2>;
#[doc = "Field `SD24OVIFG0` reader - SD24B Channel 0 Overflow Interrupt Flag"]
pub type SD24OVIFG0_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIFG0` writer - SD24B Channel 0 Overflow Interrupt Flag"]
pub type SD24OVIFG0_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 8>;
#[doc = "Field `SD24OVIFG1` reader - SD24B Channel 1 Overflow Interrupt Flag"]
pub type SD24OVIFG1_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIFG1` writer - SD24B Channel 1 Overflow Interrupt Flag"]
pub type SD24OVIFG1_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 9>;
#[doc = "Field `SD24OVIFG2` reader - SD24B Channel 2 Overflow Interrupt Flag"]
pub type SD24OVIFG2_R = crate::BitReader<bool>;
#[doc = "Field `SD24OVIFG2` writer - SD24B Channel 2 Overflow Interrupt Flag"]
pub type SD24OVIFG2_W<'a> = crate::BitWriter<'a, u16, SD24BIFG_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - SD24B Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg0(&self) -> SD24IFG0_R {
        SD24IFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SD24B Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg1(&self) -> SD24IFG1_R {
        SD24IFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD24B Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg2(&self) -> SD24IFG2_R {
        SD24IFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SD24B Channel 0 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg0(&self) -> SD24OVIFG0_R {
        SD24OVIFG0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD24B Channel 1 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg1(&self) -> SD24OVIFG1_R {
        SD24OVIFG1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SD24B Channel 2 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg2(&self) -> SD24OVIFG2_R {
        SD24OVIFG2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD24B Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg0(&mut self) -> SD24IFG0_W {
        SD24IFG0_W::new(self)
    }
    #[doc = "Bit 1 - SD24B Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg1(&mut self) -> SD24IFG1_W {
        SD24IFG1_W::new(self)
    }
    #[doc = "Bit 2 - SD24B Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ifg2(&mut self) -> SD24IFG2_W {
        SD24IFG2_W::new(self)
    }
    #[doc = "Bit 8 - SD24B Channel 0 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg0(&mut self) -> SD24OVIFG0_W {
        SD24OVIFG0_W::new(self)
    }
    #[doc = "Bit 9 - SD24B Channel 1 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg1(&mut self) -> SD24OVIFG1_W {
        SD24OVIFG1_W::new(self)
    }
    #[doc = "Bit 10 - SD24B Channel 2 Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn sd24ovifg2(&mut self) -> SD24OVIFG2_W {
        SD24OVIFG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD24B Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bifg](index.html) module"]
pub struct SD24BIFG_SPEC;
impl crate::RegisterSpec for SD24BIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bifg::R](R) reader structure"]
impl crate::Readable for SD24BIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bifg::W](W) writer structure"]
impl crate::Writable for SD24BIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BIFG to value 0"]
impl crate::Resettable for SD24BIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
