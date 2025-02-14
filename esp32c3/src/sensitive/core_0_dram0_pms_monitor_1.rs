#[doc = "Register `CORE_0_DRAM0_PMS_MONITOR_1` reader"]
pub struct R(crate::R<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_DRAM0_PMS_MONITOR_1` writer"]
pub struct W(crate::W<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>;
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
impl From<crate::W<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR` reader - core_0_dram0_pms_monitor_violate_clr"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR` writer - core_0_dram0_pms_monitor_violate_clr"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_DRAM0_PMS_MONITOR_1_SPEC, bool, O>;
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN` reader - core_0_dram0_pms_monitor_violate_en"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN` writer - core_0_dram0_pms_monitor_violate_en"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_DRAM0_PMS_MONITOR_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - core_0_dram0_pms_monitor_violate_clr"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_clr(&self) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - core_0_dram0_pms_monitor_violate_en"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_en(&self) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core_0_dram0_pms_monitor_violate_clr"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_clr(
        &mut self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_W<0> {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR_W::new(self)
    }
    #[doc = "Bit 1 - core_0_dram0_pms_monitor_violate_en"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_en(
        &mut self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_W<1> {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_pms_monitor_1](index.html) module"]
pub struct CORE_0_DRAM0_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_pms_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_PMS_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_dram0_pms_monitor_1::W](W) writer structure"]
impl crate::Writable for CORE_0_DRAM0_PMS_MONITOR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_PMS_MONITOR_1 to value 0x03"]
impl crate::Resettable for CORE_0_DRAM0_PMS_MONITOR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
