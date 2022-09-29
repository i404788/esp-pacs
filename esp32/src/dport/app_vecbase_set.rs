#[doc = "Register `APP_VECBASE_SET` reader"]
pub struct R(crate::R<APP_VECBASE_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_VECBASE_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_VECBASE_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_VECBASE_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_VECBASE_SET` writer"]
pub struct W(crate::W<APP_VECBASE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_VECBASE_SET_SPEC>;
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
impl From<crate::W<APP_VECBASE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_VECBASE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_OUT_VECBASE` reader - "]
pub type APP_OUT_VECBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `APP_OUT_VECBASE` writer - "]
pub type APP_OUT_VECBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APP_VECBASE_SET_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn app_out_vecbase(&self) -> APP_OUT_VECBASE_R {
        APP_OUT_VECBASE_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn app_out_vecbase(&mut self) -> APP_OUT_VECBASE_W<0> {
        APP_OUT_VECBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_vecbase_set](index.html) module"]
pub struct APP_VECBASE_SET_SPEC;
impl crate::RegisterSpec for APP_VECBASE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_vecbase_set::R](R) reader structure"]
impl crate::Readable for APP_VECBASE_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_vecbase_set::W](W) writer structure"]
impl crate::Writable for APP_VECBASE_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_VECBASE_SET to value 0"]
impl crate::Resettable for APP_VECBASE_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}