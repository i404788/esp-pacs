#[doc = "Register `WDTFEED` writer"]
pub struct W(crate::W<WDTFEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTFEED_SPEC>;
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
impl From<crate::W<WDTFEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTFEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_FEED` writer - wdt_feed"]
pub type WDT_FEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTFEED_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - wdt_feed"]
    #[inline(always)]
    pub fn wdt_feed(&mut self) -> WDT_FEED_W<0> {
        WDT_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_WDTFEED_REG.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtfeed](index.html) module"]
pub struct WDTFEED_SPEC;
impl crate::RegisterSpec for WDTFEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdtfeed::W](W) writer structure"]
impl crate::Writable for WDTFEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTFEED to value 0"]
impl crate::Resettable for WDTFEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
