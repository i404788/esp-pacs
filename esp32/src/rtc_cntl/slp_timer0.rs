#[doc = "Register `SLP_TIMER0` reader"]
pub struct R(crate::R<SLP_TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER0` writer"]
pub struct W(crate::W<SLP_TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER0_SPEC>;
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
impl From<crate::W<SLP_TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_LO` reader - RTC sleep timer low 32 bits"]
pub type SLP_VAL_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLP_VAL_LO` writer - RTC sleep timer low 32 bits"]
pub type SLP_VAL_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLP_TIMER0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - RTC sleep timer low 32 bits"]
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC sleep timer low 32 bits"]
    #[inline(always)]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W<0> {
        SLP_VAL_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer0](index.html) module"]
pub struct SLP_TIMER0_SPEC;
impl crate::RegisterSpec for SLP_TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer0::R](R) reader structure"]
impl crate::Readable for SLP_TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer0::W](W) writer structure"]
impl crate::Writable for SLP_TIMER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_TIMER0 to value 0"]
impl crate::Resettable for SLP_TIMER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
