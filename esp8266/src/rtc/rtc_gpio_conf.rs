#[doc = "Register `RTC_GPIO_CONF` reader"]
pub struct R(crate::R<RTC_GPIO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_GPIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_GPIO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_GPIO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_GPIO_CONF` writer"]
pub struct W(crate::W<RTC_GPIO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_GPIO_CONF_SPEC>;
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
impl From<crate::W<RTC_GPIO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_GPIO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_GPIO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_gpio_conf](index.html) module"]
pub struct RTC_GPIO_CONF_SPEC;
impl crate::RegisterSpec for RTC_GPIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_gpio_conf::R](R) reader structure"]
impl crate::Readable for RTC_GPIO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_gpio_conf::W](W) writer structure"]
impl crate::Writable for RTC_GPIO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_GPIO_CONF to value 0"]
impl crate::Resettable for RTC_GPIO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
