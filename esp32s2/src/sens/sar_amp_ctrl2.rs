#[doc = "Register `SAR_AMP_CTRL2` reader"]
pub struct R(crate::R<SAR_AMP_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_AMP_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_AMP_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_AMP_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_AMP_CTRL2` writer"]
pub struct W(crate::W<SAR_AMP_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_AMP_CTRL2_SPEC>;
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
impl From<crate::W<SAR_AMP_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_AMP_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - "]
pub type SAR1_DAC_XPD_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - "]
pub type SAR1_DAC_XPD_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - "]
pub type XPD_SAR_AMP_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - "]
pub type XPD_SAR_AMP_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - "]
pub type AMP_RST_FB_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - "]
pub type AMP_RST_FB_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - "]
pub type XPD_SAR_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - "]
pub type XPD_SAR_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - "]
pub type SAR_RSTB_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - "]
pub type SAR_RSTB_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_AMP_CTRL2_SPEC, bool, O>;
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub type SAR_AMP_WAIT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W<0> {
        SAR1_DAC_XPD_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W<1> {
        XPD_SAR_AMP_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W<2> {
        AMP_RST_FB_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W<3> {
        AMP_SHORT_REF_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AMP_SHORT_REF_GND_FSM_IDLE_W<4> {
        AMP_SHORT_REF_GND_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W<5> {
        XPD_SAR_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<6> {
        SAR_RSTB_FSM_IDLE_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<16> {
        SAR_AMP_WAIT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AMP control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_amp_ctrl2](index.html) module"]
pub struct SAR_AMP_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_amp_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_amp_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL2 to value 0x000a_0000"]
impl crate::Resettable for SAR_AMP_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0000
    }
}
