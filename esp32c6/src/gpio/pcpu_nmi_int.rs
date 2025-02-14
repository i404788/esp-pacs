#[doc = "Register `PCPU_NMI_INT` reader"]
pub struct R(crate::R<PCPU_NMI_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPU_NMI_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPU_NMI_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPU_NMI_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCPU_NMI_INT` reader - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
pub type PROCPU_NMI_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub fn procpu_nmi_int(&self) -> PROCPU_NMI_INT_R {
        PROCPU_NMI_INT_R::new(self.bits)
    }
}
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_nmi_int](index.html) module"]
pub struct PCPU_NMI_INT_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpu_nmi_int::R](R) reader structure"]
impl crate::Readable for PCPU_NMI_INT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCPU_NMI_INT to value 0"]
impl crate::Resettable for PCPU_NMI_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
