#[doc = "Register `SPI4_DMA_INT_MAP` reader"]
pub struct R(crate::R<SPI4_DMA_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI4_DMA_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI4_DMA_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI4_DMA_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI4_DMA_INT_MAP` writer"]
pub struct W(crate::W<SPI4_DMA_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI4_DMA_INT_MAP_SPEC>;
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
impl From<crate::W<SPI4_DMA_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI4_DMA_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI4_DMA_INT_MAP` reader - this register used to map spi4_dma interrupt to one of core0's external interrupt"]
pub type SPI4_DMA_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI4_DMA_INT_MAP` writer - this register used to map spi4_dma interrupt to one of core0's external interrupt"]
pub type SPI4_DMA_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI4_DMA_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map spi4_dma interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn spi4_dma_int_map(&self) -> SPI4_DMA_INT_MAP_R {
        SPI4_DMA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map spi4_dma interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn spi4_dma_int_map(&mut self) -> SPI4_DMA_INT_MAP_W<0> {
        SPI4_DMA_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi4_dma interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi4_dma_int_map](index.html) module"]
pub struct SPI4_DMA_INT_MAP_SPEC;
impl crate::RegisterSpec for SPI4_DMA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi4_dma_int_map::R](R) reader structure"]
impl crate::Readable for SPI4_DMA_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi4_dma_int_map::W](W) writer structure"]
impl crate::Writable for SPI4_DMA_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI4_DMA_INT_MAP to value 0x10"]
impl crate::Resettable for SPI4_DMA_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
