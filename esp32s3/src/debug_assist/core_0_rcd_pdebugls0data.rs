#[doc = "Register `CORE_0_RCD_PDEBUGLS0DATA` reader"]
pub struct R(crate::R<CORE_0_RCD_PDEBUGLS0DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_PDEBUGLS0DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_PDEBUGLS0DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_PDEBUGLS0DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_RCD_PDEBUGLS0DATA` reader - core0_pdebug_s0data"]
pub type CORE_0_RCD_PDEBUGLS0DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - core0_pdebug_s0data"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugls0data(&self) -> CORE_0_RCD_PDEBUGLS0DATA_R {
        CORE_0_RCD_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[doc = "core0 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_pdebugls0data](index.html) module"]
pub struct CORE_0_RCD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_pdebugls0data::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGLS0DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGLS0DATA to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGLS0DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
