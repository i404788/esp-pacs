#[doc = "Register `HCDMA0` reader"]
pub struct R(crate::R<HCDMA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA0` writer"]
pub struct W(crate::W<HCDMA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA0_SPEC>;
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
impl From<crate::W<HCDMA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_DMAADDR0` reader - "]
pub type H_DMAADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H_DMAADDR0` writer - "]
pub type H_DMAADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr0(&self) -> H_DMAADDR0_R {
        H_DMAADDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr0(&mut self) -> H_DMAADDR0_W<0> {
        H_DMAADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma0](index.html) module"]
pub struct HCDMA0_SPEC;
impl crate::RegisterSpec for HCDMA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma0::R](R) reader structure"]
impl crate::Readable for HCDMA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma0::W](W) writer structure"]
impl crate::Writable for HCDMA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA0 to value 0"]
impl crate::Resettable for HCDMA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
