#[doc = "Register `TIMER_COUNT` reader"]
pub struct R(crate::R<TIMER_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_COUNT` writer"]
pub struct W(crate::W<TIMER_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_COUNT_SPEC>;
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
impl From<crate::W<TIMER_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_COUNT` reader - "]
pub type TIMER_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_COUNT` writer - "]
pub type TIMER_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_COUNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_count(&self) -> TIMER_COUNT_R {
        TIMER_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_count(&mut self) -> TIMER_COUNT_W<0> {
        TIMER_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_count](index.html) module"]
pub struct TIMER_COUNT_SPEC;
impl crate::RegisterSpec for TIMER_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_count::R](R) reader structure"]
impl crate::Readable for TIMER_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_count::W](W) writer structure"]
impl crate::Writable for TIMER_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_COUNT to value 0"]
impl crate::Resettable for TIMER_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
