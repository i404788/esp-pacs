#[doc = "Register `SAR_TOUCH_OUT4` reader"]
pub struct R(crate::R<SAR_TOUCH_OUT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_OUT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_OUT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_OUT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_MEAS_OUT7` reader - the counter for touch pad 7"]
pub type TOUCH_MEAS_OUT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOUCH_MEAS_OUT6` reader - the counter for touch pad 6"]
pub type TOUCH_MEAS_OUT6_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 7"]
    #[inline(always)]
    pub fn touch_meas_out7(&self) -> TOUCH_MEAS_OUT7_R {
        TOUCH_MEAS_OUT7_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 6"]
    #[inline(always)]
    pub fn touch_meas_out6(&self) -> TOUCH_MEAS_OUT6_R {
        TOUCH_MEAS_OUT6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_out4](index.html) module"]
pub struct SAR_TOUCH_OUT4_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_out4::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_OUT4 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
