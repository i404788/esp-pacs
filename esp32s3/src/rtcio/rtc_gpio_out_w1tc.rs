#[doc = "Register `RTC_GPIO_OUT_W1TC` writer"]
pub struct W(crate::W<RTC_GPIO_OUT_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_OUT_W1TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_GPIO_OUT_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_OUT_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_OUT_DATA_W1TC` writer - RTC GPIO 0 ~ 21 output data write 1 to clear"]
pub type RTC_GPIO_OUT_DATA_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTC_GPIO_OUT_W1TC_SPEC, u32, u32, 22, O>;
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 output data write 1 to clear"]
    #[inline(always)]
    pub fn rtc_gpio_out_data_w1tc(&mut self) -> RTC_GPIO_OUT_DATA_W1TC_W<10> {
        RTC_GPIO_OUT_DATA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "one clear RTC GPIO output data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_out_w1tc](index.html) module"]
pub struct RTC_GPIO_OUT_W1TC_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_out_w1tc::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_OUT_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_OUT_W1TC to value 0"]
impl crate::Resettable for RTC_GPIO_OUT_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
