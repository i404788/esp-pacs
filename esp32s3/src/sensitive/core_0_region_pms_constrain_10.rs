#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_10` reader"]
pub struct R(crate::R<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_10` writer"]
pub struct W(crate::W<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>;
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
impl From<crate::W<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_REGION_PMS_CONSTRAIN_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_7` reader - Region 6 end address and Region 7 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_7` writer - Region 6 end address and Region 7 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_REGION_PMS_CONSTRAIN_10_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Region 6 end address and Region 7 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_7(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 6 end address and Region 7 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_7(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_W<0> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 region permission register 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_region_pms_constrain_10](index.html) module"]
pub struct CORE_0_REGION_PMS_CONSTRAIN_10_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_region_pms_constrain_10::R](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_region_pms_constrain_10::W](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_10 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
