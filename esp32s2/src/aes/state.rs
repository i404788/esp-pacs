#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE` reader - Stores the working status of the AES Accelerator. For details, see Table 3 for Typical AES working mode and Table 9 for DMA AES working mode. For typical AES; 0 = idle; 1 = busy. For DMA-AES; 0 = idle; 1 = busy; 2 = calculation_done."]
pub type STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Stores the working status of the AES Accelerator. For details, see Table 3 for Typical AES working mode and Table 9 for DMA AES working mode. For typical AES; 0 = idle; 1 = busy. For DMA-AES; 0 = idle; 1 = busy; 2 = calculation_done."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Operation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
