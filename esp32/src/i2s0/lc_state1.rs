#[doc = "Register `LC_STATE1` reader"]
pub struct R(crate::R<LC_STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LC_STATE1` reader - "]
pub type LC_STATE1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lc_state1(&self) -> LC_STATE1_R {
        LC_STATE1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_state1](index.html) module"]
pub struct LC_STATE1_SPEC;
impl crate::RegisterSpec for LC_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_state1::R](R) reader structure"]
impl crate::Readable for LC_STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_STATE1 to value 0"]
impl crate::Resettable for LC_STATE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
