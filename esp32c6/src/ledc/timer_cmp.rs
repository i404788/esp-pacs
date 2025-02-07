#[doc = "Register `TIMER%s_CMP` reader"]
pub struct R(crate::R<TIMER_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER%s_CMP` writer"]
pub struct W(crate::W<TIMER_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CMP_SPEC>;
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
impl From<crate::W<TIMER_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_CMP` reader - This register stores ledc timer%s compare value."]
pub type TIMER_CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_CMP` writer - This register stores ledc timer%s compare value."]
pub type TIMER_CMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_CMP_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - This register stores ledc timer%s compare value."]
    #[inline(always)]
    pub fn timer_cmp(&self) -> TIMER_CMP_R {
        TIMER_CMP_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores ledc timer%s compare value."]
    #[inline(always)]
    #[must_use]
    pub fn timer_cmp(&mut self) -> TIMER_CMP_W<0> {
        TIMER_CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc timer%s compare value register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_cmp](index.html) module"]
pub struct TIMER_CMP_SPEC;
impl crate::RegisterSpec for TIMER_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_cmp::R](R) reader structure"]
impl crate::Readable for TIMER_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_cmp::W](W) writer structure"]
impl crate::Writable for TIMER_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER%s_CMP to value 0"]
impl crate::Resettable for TIMER_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
