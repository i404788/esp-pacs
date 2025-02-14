#[doc = "Register `DIAG1` reader"]
pub struct R(crate::R<DIAG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIAG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIAG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIAG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOW_POWER_DIAG1` reader - "]
pub type LOW_POWER_DIAG1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diag1](index.html) module"]
pub struct DIAG1_SPEC;
impl crate::RegisterSpec for DIAG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diag1::R](R) reader structure"]
impl crate::Readable for DIAG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIAG1 to value 0"]
impl crate::Resettable for DIAG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
