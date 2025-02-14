#[doc = "Register `HCDMA3` reader"]
pub struct R(crate::R<HCDMA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA3` writer"]
pub struct W(crate::W<HCDMA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA3_SPEC>;
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
impl From<crate::W<HCDMA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_DMAADDR3` reader - "]
pub type H_DMAADDR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H_DMAADDR3` writer - "]
pub type H_DMAADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr3(&self) -> H_DMAADDR3_R {
        H_DMAADDR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr3(&mut self) -> H_DMAADDR3_W<0> {
        H_DMAADDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma3](index.html) module"]
pub struct HCDMA3_SPEC;
impl crate::RegisterSpec for HCDMA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma3::R](R) reader structure"]
impl crate::Readable for HCDMA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma3::W](W) writer structure"]
impl crate::Writable for HCDMA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA3 to value 0"]
impl crate::Resettable for HCDMA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
