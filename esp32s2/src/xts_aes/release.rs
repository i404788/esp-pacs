#[doc = "Register `RELEASE` writer"]
pub struct W(crate::W<RELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RELEASE_SPEC>;
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
impl From<crate::W<RELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELEASE` writer - Set to grant SPI1 access to encrypted result."]
pub type RELEASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RELEASE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set to grant SPI1 access to encrypted result."]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W<0> {
        RELEASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Release control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](index.html) module"]
pub struct RELEASE_SPEC;
impl crate::RegisterSpec for RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [release::W](W) writer structure"]
impl crate::Writable for RELEASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for RELEASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
