#[doc = "Register `STATUS_W1TS` reader"]
pub struct R(crate::R<STATUS_W1TS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_W1TS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_W1TS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS_W1TS` writer"]
pub struct W(crate::W<STATUS_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_W1TS_SPEC>;
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
impl From<crate::W<STATUS_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS_INT_W1TS` reader - GPIO0~31 interrupt status write 1 to set"]
pub type STATUS_INT_W1TS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATUS_INT_W1TS` writer - GPIO0~31 interrupt status write 1 to set"]
pub type STATUS_INT_W1TS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATUS_W1TS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&self) -> STATUS_INT_W1TS_R {
        STATUS_INT_W1TS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&mut self) -> STATUS_INT_W1TS_W<0> {
        STATUS_INT_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_w1ts](index.html) module"]
pub struct STATUS_W1TS_SPEC;
impl crate::RegisterSpec for STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_w1ts::R](R) reader structure"]
impl crate::Readable for STATUS_W1TS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status_w1ts::W](W) writer structure"]
impl crate::Writable for STATUS_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for STATUS_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
