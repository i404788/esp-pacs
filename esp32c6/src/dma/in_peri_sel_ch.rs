#[doc = "Register `IN_PERI_SEL_CH%s` reader"]
pub struct R(crate::R<IN_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_PERI_SEL_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_PERI_SEL_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_PERI_SEL_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_PERI_SEL_CH%s` writer"]
pub struct W(crate::W<IN_PERI_SEL_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_PERI_SEL_CH_SPEC>;
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
impl From<crate::W<IN_PERI_SEL_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_PERI_SEL_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_IN_SEL_CH` reader - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: Dummy. 2: UHCI0. 3: I2S0. 4: Dummy. 5: Dummy. 6: AES. 7: SHA. 8: ADC_DAC. 9: Parallel_IO. 10~15: Dummy"]
pub type PERI_IN_SEL_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERI_IN_SEL_CH` writer - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: Dummy. 2: UHCI0. 3: I2S0. 4: Dummy. 5: Dummy. 6: AES. 7: SHA. 8: ADC_DAC. 9: Parallel_IO. 10~15: Dummy"]
pub type PERI_IN_SEL_CH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IN_PERI_SEL_CH_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: Dummy. 2: UHCI0. 3: I2S0. 4: Dummy. 5: Dummy. 6: AES. 7: SHA. 8: ADC_DAC. 9: Parallel_IO. 10~15: Dummy"]
    #[inline(always)]
    pub fn peri_in_sel_ch(&self) -> PERI_IN_SEL_CH_R {
        PERI_IN_SEL_CH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: Dummy. 2: UHCI0. 3: I2S0. 4: Dummy. 5: Dummy. 6: AES. 7: SHA. 8: ADC_DAC. 9: Parallel_IO. 10~15: Dummy"]
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel_ch(&mut self) -> PERI_IN_SEL_CH_W<0> {
        PERI_IN_SEL_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral selection of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_peri_sel_ch](index.html) module"]
pub struct IN_PERI_SEL_CH_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_peri_sel_ch::R](R) reader structure"]
impl crate::Readable for IN_PERI_SEL_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_peri_sel_ch::W](W) writer structure"]
impl crate::Writable for IN_PERI_SEL_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_PERI_SEL_CH%s to value 0x3f"]
impl crate::Resettable for IN_PERI_SEL_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
