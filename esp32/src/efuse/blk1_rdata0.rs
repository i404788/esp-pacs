#[doc = "Register `BLK1_RDATA0` reader"]
pub struct R(crate::R<BLK1_RDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_RDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_RDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_RDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK1_DOUT0` reader - read for BLOCK1"]
pub type BLK1_DOUT0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn blk1_dout0(&self) -> BLK1_DOUT0_R {
        BLK1_DOUT0_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_rdata0](index.html) module"]
pub struct BLK1_RDATA0_SPEC;
impl crate::RegisterSpec for BLK1_RDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_rdata0::R](R) reader structure"]
impl crate::Readable for BLK1_RDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_RDATA0 to value 0"]
impl crate::Resettable for BLK1_RDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
