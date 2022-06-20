#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Real Timer Clock Control 0/Key"]
    pub rtcctl0: crate::Reg<rtcctl0::RTCCTL0_SPEC>,
    #[doc = "0x02 - Real Timer Clock Control 1/3"]
    pub rtcctl13: crate::Reg<rtcctl13::RTCCTL13_SPEC>,
    #[doc = "0x04 - Real Timer Clock Offset Calibartion"]
    pub rtcocal: crate::Reg<rtcocal::RTCOCAL_SPEC>,
    #[doc = "0x06 - Real Timer Temperature Compensation"]
    pub rtctcmp: crate::Reg<rtctcmp::RTCTCMP_SPEC>,
    #[doc = "0x08 - Real Timer Prescale Timer 0 Control"]
    pub rtcps0ctl: crate::Reg<rtcps0ctl::RTCPS0CTL_SPEC>,
    #[doc = "0x0a - Real Timer Prescale Timer 1 Control"]
    pub rtcps1ctl: crate::Reg<rtcps1ctl::RTCPS1CTL_SPEC>,
    #[doc = "0x0c - Real Timer Prescale Timer Control"]
    pub rtcps: crate::Reg<rtcps::RTCPS_SPEC>,
    #[doc = "0x0e - Real Time Clock Interrupt Vector"]
    pub rtciv: crate::Reg<rtciv::RTCIV_SPEC>,
    #[doc = "0x10 - Real Time Clock Seconds"]
    pub rtcsec: crate::Reg<rtcsec::RTCSEC_SPEC>,
    #[doc = "0x11 - Real Time Clock Minutes"]
    pub rtcmin: crate::Reg<rtcmin::RTCMIN_SPEC>,
    #[doc = "0x12 - Real Time Clock Hour"]
    pub rtchour: crate::Reg<rtchour::RTCHOUR_SPEC>,
    #[doc = "0x13 - Real Time Clock Day of week"]
    pub rtcdow: crate::Reg<rtcdow::RTCDOW_SPEC>,
    #[doc = "0x14 - Real Time Clock Day"]
    pub rtcday: crate::Reg<rtcday::RTCDAY_SPEC>,
    #[doc = "0x15 - Real Time Clock Month"]
    pub rtcmon: crate::Reg<rtcmon::RTCMON_SPEC>,
    #[doc = "0x16 - Real Time Clock Year"]
    pub rtcyear: crate::Reg<rtcyear::RTCYEAR_SPEC>,
    #[doc = "0x18 - Real Time Clock Alarm Min"]
    pub rtcamin: crate::Reg<rtcamin::RTCAMIN_SPEC>,
    #[doc = "0x19 - Real Time Clock Alarm Hour"]
    pub rtcahour: crate::Reg<rtcahour::RTCAHOUR_SPEC>,
    #[doc = "0x1a - Real Time Clock Alarm Day of week"]
    pub rtcadow: crate::Reg<rtcadow::RTCADOW_SPEC>,
    #[doc = "0x1b - Real Time Clock Alarm Day"]
    pub rtcaday: crate::Reg<rtcaday::RTCADAY_SPEC>,
    #[doc = "0x1c - Real Time Binary-to-BCD conversion register"]
    pub bin2bcd: crate::Reg<bin2bcd::BIN2BCD_SPEC>,
    #[doc = "0x1e - Real Time BCD-to-binary conversion register"]
    pub bcd2bin: crate::Reg<bcd2bin::BCD2BIN_SPEC>,
}
#[doc = "RTCSEC register accessor: an alias for `Reg<RTCSEC_SPEC>`"]
pub type RTCSEC = crate::Reg<rtcsec::RTCSEC_SPEC>;
#[doc = "Real Time Clock Seconds"]
pub mod rtcsec;
#[doc = "RTCMIN register accessor: an alias for `Reg<RTCMIN_SPEC>`"]
pub type RTCMIN = crate::Reg<rtcmin::RTCMIN_SPEC>;
#[doc = "Real Time Clock Minutes"]
pub mod rtcmin;
#[doc = "RTCHOUR register accessor: an alias for `Reg<RTCHOUR_SPEC>`"]
pub type RTCHOUR = crate::Reg<rtchour::RTCHOUR_SPEC>;
#[doc = "Real Time Clock Hour"]
pub mod rtchour;
#[doc = "RTCDOW register accessor: an alias for `Reg<RTCDOW_SPEC>`"]
pub type RTCDOW = crate::Reg<rtcdow::RTCDOW_SPEC>;
#[doc = "Real Time Clock Day of week"]
pub mod rtcdow;
#[doc = "RTCDAY register accessor: an alias for `Reg<RTCDAY_SPEC>`"]
pub type RTCDAY = crate::Reg<rtcday::RTCDAY_SPEC>;
#[doc = "Real Time Clock Day"]
pub mod rtcday;
#[doc = "RTCMON register accessor: an alias for `Reg<RTCMON_SPEC>`"]
pub type RTCMON = crate::Reg<rtcmon::RTCMON_SPEC>;
#[doc = "Real Time Clock Month"]
pub mod rtcmon;
#[doc = "RTCAMIN register accessor: an alias for `Reg<RTCAMIN_SPEC>`"]
pub type RTCAMIN = crate::Reg<rtcamin::RTCAMIN_SPEC>;
#[doc = "Real Time Clock Alarm Min"]
pub mod rtcamin;
#[doc = "RTCAHOUR register accessor: an alias for `Reg<RTCAHOUR_SPEC>`"]
pub type RTCAHOUR = crate::Reg<rtcahour::RTCAHOUR_SPEC>;
#[doc = "Real Time Clock Alarm Hour"]
pub mod rtcahour;
#[doc = "RTCADOW register accessor: an alias for `Reg<RTCADOW_SPEC>`"]
pub type RTCADOW = crate::Reg<rtcadow::RTCADOW_SPEC>;
#[doc = "Real Time Clock Alarm Day of week"]
pub mod rtcadow;
#[doc = "RTCADAY register accessor: an alias for `Reg<RTCADAY_SPEC>`"]
pub type RTCADAY = crate::Reg<rtcaday::RTCADAY_SPEC>;
#[doc = "Real Time Clock Alarm Day"]
pub mod rtcaday;
#[doc = "RTCCTL0 register accessor: an alias for `Reg<RTCCTL0_SPEC>`"]
pub type RTCCTL0 = crate::Reg<rtcctl0::RTCCTL0_SPEC>;
#[doc = "Real Timer Clock Control 0/Key"]
pub mod rtcctl0;
#[doc = "RTCCTL13 register accessor: an alias for `Reg<RTCCTL13_SPEC>`"]
pub type RTCCTL13 = crate::Reg<rtcctl13::RTCCTL13_SPEC>;
#[doc = "Real Timer Clock Control 1/3"]
pub mod rtcctl13;
#[doc = "RTCOCAL register accessor: an alias for `Reg<RTCOCAL_SPEC>`"]
pub type RTCOCAL = crate::Reg<rtcocal::RTCOCAL_SPEC>;
#[doc = "Real Timer Clock Offset Calibartion"]
pub mod rtcocal;
#[doc = "RTCTCMP register accessor: an alias for `Reg<RTCTCMP_SPEC>`"]
pub type RTCTCMP = crate::Reg<rtctcmp::RTCTCMP_SPEC>;
#[doc = "Real Timer Temperature Compensation"]
pub mod rtctcmp;
#[doc = "RTCPS0CTL register accessor: an alias for `Reg<RTCPS0CTL_SPEC>`"]
pub type RTCPS0CTL = crate::Reg<rtcps0ctl::RTCPS0CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 0 Control"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL register accessor: an alias for `Reg<RTCPS1CTL_SPEC>`"]
pub type RTCPS1CTL = crate::Reg<rtcps1ctl::RTCPS1CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 1 Control"]
pub mod rtcps1ctl;
#[doc = "RTCPS register accessor: an alias for `Reg<RTCPS_SPEC>`"]
pub type RTCPS = crate::Reg<rtcps::RTCPS_SPEC>;
#[doc = "Real Timer Prescale Timer Control"]
pub mod rtcps;
#[doc = "RTCIV register accessor: an alias for `Reg<RTCIV_SPEC>`"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "Real Time Clock Interrupt Vector"]
pub mod rtciv;
#[doc = "RTCYEAR register accessor: an alias for `Reg<RTCYEAR_SPEC>`"]
pub type RTCYEAR = crate::Reg<rtcyear::RTCYEAR_SPEC>;
#[doc = "Real Time Clock Year"]
pub mod rtcyear;
#[doc = "BIN2BCD register accessor: an alias for `Reg<BIN2BCD_SPEC>`"]
pub type BIN2BCD = crate::Reg<bin2bcd::BIN2BCD_SPEC>;
#[doc = "Real Time Binary-to-BCD conversion register"]
pub mod bin2bcd;
#[doc = "BCD2BIN register accessor: an alias for `Reg<BCD2BIN_SPEC>`"]
pub type BCD2BIN = crate::Reg<bcd2bin::BCD2BIN_SPEC>;
#[doc = "Real Time BCD-to-binary conversion register"]
pub mod bcd2bin;
