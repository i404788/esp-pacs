#[doc = "Register `W11` reader"]
pub struct R(crate::R<W11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W11` writer"]
pub struct W(crate::W<W11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W11_SPEC>;
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
impl From<crate::W<W11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF11` reader - data buffer"]
pub type BUF11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF11` writer - data buffer"]
pub type BUF11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, W11_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf11(&self) -> BUF11_R {
        BUF11_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf11(&mut self) -> BUF11_W<0> {
        BUF11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w11](index.html) module"]
pub struct W11_SPEC;
impl crate::RegisterSpec for W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w11::R](R) reader structure"]
impl crate::Readable for W11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w11::W](W) writer structure"]
impl crate::Writable for W11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W11 to value 0"]
impl crate::Resettable for W11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
