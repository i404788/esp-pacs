#[doc = "Register `INLINK_DSCR` reader"]
pub struct R(crate::R<INLINK_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INLINK_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INLINK_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INLINK_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_INLINK_DSCR` reader - The content of current in descriptor pointer."]
pub type DMA_INLINK_DSCR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current in descriptor pointer."]
    #[inline(always)]
    pub fn dma_inlink_dscr(&self) -> DMA_INLINK_DSCR_R {
        DMA_INLINK_DSCR_R::new(self.bits)
    }
}
#[doc = "Current SPI DMA RX descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inlink_dscr](index.html) module"]
pub struct INLINK_DSCR_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inlink_dscr::R](R) reader structure"]
impl crate::Readable for INLINK_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INLINK_DSCR to value 0"]
impl crate::Resettable for INLINK_DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
