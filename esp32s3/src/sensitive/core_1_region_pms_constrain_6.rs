#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_6` reader"]
pub struct R(crate::R<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_6` writer"]
pub struct W(crate::W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>;
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
impl From<crate::W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_REGION_PMS_CONSTRAIN_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_3` reader - Region 2 end address and Region 3 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_3` writer - Region 2 end address and Region 3 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_1_REGION_PMS_CONSTRAIN_6_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Region 2 end address and Region 3 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_3(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 2 end address and Region 3 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_3(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W<0> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 region permission register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_region_pms_constrain_6](index.html) module"]
pub struct CORE_1_REGION_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_region_pms_constrain_6::R](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_region_pms_constrain_6::W](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_6 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
