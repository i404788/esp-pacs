#[doc = "Register `UART_CONF0` reader"]
pub struct R(crate::R<UART_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CONF0` writer"]
pub struct W(crate::W<UART_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CONF0_SPEC>;
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
impl From<crate::W<UART_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `parity` reader - Set parity check: 0:even 1:odd, UART CONFIG1"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `parity` writer - Set parity check: 0:even 1:odd, UART CONFIG1"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `parity_en` reader - Set this bit to enable uart parity check"]
pub type PARITY_EN_R = crate::BitReader<bool>;
#[doc = "Field `parity_en` writer - Set this bit to enable uart parity check"]
pub type PARITY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `bit_num` reader - Set bit num: 0:5bits 1:6bits 2:7bits 3:8bits"]
pub type BIT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bit_num` writer - Set bit num: 0:5bits 1:6bits 2:7bits 3:8bits"]
pub type BIT_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `stop_bit_num` reader - Set stop bit: 1:1bit 2:1.5bits 3:2bits"]
pub type STOP_BIT_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `stop_bit_num` writer - Set stop bit: 1:1bit 2:1.5bits 3:2bits"]
pub type STOP_BIT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sw_rts` reader - sw rts"]
pub type SW_RTS_R = crate::BitReader<bool>;
#[doc = "Field `sw_rts` writer - sw rts"]
pub type SW_RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `sw_dtr` reader - sw dtr"]
pub type SW_DTR_R = crate::BitReader<bool>;
#[doc = "Field `sw_dtr` writer - sw dtr"]
pub type SW_DTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `txd_brk` reader - RESERVED, DO NOT CHANGE THIS BIT"]
pub type TXD_BRK_R = crate::BitReader<bool>;
#[doc = "Field `txd_brk` writer - RESERVED, DO NOT CHANGE THIS BIT"]
pub type TXD_BRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_loopback` reader - Set this bit to enable uart loopback test mode"]
pub type UART_LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `uart_loopback` writer - Set this bit to enable uart loopback test mode"]
pub type UART_LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `tx_flow_en` reader - Set this bit to enable uart tx hardware flow control"]
pub type TX_FLOW_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_flow_en` writer - Set this bit to enable uart tx hardware flow control"]
pub type TX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `rxfifo_rst` reader - Set this bit to reset uart rx fifo"]
pub type RXFIFO_RST_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_rst` writer - Set this bit to reset uart rx fifo"]
pub type RXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `txfifo_rst` reader - Set this bit to reset uart tx fifo"]
pub type TXFIFO_RST_R = crate::BitReader<bool>;
#[doc = "Field `txfifo_rst` writer - Set this bit to reset uart tx fifo"]
pub type TXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_rxd_inv` reader - Set this bit to inverse uart rxd level"]
pub type UART_RXD_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_rxd_inv` writer - Set this bit to inverse uart rxd level"]
pub type UART_RXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_cts_inv` reader - Set this bit to inverse uart cts level"]
pub type UART_CTS_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_cts_inv` writer - Set this bit to inverse uart cts level"]
pub type UART_CTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_dsr_inv` reader - Set this bit to inverse uart dsr level"]
pub type UART_DSR_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_dsr_inv` writer - Set this bit to inverse uart dsr level"]
pub type UART_DSR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_txd_inv` reader - Set this bit to inverse uart txd level"]
pub type UART_TXD_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_txd_inv` writer - Set this bit to inverse uart txd level"]
pub type UART_TXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_rts_inv` reader - Set this bit to inverse uart rts level"]
pub type UART_RTS_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_rts_inv` writer - Set this bit to inverse uart rts level"]
pub type UART_RTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
#[doc = "Field `uart_dtr_inv` reader - Set this bit to inverse uart dtr level"]
pub type UART_DTR_INV_R = crate::BitReader<bool>;
#[doc = "Field `uart_dtr_inv` writer - Set this bit to inverse uart dtr level"]
pub type UART_DTR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set parity check: 0:even 1:odd, UART CONFIG1"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check"]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Set bit num: 0:5bits 1:6bits 2:7bits 3:8bits"]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set stop bit: 1:1bit 2:1.5bits 3:2bits"]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - sw rts"]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sw dtr"]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RESERVED, DO NOT CHANGE THIS BIT"]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable uart loopback test mode"]
    #[inline(always)]
    pub fn uart_loopback(&self) -> UART_LOOPBACK_R {
        UART_LOOPBACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable uart tx hardware flow control"]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to reset uart rx fifo"]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to reset uart tx fifo"]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to inverse uart rxd level"]
    #[inline(always)]
    pub fn uart_rxd_inv(&self) -> UART_RXD_INV_R {
        UART_RXD_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to inverse uart cts level"]
    #[inline(always)]
    pub fn uart_cts_inv(&self) -> UART_CTS_INV_R {
        UART_CTS_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to inverse uart dsr level"]
    #[inline(always)]
    pub fn uart_dsr_inv(&self) -> UART_DSR_INV_R {
        UART_DSR_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to inverse uart txd level"]
    #[inline(always)]
    pub fn uart_txd_inv(&self) -> UART_TXD_INV_R {
        UART_TXD_INV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to inverse uart rts level"]
    #[inline(always)]
    pub fn uart_rts_inv(&self) -> UART_RTS_INV_R {
        UART_RTS_INV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to inverse uart dtr level"]
    #[inline(always)]
    pub fn uart_dtr_inv(&self) -> UART_DTR_INV_R {
        UART_DTR_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set parity check: 0:even 1:odd, UART CONFIG1"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<0> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W<1> {
        PARITY_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Set bit num: 0:5bits 1:6bits 2:7bits 3:8bits"]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W<2> {
        BIT_NUM_W::new(self)
    }
    #[doc = "Bits 4:5 - Set stop bit: 1:1bit 2:1.5bits 3:2bits"]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<4> {
        STOP_BIT_NUM_W::new(self)
    }
    #[doc = "Bit 6 - sw rts"]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W<6> {
        SW_RTS_W::new(self)
    }
    #[doc = "Bit 7 - sw dtr"]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SW_DTR_W<7> {
        SW_DTR_W::new(self)
    }
    #[doc = "Bit 8 - RESERVED, DO NOT CHANGE THIS BIT"]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<8> {
        TXD_BRK_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable uart loopback test mode"]
    #[inline(always)]
    pub fn uart_loopback(&mut self) -> UART_LOOPBACK_W<14> {
        UART_LOOPBACK_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to enable uart tx hardware flow control"]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<15> {
        TX_FLOW_EN_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to reset uart rx fifo"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<17> {
        RXFIFO_RST_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to reset uart tx fifo"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<18> {
        TXFIFO_RST_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to inverse uart rxd level"]
    #[inline(always)]
    pub fn uart_rxd_inv(&mut self) -> UART_RXD_INV_W<19> {
        UART_RXD_INV_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to inverse uart cts level"]
    #[inline(always)]
    pub fn uart_cts_inv(&mut self) -> UART_CTS_INV_W<20> {
        UART_CTS_INV_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to inverse uart dsr level"]
    #[inline(always)]
    pub fn uart_dsr_inv(&mut self) -> UART_DSR_INV_W<21> {
        UART_DSR_INV_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to inverse uart txd level"]
    #[inline(always)]
    pub fn uart_txd_inv(&mut self) -> UART_TXD_INV_W<22> {
        UART_TXD_INV_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to inverse uart rts level"]
    #[inline(always)]
    pub fn uart_rts_inv(&mut self) -> UART_RTS_INV_W<23> {
        UART_RTS_INV_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to inverse uart dtr level"]
    #[inline(always)]
    pub fn uart_dtr_inv(&mut self) -> UART_DTR_INV_W<24> {
        UART_DTR_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART CONFIG0(UART0 and UART1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_conf0](index.html) module"]
pub struct UART_CONF0_SPEC;
impl crate::RegisterSpec for UART_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_conf0::R](R) reader structure"]
impl crate::Readable for UART_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_conf0::W](W) writer structure"]
impl crate::Writable for UART_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CONF0 to value 0"]
impl crate::Resettable for UART_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
