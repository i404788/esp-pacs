#[doc = "Register `EXT_WAKEUP1` reader"]
pub struct R(crate::R<EXT_WAKEUP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP1` writer"]
pub struct W(crate::W<EXT_WAKEUP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP1_SPEC>;
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
impl From<crate::W<EXT_WAKEUP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Selects a RTC GPIO to be the EXT1 wakeup source."]
pub type SEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEL` writer - Selects a RTC GPIO to be the EXT1 wakeup source."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXT_WAKEUP1_SPEC, u32, u32, 22, O>;
#[doc = "Field `STATUS_CLR` writer - Clears the EXT1 wakeup status."]
pub type STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_WAKEUP1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:21 - Selects a RTC GPIO to be the EXT1 wakeup source."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Selects a RTC GPIO to be the EXT1 wakeup source."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 22 - Clears the EXT1 wakeup status."]
    #[inline(always)]
    pub fn status_clr(&mut self) -> STATUS_CLR_W<22> {
        STATUS_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXT1 wakeup configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup1](index.html) module"]
pub struct EXT_WAKEUP1_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup1::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup1::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_WAKEUP1 to value 0"]
impl crate::Resettable for EXT_WAKEUP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
