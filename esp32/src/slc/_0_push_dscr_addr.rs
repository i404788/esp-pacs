#[doc = "Register `_0_PUSH_DSCR_ADDR` reader"]
pub struct R(crate::R<_0_PUSH_DSCR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_PUSH_DSCR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_PUSH_DSCR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_PUSH_DSCR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_RX_PUSH_DSCR_ADDR` reader - "]
pub type SLC0_RX_PUSH_DSCR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rx_push_dscr_addr(&self) -> SLC0_RX_PUSH_DSCR_ADDR_R {
        SLC0_RX_PUSH_DSCR_ADDR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_push_dscr_addr](index.html) module"]
pub struct _0_PUSH_DSCR_ADDR_SPEC;
impl crate::RegisterSpec for _0_PUSH_DSCR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_push_dscr_addr::R](R) reader structure"]
impl crate::Readable for _0_PUSH_DSCR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_PUSH_DSCR_ADDR to value 0"]
impl crate::Resettable for _0_PUSH_DSCR_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
