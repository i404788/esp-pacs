#[doc = "Register `SPI_W4` reader"]
pub struct R(crate::R<SPI_W4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W4` writer"]
pub struct W(crate::W<SPI_W4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W4_SPEC>;
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
impl From<crate::W<SPI_W4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_w4` reader - the data inside the buffer of the SPI module, word 4"]
pub type SPI_W4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `spi_w4` writer - the data inside the buffer of the SPI module, word 4"]
pub type SPI_W4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_W4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 4"]
    #[inline(always)]
    pub fn spi_w4(&self) -> SPI_W4_R {
        SPI_W4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 4"]
    #[inline(always)]
    pub fn spi_w4(&mut self) -> SPI_W4_W<0> {
        SPI_W4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the data inside the buffer of the SPI module, word 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w4](index.html) module"]
pub struct SPI_W4_SPEC;
impl crate::RegisterSpec for SPI_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w4::R](R) reader structure"]
impl crate::Readable for SPI_W4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w4::W](W) writer structure"]
impl crate::Writable for SPI_W4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_W4 to value 0"]
impl crate::Resettable for SPI_W4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
