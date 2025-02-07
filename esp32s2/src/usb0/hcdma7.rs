#[doc = "Register `HCDMA7` reader"]
pub struct R(crate::R<HCDMA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA7` writer"]
pub struct W(crate::W<HCDMA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA7_SPEC>;
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
impl From<crate::W<HCDMA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_DMAADDR7` reader - "]
pub type H_DMAADDR7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H_DMAADDR7` writer - "]
pub type H_DMAADDR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr7(&self) -> H_DMAADDR7_R {
        H_DMAADDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr7(&mut self) -> H_DMAADDR7_W<0> {
        H_DMAADDR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma7](index.html) module"]
pub struct HCDMA7_SPEC;
impl crate::RegisterSpec for HCDMA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma7::R](R) reader structure"]
impl crate::Readable for HCDMA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma7::W](W) writer structure"]
impl crate::Writable for HCDMA7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA7 to value 0"]
impl crate::Resettable for HCDMA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
