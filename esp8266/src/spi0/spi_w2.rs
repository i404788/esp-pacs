#[doc = "Register `SPI_W2` reader"]
pub struct R(crate::R<SPI_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W2` writer"]
pub struct W(crate::W<SPI_W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W2_SPEC>;
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
impl From<crate::W<SPI_W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_w2` reader - the data inside the buffer of the SPI module, word 2"]
pub type SPI_W2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `spi_w2` writer - the data inside the buffer of the SPI module, word 2"]
pub type SPI_W2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_W2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 2"]
    #[inline(always)]
    pub fn spi_w2(&self) -> SPI_W2_R {
        SPI_W2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 2"]
    #[inline(always)]
    pub fn spi_w2(&mut self) -> SPI_W2_W<0> {
        SPI_W2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the data inside the buffer of the SPI module, word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w2](index.html) module"]
pub struct SPI_W2_SPEC;
impl crate::RegisterSpec for SPI_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w2::R](R) reader structure"]
impl crate::Readable for SPI_W2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w2::W](W) writer structure"]
impl crate::Writable for SPI_W2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_W2 to value 0"]
impl crate::Resettable for SPI_W2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
