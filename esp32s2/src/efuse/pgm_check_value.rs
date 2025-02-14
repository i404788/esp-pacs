#[doc = "Register `PGM_CHECK_VALUE%s` reader"]
pub struct R(crate::R<PGM_CHECK_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_CHECK_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_CHECK_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_CHECK_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_CHECK_VALUE%s` writer"]
pub struct W(crate::W<PGM_CHECK_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_CHECK_VALUE_SPEC>;
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
impl From<crate::W<PGM_CHECK_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_CHECK_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGM_RS_DATA_0` reader - The content of the %sth 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PGM_RS_DATA_0` writer - The content of the %sth 32-bit RS code to be programmed."]
pub type PGM_RS_DATA_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PGM_CHECK_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_0(&self) -> PGM_RS_DATA_0_R {
        PGM_RS_DATA_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of the %sth 32-bit RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data_0(&mut self) -> PGM_RS_DATA_0_W<0> {
        PGM_RS_DATA_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register %s that stores the RS code to be programmed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_check_value](index.html) module"]
pub struct PGM_CHECK_VALUE_SPEC;
impl crate::RegisterSpec for PGM_CHECK_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_check_value::R](R) reader structure"]
impl crate::Readable for PGM_CHECK_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_check_value::W](W) writer structure"]
impl crate::Writable for PGM_CHECK_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE%s to value 0"]
impl crate::Resettable for PGM_CHECK_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
