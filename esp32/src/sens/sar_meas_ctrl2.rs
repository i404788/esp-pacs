#[doc = "Register `SAR_MEAS_CTRL2` reader"]
pub struct R(crate::R<SAR_MEAS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_CTRL2` writer"]
pub struct W(crate::W<SAR_MEAS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_CTRL2_SPEC>;
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
impl From<crate::W<SAR_MEAS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_DAC_XPD_FSM` reader - "]
pub type SAR1_DAC_XPD_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR1_DAC_XPD_FSM` writer - "]
pub type SAR1_DAC_XPD_FSM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - "]
pub type SAR1_DAC_XPD_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - "]
pub type SAR1_DAC_XPD_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - "]
pub type XPD_SAR_AMP_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - "]
pub type XPD_SAR_AMP_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - "]
pub type AMP_RST_FB_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - "]
pub type AMP_RST_FB_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - "]
pub type AMP_SHORT_REF_GND_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - "]
pub type XPD_SAR_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - "]
pub type XPD_SAR_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - "]
pub type SAR_RSTB_FSM_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - "]
pub type SAR_RSTB_FSM_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, bool, O>;
#[doc = "Field `SAR2_RSTB_FORCE` reader - "]
pub type SAR2_RSTB_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_RSTB_FORCE` writer - "]
pub type SAR2_RSTB_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `AMP_RST_FB_FORCE` reader - "]
pub type AMP_RST_FB_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP_RST_FB_FORCE` writer - "]
pub type AMP_RST_FB_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `AMP_SHORT_REF_FORCE` reader - "]
pub type AMP_SHORT_REF_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP_SHORT_REF_FORCE` writer - "]
pub type AMP_SHORT_REF_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` reader - "]
pub type AMP_SHORT_REF_GND_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` writer - "]
pub type AMP_SHORT_REF_GND_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS_CTRL2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> SAR1_DAC_XPD_FSM_R {
        SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&mut self) -> SAR1_DAC_XPD_FSM_W<0> {
        SAR1_DAC_XPD_FSM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W<4> {
        SAR1_DAC_XPD_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W<5> {
        XPD_SAR_AMP_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W<6> {
        AMP_RST_FB_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W<7> {
        AMP_SHORT_REF_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AMP_SHORT_REF_GND_FSM_IDLE_W<8> {
        AMP_SHORT_REF_GND_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W<9> {
        XPD_SAR_FSM_IDLE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W<10> {
        SAR_RSTB_FSM_IDLE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W<11> {
        SAR2_RSTB_FORCE_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W<13> {
        AMP_RST_FB_FORCE_W::new(self)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W<15> {
        AMP_SHORT_REF_FORCE_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W<17> {
        AMP_SHORT_REF_GND_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_ctrl2](index.html) module"]
pub struct SAR_MEAS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL2 to value 0x03"]
impl crate::Resettable for SAR_MEAS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
