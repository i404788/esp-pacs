#[doc = "Register `ULP_CP_TIMER_1` reader"]
pub struct R(crate::R<ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_TIMER_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_TIMER_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_TIMER_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_TIMER_1` writer"]
pub struct W(crate::W<ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_TIMER_1_SPEC>;
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
impl From<crate::W<ULP_CP_TIMER_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_TIMER_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` reader - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` writer - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ULP_CP_TIMER_1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&self) -> ULP_CP_TIMER_SLP_CYCLE_R {
        ULP_CP_TIMER_SLP_CYCLE_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W<8> {
        ULP_CP_TIMER_SLP_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_timer_1](index.html) module"]
pub struct ULP_CP_TIMER_1_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_timer_1::R](R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_timer_1::W](W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULP_CP_TIMER_1 to value 0xc800"]
impl crate::Resettable for ULP_CP_TIMER_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc800
    }
}
