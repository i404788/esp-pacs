#[doc = "Register `CACHE_MMU_FAULT_VADDR` reader"]
pub struct R(crate::R<CACHE_MMU_FAULT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_FAULT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_FAULT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_FAULT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACHE_MMU_FAULT_VADDR` reader - The bits are used to indicate the virtual address which cause mmu fault.."]
pub type CACHE_MMU_FAULT_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address which cause mmu fault.."]
    #[inline(always)]
    pub fn cache_mmu_fault_vaddr(&self) -> CACHE_MMU_FAULT_VADDR_R {
        CACHE_MMU_FAULT_VADDR_R::new(self.bits)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_fault_vaddr](index.html) module"]
pub struct CACHE_MMU_FAULT_VADDR_SPEC;
impl crate::RegisterSpec for CACHE_MMU_FAULT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_fault_vaddr::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_FAULT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_MMU_FAULT_VADDR to value 0"]
impl crate::Resettable for CACHE_MMU_FAULT_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
