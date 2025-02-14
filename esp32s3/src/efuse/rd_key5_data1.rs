#[doc = "Register `RD_KEY5_DATA1` reader"]
pub struct R(crate::R<RD_KEY5_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY5_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY5_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY5_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY5_DATA1` reader - Stores the first 32 bits of KEY5."]
pub type KEY5_DATA1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the first 32 bits of KEY5."]
    #[inline(always)]
    pub fn key5_data1(&self) -> KEY5_DATA1_R {
        KEY5_DATA1_R::new(self.bits)
    }
}
#[doc = "Register 1 of BLOCK9 (KEY5).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data1](index.html) module"]
pub struct RD_KEY5_DATA1_SPEC;
impl crate::RegisterSpec for RD_KEY5_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key5_data1::R](R) reader structure"]
impl crate::Readable for RD_KEY5_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY5_DATA1 to value 0"]
impl crate::Resettable for RD_KEY5_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
