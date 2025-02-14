#[doc = "Register `CLK_EDGE_SEL` reader"]
pub struct R(crate::R<CLK_EDGE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_EDGE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_EDGE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_EDGE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_EDGE_SEL` writer"]
pub struct W(crate::W<CLK_EDGE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_EDGE_SEL_SPEC>;
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
impl From<crate::W<CLK_EDGE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_EDGE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` reader - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_DRV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` writer - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_DRV_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` reader - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SAM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` writer - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SAM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` reader - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SLF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` writer - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SLF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCLLKIN_EDGE_H` reader - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLLKIN_EDGE_H` writer - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCLLKIN_EDGE_L` reader - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub type CCLLKIN_EDGE_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLLKIN_EDGE_L` writer - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub type CCLLKIN_EDGE_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCLLKIN_EDGE_N` reader - The value should be equal to CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLLKIN_EDGE_N` writer - The value should be equal to CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_EDGE_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ESDIO_MODE` reader - Enable esdio mode."]
pub type ESDIO_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ESDIO_MODE` writer - Enable esdio mode."]
pub type ESDIO_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_EDGE_SEL_SPEC, bool, O>;
#[doc = "Field `ESD_MODE` reader - Enable esd mode."]
pub type ESD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ESD_MODE` writer - Enable esd mode."]
pub type ESD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_EDGE_SEL_SPEC, bool, O>;
#[doc = "Field `CCLK_EN` reader - Sdio clock enable"]
pub type CCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCLK_EN` writer - Sdio clock enable"]
pub type CCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_EDGE_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&self) -> CCLKIN_EDGE_DRV_SEL_R {
        CCLKIN_EDGE_DRV_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&self) -> CCLKIN_EDGE_SAM_SEL_R {
        CCLKIN_EDGE_SAM_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&self) -> CCLKIN_EDGE_SLF_SEL_R {
        CCLKIN_EDGE_SLF_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_h(&self) -> CCLLKIN_EDGE_H_R {
        CCLLKIN_EDGE_H_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    pub fn ccllkin_edge_l(&self) -> CCLLKIN_EDGE_L_R {
        CCLLKIN_EDGE_L_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - The value should be equal to CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_n(&self) -> CCLLKIN_EDGE_N_R {
        CCLLKIN_EDGE_N_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    pub fn esdio_mode(&self) -> ESDIO_MODE_R {
        ESDIO_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    pub fn esd_mode(&self) -> ESD_MODE_R {
        ESD_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sdio clock enable"]
    #[inline(always)]
    pub fn cclk_en(&self) -> CCLK_EN_R {
        CCLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&mut self) -> CCLKIN_EDGE_DRV_SEL_W<0> {
        CCLKIN_EDGE_DRV_SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&mut self) -> CCLKIN_EDGE_SAM_SEL_W<3> {
        CCLKIN_EDGE_SAM_SEL_W::new(self)
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&mut self) -> CCLKIN_EDGE_SLF_SEL_W<6> {
        CCLKIN_EDGE_SLF_SEL_W::new(self)
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_h(&mut self) -> CCLLKIN_EDGE_H_W<9> {
        CCLLKIN_EDGE_H_W::new(self)
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    pub fn ccllkin_edge_l(&mut self) -> CCLLKIN_EDGE_L_W<13> {
        CCLLKIN_EDGE_L_W::new(self)
    }
    #[doc = "Bits 17:20 - The value should be equal to CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_n(&mut self) -> CCLLKIN_EDGE_N_W<17> {
        CCLLKIN_EDGE_N_W::new(self)
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    pub fn esdio_mode(&mut self) -> ESDIO_MODE_W<21> {
        ESDIO_MODE_W::new(self)
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    pub fn esd_mode(&mut self) -> ESD_MODE_W<22> {
        ESD_MODE_W::new(self)
    }
    #[doc = "Bit 23 - Sdio clock enable"]
    #[inline(always)]
    pub fn cclk_en(&mut self) -> CCLK_EN_W<23> {
        CCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_edge_sel](index.html) module"]
pub struct CLK_EDGE_SEL_SPEC;
impl crate::RegisterSpec for CLK_EDGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_edge_sel::R](R) reader structure"]
impl crate::Readable for CLK_EDGE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_edge_sel::W](W) writer structure"]
impl crate::Writable for CLK_EDGE_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_EDGE_SEL to value 0x0082_0200"]
impl crate::Resettable for CLK_EDGE_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0082_0200
    }
}
