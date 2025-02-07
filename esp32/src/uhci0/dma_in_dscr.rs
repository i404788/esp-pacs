#[doc = "Register `DMA_IN_DSCR` reader"]
pub struct R(crate::R<DMA_IN_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR` reader - The content of current in link descriptor's third dword"]
pub type INLINK_DSCR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current in link descriptor's third dword"]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr](index.html) module"]
pub struct DMA_IN_DSCR_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_dscr::R](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_DSCR to value 0"]
impl crate::Resettable for DMA_IN_DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
