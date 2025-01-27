#[doc = "Register `SLP_WAKEUP_CNTL6` reader"]
pub struct R(crate::R<SLP_WAKEUP_CNTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CNTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CNTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CNTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKEUP_CNTL6` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL6_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOC_WAKEUP_WAIT` reader - need_des"]
pub type SOC_WAKEUP_WAIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SOC_WAKEUP_WAIT` writer - need_des"]
pub type SOC_WAKEUP_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLP_WAKEUP_CNTL6_SPEC, u32, u32, 20, O>;
#[doc = "Field `SOC_WAKEUP_WAIT_CFG` reader - need_des"]
pub type SOC_WAKEUP_WAIT_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC_WAKEUP_WAIT_CFG` writer - need_des"]
pub type SOC_WAKEUP_WAIT_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLP_WAKEUP_CNTL6_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:19 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_wait(&self) -> SOC_WAKEUP_WAIT_R {
        SOC_WAKEUP_WAIT_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_wait_cfg(&self) -> SOC_WAKEUP_WAIT_CFG_R {
        SOC_WAKEUP_WAIT_CFG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_wait(&mut self) -> SOC_WAKEUP_WAIT_W<0> {
        SOC_WAKEUP_WAIT_W::new(self)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_wait_cfg(&mut self) -> SOC_WAKEUP_WAIT_CFG_W<30> {
        SOC_WAKEUP_WAIT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl6](index.html) module"]
pub struct SLP_WAKEUP_CNTL6_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cntl6::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl6::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL6 to value 0x80"]
impl crate::Resettable for SLP_WAKEUP_CNTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
