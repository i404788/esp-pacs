#[doc = "Register `T0LOADHI` reader"]
pub struct R(crate::R<T0LOADHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LOADHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LOADHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LOADHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0LOADHI` writer"]
pub struct W(crate::W<T0LOADHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOADHI_SPEC>;
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
impl From<crate::W<T0LOADHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOADHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_HI` reader - higher 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD_HI` writer - higher 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0LOADHI_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    pub fn load_hi(&mut self) -> LOAD_HI_W<0> {
        LOAD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadhi](index.html) module"]
pub struct T0LOADHI_SPEC;
impl crate::RegisterSpec for T0LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0loadhi::R](R) reader structure"]
impl crate::Readable for T0LOADHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0loadhi::W](W) writer structure"]
impl crate::Writable for T0LOADHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0LOADHI to value 0"]
impl crate::Resettable for T0LOADHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
