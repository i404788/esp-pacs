#[doc = "Register `T0LO` reader"]
pub struct R(crate::R<T0LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T_LO` reader - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
pub type T_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn t_lo(&self) -> T_LO_R {
        T_LO_R::new(self.bits)
    }
}
#[doc = "Timer %s current value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0lo](index.html) module"]
pub struct T0LO_SPEC;
impl crate::RegisterSpec for T0LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0lo::R](R) reader structure"]
impl crate::Readable for T0LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0LO to value 0"]
impl crate::Resettable for T0LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}