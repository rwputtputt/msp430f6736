#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC10 Control 0"]
    pub adc10ctl0: crate::Reg<adc10ctl0::ADC10CTL0_SPEC>,
    #[doc = "0x02 - ADC10 Control 1"]
    pub adc10ctl1: crate::Reg<adc10ctl1::ADC10CTL1_SPEC>,
    #[doc = "0x04 - ADC10 Control 2"]
    pub adc10ctl2: crate::Reg<adc10ctl2::ADC10CTL2_SPEC>,
    #[doc = "0x06 - ADC10 Window Comparator High Threshold"]
    pub adc10lo: crate::Reg<adc10lo::ADC10LO_SPEC>,
    #[doc = "0x08 - ADC10 Window Comparator High Threshold"]
    pub adc10hi: crate::Reg<adc10hi::ADC10HI_SPEC>,
    #[doc = "0x0a - ADC10 Memory Control 0"]
    pub adc10mctl0: crate::Reg<adc10mctl0::ADC10MCTL0_SPEC>,
    _reserved6: [u8; 0x06],
    #[doc = "0x12 - ADC10 Conversion Memory 0"]
    pub adc10mem0: crate::Reg<adc10mem0::ADC10MEM0_SPEC>,
    _reserved7: [u8; 0x06],
    #[doc = "0x1a - ADC10 Interrupt Enable"]
    pub adc10ie: crate::Reg<adc10ie::ADC10IE_SPEC>,
    #[doc = "0x1c - ADC10 Interrupt Flag"]
    pub adc10ifg: crate::Reg<adc10ifg::ADC10IFG_SPEC>,
    #[doc = "0x1e - ADC10 Interrupt Vector Word"]
    pub adc10iv: crate::Reg<adc10iv::ADC10IV_SPEC>,
}
#[doc = "ADC10CTL0 register accessor: an alias for `Reg<ADC10CTL0_SPEC>`"]
pub type ADC10CTL0 = crate::Reg<adc10ctl0::ADC10CTL0_SPEC>;
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10CTL1 register accessor: an alias for `Reg<ADC10CTL1_SPEC>`"]
pub type ADC10CTL1 = crate::Reg<adc10ctl1::ADC10CTL1_SPEC>;
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10CTL2 register accessor: an alias for `Reg<ADC10CTL2_SPEC>`"]
pub type ADC10CTL2 = crate::Reg<adc10ctl2::ADC10CTL2_SPEC>;
#[doc = "ADC10 Control 2"]
pub mod adc10ctl2;
#[doc = "ADC10LO register accessor: an alias for `Reg<ADC10LO_SPEC>`"]
pub type ADC10LO = crate::Reg<adc10lo::ADC10LO_SPEC>;
#[doc = "ADC10 Window Comparator High Threshold"]
pub mod adc10lo;
#[doc = "ADC10HI register accessor: an alias for `Reg<ADC10HI_SPEC>`"]
pub type ADC10HI = crate::Reg<adc10hi::ADC10HI_SPEC>;
#[doc = "ADC10 Window Comparator High Threshold"]
pub mod adc10hi;
#[doc = "ADC10MCTL0 register accessor: an alias for `Reg<ADC10MCTL0_SPEC>`"]
pub type ADC10MCTL0 = crate::Reg<adc10mctl0::ADC10MCTL0_SPEC>;
#[doc = "ADC10 Memory Control 0"]
pub mod adc10mctl0;
#[doc = "ADC10MEM0 register accessor: an alias for `Reg<ADC10MEM0_SPEC>`"]
pub type ADC10MEM0 = crate::Reg<adc10mem0::ADC10MEM0_SPEC>;
#[doc = "ADC10 Conversion Memory 0"]
pub mod adc10mem0;
#[doc = "ADC10IE register accessor: an alias for `Reg<ADC10IE_SPEC>`"]
pub type ADC10IE = crate::Reg<adc10ie::ADC10IE_SPEC>;
#[doc = "ADC10 Interrupt Enable"]
pub mod adc10ie;
#[doc = "ADC10IFG register accessor: an alias for `Reg<ADC10IFG_SPEC>`"]
pub type ADC10IFG = crate::Reg<adc10ifg::ADC10IFG_SPEC>;
#[doc = "ADC10 Interrupt Flag"]
pub mod adc10ifg;
#[doc = "ADC10IV register accessor: an alias for `Reg<ADC10IV_SPEC>`"]
pub type ADC10IV = crate::Reg<adc10iv::ADC10IV_SPEC>;
#[doc = "ADC10 Interrupt Vector Word"]
pub mod adc10iv;
