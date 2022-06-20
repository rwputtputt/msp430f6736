#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Battery Backup Memory 0"]
    pub bakmem0: crate::Reg<bakmem0::BAKMEM0_SPEC>,
    #[doc = "0x02 - Battery Backup Memory 1"]
    pub bakmem1: crate::Reg<bakmem1::BAKMEM1_SPEC>,
    #[doc = "0x04 - Battery Backup Memory 2"]
    pub bakmem2: crate::Reg<bakmem2::BAKMEM2_SPEC>,
    #[doc = "0x06 - Battery Backup Memory 3"]
    pub bakmem3: crate::Reg<bakmem3::BAKMEM3_SPEC>,
}
#[doc = "BAKMEM0 register accessor: an alias for `Reg<BAKMEM0_SPEC>`"]
pub type BAKMEM0 = crate::Reg<bakmem0::BAKMEM0_SPEC>;
#[doc = "Battery Backup Memory 0"]
pub mod bakmem0;
#[doc = "BAKMEM1 register accessor: an alias for `Reg<BAKMEM1_SPEC>`"]
pub type BAKMEM1 = crate::Reg<bakmem1::BAKMEM1_SPEC>;
#[doc = "Battery Backup Memory 1"]
pub mod bakmem1;
#[doc = "BAKMEM2 register accessor: an alias for `Reg<BAKMEM2_SPEC>`"]
pub type BAKMEM2 = crate::Reg<bakmem2::BAKMEM2_SPEC>;
#[doc = "Battery Backup Memory 2"]
pub mod bakmem2;
#[doc = "BAKMEM3 register accessor: an alias for `Reg<BAKMEM3_SPEC>`"]
pub type BAKMEM3 = crate::Reg<bakmem3::BAKMEM3_SPEC>;
#[doc = "Battery Backup Memory 3"]
pub mod bakmem3;
