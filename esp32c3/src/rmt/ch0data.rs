#[doc = "Register `CH0DATA` reader"]
pub struct R(crate::R<CH0DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0DATA` writer"]
pub struct W(crate::W<CH0DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0DATA_SPEC>;
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
impl From<crate::W<CH0DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Reserved."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Reserved."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH0DATA_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0data](index.html) module"]
pub struct CH0DATA_SPEC;
impl crate::RegisterSpec for CH0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0data::R](R) reader structure"]
impl crate::Readable for CH0DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0data::W](W) writer structure"]
impl crate::Writable for CH0DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH0DATA to value 0"]
impl crate::Resettable for CH0DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
