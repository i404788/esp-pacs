#[doc = "Register `SDIO_0` reader"]
pub struct R(crate::R<SDIO_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_0` writer"]
pub struct W(crate::W<SDIO_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_0_SPEC>;
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
impl From<crate::W<SDIO_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_LOCK` reader - Lock register. Setting to 1 locks SDIO permission control registers."]
pub type SDIO_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_LOCK` writer - Lock register. Setting to 1 locks SDIO permission control registers."]
pub type SDIO_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks SDIO permission control registers."]
    #[inline(always)]
    pub fn sdio_lock(&self) -> SDIO_LOCK_R {
        SDIO_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks SDIO permission control registers."]
    #[inline(always)]
    pub fn sdio_lock(&mut self) -> SDIO_LOCK_W<0> {
        SDIO_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO permission control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_0](index.html) module"]
pub struct SDIO_0_SPEC;
impl crate::RegisterSpec for SDIO_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_0::R](R) reader structure"]
impl crate::Readable for SDIO_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_0::W](W) writer structure"]
impl crate::Writable for SDIO_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_0 to value 0"]
impl crate::Resettable for SDIO_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
