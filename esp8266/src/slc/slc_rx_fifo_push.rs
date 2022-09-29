#[doc = "Register `SLC_RX_FIFO_PUSH` reader"]
pub struct R(crate::R<SLC_RX_FIFO_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_RX_FIFO_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_RX_FIFO_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_RX_FIFO_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_RX_FIFO_PUSH` writer"]
pub struct W(crate::W<SLC_RX_FIFO_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_RX_FIFO_PUSH_SPEC>;
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
impl From<crate::W<SLC_RX_FIFO_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_RX_FIFO_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_RXFIFO_WDATA` reader - "]
pub type SLC_RXFIFO_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC_RXFIFO_WDATA` writer - "]
pub type SLC_RXFIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLC_RX_FIFO_PUSH_SPEC, u16, u16, 9, O>;
#[doc = "Field `SLC_RXFIFO_PUSH` reader - "]
pub type SLC_RXFIFO_PUSH_R = crate::BitReader<bool>;
#[doc = "Field `SLC_RXFIFO_PUSH` writer - "]
pub type SLC_RXFIFO_PUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLC_RX_FIFO_PUSH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc_rxfifo_wdata(&self) -> SLC_RXFIFO_WDATA_R {
        SLC_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_rxfifo_push(&self) -> SLC_RXFIFO_PUSH_R {
        SLC_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc_rxfifo_wdata(&mut self) -> SLC_RXFIFO_WDATA_W<0> {
        SLC_RXFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_rxfifo_push(&mut self) -> SLC_RXFIFO_PUSH_W<16> {
        SLC_RXFIFO_PUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_RX_FIFO_PUSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_rx_fifo_push](index.html) module"]
pub struct SLC_RX_FIFO_PUSH_SPEC;
impl crate::RegisterSpec for SLC_RX_FIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_rx_fifo_push::R](R) reader structure"]
impl crate::Readable for SLC_RX_FIFO_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_rx_fifo_push::W](W) writer structure"]
impl crate::Writable for SLC_RX_FIFO_PUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_RX_FIFO_PUSH to value 0"]
impl crate::Resettable for SLC_RX_FIFO_PUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}