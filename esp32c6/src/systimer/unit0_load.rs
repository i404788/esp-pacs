#[doc = "Register `UNIT0_LOAD` writer"]
pub struct W(crate::W<UNIT0_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIT0_LOAD_SPEC>;
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
impl From<crate::W<UNIT0_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIT0_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT0_LOAD` writer - timer unit0 sync enable signal"]
pub type TIMER_UNIT0_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, UNIT0_LOAD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - timer unit0 sync enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_load(&mut self) -> TIMER_UNIT0_LOAD_W<0> {
        TIMER_UNIT0_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer unit0 conf sync register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit0_load](index.html) module"]
pub struct UNIT0_LOAD_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [unit0_load::W](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNIT0_LOAD to value 0"]
impl crate::Resettable for UNIT0_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
