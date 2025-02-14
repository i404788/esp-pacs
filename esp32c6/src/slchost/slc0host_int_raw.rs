#[doc = "Register `SLC0HOST_INT_RAW` reader"]
pub struct R(crate::R<SLC0HOST_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC0HOST_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC0HOST_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC0HOST_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_TOHOST_BIT0_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT1_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT2_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT2_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT3_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT3_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT4_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT4_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT5_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT5_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT6_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT6_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_BIT7_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOHOST_BIT7_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOKEN0_1TO0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOKEN1_1TO0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN0_0TO1_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOKEN0_0TO1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN1_0TO1_INT_RAW` reader - *******Description***********"]
pub type SLC0_TOKEN1_0TO1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0HOST_RX_SOF_INT_RAW` reader - *******Description***********"]
pub type SLC0HOST_RX_SOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0HOST_RX_EOF_INT_RAW` reader - *******Description***********"]
pub type SLC0HOST_RX_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0HOST_RX_START_INT_RAW` reader - *******Description***********"]
pub type SLC0HOST_RX_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0HOST_TX_START_INT_RAW` reader - *******Description***********"]
pub type SLC0HOST_TX_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_UDF_INT_RAW` reader - *******Description***********"]
pub type SLC0_RX_UDF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_OVF_INT_RAW` reader - *******Description***********"]
pub type SLC0_TX_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_PF_VALID_INT_RAW` reader - *******Description***********"]
pub type SLC0_RX_PF_VALID_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_EXT_BIT0_INT_RAW` reader - *******Description***********"]
pub type SLC0_EXT_BIT0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_EXT_BIT1_INT_RAW` reader - *******Description***********"]
pub type SLC0_EXT_BIT1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_EXT_BIT2_INT_RAW` reader - *******Description***********"]
pub type SLC0_EXT_BIT2_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_EXT_BIT3_INT_RAW` reader - *******Description***********"]
pub type SLC0_EXT_BIT3_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_NEW_PACKET_INT_RAW` reader - *******Description***********"]
pub type SLC0_RX_NEW_PACKET_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_HOST_RD_RETRY_INT_RAW` reader - *******Description***********"]
pub type SLC0_HOST_RD_RETRY_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_SDIO_INT_RAW` reader - *******Description***********"]
pub type GPIO_SDIO_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit0_int_raw(&self) -> SLC0_TOHOST_BIT0_INT_RAW_R {
        SLC0_TOHOST_BIT0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit1_int_raw(&self) -> SLC0_TOHOST_BIT1_INT_RAW_R {
        SLC0_TOHOST_BIT1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit2_int_raw(&self) -> SLC0_TOHOST_BIT2_INT_RAW_R {
        SLC0_TOHOST_BIT2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit3_int_raw(&self) -> SLC0_TOHOST_BIT3_INT_RAW_R {
        SLC0_TOHOST_BIT3_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit4_int_raw(&self) -> SLC0_TOHOST_BIT4_INT_RAW_R {
        SLC0_TOHOST_BIT4_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit5_int_raw(&self) -> SLC0_TOHOST_BIT5_INT_RAW_R {
        SLC0_TOHOST_BIT5_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit6_int_raw(&self) -> SLC0_TOHOST_BIT6_INT_RAW_R {
        SLC0_TOHOST_BIT6_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tohost_bit7_int_raw(&self) -> SLC0_TOHOST_BIT7_INT_RAW_R {
        SLC0_TOHOST_BIT7_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_token0_1to0_int_raw(&self) -> SLC0_TOKEN0_1TO0_INT_RAW_R {
        SLC0_TOKEN0_1TO0_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_token1_1to0_int_raw(&self) -> SLC0_TOKEN1_1TO0_INT_RAW_R {
        SLC0_TOKEN1_1TO0_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_token0_0to1_int_raw(&self) -> SLC0_TOKEN0_0TO1_INT_RAW_R {
        SLC0_TOKEN0_0TO1_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_token1_0to1_int_raw(&self) -> SLC0_TOKEN1_0TO1_INT_RAW_R {
        SLC0_TOKEN1_0TO1_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_rx_sof_int_raw(&self) -> SLC0HOST_RX_SOF_INT_RAW_R {
        SLC0HOST_RX_SOF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_rx_eof_int_raw(&self) -> SLC0HOST_RX_EOF_INT_RAW_R {
        SLC0HOST_RX_EOF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_rx_start_int_raw(&self) -> SLC0HOST_RX_START_INT_RAW_R {
        SLC0HOST_RX_START_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_tx_start_int_raw(&self) -> SLC0HOST_TX_START_INT_RAW_R {
        SLC0HOST_TX_START_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_rx_udf_int_raw(&self) -> SLC0_RX_UDF_INT_RAW_R {
        SLC0_RX_UDF_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_tx_ovf_int_raw(&self) -> SLC0_TX_OVF_INT_RAW_R {
        SLC0_TX_OVF_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_rx_pf_valid_int_raw(&self) -> SLC0_RX_PF_VALID_INT_RAW_R {
        SLC0_RX_PF_VALID_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_ext_bit0_int_raw(&self) -> SLC0_EXT_BIT0_INT_RAW_R {
        SLC0_EXT_BIT0_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_ext_bit1_int_raw(&self) -> SLC0_EXT_BIT1_INT_RAW_R {
        SLC0_EXT_BIT1_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_ext_bit2_int_raw(&self) -> SLC0_EXT_BIT2_INT_RAW_R {
        SLC0_EXT_BIT2_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_ext_bit3_int_raw(&self) -> SLC0_EXT_BIT3_INT_RAW_R {
        SLC0_EXT_BIT3_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_rx_new_packet_int_raw(&self) -> SLC0_RX_NEW_PACKET_INT_RAW_R {
        SLC0_RX_NEW_PACKET_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_host_rd_retry_int_raw(&self) -> SLC0_HOST_RD_RETRY_INT_RAW_R {
        SLC0_HOST_RD_RETRY_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    pub fn gpio_sdio_int_raw(&self) -> GPIO_SDIO_INT_RAW_R {
        GPIO_SDIO_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc0host_int_raw](index.html) module"]
pub struct SLC0HOST_INT_RAW_SPEC;
impl crate::RegisterSpec for SLC0HOST_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc0host_int_raw::R](R) reader structure"]
impl crate::Readable for SLC0HOST_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLC0HOST_INT_RAW to value 0"]
impl crate::Resettable for SLC0HOST_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
