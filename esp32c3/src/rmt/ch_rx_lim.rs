#[doc = "Register `CH%s_RX_LIM` reader"]
pub struct R(crate::R<CH_RX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_RX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_RX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_RX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_RX_LIM` writer"]
pub struct W(crate::W<CH_RX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_RX_LIM_SPEC>;
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
impl From<crate::W<CH_RX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_RX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_LIM` reader - reg_rmt_rx_lim_ch2."]
pub type RX_LIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_LIM` writer - reg_rmt_rx_lim_ch2."]
pub type RX_LIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_RX_LIM_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - reg_rmt_rx_lim_ch2."]
    #[inline(always)]
    pub fn rx_lim(&self) -> RX_LIM_R {
        RX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_rmt_rx_lim_ch2."]
    #[inline(always)]
    pub fn rx_lim(&mut self) -> RX_LIM_W<0> {
        RX_LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH2_RX_LIM_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_rx_lim](index.html) module"]
pub struct CH_RX_LIM_SPEC;
impl crate::RegisterSpec for CH_RX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_rx_lim::R](R) reader structure"]
impl crate::Readable for CH_RX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_rx_lim::W](W) writer structure"]
impl crate::Writable for CH_RX_LIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_RX_LIM to value 0x80"]
impl crate::Resettable for CH_RX_LIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
