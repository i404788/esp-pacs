#[doc = "Register `_1INT_ENA1` reader"]
pub struct R(crate::R<_1INT_ENA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1INT_ENA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1INT_ENA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1INT_ENA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1INT_ENA1` writer"]
pub struct W(crate::W<_1INT_ENA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1INT_ENA1_SPEC>;
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
impl From<crate::W<_1INT_ENA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1INT_ENA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRHOST_BIT8_INT_ENA1` reader - "]
pub type FRHOST_BIT8_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT8_INT_ENA1` writer - "]
pub type FRHOST_BIT8_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT9_INT_ENA1` reader - "]
pub type FRHOST_BIT9_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT9_INT_ENA1` writer - "]
pub type FRHOST_BIT9_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT10_INT_ENA1` reader - "]
pub type FRHOST_BIT10_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT10_INT_ENA1` writer - "]
pub type FRHOST_BIT10_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT11_INT_ENA1` reader - "]
pub type FRHOST_BIT11_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT11_INT_ENA1` writer - "]
pub type FRHOST_BIT11_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT12_INT_ENA1` reader - "]
pub type FRHOST_BIT12_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT12_INT_ENA1` writer - "]
pub type FRHOST_BIT12_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT13_INT_ENA1` reader - "]
pub type FRHOST_BIT13_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT13_INT_ENA1` writer - "]
pub type FRHOST_BIT13_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT14_INT_ENA1` reader - "]
pub type FRHOST_BIT14_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT14_INT_ENA1` writer - "]
pub type FRHOST_BIT14_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `FRHOST_BIT15_INT_ENA1` reader - "]
pub type FRHOST_BIT15_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT15_INT_ENA1` writer - "]
pub type FRHOST_BIT15_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_RX_START_INT_ENA1` reader - "]
pub type SLC1_RX_START_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_START_INT_ENA1` writer - "]
pub type SLC1_RX_START_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_START_INT_ENA1` reader - "]
pub type SLC1_TX_START_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_START_INT_ENA1` writer - "]
pub type SLC1_TX_START_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_RX_UDF_INT_ENA1` reader - "]
pub type SLC1_RX_UDF_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_UDF_INT_ENA1` writer - "]
pub type SLC1_RX_UDF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_OVF_INT_ENA1` reader - "]
pub type SLC1_TX_OVF_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_OVF_INT_ENA1` writer - "]
pub type SLC1_TX_OVF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ENA1` reader - "]
pub type SLC1_TOKEN0_1TO0_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ENA1` writer - "]
pub type SLC1_TOKEN0_1TO0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ENA1` reader - "]
pub type SLC1_TOKEN1_1TO0_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ENA1` writer - "]
pub type SLC1_TOKEN1_1TO0_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_DONE_INT_ENA1` reader - "]
pub type SLC1_TX_DONE_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_DONE_INT_ENA1` writer - "]
pub type SLC1_TX_DONE_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_SUC_EOF_INT_ENA1` reader - "]
pub type SLC1_TX_SUC_EOF_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_SUC_EOF_INT_ENA1` writer - "]
pub type SLC1_TX_SUC_EOF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_RX_DONE_INT_ENA1` reader - "]
pub type SLC1_RX_DONE_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_DONE_INT_ENA1` writer - "]
pub type SLC1_RX_DONE_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_RX_EOF_INT_ENA1` reader - "]
pub type SLC1_RX_EOF_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_EOF_INT_ENA1` writer - "]
pub type SLC1_RX_EOF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TOHOST_INT_ENA1` reader - "]
pub type SLC1_TOHOST_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TOHOST_INT_ENA1` writer - "]
pub type SLC1_TOHOST_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_DSCR_ERR_INT_ENA1` reader - "]
pub type SLC1_TX_DSCR_ERR_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_DSCR_ERR_INT_ENA1` writer - "]
pub type SLC1_TX_DSCR_ERR_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_RX_DSCR_ERR_INT_ENA1` reader - "]
pub type SLC1_RX_DSCR_ERR_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_RX_DSCR_ERR_INT_ENA1` writer - "]
pub type SLC1_RX_DSCR_ERR_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_DSCR_EMPTY_INT_ENA1` reader - "]
pub type SLC1_TX_DSCR_EMPTY_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_DSCR_EMPTY_INT_ENA1` writer - "]
pub type SLC1_TX_DSCR_EMPTY_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_HOST_RD_ACK_INT_ENA1` reader - "]
pub type SLC1_HOST_RD_ACK_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_HOST_RD_ACK_INT_ENA1` writer - "]
pub type SLC1_HOST_RD_ACK_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_WR_RETRY_DONE_INT_ENA1` reader - "]
pub type SLC1_WR_RETRY_DONE_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_WR_RETRY_DONE_INT_ENA1` writer - "]
pub type SLC1_WR_RETRY_DONE_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_ERR_EOF_INT_ENA1` reader - "]
pub type SLC1_TX_ERR_EOF_INT_ENA1_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_ERR_EOF_INT_ENA1` writer - "]
pub type SLC1_TX_ERR_EOF_INT_ENA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, _1INT_ENA1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit8_int_ena1(&self) -> FRHOST_BIT8_INT_ENA1_R {
        FRHOST_BIT8_INT_ENA1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit9_int_ena1(&self) -> FRHOST_BIT9_INT_ENA1_R {
        FRHOST_BIT9_INT_ENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit10_int_ena1(&self) -> FRHOST_BIT10_INT_ENA1_R {
        FRHOST_BIT10_INT_ENA1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit11_int_ena1(&self) -> FRHOST_BIT11_INT_ENA1_R {
        FRHOST_BIT11_INT_ENA1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit12_int_ena1(&self) -> FRHOST_BIT12_INT_ENA1_R {
        FRHOST_BIT12_INT_ENA1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit13_int_ena1(&self) -> FRHOST_BIT13_INT_ENA1_R {
        FRHOST_BIT13_INT_ENA1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit14_int_ena1(&self) -> FRHOST_BIT14_INT_ENA1_R {
        FRHOST_BIT14_INT_ENA1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit15_int_ena1(&self) -> FRHOST_BIT15_INT_ENA1_R {
        FRHOST_BIT15_INT_ENA1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc1_rx_start_int_ena1(&self) -> SLC1_RX_START_INT_ENA1_R {
        SLC1_RX_START_INT_ENA1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc1_tx_start_int_ena1(&self) -> SLC1_TX_START_INT_ENA1_R {
        SLC1_TX_START_INT_ENA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_ena1(&self) -> SLC1_RX_UDF_INT_ENA1_R {
        SLC1_RX_UDF_INT_ENA1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_ena1(&self) -> SLC1_TX_OVF_INT_ENA1_R {
        SLC1_TX_OVF_INT_ENA1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_ena1(&self) -> SLC1_TOKEN0_1TO0_INT_ENA1_R {
        SLC1_TOKEN0_1TO0_INT_ENA1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_ena1(&self) -> SLC1_TOKEN1_1TO0_INT_ENA1_R {
        SLC1_TOKEN1_1TO0_INT_ENA1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_done_int_ena1(&self) -> SLC1_TX_DONE_INT_ENA1_R {
        SLC1_TX_DONE_INT_ENA1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc1_tx_suc_eof_int_ena1(&self) -> SLC1_TX_SUC_EOF_INT_ENA1_R {
        SLC1_TX_SUC_EOF_INT_ENA1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_done_int_ena1(&self) -> SLC1_RX_DONE_INT_ENA1_R {
        SLC1_RX_DONE_INT_ENA1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_eof_int_ena1(&self) -> SLC1_RX_EOF_INT_ENA1_R {
        SLC1_RX_EOF_INT_ENA1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_tohost_int_ena1(&self) -> SLC1_TOHOST_INT_ENA1_R {
        SLC1_TOHOST_INT_ENA1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_tx_dscr_err_int_ena1(&self) -> SLC1_TX_DSCR_ERR_INT_ENA1_R {
        SLC1_TX_DSCR_ERR_INT_ENA1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_dscr_err_int_ena1(&self) -> SLC1_RX_DSCR_ERR_INT_ENA1_R {
        SLC1_RX_DSCR_ERR_INT_ENA1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_tx_dscr_empty_int_ena1(&self) -> SLC1_TX_DSCR_EMPTY_INT_ENA1_R {
        SLC1_TX_DSCR_EMPTY_INT_ENA1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_host_rd_ack_int_ena1(&self) -> SLC1_HOST_RD_ACK_INT_ENA1_R {
        SLC1_HOST_RD_ACK_INT_ENA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_wr_retry_done_int_ena1(&self) -> SLC1_WR_RETRY_DONE_INT_ENA1_R {
        SLC1_WR_RETRY_DONE_INT_ENA1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_tx_err_eof_int_ena1(&self) -> SLC1_TX_ERR_EOF_INT_ENA1_R {
        SLC1_TX_ERR_EOF_INT_ENA1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit8_int_ena1(&mut self) -> FRHOST_BIT8_INT_ENA1_W<0> {
        FRHOST_BIT8_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit9_int_ena1(&mut self) -> FRHOST_BIT9_INT_ENA1_W<1> {
        FRHOST_BIT9_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit10_int_ena1(&mut self) -> FRHOST_BIT10_INT_ENA1_W<2> {
        FRHOST_BIT10_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit11_int_ena1(&mut self) -> FRHOST_BIT11_INT_ENA1_W<3> {
        FRHOST_BIT11_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit12_int_ena1(&mut self) -> FRHOST_BIT12_INT_ENA1_W<4> {
        FRHOST_BIT12_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit13_int_ena1(&mut self) -> FRHOST_BIT13_INT_ENA1_W<5> {
        FRHOST_BIT13_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit14_int_ena1(&mut self) -> FRHOST_BIT14_INT_ENA1_W<6> {
        FRHOST_BIT14_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit15_int_ena1(&mut self) -> FRHOST_BIT15_INT_ENA1_W<7> {
        FRHOST_BIT15_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc1_rx_start_int_ena1(&mut self) -> SLC1_RX_START_INT_ENA1_W<8> {
        SLC1_RX_START_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc1_tx_start_int_ena1(&mut self) -> SLC1_TX_START_INT_ENA1_W<9> {
        SLC1_TX_START_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_ena1(&mut self) -> SLC1_RX_UDF_INT_ENA1_W<10> {
        SLC1_RX_UDF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_ena1(&mut self) -> SLC1_TX_OVF_INT_ENA1_W<11> {
        SLC1_TX_OVF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_ena1(&mut self) -> SLC1_TOKEN0_1TO0_INT_ENA1_W<12> {
        SLC1_TOKEN0_1TO0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_ena1(&mut self) -> SLC1_TOKEN1_1TO0_INT_ENA1_W<13> {
        SLC1_TOKEN1_1TO0_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_done_int_ena1(&mut self) -> SLC1_TX_DONE_INT_ENA1_W<14> {
        SLC1_TX_DONE_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc1_tx_suc_eof_int_ena1(&mut self) -> SLC1_TX_SUC_EOF_INT_ENA1_W<15> {
        SLC1_TX_SUC_EOF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_done_int_ena1(&mut self) -> SLC1_RX_DONE_INT_ENA1_W<16> {
        SLC1_RX_DONE_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_eof_int_ena1(&mut self) -> SLC1_RX_EOF_INT_ENA1_W<17> {
        SLC1_RX_EOF_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_tohost_int_ena1(&mut self) -> SLC1_TOHOST_INT_ENA1_W<18> {
        SLC1_TOHOST_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_tx_dscr_err_int_ena1(&mut self) -> SLC1_TX_DSCR_ERR_INT_ENA1_W<19> {
        SLC1_TX_DSCR_ERR_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_dscr_err_int_ena1(&mut self) -> SLC1_RX_DSCR_ERR_INT_ENA1_W<20> {
        SLC1_RX_DSCR_ERR_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_tx_dscr_empty_int_ena1(&mut self) -> SLC1_TX_DSCR_EMPTY_INT_ENA1_W<21> {
        SLC1_TX_DSCR_EMPTY_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_host_rd_ack_int_ena1(&mut self) -> SLC1_HOST_RD_ACK_INT_ENA1_W<22> {
        SLC1_HOST_RD_ACK_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_wr_retry_done_int_ena1(&mut self) -> SLC1_WR_RETRY_DONE_INT_ENA1_W<23> {
        SLC1_WR_RETRY_DONE_INT_ENA1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_tx_err_eof_int_ena1(&mut self) -> SLC1_TX_ERR_EOF_INT_ENA1_W<24> {
        SLC1_TX_ERR_EOF_INT_ENA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1int_ena1](index.html) module"]
pub struct _1INT_ENA1_SPEC;
impl crate::RegisterSpec for _1INT_ENA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1int_ena1::R](R) reader structure"]
impl crate::Readable for _1INT_ENA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1int_ena1::W](W) writer structure"]
impl crate::Writable for _1INT_ENA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1INT_ENA1 to value 0"]
impl crate::Resettable for _1INT_ENA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
