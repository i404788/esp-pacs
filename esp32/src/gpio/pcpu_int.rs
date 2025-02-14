#[doc = "Register `PCPU_INT` reader"]
pub struct R(crate::R<PCPU_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPU_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPU_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPU_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCPU_INT` reader - GPIO0~31 PRO CPU interrupt status"]
pub type PROCPU_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 PRO CPU interrupt status"]
    #[inline(always)]
    pub fn procpu_int(&self) -> PROCPU_INT_R {
        PROCPU_INT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_int](index.html) module"]
pub struct PCPU_INT_SPEC;
impl crate::RegisterSpec for PCPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpu_int::R](R) reader structure"]
impl crate::Readable for PCPU_INT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCPU_INT to value 0"]
impl crate::Resettable for PCPU_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
