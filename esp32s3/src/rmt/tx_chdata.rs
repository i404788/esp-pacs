#[doc = "Register `TX_CH%sDATA` reader"]
pub struct R(crate::R<TX_CHDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CHDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CHDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CHDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHDATA` reader - Read and write data for channel %s via APB FIFO."]
pub type CHDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read and write data for channel %s via APB FIFO."]
    #[inline(always)]
    pub fn chdata(&self) -> CHDATA_R {
        CHDATA_R::new(self.bits)
    }
}
#[doc = "The read and write data register for CHANNEL%s by apb fifo access.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_chdata](index.html) module"]
pub struct TX_CHDATA_SPEC;
impl crate::RegisterSpec for TX_CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_chdata::R](R) reader structure"]
impl crate::Readable for TX_CHDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CH%sDATA to value 0"]
impl crate::Resettable for TX_CHDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
