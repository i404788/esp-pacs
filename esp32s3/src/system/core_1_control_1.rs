#[doc = "Register `CORE_1_CONTROL_1` reader"]
pub struct R(crate::R<CORE_1_CONTROL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_CONTROL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_CONTROL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_CONTROL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_CONTROL_1` writer"]
pub struct W(crate::W<CORE_1_CONTROL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_CONTROL_1_SPEC>;
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
impl From<crate::W<CORE_1_CONTROL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_CONTROL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROL_CORE_1_MESSAGE` reader - it's only a R/W register, no function, software can write any value"]
pub type CONTROL_CORE_1_MESSAGE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONTROL_CORE_1_MESSAGE` writer - it's only a R/W register, no function, software can write any value"]
pub type CONTROL_CORE_1_MESSAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_CONTROL_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - it's only a R/W register, no function, software can write any value"]
    #[inline(always)]
    pub fn control_core_1_message(&self) -> CONTROL_CORE_1_MESSAGE_R {
        CONTROL_CORE_1_MESSAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - it's only a R/W register, no function, software can write any value"]
    #[inline(always)]
    pub fn control_core_1_message(&mut self) -> CONTROL_CORE_1_MESSAGE_W<0> {
        CONTROL_CORE_1_MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 control regiter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_control_1](index.html) module"]
pub struct CORE_1_CONTROL_1_SPEC;
impl crate::RegisterSpec for CORE_1_CONTROL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_control_1::R](R) reader structure"]
impl crate::Readable for CORE_1_CONTROL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_control_1::W](W) writer structure"]
impl crate::Writable for CORE_1_CONTROL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_CONTROL_1 to value 0"]
impl crate::Resettable for CORE_1_CONTROL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
