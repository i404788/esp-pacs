#[doc = "Register `I2SRXEOF_NUM` reader"]
pub struct R(crate::R<I2SRXEOF_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SRXEOF_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SRXEOF_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SRXEOF_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SRXEOF_NUM` writer"]
pub struct W(crate::W<I2SRXEOF_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SRXEOF_NUM_SPEC>;
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
impl From<crate::W<I2SRXEOF_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SRXEOF_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_I2S_RX_EOF_NUM` reader - "]
pub type I2S_I2S_RX_EOF_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `I2S_I2S_RX_EOF_NUM` writer - "]
pub type I2S_I2S_RX_EOF_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SRXEOF_NUM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2s_rx_eof_num(&self) -> I2S_I2S_RX_EOF_NUM_R {
        I2S_I2S_RX_EOF_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_i2s_rx_eof_num(&mut self) -> I2S_I2S_RX_EOF_NUM_W<0> {
        I2S_I2S_RX_EOF_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SRXEOF_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2srxeof_num](index.html) module"]
pub struct I2SRXEOF_NUM_SPEC;
impl crate::RegisterSpec for I2SRXEOF_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2srxeof_num::R](R) reader structure"]
impl crate::Readable for I2SRXEOF_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2srxeof_num::W](W) writer structure"]
impl crate::Writable for I2SRXEOF_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SRXEOF_NUM to value 0"]
impl crate::Resettable for I2SRXEOF_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
