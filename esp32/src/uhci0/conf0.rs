#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST` reader - Set this bit to reset in link operations."]
pub type IN_RST_R = crate::BitReader<bool>;
#[doc = "Field `IN_RST` writer - Set this bit to reset in link operations."]
pub type IN_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out link operations."]
pub type OUT_RST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RST` writer - Set this bit to reset out link operations."]
pub type OUT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset dma ahb fifo."]
pub type AHBM_FIFO_RST_R = crate::BitReader<bool>;
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset dma ahb fifo."]
pub type AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `AHBM_RST` reader - Set this bit to reset dma ahb interface."]
pub type AHBM_RST_R = crate::BitReader<bool>;
#[doc = "Field `AHBM_RST` writer - Set this bit to reset dma ahb interface."]
pub type AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `IN_LOOP_TEST` reader - Set this bit to enable loop test for in links."]
pub type IN_LOOP_TEST_R = crate::BitReader<bool>;
#[doc = "Field `IN_LOOP_TEST` writer - Set this bit to enable loop test for in links."]
pub type IN_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_LOOP_TEST` reader - Set this bit to enable loop test for out links."]
pub type OUT_LOOP_TEST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_LOOP_TEST` writer - Set this bit to enable loop test for out links."]
pub type OUT_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - when in link's length is 0 go on to use the next in link automatically."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader<bool>;
#[doc = "Field `OUT_AUTO_WRBACK` writer - when in link's length is 0 go on to use the next in link automatically."]
pub type OUT_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - don't use"]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader<bool>;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - don't use"]
pub type OUT_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_EOF_MODE` reader - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
pub type OUT_EOF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_MODE` writer - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
pub type OUT_EOF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `UART0_CE` reader - Set this bit to use UART to transmit or receive data."]
pub type UART0_CE_R = crate::BitReader<bool>;
#[doc = "Field `UART0_CE` writer - Set this bit to use UART to transmit or receive data."]
pub type UART0_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `UART1_CE` reader - Set this bit to use UART1 to transmit or receive data."]
pub type UART1_CE_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CE` writer - Set this bit to use UART1 to transmit or receive data."]
pub type UART1_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `UART2_CE` reader - Set this bit to use UART2 to transmit or receive data."]
pub type UART2_CE_R = crate::BitReader<bool>;
#[doc = "Field `UART2_CE` writer - Set this bit to use UART2 to transmit or receive data."]
pub type UART2_CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - Set this bit to enable DMA in links to use burst mode."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUTDSCR_BURST_EN` writer - Set this bit to enable DMA in links to use burst mode."]
pub type OUTDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to enable DMA out links to use burst mode."]
pub type INDSCR_BURST_EN_R = crate::BitReader<bool>;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to enable DMA out links to use burst mode."]
pub type INDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - Set this bit to enable DMA burst MODE"]
pub type OUT_DATA_BURST_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DATA_BURST_EN` writer - Set this bit to enable DMA burst MODE"]
pub type OUT_DATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `MEM_TRANS_EN` reader - "]
pub type MEM_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `MEM_TRANS_EN` writer - "]
pub type MEM_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `SEPER_EN` reader - Set this bit to use special char to separate the data frame."]
pub type SEPER_EN_R = crate::BitReader<bool>;
#[doc = "Field `SEPER_EN` writer - Set this bit to use special char to separate the data frame."]
pub type SEPER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `HEAD_EN` reader - Set this bit to enable to use head packet before the data frame."]
pub type HEAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `HEAD_EN` writer - Set this bit to enable to use head packet before the data frame."]
pub type HEAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `CRC_REC_EN` reader - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
pub type CRC_REC_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRC_REC_EN` writer - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
pub type CRC_REC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `UART_IDLE_EOF_EN` reader - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
pub type UART_IDLE_EOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_IDLE_EOF_EN` writer - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
pub type UART_IDLE_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `LEN_EOF_EN` reader - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
pub type LEN_EOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEN_EOF_EN` writer - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
pub type LEN_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `ENCODE_CRC_EN` reader - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
pub type ENCODE_CRC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ENCODE_CRC_EN` writer - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
pub type ENCODE_CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clock-gating for read or write registers."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clock-gating for read or write registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
#[doc = "Field `UART_RX_BRK_EOF_EN` reader - Set this bit to enable to use brk char as the end of a data frame."]
pub type UART_RX_BRK_EOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_RX_BRK_EOF_EN` writer - Set this bit to enable to use brk char as the end of a data frame."]
pub type UART_RX_BRK_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uart0_ce(&self) -> UART0_CE_R {
        UART0_CE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uart1_ce(&self) -> UART1_CE_R {
        UART1_CE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uart2_ce(&self) -> UART2_CE_R {
        UART2_CE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn seper_en(&self) -> SEPER_EN_R {
        SEPER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn head_en(&self) -> HEAD_EN_R {
        HEAD_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn crc_rec_en(&self) -> CRC_REC_EN_R {
        CRC_REC_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&self) -> UART_IDLE_EOF_EN_R {
        UART_IDLE_EOF_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn len_eof_en(&self) -> LEN_EOF_EN_R {
        LEN_EOF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn encode_crc_en(&self) -> ENCODE_CRC_EN_R {
        ENCODE_CRC_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&self) -> UART_RX_BRK_EOF_EN_R {
        UART_RX_BRK_EOF_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset in link operations."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<0> {
        IN_RST_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset out link operations."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W<1> {
        OUT_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset dma ahb fifo."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<2> {
        AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to reset dma ahb interface."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<3> {
        AHBM_RST_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to enable loop test for in links."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<4> {
        IN_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to enable loop test for out links."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<5> {
        OUT_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 6 - when in link's length is 0 go on to use the next in link automatically."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<6> {
        OUT_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 7 - don't use"]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<7> {
        OUT_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to produce eof after DMA pops all data clear this bit to produce eof after DMA pushes all data"]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<8> {
        OUT_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to use UART to transmit or receive data."]
    #[inline(always)]
    pub fn uart0_ce(&mut self) -> UART0_CE_W<9> {
        UART0_CE_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to use UART1 to transmit or receive data."]
    #[inline(always)]
    pub fn uart1_ce(&mut self) -> UART1_CE_W<10> {
        UART1_CE_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to use UART2 to transmit or receive data."]
    #[inline(always)]
    pub fn uart2_ce(&mut self) -> UART2_CE_W<11> {
        UART2_CE_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to enable DMA in links to use burst mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<12> {
        OUTDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to enable DMA out links to use burst mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<13> {
        INDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable DMA burst MODE"]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<14> {
        OUT_DATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<15> {
        MEM_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to use special char to separate the data frame."]
    #[inline(always)]
    pub fn seper_en(&mut self) -> SEPER_EN_W<16> {
        SEPER_EN_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to enable to use head packet before the data frame."]
    #[inline(always)]
    pub fn head_en(&mut self) -> HEAD_EN_W<17> {
        HEAD_EN_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to enable receiver''s ability of crc calculation when crc_en bit in head packet is 1 then there will be crc bytes after data_frame"]
    #[inline(always)]
    pub fn crc_rec_en(&mut self) -> CRC_REC_EN_W<18> {
        CRC_REC_EN_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to enable to use idle time when the idle time after data frame is satisfied this means the end of a data frame."]
    #[inline(always)]
    pub fn uart_idle_eof_en(&mut self) -> UART_IDLE_EOF_EN_W<19> {
        UART_IDLE_EOF_EN_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to enable to use packet_len in packet head when the received data is equal to packet_len this means the end of a data frame."]
    #[inline(always)]
    pub fn len_eof_en(&mut self) -> LEN_EOF_EN_W<20> {
        LEN_EOF_EN_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to enable crc calculation for data frame when bit6 in the head packet is 1."]
    #[inline(always)]
    pub fn encode_crc_en(&mut self) -> ENCODE_CRC_EN_W<21> {
        ENCODE_CRC_EN_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to enable clock-gating for read or write registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<22> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to enable to use brk char as the end of a data frame."]
    #[inline(always)]
    pub fn uart_rx_brk_eof_en(&mut self) -> UART_RX_BRK_EOF_EN_W<23> {
        UART_RX_BRK_EOF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0 to value 0x0037_0100"]
impl crate::Resettable for CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0037_0100
    }
}