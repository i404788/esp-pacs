#[doc = "Register `SDIO_1` reader"]
pub struct R(crate::R<SDIO_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_1` writer"]
pub struct W(crate::W<SDIO_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_1_SPEC>;
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
impl From<crate::W<SDIO_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_DISABLE` reader - Setting to 1 disables the SDIO function."]
pub type SDIO_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_DISABLE` writer - Setting to 1 disables the SDIO function."]
pub type SDIO_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    pub fn sdio_disable(&self) -> SDIO_DISABLE_R {
        SDIO_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    pub fn sdio_disable(&mut self) -> SDIO_DISABLE_W<0> {
        SDIO_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_1](index.html) module"]
pub struct SDIO_1_SPEC;
impl crate::RegisterSpec for SDIO_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_1::R](R) reader structure"]
impl crate::Readable for SDIO_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_1::W](W) writer structure"]
impl crate::Writable for SDIO_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_1 to value 0"]
impl crate::Resettable for SDIO_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
