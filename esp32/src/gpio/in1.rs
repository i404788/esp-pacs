#[doc = "Register `IN1` reader"]
pub struct R(crate::R<IN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN1` writer"]
pub struct W(crate::W<IN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN1_SPEC>;
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
impl From<crate::W<IN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NEXT` reader - GPIO32~39 input value"]
pub type DATA_NEXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_NEXT` writer - GPIO32~39 input value"]
pub type DATA_NEXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IN1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn data_next(&mut self) -> DATA_NEXT_W<0> {
        DATA_NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in1](index.html) module"]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in1::R](R) reader structure"]
impl crate::Readable for IN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in1::W](W) writer structure"]
impl crate::Writable for IN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
