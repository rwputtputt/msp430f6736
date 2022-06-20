#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD_C Control Register 0"]
    pub lcdcctl0: crate::Reg<lcdcctl0::LCDCCTL0_SPEC>,
    #[doc = "0x02 - LCD_C Control Register 1"]
    pub lcdcctl1: crate::Reg<lcdcctl1::LCDCCTL1_SPEC>,
    #[doc = "0x04 - LCD_C blinking control register"]
    pub lcdcblkctl: crate::Reg<lcdcblkctl::LCDCBLKCTL_SPEC>,
    #[doc = "0x06 - LCD_C memory control register"]
    pub lcdcmemctl: crate::Reg<lcdcmemctl::LCDCMEMCTL_SPEC>,
    #[doc = "0x08 - LCD_C Voltage Control Register"]
    pub lcdcvctl: crate::Reg<lcdcvctl::LCDCVCTL_SPEC>,
    #[doc = "0x0a - LCD_C Port Control Register 0"]
    pub lcdcpctl0: crate::Reg<lcdcpctl0::LCDCPCTL0_SPEC>,
    #[doc = "0x0c - LCD_C Port Control Register 1"]
    pub lcdcpctl1: crate::Reg<lcdcpctl1::LCDCPCTL1_SPEC>,
    #[doc = "0x0e - LCD_C Port Control Register 2"]
    pub lcdcpctl2: crate::Reg<lcdcpctl2::LCDCPCTL2_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x12 - LCD_C Charge Pump Control Register 3"]
    pub lcdccpctl: crate::Reg<lcdccpctl::LCDCCPCTL_SPEC>,
    _reserved9: [u8; 0x0a],
    #[doc = "0x1e - LCD_C Interrupt Vector Register"]
    pub lcdciv: crate::Reg<lcdciv::LCDCIV_SPEC>,
    #[doc = "0x20 - LCD Memory 1"]
    pub lcdm1: crate::Reg<lcdm1::LCDM1_SPEC>,
    #[doc = "0x21 - LCD Memory 2"]
    pub lcdm2: crate::Reg<lcdm2::LCDM2_SPEC>,
    #[doc = "0x22 - LCD Memory 3"]
    pub lcdm3: crate::Reg<lcdm3::LCDM3_SPEC>,
    #[doc = "0x23 - LCD Memory 4"]
    pub lcdm4: crate::Reg<lcdm4::LCDM4_SPEC>,
    #[doc = "0x24 - LCD Memory 5"]
    pub lcdm5: crate::Reg<lcdm5::LCDM5_SPEC>,
    #[doc = "0x25 - LCD Memory 6"]
    pub lcdm6: crate::Reg<lcdm6::LCDM6_SPEC>,
    #[doc = "0x26 - LCD Memory 7"]
    pub lcdm7: crate::Reg<lcdm7::LCDM7_SPEC>,
    #[doc = "0x27 - LCD Memory 8"]
    pub lcdm8: crate::Reg<lcdm8::LCDM8_SPEC>,
    #[doc = "0x28 - LCD Memory 9"]
    pub lcdm9: crate::Reg<lcdm9::LCDM9_SPEC>,
    #[doc = "0x29 - LCD Memory 10"]
    pub lcdm10: crate::Reg<lcdm10::LCDM10_SPEC>,
    #[doc = "0x2a - LCD Memory 11"]
    pub lcdm11: crate::Reg<lcdm11::LCDM11_SPEC>,
    #[doc = "0x2b - LCD Memory 12"]
    pub lcdm12: crate::Reg<lcdm12::LCDM12_SPEC>,
    #[doc = "0x2c - LCD Memory 13"]
    pub lcdm13: crate::Reg<lcdm13::LCDM13_SPEC>,
    #[doc = "0x2d - LCD Memory 14"]
    pub lcdm14: crate::Reg<lcdm14::LCDM14_SPEC>,
    #[doc = "0x2e - LCD Memory 15"]
    pub lcdm15: crate::Reg<lcdm15::LCDM15_SPEC>,
    #[doc = "0x2f - LCD Memory 16"]
    pub lcdm16: crate::Reg<lcdm16::LCDM16_SPEC>,
    #[doc = "0x30 - LCD Memory 17"]
    pub lcdm17: crate::Reg<lcdm17::LCDM17_SPEC>,
    #[doc = "0x31 - LCD Memory 18"]
    pub lcdm18: crate::Reg<lcdm18::LCDM18_SPEC>,
    #[doc = "0x32 - LCD Memory 19"]
    pub lcdm19: crate::Reg<lcdm19::LCDM19_SPEC>,
    #[doc = "0x33 - LCD Memory 20"]
    pub lcdm20: crate::Reg<lcdm20::LCDM20_SPEC>,
    #[doc = "0x34 - LCD Memory 21"]
    pub lcdm21: crate::Reg<lcdm21::LCDM21_SPEC>,
    #[doc = "0x35 - LCD Memory 22"]
    pub lcdm22: crate::Reg<lcdm22::LCDM22_SPEC>,
    #[doc = "0x36 - LCD Memory 23"]
    pub lcdm23: crate::Reg<lcdm23::LCDM23_SPEC>,
    #[doc = "0x37 - LCD Memory 24"]
    pub lcdm24: crate::Reg<lcdm24::LCDM24_SPEC>,
    #[doc = "0x38 - LCD Memory 25"]
    pub lcdm25: crate::Reg<lcdm25::LCDM25_SPEC>,
    #[doc = "0x39 - LCD Memory 26"]
    pub lcdm26: crate::Reg<lcdm26::LCDM26_SPEC>,
    #[doc = "0x3a - LCD Memory 27"]
    pub lcdm27: crate::Reg<lcdm27::LCDM27_SPEC>,
    #[doc = "0x3b - LCD Memory 28"]
    pub lcdm28: crate::Reg<lcdm28::LCDM28_SPEC>,
    #[doc = "0x3c - LCD Memory 29"]
    pub lcdm29: crate::Reg<lcdm29::LCDM29_SPEC>,
    #[doc = "0x3d - LCD Memory 30"]
    pub lcdm30: crate::Reg<lcdm30::LCDM30_SPEC>,
    #[doc = "0x3e - LCD Memory 31"]
    pub lcdm31: crate::Reg<lcdm31::LCDM31_SPEC>,
    #[doc = "0x3f - LCD Memory 32"]
    pub lcdm32: crate::Reg<lcdm32::LCDM32_SPEC>,
    _reserved_42_lcdbm1: [u8; 0x01],
    _reserved_43_lcdbm2: [u8; 0x01],
    _reserved_44_lcdbm3: [u8; 0x01],
    _reserved_45_lcdbm4: [u8; 0x01],
    _reserved_46_lcdbm5: [u8; 0x01],
    _reserved_47_lcdbm6: [u8; 0x01],
    _reserved_48_lcdbm7: [u8; 0x01],
    _reserved_49_lcdbm8: [u8; 0x01],
    #[doc = "0x48 - LCD Blinking Memory 9"]
    pub lcdbm9: crate::Reg<lcdbm9::LCDBM9_SPEC>,
    #[doc = "0x49 - LCD Blinking Memory 10"]
    pub lcdbm10: crate::Reg<lcdbm10::LCDBM10_SPEC>,
    #[doc = "0x4a - LCD Blinking Memory 11"]
    pub lcdbm11: crate::Reg<lcdbm11::LCDBM11_SPEC>,
    #[doc = "0x4b - LCD Blinking Memory 12"]
    pub lcdbm12: crate::Reg<lcdbm12::LCDBM12_SPEC>,
    #[doc = "0x4c - LCD Blinking Memory 13"]
    pub lcdbm13: crate::Reg<lcdbm13::LCDBM13_SPEC>,
    #[doc = "0x4d - LCD Blinking Memory 14"]
    pub lcdbm14: crate::Reg<lcdbm14::LCDBM14_SPEC>,
    #[doc = "0x4e - LCD Blinking Memory 15"]
    pub lcdbm15: crate::Reg<lcdbm15::LCDBM15_SPEC>,
    #[doc = "0x4f - LCD Blinking Memory 16"]
    pub lcdbm16: crate::Reg<lcdbm16::LCDBM16_SPEC>,
    #[doc = "0x50 - LCD Blinking Memory 17"]
    pub lcdbm17: crate::Reg<lcdbm17::LCDBM17_SPEC>,
    #[doc = "0x51 - LCD Blinking Memory 18"]
    pub lcdbm18: crate::Reg<lcdbm18::LCDBM18_SPEC>,
    #[doc = "0x52 - LCD Blinking Memory 19"]
    pub lcdbm19: crate::Reg<lcdbm19::LCDBM19_SPEC>,
    #[doc = "0x53 - LCD Blinking Memory 20"]
    pub lcdbm20: crate::Reg<lcdbm20::LCDBM20_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x40 - LCD Memory 33"]
    #[inline(always)]
    pub fn lcdm33(&self) -> &crate::Reg<lcdm33::LCDM33_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<lcdm33::LCDM33_SPEC>)
        }
    }
    #[doc = "0x40 - LCD Blinking Memory 1"]
    #[inline(always)]
    pub fn lcdbm1(&self) -> &crate::Reg<lcdbm1::LCDBM1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const crate::Reg<lcdbm1::LCDBM1_SPEC>)
        }
    }
    #[doc = "0x41 - LCD Memory 34"]
    #[inline(always)]
    pub fn lcdm34(&self) -> &crate::Reg<lcdm34::LCDM34_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(65usize)
                as *const crate::Reg<lcdm34::LCDM34_SPEC>)
        }
    }
    #[doc = "0x41 - LCD Blinking Memory 2"]
    #[inline(always)]
    pub fn lcdbm2(&self) -> &crate::Reg<lcdbm2::LCDBM2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(65usize)
                as *const crate::Reg<lcdbm2::LCDBM2_SPEC>)
        }
    }
    #[doc = "0x42 - LCD Memory 35"]
    #[inline(always)]
    pub fn lcdm35(&self) -> &crate::Reg<lcdm35::LCDM35_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(66usize)
                as *const crate::Reg<lcdm35::LCDM35_SPEC>)
        }
    }
    #[doc = "0x42 - LCD Blinking Memory 3"]
    #[inline(always)]
    pub fn lcdbm3(&self) -> &crate::Reg<lcdbm3::LCDBM3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(66usize)
                as *const crate::Reg<lcdbm3::LCDBM3_SPEC>)
        }
    }
    #[doc = "0x43 - LCD Memory 36"]
    #[inline(always)]
    pub fn lcdm36(&self) -> &crate::Reg<lcdm36::LCDM36_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(67usize)
                as *const crate::Reg<lcdm36::LCDM36_SPEC>)
        }
    }
    #[doc = "0x43 - LCD Blinking Memory 4"]
    #[inline(always)]
    pub fn lcdbm4(&self) -> &crate::Reg<lcdbm4::LCDBM4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(67usize)
                as *const crate::Reg<lcdbm4::LCDBM4_SPEC>)
        }
    }
    #[doc = "0x44 - LCD Memory 37"]
    #[inline(always)]
    pub fn lcdm37(&self) -> &crate::Reg<lcdm37::LCDM37_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<lcdm37::LCDM37_SPEC>)
        }
    }
    #[doc = "0x44 - LCD Blinking Memory 5"]
    #[inline(always)]
    pub fn lcdbm5(&self) -> &crate::Reg<lcdbm5::LCDBM5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const crate::Reg<lcdbm5::LCDBM5_SPEC>)
        }
    }
    #[doc = "0x45 - LCD Memory 38"]
    #[inline(always)]
    pub fn lcdm38(&self) -> &crate::Reg<lcdm38::LCDM38_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(69usize)
                as *const crate::Reg<lcdm38::LCDM38_SPEC>)
        }
    }
    #[doc = "0x45 - LCD Blinking Memory 6"]
    #[inline(always)]
    pub fn lcdbm6(&self) -> &crate::Reg<lcdbm6::LCDBM6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(69usize)
                as *const crate::Reg<lcdbm6::LCDBM6_SPEC>)
        }
    }
    #[doc = "0x46 - LCD Memory 39"]
    #[inline(always)]
    pub fn lcdm39(&self) -> &crate::Reg<lcdm39::LCDM39_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(70usize)
                as *const crate::Reg<lcdm39::LCDM39_SPEC>)
        }
    }
    #[doc = "0x46 - LCD Blinking Memory 7"]
    #[inline(always)]
    pub fn lcdbm7(&self) -> &crate::Reg<lcdbm7::LCDBM7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(70usize)
                as *const crate::Reg<lcdbm7::LCDBM7_SPEC>)
        }
    }
    #[doc = "0x47 - LCD Memory 40"]
    #[inline(always)]
    pub fn lcdm40(&self) -> &crate::Reg<lcdm40::LCDM40_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(71usize)
                as *const crate::Reg<lcdm40::LCDM40_SPEC>)
        }
    }
    #[doc = "0x47 - LCD Blinking Memory 8"]
    #[inline(always)]
    pub fn lcdbm8(&self) -> &crate::Reg<lcdbm8::LCDBM8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(71usize)
                as *const crate::Reg<lcdbm8::LCDBM8_SPEC>)
        }
    }
}
#[doc = "LCDM1 register accessor: an alias for `Reg<LCDM1_SPEC>`"]
pub type LCDM1 = crate::Reg<lcdm1::LCDM1_SPEC>;
#[doc = "LCD Memory 1"]
pub mod lcdm1;
#[doc = "LCDM2 register accessor: an alias for `Reg<LCDM2_SPEC>`"]
pub type LCDM2 = crate::Reg<lcdm2::LCDM2_SPEC>;
#[doc = "LCD Memory 2"]
pub mod lcdm2;
#[doc = "LCDM3 register accessor: an alias for `Reg<LCDM3_SPEC>`"]
pub type LCDM3 = crate::Reg<lcdm3::LCDM3_SPEC>;
#[doc = "LCD Memory 3"]
pub mod lcdm3;
#[doc = "LCDM4 register accessor: an alias for `Reg<LCDM4_SPEC>`"]
pub type LCDM4 = crate::Reg<lcdm4::LCDM4_SPEC>;
#[doc = "LCD Memory 4"]
pub mod lcdm4;
#[doc = "LCDM5 register accessor: an alias for `Reg<LCDM5_SPEC>`"]
pub type LCDM5 = crate::Reg<lcdm5::LCDM5_SPEC>;
#[doc = "LCD Memory 5"]
pub mod lcdm5;
#[doc = "LCDM6 register accessor: an alias for `Reg<LCDM6_SPEC>`"]
pub type LCDM6 = crate::Reg<lcdm6::LCDM6_SPEC>;
#[doc = "LCD Memory 6"]
pub mod lcdm6;
#[doc = "LCDM7 register accessor: an alias for `Reg<LCDM7_SPEC>`"]
pub type LCDM7 = crate::Reg<lcdm7::LCDM7_SPEC>;
#[doc = "LCD Memory 7"]
pub mod lcdm7;
#[doc = "LCDM8 register accessor: an alias for `Reg<LCDM8_SPEC>`"]
pub type LCDM8 = crate::Reg<lcdm8::LCDM8_SPEC>;
#[doc = "LCD Memory 8"]
pub mod lcdm8;
#[doc = "LCDM9 register accessor: an alias for `Reg<LCDM9_SPEC>`"]
pub type LCDM9 = crate::Reg<lcdm9::LCDM9_SPEC>;
#[doc = "LCD Memory 9"]
pub mod lcdm9;
#[doc = "LCDM10 register accessor: an alias for `Reg<LCDM10_SPEC>`"]
pub type LCDM10 = crate::Reg<lcdm10::LCDM10_SPEC>;
#[doc = "LCD Memory 10"]
pub mod lcdm10;
#[doc = "LCDM11 register accessor: an alias for `Reg<LCDM11_SPEC>`"]
pub type LCDM11 = crate::Reg<lcdm11::LCDM11_SPEC>;
#[doc = "LCD Memory 11"]
pub mod lcdm11;
#[doc = "LCDM12 register accessor: an alias for `Reg<LCDM12_SPEC>`"]
pub type LCDM12 = crate::Reg<lcdm12::LCDM12_SPEC>;
#[doc = "LCD Memory 12"]
pub mod lcdm12;
#[doc = "LCDM13 register accessor: an alias for `Reg<LCDM13_SPEC>`"]
pub type LCDM13 = crate::Reg<lcdm13::LCDM13_SPEC>;
#[doc = "LCD Memory 13"]
pub mod lcdm13;
#[doc = "LCDM14 register accessor: an alias for `Reg<LCDM14_SPEC>`"]
pub type LCDM14 = crate::Reg<lcdm14::LCDM14_SPEC>;
#[doc = "LCD Memory 14"]
pub mod lcdm14;
#[doc = "LCDM15 register accessor: an alias for `Reg<LCDM15_SPEC>`"]
pub type LCDM15 = crate::Reg<lcdm15::LCDM15_SPEC>;
#[doc = "LCD Memory 15"]
pub mod lcdm15;
#[doc = "LCDM16 register accessor: an alias for `Reg<LCDM16_SPEC>`"]
pub type LCDM16 = crate::Reg<lcdm16::LCDM16_SPEC>;
#[doc = "LCD Memory 16"]
pub mod lcdm16;
#[doc = "LCDM17 register accessor: an alias for `Reg<LCDM17_SPEC>`"]
pub type LCDM17 = crate::Reg<lcdm17::LCDM17_SPEC>;
#[doc = "LCD Memory 17"]
pub mod lcdm17;
#[doc = "LCDM18 register accessor: an alias for `Reg<LCDM18_SPEC>`"]
pub type LCDM18 = crate::Reg<lcdm18::LCDM18_SPEC>;
#[doc = "LCD Memory 18"]
pub mod lcdm18;
#[doc = "LCDM19 register accessor: an alias for `Reg<LCDM19_SPEC>`"]
pub type LCDM19 = crate::Reg<lcdm19::LCDM19_SPEC>;
#[doc = "LCD Memory 19"]
pub mod lcdm19;
#[doc = "LCDM20 register accessor: an alias for `Reg<LCDM20_SPEC>`"]
pub type LCDM20 = crate::Reg<lcdm20::LCDM20_SPEC>;
#[doc = "LCD Memory 20"]
pub mod lcdm20;
#[doc = "LCDM21 register accessor: an alias for `Reg<LCDM21_SPEC>`"]
pub type LCDM21 = crate::Reg<lcdm21::LCDM21_SPEC>;
#[doc = "LCD Memory 21"]
pub mod lcdm21;
#[doc = "LCDM22 register accessor: an alias for `Reg<LCDM22_SPEC>`"]
pub type LCDM22 = crate::Reg<lcdm22::LCDM22_SPEC>;
#[doc = "LCD Memory 22"]
pub mod lcdm22;
#[doc = "LCDM23 register accessor: an alias for `Reg<LCDM23_SPEC>`"]
pub type LCDM23 = crate::Reg<lcdm23::LCDM23_SPEC>;
#[doc = "LCD Memory 23"]
pub mod lcdm23;
#[doc = "LCDM24 register accessor: an alias for `Reg<LCDM24_SPEC>`"]
pub type LCDM24 = crate::Reg<lcdm24::LCDM24_SPEC>;
#[doc = "LCD Memory 24"]
pub mod lcdm24;
#[doc = "LCDM25 register accessor: an alias for `Reg<LCDM25_SPEC>`"]
pub type LCDM25 = crate::Reg<lcdm25::LCDM25_SPEC>;
#[doc = "LCD Memory 25"]
pub mod lcdm25;
#[doc = "LCDM26 register accessor: an alias for `Reg<LCDM26_SPEC>`"]
pub type LCDM26 = crate::Reg<lcdm26::LCDM26_SPEC>;
#[doc = "LCD Memory 26"]
pub mod lcdm26;
#[doc = "LCDM27 register accessor: an alias for `Reg<LCDM27_SPEC>`"]
pub type LCDM27 = crate::Reg<lcdm27::LCDM27_SPEC>;
#[doc = "LCD Memory 27"]
pub mod lcdm27;
#[doc = "LCDM28 register accessor: an alias for `Reg<LCDM28_SPEC>`"]
pub type LCDM28 = crate::Reg<lcdm28::LCDM28_SPEC>;
#[doc = "LCD Memory 28"]
pub mod lcdm28;
#[doc = "LCDM29 register accessor: an alias for `Reg<LCDM29_SPEC>`"]
pub type LCDM29 = crate::Reg<lcdm29::LCDM29_SPEC>;
#[doc = "LCD Memory 29"]
pub mod lcdm29;
#[doc = "LCDM30 register accessor: an alias for `Reg<LCDM30_SPEC>`"]
pub type LCDM30 = crate::Reg<lcdm30::LCDM30_SPEC>;
#[doc = "LCD Memory 30"]
pub mod lcdm30;
#[doc = "LCDM31 register accessor: an alias for `Reg<LCDM31_SPEC>`"]
pub type LCDM31 = crate::Reg<lcdm31::LCDM31_SPEC>;
#[doc = "LCD Memory 31"]
pub mod lcdm31;
#[doc = "LCDM32 register accessor: an alias for `Reg<LCDM32_SPEC>`"]
pub type LCDM32 = crate::Reg<lcdm32::LCDM32_SPEC>;
#[doc = "LCD Memory 32"]
pub mod lcdm32;
#[doc = "LCDBM1 register accessor: an alias for `Reg<LCDBM1_SPEC>`"]
pub type LCDBM1 = crate::Reg<lcdbm1::LCDBM1_SPEC>;
#[doc = "LCD Blinking Memory 1"]
pub mod lcdbm1;
#[doc = "LCDM33 register accessor: an alias for `Reg<LCDM33_SPEC>`"]
pub type LCDM33 = crate::Reg<lcdm33::LCDM33_SPEC>;
#[doc = "LCD Memory 33"]
pub mod lcdm33;
#[doc = "LCDBM2 register accessor: an alias for `Reg<LCDBM2_SPEC>`"]
pub type LCDBM2 = crate::Reg<lcdbm2::LCDBM2_SPEC>;
#[doc = "LCD Blinking Memory 2"]
pub mod lcdbm2;
#[doc = "LCDM34 register accessor: an alias for `Reg<LCDM34_SPEC>`"]
pub type LCDM34 = crate::Reg<lcdm34::LCDM34_SPEC>;
#[doc = "LCD Memory 34"]
pub mod lcdm34;
#[doc = "LCDBM3 register accessor: an alias for `Reg<LCDBM3_SPEC>`"]
pub type LCDBM3 = crate::Reg<lcdbm3::LCDBM3_SPEC>;
#[doc = "LCD Blinking Memory 3"]
pub mod lcdbm3;
#[doc = "LCDM35 register accessor: an alias for `Reg<LCDM35_SPEC>`"]
pub type LCDM35 = crate::Reg<lcdm35::LCDM35_SPEC>;
#[doc = "LCD Memory 35"]
pub mod lcdm35;
#[doc = "LCDBM4 register accessor: an alias for `Reg<LCDBM4_SPEC>`"]
pub type LCDBM4 = crate::Reg<lcdbm4::LCDBM4_SPEC>;
#[doc = "LCD Blinking Memory 4"]
pub mod lcdbm4;
#[doc = "LCDM36 register accessor: an alias for `Reg<LCDM36_SPEC>`"]
pub type LCDM36 = crate::Reg<lcdm36::LCDM36_SPEC>;
#[doc = "LCD Memory 36"]
pub mod lcdm36;
#[doc = "LCDBM5 register accessor: an alias for `Reg<LCDBM5_SPEC>`"]
pub type LCDBM5 = crate::Reg<lcdbm5::LCDBM5_SPEC>;
#[doc = "LCD Blinking Memory 5"]
pub mod lcdbm5;
#[doc = "LCDM37 register accessor: an alias for `Reg<LCDM37_SPEC>`"]
pub type LCDM37 = crate::Reg<lcdm37::LCDM37_SPEC>;
#[doc = "LCD Memory 37"]
pub mod lcdm37;
#[doc = "LCDBM6 register accessor: an alias for `Reg<LCDBM6_SPEC>`"]
pub type LCDBM6 = crate::Reg<lcdbm6::LCDBM6_SPEC>;
#[doc = "LCD Blinking Memory 6"]
pub mod lcdbm6;
#[doc = "LCDM38 register accessor: an alias for `Reg<LCDM38_SPEC>`"]
pub type LCDM38 = crate::Reg<lcdm38::LCDM38_SPEC>;
#[doc = "LCD Memory 38"]
pub mod lcdm38;
#[doc = "LCDBM7 register accessor: an alias for `Reg<LCDBM7_SPEC>`"]
pub type LCDBM7 = crate::Reg<lcdbm7::LCDBM7_SPEC>;
#[doc = "LCD Blinking Memory 7"]
pub mod lcdbm7;
#[doc = "LCDM39 register accessor: an alias for `Reg<LCDM39_SPEC>`"]
pub type LCDM39 = crate::Reg<lcdm39::LCDM39_SPEC>;
#[doc = "LCD Memory 39"]
pub mod lcdm39;
#[doc = "LCDBM8 register accessor: an alias for `Reg<LCDBM8_SPEC>`"]
pub type LCDBM8 = crate::Reg<lcdbm8::LCDBM8_SPEC>;
#[doc = "LCD Blinking Memory 8"]
pub mod lcdbm8;
#[doc = "LCDM40 register accessor: an alias for `Reg<LCDM40_SPEC>`"]
pub type LCDM40 = crate::Reg<lcdm40::LCDM40_SPEC>;
#[doc = "LCD Memory 40"]
pub mod lcdm40;
#[doc = "LCDBM9 register accessor: an alias for `Reg<LCDBM9_SPEC>`"]
pub type LCDBM9 = crate::Reg<lcdbm9::LCDBM9_SPEC>;
#[doc = "LCD Blinking Memory 9"]
pub mod lcdbm9;
#[doc = "LCDBM10 register accessor: an alias for `Reg<LCDBM10_SPEC>`"]
pub type LCDBM10 = crate::Reg<lcdbm10::LCDBM10_SPEC>;
#[doc = "LCD Blinking Memory 10"]
pub mod lcdbm10;
#[doc = "LCDBM11 register accessor: an alias for `Reg<LCDBM11_SPEC>`"]
pub type LCDBM11 = crate::Reg<lcdbm11::LCDBM11_SPEC>;
#[doc = "LCD Blinking Memory 11"]
pub mod lcdbm11;
#[doc = "LCDBM12 register accessor: an alias for `Reg<LCDBM12_SPEC>`"]
pub type LCDBM12 = crate::Reg<lcdbm12::LCDBM12_SPEC>;
#[doc = "LCD Blinking Memory 12"]
pub mod lcdbm12;
#[doc = "LCDBM13 register accessor: an alias for `Reg<LCDBM13_SPEC>`"]
pub type LCDBM13 = crate::Reg<lcdbm13::LCDBM13_SPEC>;
#[doc = "LCD Blinking Memory 13"]
pub mod lcdbm13;
#[doc = "LCDBM14 register accessor: an alias for `Reg<LCDBM14_SPEC>`"]
pub type LCDBM14 = crate::Reg<lcdbm14::LCDBM14_SPEC>;
#[doc = "LCD Blinking Memory 14"]
pub mod lcdbm14;
#[doc = "LCDBM15 register accessor: an alias for `Reg<LCDBM15_SPEC>`"]
pub type LCDBM15 = crate::Reg<lcdbm15::LCDBM15_SPEC>;
#[doc = "LCD Blinking Memory 15"]
pub mod lcdbm15;
#[doc = "LCDBM16 register accessor: an alias for `Reg<LCDBM16_SPEC>`"]
pub type LCDBM16 = crate::Reg<lcdbm16::LCDBM16_SPEC>;
#[doc = "LCD Blinking Memory 16"]
pub mod lcdbm16;
#[doc = "LCDBM17 register accessor: an alias for `Reg<LCDBM17_SPEC>`"]
pub type LCDBM17 = crate::Reg<lcdbm17::LCDBM17_SPEC>;
#[doc = "LCD Blinking Memory 17"]
pub mod lcdbm17;
#[doc = "LCDBM18 register accessor: an alias for `Reg<LCDBM18_SPEC>`"]
pub type LCDBM18 = crate::Reg<lcdbm18::LCDBM18_SPEC>;
#[doc = "LCD Blinking Memory 18"]
pub mod lcdbm18;
#[doc = "LCDBM19 register accessor: an alias for `Reg<LCDBM19_SPEC>`"]
pub type LCDBM19 = crate::Reg<lcdbm19::LCDBM19_SPEC>;
#[doc = "LCD Blinking Memory 19"]
pub mod lcdbm19;
#[doc = "LCDBM20 register accessor: an alias for `Reg<LCDBM20_SPEC>`"]
pub type LCDBM20 = crate::Reg<lcdbm20::LCDBM20_SPEC>;
#[doc = "LCD Blinking Memory 20"]
pub mod lcdbm20;
#[doc = "LCDCCTL0 register accessor: an alias for `Reg<LCDCCTL0_SPEC>`"]
pub type LCDCCTL0 = crate::Reg<lcdcctl0::LCDCCTL0_SPEC>;
#[doc = "LCD_C Control Register 0"]
pub mod lcdcctl0;
#[doc = "LCDCCTL1 register accessor: an alias for `Reg<LCDCCTL1_SPEC>`"]
pub type LCDCCTL1 = crate::Reg<lcdcctl1::LCDCCTL1_SPEC>;
#[doc = "LCD_C Control Register 1"]
pub mod lcdcctl1;
#[doc = "LCDCBLKCTL register accessor: an alias for `Reg<LCDCBLKCTL_SPEC>`"]
pub type LCDCBLKCTL = crate::Reg<lcdcblkctl::LCDCBLKCTL_SPEC>;
#[doc = "LCD_C blinking control register"]
pub mod lcdcblkctl;
#[doc = "LCDCMEMCTL register accessor: an alias for `Reg<LCDCMEMCTL_SPEC>`"]
pub type LCDCMEMCTL = crate::Reg<lcdcmemctl::LCDCMEMCTL_SPEC>;
#[doc = "LCD_C memory control register"]
pub mod lcdcmemctl;
#[doc = "LCDCVCTL register accessor: an alias for `Reg<LCDCVCTL_SPEC>`"]
pub type LCDCVCTL = crate::Reg<lcdcvctl::LCDCVCTL_SPEC>;
#[doc = "LCD_C Voltage Control Register"]
pub mod lcdcvctl;
#[doc = "LCDCPCTL0 register accessor: an alias for `Reg<LCDCPCTL0_SPEC>`"]
pub type LCDCPCTL0 = crate::Reg<lcdcpctl0::LCDCPCTL0_SPEC>;
#[doc = "LCD_C Port Control Register 0"]
pub mod lcdcpctl0;
#[doc = "LCDCPCTL1 register accessor: an alias for `Reg<LCDCPCTL1_SPEC>`"]
pub type LCDCPCTL1 = crate::Reg<lcdcpctl1::LCDCPCTL1_SPEC>;
#[doc = "LCD_C Port Control Register 1"]
pub mod lcdcpctl1;
#[doc = "LCDCPCTL2 register accessor: an alias for `Reg<LCDCPCTL2_SPEC>`"]
pub type LCDCPCTL2 = crate::Reg<lcdcpctl2::LCDCPCTL2_SPEC>;
#[doc = "LCD_C Port Control Register 2"]
pub mod lcdcpctl2;
#[doc = "LCDCCPCTL register accessor: an alias for `Reg<LCDCCPCTL_SPEC>`"]
pub type LCDCCPCTL = crate::Reg<lcdccpctl::LCDCCPCTL_SPEC>;
#[doc = "LCD_C Charge Pump Control Register 3"]
pub mod lcdccpctl;
#[doc = "LCDCIV register accessor: an alias for `Reg<LCDCIV_SPEC>`"]
pub type LCDCIV = crate::Reg<lcdciv::LCDCIV_SPEC>;
#[doc = "LCD_C Interrupt Vector Register"]
pub mod lcdciv;
