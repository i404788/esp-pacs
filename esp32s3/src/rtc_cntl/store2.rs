#[doc = "Register `STORE2` reader"]
pub struct R(crate::R<STORE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE2` writer"]
pub struct W(crate::W<STORE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE2_SPEC>;
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
impl From<crate::W<STORE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH2` reader - Reserved register"]
pub type SCRATCH2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCRATCH2` writer - Reserved register"]
pub type SCRATCH2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STORE2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn scratch2(&self) -> SCRATCH2_R {
        SCRATCH2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn scratch2(&mut self) -> SCRATCH2_W<0> {
        SCRATCH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reserved register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store2](index.html) module"]
pub struct STORE2_SPEC;
impl crate::RegisterSpec for STORE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store2::R](R) reader structure"]
impl crate::Readable for STORE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store2::W](W) writer structure"]
impl crate::Writable for STORE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE2 to value 0"]
impl crate::Resettable for STORE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
