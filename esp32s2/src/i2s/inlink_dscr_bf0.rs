#[doc = "Register `INLINK_DSCR_BF0` reader"]
pub struct R(crate::R<INLINK_DSCR_BF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INLINK_DSCR_BF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INLINK_DSCR_BF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INLINK_DSCR_BF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_BF0` reader - The address of next inlink descriptor."]
pub type INLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of next inlink descriptor."]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[doc = "Address of next inlink descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inlink_dscr_bf0](index.html) module"]
pub struct INLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for INLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inlink_dscr_bf0::R](R) reader structure"]
impl crate::Readable for INLINK_DSCR_BF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for INLINK_DSCR_BF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
