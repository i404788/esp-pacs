#[doc = "Register `SET_RESULT_FINISH` writer"]
pub struct W(crate::W<SET_RESULT_FINISH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_RESULT_FINISH_SPEC>;
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
impl From<crate::W<SET_RESULT_FINISH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_RESULT_FINISH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_RESULT_END` writer - Set this bit to end upstream and clear the calculation result."]
pub type SET_RESULT_END_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SET_RESULT_FINISH_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to end upstream and clear the calculation result."]
    #[inline(always)]
    pub fn set_result_end(&mut self) -> SET_RESULT_END_W<0> {
        SET_RESULT_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HMAC read result completion register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_result_finish](index.html) module"]
pub struct SET_RESULT_FINISH_SPEC;
impl crate::RegisterSpec for SET_RESULT_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_result_finish::W](W) writer structure"]
impl crate::Writable for SET_RESULT_FINISH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_RESULT_FINISH to value 0"]
impl crate::Resettable for SET_RESULT_FINISH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
