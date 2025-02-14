#[doc = "Register `WDTFEED` reader"]
pub struct R(crate::R<WDTFEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTFEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTFEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTFEED_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `WDT_FEED` reader - Need add desc"]
pub type WDT_FEED_R = crate::BitReader<bool>;
#[doc = "Field `WDT_FEED` writer - Need add desc"]
pub type WDT_FEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTFEED_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_feed(&self) -> WDT_FEED_R {
        WDT_FEED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_feed(&mut self) -> WDT_FEED_W<31> {
        WDT_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtfeed](index.html) module"]
pub struct WDTFEED_SPEC;
impl crate::RegisterSpec for WDTFEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtfeed::R](R) reader structure"]
impl crate::Readable for WDTFEED_SPEC {
    type Reader = R;
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
