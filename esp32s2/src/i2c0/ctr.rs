#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FORCE_OUT` reader - 0: direct output. 1: open drain output."]
pub type SDA_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SDA_FORCE_OUT` writer - 0: direct output. 1: open drain output."]
pub type SDA_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `SCL_FORCE_OUT` reader - 0: direct output. 1: open drain output."]
pub type SCL_FORCE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SCL_FORCE_OUT` writer - 0: direct output. 1: open drain output."]
pub type SCL_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `RX_FULL_ACK_LEVEL` reader - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL_ACK_LEVEL` writer - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
pub type MS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MS_MODE` writer - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
pub type MS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `TRANS_START` reader - Set this bit to start sending the data in TX FIFO."]
pub type TRANS_START_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_START` writer - Set this bit to start sending the data in TX FIFO."]
pub type TRANS_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit. 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit. 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit. 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit. 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `CLK_EN` reader - Reserved."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Reserved."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `ARBITRATION_EN` reader - This is the enable bit for I2C bus arbitration function."]
pub type ARBITRATION_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_EN` writer - This is the enable bit for I2C bus arbitration function."]
pub type ARBITRATION_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `FSM_RST` reader - This register is used to reset the SCL_FSM."]
pub type FSM_RST_R = crate::BitReader<bool>;
#[doc = "Field `FSM_RST` writer - This register is used to reset the SCL_FSM."]
pub type FSM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
#[doc = "Field `REF_ALWAYS_ON` reader - This register is used to control the REF_TICK."]
pub type REF_ALWAYS_ON_R = crate::BitReader<bool>;
#[doc = "Field `REF_ALWAYS_ON` writer - This register is used to control the REF_TICK."]
pub type REF_ALWAYS_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: direct output. 1: open drain output."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: direct output. 1: open drain output."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> RX_FULL_ACK_LEVEL_R {
        RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to start sending the data in TX FIFO."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit. 0: send data from the most significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit. 0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for I2C bus arbitration function."]
    #[inline(always)]
    pub fn arbitration_en(&self) -> ARBITRATION_EN_R {
        ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This register is used to reset the SCL_FSM."]
    #[inline(always)]
    pub fn fsm_rst(&self) -> FSM_RST_R {
        FSM_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This register is used to control the REF_TICK."]
    #[inline(always)]
    pub fn ref_always_on(&self) -> REF_ALWAYS_ON_R {
        REF_ALWAYS_ON_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: direct output. 1: open drain output."]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<0> {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - 0: direct output. 1: open drain output."]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<1> {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W<2> {
        SAMPLE_SCL_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn rx_full_ack_level(&mut self) -> RX_FULL_ACK_LEVEL_W<3> {
        RX_FULL_ACK_LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as an I2C Master. Clear this bit to configure the module as an I2C Slave."]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W<4> {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to start sending the data in TX FIFO."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W<5> {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit. 0: send data from the most significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<6> {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit. 0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<7> {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 8 - Reserved."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<8> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - This is the enable bit for I2C bus arbitration function."]
    #[inline(always)]
    pub fn arbitration_en(&mut self) -> ARBITRATION_EN_W<9> {
        ARBITRATION_EN_W::new(self)
    }
    #[doc = "Bit 10 - This register is used to reset the SCL_FSM."]
    #[inline(always)]
    pub fn fsm_rst(&mut self) -> FSM_RST_W<10> {
        FSM_RST_W::new(self)
    }
    #[doc = "Bit 11 - This register is used to control the REF_TICK."]
    #[inline(always)]
    pub fn ref_always_on(&mut self) -> REF_ALWAYS_ON_W<11> {
        REF_ALWAYS_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR to value 0x0a0b"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a0b
    }
}
