#[doc = "Register `TRIGGER` writer"]
pub struct W(crate::W<TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGGER_SPEC>;
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
impl From<crate::W<TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` writer - Set this bit to start manual encryption calculation"]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIGGER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to start manual encryption calculation"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W<0> {
        TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](index.html) module"]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [trigger::W](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
