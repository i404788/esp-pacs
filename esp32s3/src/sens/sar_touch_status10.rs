#[doc = "Register `SAR_TOUCH_STATUS10` reader"]
pub struct R(crate::R<SAR_TOUCH_STATUS10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_STATUS10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_STATUS10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_STATUS10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR_TOUCH_PAD10_DATA` reader - touch data debounce of touch pad 10"]
pub type SAR_TOUCH_PAD10_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_TOUCH_PAD10_DEBOUNCE` reader - touch current debounce of touch pad 10"]
pub type SAR_TOUCH_PAD10_DEBOUNCE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:21 - touch data debounce of touch pad 10"]
    #[inline(always)]
    pub fn sar_touch_pad10_data(&self) -> SAR_TOUCH_PAD10_DATA_R {
        SAR_TOUCH_PAD10_DATA_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 29:31 - touch current debounce of touch pad 10"]
    #[inline(always)]
    pub fn sar_touch_pad10_debounce(&self) -> SAR_TOUCH_PAD10_DEBOUNCE_R {
        SAR_TOUCH_PAD10_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "touch channel status of touch pad 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status10](index.html) module"]
pub struct SAR_TOUCH_STATUS10_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_status10::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS10 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
