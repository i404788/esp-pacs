#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_CLR` writer - Set this bit to clear UART_THE RXFIFO_FULL_INT interrupt."]
pub type RXFIFO_FULL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TXFIFO_EMPTY_INT_CLR` writer - Set this bit to clear UART_TXFIFO_EMPTY_INT interrupt."]
pub type TXFIFO_EMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `PARITY_ERR_INT_CLR` writer - Set this bit to clear UART_PARITY_ERR_INT interrupt."]
pub type PARITY_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `FRM_ERR_INT_CLR` writer - Set this bit to clear UART_FRM_ERR_INT interrupt."]
pub type FRM_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - Set this bit to clear UART_UART_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `DSR_CHG_INT_CLR` writer - Set this bit to clear UART_DSR_CHG_INT interrupt."]
pub type DSR_CHG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CTS_CHG_INT_CLR` writer - Set this bit to clear UART_CTS_CHG_INT interrupt."]
pub type CTS_CHG_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `BRK_DET_INT_CLR` writer - Set this bit to clear UART_BRK_DET_INT interrupt."]
pub type BRK_DET_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RXFIFO_TOUT_INT_CLR` writer - Set this bit to clear UART_RXFIFO_TOUT_INT interrupt."]
pub type RXFIFO_TOUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `SW_XON_INT_CLR` writer - Set this bit to clear UART_SW_XON_INT interrupt."]
pub type SW_XON_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `SW_XOFF_INT_CLR` writer - Set this bit to clear UART_SW_XOFF_INT interrupt."]
pub type SW_XOFF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Set this bit to clear UART_GLITCH_DET_INT interrupt."]
pub type GLITCH_DET_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_BRK_DONE_INT_CLR` writer - Set this bit to clear UART_TX_BRK_DONE_INT interrupt."]
pub type TX_BRK_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_CLR` writer - Set this bit to clear UART_TX_BRK_IDLE_DONE_INT interrupt."]
pub type TX_BRK_IDLE_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_DONE_INT_CLR` writer - Set this bit to clear UART_TX_DONE_INT interrupt."]
pub type TX_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RS485_PARITY_ERR_INT_CLR` writer - Set this bit to clear UART_RS485_PARITY_ERR_INT interrupt."]
pub type RS485_PARITY_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RS485_FRM_ERR_INT_CLR` writer - Set this bit to clear UART_RS485_FRM_ERR_INT interrupt."]
pub type RS485_FRM_ERR_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RS485_CLASH_INT_CLR` writer - Set this bit to clear UART_RS485_CLASH_INT interrupt."]
pub type RS485_CLASH_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_CLR` writer - Set this bit to clear UART_AT_CMD_CHAR_DET_INT interrupt."]
pub type AT_CMD_CHAR_DET_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `WAKEUP_INT_CLR` writer - Set this bit to clear UART_WAKEUP_INT interrupt."]
pub type WAKEUP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear UART_THE RXFIFO_FULL_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W<0> {
        RXFIFO_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear UART_TXFIFO_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W<1> {
        TXFIFO_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear UART_PARITY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn parity_err_int_clr(&mut self) -> PARITY_ERR_INT_CLR_W<2> {
        PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear UART_FRM_ERR_INT interrupt."]
    #[inline(always)]
    pub fn frm_err_int_clr(&mut self) -> FRM_ERR_INT_CLR_W<3> {
        FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear UART_UART_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<4> {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear UART_DSR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dsr_chg_int_clr(&mut self) -> DSR_CHG_INT_CLR_W<5> {
        DSR_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear UART_CTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn cts_chg_int_clr(&mut self) -> CTS_CHG_INT_CLR_W<6> {
        CTS_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear UART_BRK_DET_INT interrupt."]
    #[inline(always)]
    pub fn brk_det_int_clr(&mut self) -> BRK_DET_INT_CLR_W<7> {
        BRK_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear UART_RXFIFO_TOUT_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_tout_int_clr(&mut self) -> RXFIFO_TOUT_INT_CLR_W<8> {
        RXFIFO_TOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear UART_SW_XON_INT interrupt."]
    #[inline(always)]
    pub fn sw_xon_int_clr(&mut self) -> SW_XON_INT_CLR_W<9> {
        SW_XON_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear UART_SW_XOFF_INT interrupt."]
    #[inline(always)]
    pub fn sw_xoff_int_clr(&mut self) -> SW_XOFF_INT_CLR_W<10> {
        SW_XOFF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear UART_GLITCH_DET_INT interrupt."]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W<11> {
        GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear UART_TX_BRK_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_brk_done_int_clr(&mut self) -> TX_BRK_DONE_INT_CLR_W<12> {
        TX_BRK_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear UART_TX_BRK_IDLE_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_clr(&mut self) -> TX_BRK_IDLE_DONE_INT_CLR_W<13> {
        TX_BRK_IDLE_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear UART_TX_DONE_INT interrupt."]
    #[inline(always)]
    pub fn tx_done_int_clr(&mut self) -> TX_DONE_INT_CLR_W<14> {
        TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear UART_RS485_PARITY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn rs485_parity_err_int_clr(&mut self) -> RS485_PARITY_ERR_INT_CLR_W<15> {
        RS485_PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear UART_RS485_FRM_ERR_INT interrupt."]
    #[inline(always)]
    pub fn rs485_frm_err_int_clr(&mut self) -> RS485_FRM_ERR_INT_CLR_W<16> {
        RS485_FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear UART_RS485_CLASH_INT interrupt."]
    #[inline(always)]
    pub fn rs485_clash_int_clr(&mut self) -> RS485_CLASH_INT_CLR_W<17> {
        RS485_CLASH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear UART_AT_CMD_CHAR_DET_INT interrupt."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_clr(&mut self) -> AT_CMD_CHAR_DET_INT_CLR_W<18> {
        AT_CMD_CHAR_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear UART_WAKEUP_INT interrupt."]
    #[inline(always)]
    pub fn wakeup_int_clr(&mut self) -> WAKEUP_INT_CLR_W<19> {
        WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
