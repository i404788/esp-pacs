#[doc = "Register `SWFC_CONF1` reader"]
pub struct R(crate::R<SWFC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF1` writer"]
pub struct W(crate::W<SWFC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF1_SPEC>;
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
impl From<crate::W<SWFC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XON_THRESHOLD` reader - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
pub type XON_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XON_THRESHOLD` writer - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
pub type XON_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWFC_CONF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
pub type XOFF_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
pub type XOFF_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWFC_CONF1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
    #[inline(always)]
    #[must_use]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<0> {
        XON_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<8> {
        XOFF_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software flow-control character configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf1](index.html) module"]
pub struct SWFC_CONF1_SPEC;
impl crate::RegisterSpec for SWFC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf1::R](R) reader structure"]
impl crate::Readable for SWFC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf1::W](W) writer structure"]
impl crate::Writable for SWFC_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWFC_CONF1 to value 0xe000"]
impl crate::Resettable for SWFC_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0xe000;
}