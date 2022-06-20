#[doc = "Register `REFCTL0` reader"]
pub struct R(crate::R<REFCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCTL0` writer"]
pub struct W(crate::W<REFCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCTL0_SPEC>;
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
impl From<crate::W<REFCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFON` reader - REF Reference On"]
pub type REFON_R = crate::BitReader<bool>;
#[doc = "Field `REFON` writer - REF Reference On"]
pub type REFON_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 0>;
#[doc = "Field `REFOUT` reader - REF Reference output Buffer On"]
pub type REFOUT_R = crate::BitReader<bool>;
#[doc = "Field `REFOUT` writer - REF Reference output Buffer On"]
pub type REFOUT_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 1>;
#[doc = "Field `REFTCOFF` reader - REF Temp.Sensor off"]
pub type REFTCOFF_R = crate::BitReader<bool>;
#[doc = "Field `REFTCOFF` writer - REF Temp.Sensor off"]
pub type REFTCOFF_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 3>;
#[doc = "REF Reference Voltage Level Select Bit:0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: REF Reference Voltage Level Select 1.5V"]
    REFVSEL_0 = 0,
    #[doc = "1: REF Reference Voltage Level Select 2.0V"]
    REFVSEL_1 = 1,
    #[doc = "2: REF Reference Voltage Level Select 2.5V"]
    REFVSEL_2 = 2,
    #[doc = "3: REF Reference Voltage Level Select 2.5V"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFVSEL` reader - REF Reference Voltage Level Select Bit:0"]
pub type REFVSEL_R = crate::FieldReader<u8, REFVSEL_A>;
impl REFVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Field `REFVSEL` writer - REF Reference Voltage Level Select Bit:0"]
pub type REFVSEL_W<'a> = crate::FieldWriterSafe<'a, u16, REFCTL0_SPEC, u8, REFVSEL_A, 2, 4>;
impl<'a> REFVSEL_W<'a> {
    #[doc = "REF Reference Voltage Level Select 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "REF Reference Voltage Level Select 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "REF Reference Voltage Level Select 2.5V"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
}
#[doc = "Field `REFMSTR` reader - REF Master Control"]
pub type REFMSTR_R = crate::BitReader<bool>;
#[doc = "Field `REFMSTR` writer - REF Master Control"]
pub type REFMSTR_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 7>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type REFGENACT_R = crate::BitReader<bool>;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type REFGENACT_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 8>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type REFBGACT_R = crate::BitReader<bool>;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type REFBGACT_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 9>;
#[doc = "Field `REFGENBUSY` reader - REF Reference generator busy"]
pub type REFGENBUSY_R = crate::BitReader<bool>;
#[doc = "Field `REFGENBUSY` writer - REF Reference generator busy"]
pub type REFGENBUSY_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 10>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BGMODE_R = crate::BitReader<bool>;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BGMODE_W<'a> = crate::BitWriter<'a, u16, REFCTL0_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    pub fn reftcoff(&self) -> REFTCOFF_R {
        REFTCOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    pub fn refmstr(&self) -> REFMSTR_R {
        REFMSTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> REFGENBUSY_R {
        REFGENBUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REF Reference On"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W {
        REFON_W::new(self)
    }
    #[doc = "Bit 1 - REF Reference output Buffer On"]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W::new(self)
    }
    #[doc = "Bit 3 - REF Temp.Sensor off"]
    #[inline(always)]
    pub fn reftcoff(&mut self) -> REFTCOFF_W {
        REFTCOFF_W::new(self)
    }
    #[doc = "Bits 4:5 - REF Reference Voltage Level Select Bit:0"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W {
        REFVSEL_W::new(self)
    }
    #[doc = "Bit 7 - REF Master Control"]
    #[inline(always)]
    pub fn refmstr(&mut self) -> REFMSTR_W {
        REFMSTR_W::new(self)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> REFGENACT_W {
        REFGENACT_W::new(self)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> REFBGACT_W {
        REFBGACT_W::new(self)
    }
    #[doc = "Bit 10 - REF Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&mut self) -> REFGENBUSY_W {
        REFGENBUSY_W::new(self)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W {
        BGMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REF Shared Reference control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctl0](index.html) module"]
pub struct REFCTL0_SPEC;
impl crate::RegisterSpec for REFCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [refctl0::R](R) reader structure"]
impl crate::Readable for REFCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refctl0::W](W) writer structure"]
impl crate::Writable for REFCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFCTL0 to value 0"]
impl crate::Resettable for REFCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
