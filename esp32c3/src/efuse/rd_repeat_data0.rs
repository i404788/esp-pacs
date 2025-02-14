#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub struct R(crate::R<RD_REPEAT_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS` reader - Set this bit to disable reading from BlOCK4-10."]
pub type RD_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS_RTC_RAM_BOOT` reader - Set this bit to disable boot from RTC RAM."]
pub type DIS_RTC_RAM_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `DIS_ICACHE` reader - Set this bit to disable Icache."]
pub type DIS_ICACHE_R = crate::BitReader<bool>;
#[doc = "Field `DIS_USB_JTAG` reader - Set this bit to disable function of usb switch to jtag in module of usb device."]
pub type DIS_USB_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOWNLOAD_ICACHE` reader - Set this bit to disable Icache in download mode (boot_mode\\[3:0\\] is 0, 1, 2, 3, 6, 7)."]
pub type DIS_DOWNLOAD_ICACHE_R = crate::BitReader<bool>;
#[doc = "Field `DIS_USB_DEVICE` reader - Set this bit to disable usb device."]
pub type DIS_USB_DEVICE_R = crate::BitReader<bool>;
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - Set this bit to disable the function that forces chip into download mode."]
pub type DIS_FORCE_DOWNLOAD_R = crate::BitReader<bool>;
#[doc = "Field `RPT4_RESERVED6` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED6_R = crate::BitReader<bool>;
#[doc = "Field `DIS_CAN` reader - Set this bit to disable CAN function."]
pub type DIS_CAN_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_SEL_ENABLE` reader - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio10 when both reg_dis_usb_jtag and reg_dis_pad_jtag are equal to 0."]
pub type JTAG_SEL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_DIS_JTAG` reader - Set these bits to disable JTAG in the soft way (odd number 1 means disable ). JTAG can be enabled in HMAC module."]
pub type SOFT_DIS_JTAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS_PAD_JTAG` reader - Set this bit to disable JTAG in the hard way. JTAG is disabled permanently."]
pub type DIS_PAD_JTAG_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - Set this bit to disable flash encryption when in download boot modes."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader<bool>;
#[doc = "Field `USB_DREFH` reader - Controls single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse."]
pub type USB_DREFH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_DREFL` reader - Controls single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse."]
pub type USB_DREFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_EXCHG_PINS` reader - Set this bit to exchange USB D+ and D- pins."]
pub type USB_EXCHG_PINS_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_AS_GPIO` reader - Set this bit to vdd spi pin function as gpio."]
pub type VDD_SPI_AS_GPIO_R = crate::BitReader<bool>;
#[doc = "Field `BTLC_GPIO_ENABLE` reader - Enable btlc gpio."]
pub type BTLC_GPIO_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POWERGLITCH_EN` reader - Set this bit to enable power glitch function."]
pub type POWERGLITCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `POWER_GLITCH_DSENSE` reader - Sample delay configuration of power glitch."]
pub type POWER_GLITCH_DSENSE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Set this bit to disable reading from BlOCK4-10."]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Set this bit to disable boot from RTC RAM."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&self) -> DIS_RTC_RAM_BOOT_R {
        DIS_RTC_RAM_BOOT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to disable Icache."]
    #[inline(always)]
    pub fn dis_icache(&self) -> DIS_ICACHE_R {
        DIS_ICACHE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to disable function of usb switch to jtag in module of usb device."]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to disable Icache in download mode (boot_mode\\[3:0\\] is 0, 1, 2, 3, 6, 7)."]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to disable usb device."]
    #[inline(always)]
    pub fn dis_usb_device(&self) -> DIS_USB_DEVICE_R {
        DIS_USB_DEVICE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to disable the function that forces chip into download mode."]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved6(&self) -> RPT4_RESERVED6_R {
        RPT4_RESERVED6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to disable CAN function."]
    #[inline(always)]
    pub fn dis_can(&self) -> DIS_CAN_R {
        DIS_CAN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio10 when both reg_dis_usb_jtag and reg_dis_pad_jtag are equal to 0."]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Set these bits to disable JTAG in the soft way (odd number 1 means disable ). JTAG can be enabled in HMAC module."]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Set this bit to disable JTAG in the hard way. JTAG is disabled permanently."]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to disable flash encryption when in download boot modes."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Controls single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse."]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Controls single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse."]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Set this bit to exchange USB D+ and D- pins."]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to vdd spi pin function as gpio."]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&self) -> VDD_SPI_AS_GPIO_R {
        VDD_SPI_AS_GPIO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Enable btlc gpio."]
    #[inline(always)]
    pub fn btlc_gpio_enable(&self) -> BTLC_GPIO_ENABLE_R {
        BTLC_GPIO_ENABLE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable power glitch function."]
    #[inline(always)]
    pub fn powerglitch_en(&self) -> POWERGLITCH_EN_R {
        POWERGLITCH_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Sample delay configuration of power glitch."]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "BLOCK0 data register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data0](index.html) module"]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data0::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
