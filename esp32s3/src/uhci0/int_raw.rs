#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_RAW` reader - This is the interrupt raw bit. Triggered when a separator char has been sent."]
pub type RX_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_START_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
pub type TX_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RX_HUNG_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
pub type RX_HUNG_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_HUNG_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
pub type TX_HUNG_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
pub type SEND_S_REG_Q_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
pub type SEND_A_REG_Q_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL0_INT_RAW` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub type APP_CTRL0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL0_INT_RAW` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub type APP_CTRL0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, O>;
#[doc = "Field `APP_CTRL1_INT_RAW` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub type APP_CTRL1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL1_INT_RAW` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub type APP_CTRL1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the interrupt raw bit. Triggered when a separator char has been sent."]
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> RX_START_INT_RAW_R {
        RX_START_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> TX_START_INT_RAW_R {
        TX_START_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&self) -> SEND_S_REG_Q_INT_RAW_R {
        SEND_S_REG_Q_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&self) -> SEND_A_REG_Q_INT_RAW_R {
        SEND_A_REG_Q_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&self) -> APP_CTRL0_INT_RAW_R {
        APP_CTRL0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&self) -> APP_CTRL1_INT_RAW_R {
        APP_CTRL1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&mut self) -> APP_CTRL0_INT_RAW_W<7> {
        APP_CTRL0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&mut self) -> APP_CTRL1_INT_RAW_W<8> {
        APP_CTRL1_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
