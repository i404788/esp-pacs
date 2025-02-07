#[doc = "Register `SLP_REJECT_CAUSE` reader"]
pub struct R(crate::R<SLP_REJECT_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_REJECT_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_REJECT_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_REJECT_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub type REJECT_CAUSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_cause](index.html) module"]
pub struct SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_reject_cause::R](R) reader structure"]
impl crate::Readable for SLP_REJECT_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLP_REJECT_CAUSE to value 0"]
impl crate::Resettable for SLP_REJECT_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
