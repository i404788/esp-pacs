#[doc = "Register `SPI_W9` reader"]
pub struct R(crate::R<SPI_W9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W9` writer"]
pub struct W(crate::W<SPI_W9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W9_SPEC>;
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
impl From<crate::W<SPI_W9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_BUF9` reader - data buffer"]
pub type SPI_BUF9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_BUF9` writer - data buffer"]
pub type SPI_BUF9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_W9_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf9(&self) -> SPI_BUF9_R {
        SPI_BUF9_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_buf9(&mut self) -> SPI_BUF9_W<0> {
        SPI_BUF9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w9](index.html) module"]
pub struct SPI_W9_SPEC;
impl crate::RegisterSpec for SPI_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w9::R](R) reader structure"]
impl crate::Readable for SPI_W9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w9::W](W) writer structure"]
impl crate::Writable for SPI_W9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_W9 to value 0"]
impl crate::Resettable for SPI_W9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
