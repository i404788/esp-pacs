#[doc = "Register `HSTIMER%s_VALUE` reader"]
pub struct R(crate::R<HSTIMER_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTIMER_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTIMER_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTIMER_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - software can read this register to get the current counter value in high speed timer0"]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer0"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstimer_value](index.html) module"]
pub struct HSTIMER_VALUE_SPEC;
impl crate::RegisterSpec for HSTIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstimer_value::R](R) reader structure"]
impl crate::Readable for HSTIMER_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSTIMER%s_VALUE to value 0"]
impl crate::Resettable for HSTIMER_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
