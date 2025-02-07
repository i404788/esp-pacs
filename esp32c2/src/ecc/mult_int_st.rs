#[doc = "Register `MULT_INT_ST` reader"]
pub struct R(crate::R<MULT_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALC_DONE_INT_ST` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type CALC_DONE_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done_int_st(&self) -> CALC_DONE_INT_ST_R {
        CALC_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2S interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_int_st](index.html) module"]
pub struct MULT_INT_ST_SPEC;
impl crate::RegisterSpec for MULT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_int_st::R](R) reader structure"]
impl crate::Readable for MULT_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULT_INT_ST to value 0"]
impl crate::Resettable for MULT_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
