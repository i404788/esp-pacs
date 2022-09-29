#[doc = "Register `EXTMEM_REJECT_INT_ENA` reader"]
pub struct R(crate::R<EXTMEM_REJECT_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTMEM_REJECT_INT_ENA` writer"]
pub struct W(crate::W<EXTMEM_REJECT_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTMEM_REJECT_INT_ENA_SPEC>;
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
impl From<crate::W<EXTMEM_REJECT_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTMEM_REJECT_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTMEM_REJECT_INT_ENA` reader - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `EXTMEM_REJECT_INT_ENA` writer - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
pub type EXTMEM_REJECT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTMEM_REJECT_INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_ena(&self) -> EXTMEM_REJECT_INT_ENA_R {
        EXTMEM_REJECT_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the EXTMEM_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn extmem_reject_int_ena(&mut self) -> EXTMEM_REJECT_INT_ENA_W<0> {
        EXTMEM_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits of external RAM permission\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_int_ena](index.html) module"]
pub struct EXTMEM_REJECT_INT_ENA_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_int_ena::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extmem_reject_int_ena::W](W) writer structure"]
impl crate::Writable for EXTMEM_REJECT_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_INT_ENA to value 0"]
impl crate::Resettable for EXTMEM_REJECT_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}