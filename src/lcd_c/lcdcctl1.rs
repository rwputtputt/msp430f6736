#[doc = "Register `LCDCCTL1` reader"]
pub struct R(crate::R<LCDCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCCTL1` writer"]
pub struct W(crate::W<LCDCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCCTL1_SPEC>;
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
impl From<crate::W<LCDCCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDFRMIFG` reader - LCD_C LCD frame interrupt flag"]
pub type LCDFRMIFG_R = crate::BitReader<bool>;
#[doc = "Field `LCDFRMIFG` writer - LCD_C LCD frame interrupt flag"]
pub type LCDFRMIFG_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 0>;
#[doc = "Field `LCDBLKOFFIFG` reader - LCD_C LCD blinking off interrupt flag"]
pub type LCDBLKOFFIFG_R = crate::BitReader<bool>;
#[doc = "Field `LCDBLKOFFIFG` writer - LCD_C LCD blinking off interrupt flag"]
pub type LCDBLKOFFIFG_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 1>;
#[doc = "Field `LCDBLKONIFG` reader - LCD_C LCD blinking on interrupt flag"]
pub type LCDBLKONIFG_R = crate::BitReader<bool>;
#[doc = "Field `LCDBLKONIFG` writer - LCD_C LCD blinking on interrupt flag"]
pub type LCDBLKONIFG_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 2>;
#[doc = "Field `LCDNOCAPIFG` reader - LCD_C No cpacitance connected interrupt flag"]
pub type LCDNOCAPIFG_R = crate::BitReader<bool>;
#[doc = "Field `LCDNOCAPIFG` writer - LCD_C No cpacitance connected interrupt flag"]
pub type LCDNOCAPIFG_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 3>;
#[doc = "Field `LCDFRMIE` reader - LCD_C LCD frame interrupt enable"]
pub type LCDFRMIE_R = crate::BitReader<bool>;
#[doc = "Field `LCDFRMIE` writer - LCD_C LCD frame interrupt enable"]
pub type LCDFRMIE_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 8>;
#[doc = "Field `LCDBLKOFFIE` reader - LCD_C LCD blinking off interrupt flag"]
pub type LCDBLKOFFIE_R = crate::BitReader<bool>;
#[doc = "Field `LCDBLKOFFIE` writer - LCD_C LCD blinking off interrupt flag"]
pub type LCDBLKOFFIE_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 9>;
#[doc = "Field `LCDBLKONIE` reader - LCD_C LCD blinking on interrupt flag"]
pub type LCDBLKONIE_R = crate::BitReader<bool>;
#[doc = "Field `LCDBLKONIE` writer - LCD_C LCD blinking on interrupt flag"]
pub type LCDBLKONIE_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 10>;
#[doc = "Field `LCDNOCAPIE` reader - LCD_C No cpacitance connected interrupt enable"]
pub type LCDNOCAPIE_R = crate::BitReader<bool>;
#[doc = "Field `LCDNOCAPIE` writer - LCD_C No cpacitance connected interrupt enable"]
pub type LCDNOCAPIE_W<'a> = crate::BitWriter<'a, u16, LCDCCTL1_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LCDFRMIFG_R {
        LCDFRMIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LCDBLKOFFIFG_R {
        LCDBLKOFFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LCDBLKONIFG_R {
        LCDBLKONIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&self) -> LCDNOCAPIFG_R {
        LCDNOCAPIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LCDFRMIE_R {
        LCDFRMIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LCDBLKOFFIE_R {
        LCDBLKOFFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LCDBLKONIE_R {
        LCDBLKONIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&self) -> LCDNOCAPIE_R {
        LCDNOCAPIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LCDFRMIFG_W {
        LCDFRMIFG_W::new(self)
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LCDBLKOFFIFG_W {
        LCDBLKOFFIFG_W::new(self)
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LCDBLKONIFG_W {
        LCDBLKONIFG_W::new(self)
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&mut self) -> LCDNOCAPIFG_W {
        LCDNOCAPIFG_W::new(self)
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LCDFRMIE_W {
        LCDFRMIE_W::new(self)
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LCDBLKOFFIE_W {
        LCDBLKOFFIE_W::new(self)
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LCDBLKONIE_W {
        LCDBLKONIE_W::new(self)
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&mut self) -> LCDNOCAPIE_W {
        LCDNOCAPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcctl1](index.html) module"]
pub struct LCDCCTL1_SPEC;
impl crate::RegisterSpec for LCDCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcctl1::R](R) reader structure"]
impl crate::Readable for LCDCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcctl1::W](W) writer structure"]
impl crate::Writable for LCDCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCCTL1 to value 0"]
impl crate::Resettable for LCDCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
