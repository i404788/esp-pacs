#[doc = "Register `UNIT0_VALUE_HI` reader"]
pub struct R(crate::R<UNIT0_VALUE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT0_VALUE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT0_VALUE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT0_VALUE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_UNIT0_VALUE_HI` reader - timer read value high 20bits"]
pub type TIMER_UNIT0_VALUE_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - timer read value high 20bits"]
    #[inline(always)]
    pub fn timer_unit0_value_hi(&self) -> TIMER_UNIT0_VALUE_HI_R {
        TIMER_UNIT0_VALUE_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "system timer unit0 value high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit0_value_hi](index.html) module"]
pub struct UNIT0_VALUE_HI_SPEC;
impl crate::RegisterSpec for UNIT0_VALUE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit0_value_hi::R](R) reader structure"]
impl crate::Readable for UNIT0_VALUE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNIT0_VALUE_HI to value 0"]
impl crate::Resettable for UNIT0_VALUE_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
