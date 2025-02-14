#[doc = "Register `CORE_1_IRAM0_EXCEPTION_MONITOR_1` reader"]
pub struct R(crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_IRAM0_RECORDING_ADDR_1` reader - The second iram0's addr\\[25:2\\] status when trigger IRAM busy interrupt"]
pub type CORE_1_IRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_1_IRAM0_RECORDING_WR_1` reader - The second iram0's wr status when trigger IRAM busy interrupt"]
pub type CORE_1_IRAM0_RECORDING_WR_1_R = crate::BitReader<bool>;
#[doc = "Field `CORE_1_IRAM0_RECORDING_LOADSTORE_1` reader - The second iram0's loadstore status when trigger IRAM busy interrupt"]
pub type CORE_1_IRAM0_RECORDING_LOADSTORE_1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:23 - The second iram0's addr\\[25:2\\] status when trigger IRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_iram0_recording_addr_1(&self) -> CORE_1_IRAM0_RECORDING_ADDR_1_R {
        CORE_1_IRAM0_RECORDING_ADDR_1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - The second iram0's wr status when trigger IRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_iram0_recording_wr_1(&self) -> CORE_1_IRAM0_RECORDING_WR_1_R {
        CORE_1_IRAM0_RECORDING_WR_1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The second iram0's loadstore status when trigger IRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_iram0_recording_loadstore_1(&self) -> CORE_1_IRAM0_RECORDING_LOADSTORE_1_R {
        CORE_1_IRAM0_RECORDING_LOADSTORE_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Core1 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_iram0_exception_monitor_1](index.html) module"]
pub struct CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_iram0_exception_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_IRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
