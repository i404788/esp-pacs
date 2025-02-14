#[doc = "Register `RESET_STATE` reader"]
pub struct R(crate::R<RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATE` writer"]
pub struct W(crate::W<RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATE_SPEC>;
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
impl From<crate::W<RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_CAUSE_PROCPU` reader - Stores the CPU reset cause."]
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESET_CAUSE_APPCPU` reader - reset cause of APP CPU"]
pub type RESET_CAUSE_APPCPU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` reader - APP CPU state vector sel"]
pub type APPCPU_STAT_VECTOR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `APPCPU_STAT_VECTOR_SEL` writer - APP CPU state vector sel"]
pub type APPCPU_STAT_VECTOR_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESET_STATE_SPEC, bool, O>;
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` reader - Selects the CPU state vector."]
pub type PROCPU_STAT_VECTOR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PROCPU_STAT_VECTOR_SEL` writer - Selects the CPU state vector."]
pub type PROCPU_STAT_VECTOR_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESET_STATE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Stores the CPU reset cause."]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reset cause of APP CPU"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&self) -> APPCPU_STAT_VECTOR_SEL_R {
        APPCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects the CPU state vector."]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&self) -> PROCPU_STAT_VECTOR_SEL_R {
        PROCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - APP CPU state vector sel"]
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&mut self) -> APPCPU_STAT_VECTOR_SEL_W<12> {
        APPCPU_STAT_VECTOR_SEL_W::new(self)
    }
    #[doc = "Bit 13 - Selects the CPU state vector."]
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&mut self) -> PROCPU_STAT_VECTOR_SEL_W<13> {
        PROCPU_STAT_VECTOR_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates the CPU reset source. For more information about the reset cause, please refer to Table \\ref{table:resetreasons} in Chapter \\ref{module:ResetandClock} \\textit{\nameref{module:ResetandClock}}.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_state::W](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_STATE to value 0x3000"]
impl crate::Resettable for RESET_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000
    }
}
