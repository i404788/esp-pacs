#[doc = "Register `STORE5` reader"]
pub struct R(crate::R<STORE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE5` writer"]
pub struct W(crate::W<STORE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE5_SPEC>;
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
impl From<crate::W<STORE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_AON_STORE5` reader - need_des"]
pub type LP_AON_STORE5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LP_AON_STORE5` writer - need_des"]
pub type LP_AON_STORE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store5(&self) -> LP_AON_STORE5_R {
        LP_AON_STORE5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store5(&mut self) -> LP_AON_STORE5_W<0> {
        LP_AON_STORE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store5](index.html) module"]
pub struct STORE5_SPEC;
impl crate::RegisterSpec for STORE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store5::R](R) reader structure"]
impl crate::Readable for STORE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store5::W](W) writer structure"]
impl crate::Writable for STORE5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE5 to value 0"]
impl crate::Resettable for STORE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
