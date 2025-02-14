#[doc = "Register `THRES_CTRL` reader"]
pub struct R(crate::R<THRES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CTRL` writer"]
pub struct W(crate::W<THRES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CTRL_SPEC>;
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
impl From<crate::W<THRES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Clock gate enable."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Clock gate enable."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRES_CTRL_SPEC, bool, O>;
#[doc = "Field `ADC2_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC2_THRES_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC2_THRES_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRES_CTRL_SPEC, bool, O>;
#[doc = "Field `ADC1_THRES_MODE` reader - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC1_THRES_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_THRES_MODE` writer - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
pub type ADC1_THRES_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRES_CTRL_SPEC, bool, O>;
#[doc = "Field `ADC2_THRES` reader - ADC2 threshold."]
pub type ADC2_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC2_THRES` writer - ADC2 threshold."]
pub type ADC2_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES_CTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `ADC1_THRES` reader - ADC1 threshold."]
pub type ADC1_THRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC1_THRES` writer - ADC1 threshold."]
pub type ADC1_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES_CTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `ADC2_THRES_EN` reader - Enable ADC2 threshold monitor."]
pub type ADC2_THRES_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_THRES_EN` writer - Enable ADC2 threshold monitor."]
pub type ADC2_THRES_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRES_CTRL_SPEC, bool, O>;
#[doc = "Field `ADC1_THRES_EN` reader - Enable ADC1 threshold monitor."]
pub type ADC1_THRES_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_THRES_EN` writer - Enable ADC1 threshold monitor."]
pub type ADC1_THRES_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRES_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&self) -> ADC2_THRES_MODE_R {
        ADC2_THRES_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&self) -> ADC1_THRES_MODE_R {
        ADC1_THRES_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&self) -> ADC2_THRES_R {
        ADC2_THRES_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&self) -> ADC1_THRES_R {
        ADC1_THRES_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&self) -> ADC2_THRES_EN_R {
        ADC2_THRES_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&self) -> ADC1_THRES_EN_R {
        ADC1_THRES_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock gate enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc2_thres_mode(&mut self) -> ADC2_THRES_MODE_W<2> {
        ADC2_THRES_MODE_W::new(self)
    }
    #[doc = "Bit 3 - 1: ADC_DATA > = threshold, generate interrupt. 0: ADC_DATA < threshold, generate interrupt."]
    #[inline(always)]
    pub fn adc1_thres_mode(&mut self) -> ADC1_THRES_MODE_W<3> {
        ADC1_THRES_MODE_W::new(self)
    }
    #[doc = "Bits 4:16 - ADC2 threshold."]
    #[inline(always)]
    pub fn adc2_thres(&mut self) -> ADC2_THRES_W<4> {
        ADC2_THRES_W::new(self)
    }
    #[doc = "Bits 17:29 - ADC1 threshold."]
    #[inline(always)]
    pub fn adc1_thres(&mut self) -> ADC1_THRES_W<17> {
        ADC1_THRES_W::new(self)
    }
    #[doc = "Bit 30 - Enable ADC2 threshold monitor."]
    #[inline(always)]
    pub fn adc2_thres_en(&mut self) -> ADC2_THRES_EN_W<30> {
        ADC2_THRES_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable ADC1 threshold monitor."]
    #[inline(always)]
    pub fn adc1_thres_en(&mut self) -> ADC1_THRES_EN_W<31> {
        ADC1_THRES_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure monitor threshold for DIG ADC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_ctrl](index.html) module"]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_ctrl::R](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_ctrl::W](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
