#[doc = "Register `EXT_XTL_CONF` reader"]
pub struct R(crate::R<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_XTL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_XTL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_XTL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_XTL_CONF` writer"]
pub struct W(crate::W<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_XTL_CONF_SPEC>;
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
impl From<crate::W<EXT_XTL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_XTL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_WDT_EN` reader - Set this bit to enable the 32 kHz crystal watchdog."]
pub type XTAL32K_WDT_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_WDT_EN` writer - Set this bit to enable the 32 kHz crystal watchdog."]
pub type XTAL32K_WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_WDT_CLK_FO` reader - Set this bit to FPU the 32 kHz crystal watchdog clock."]
pub type XTAL32K_WDT_CLK_FO_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_WDT_CLK_FO` writer - Set this bit to FPU the 32 kHz crystal watchdog clock."]
pub type XTAL32K_WDT_CLK_FO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_WDT_RESET` reader - Set this bit to reset the 32 kHz crystal watchdog by SW."]
pub type XTAL32K_WDT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_WDT_RESET` writer - Set this bit to reset the 32 kHz crystal watchdog by SW."]
pub type XTAL32K_WDT_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_EXT_CLK_FO` reader - Set this bit to FPU the external clock of 32 kHz crystal."]
pub type XTAL32K_EXT_CLK_FO_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_EXT_CLK_FO` writer - Set this bit to FPU the external clock of 32 kHz crystal."]
pub type XTAL32K_EXT_CLK_FO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_AUTO_BACKUP` reader - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_BACKUP_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_AUTO_BACKUP` writer - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_BACKUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_AUTO_RESTART` reader - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_RESTART_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_AUTO_RESTART` writer - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
pub type XTAL32K_AUTO_RESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_AUTO_RETURN` reader - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
pub type XTAL32K_AUTO_RETURN_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_AUTO_RETURN` writer - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
pub type XTAL32K_AUTO_RETURN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTAL32K_XPD_FORCE` reader - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
pub type XTAL32K_XPD_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_XPD_FORCE` writer - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
pub type XTAL32K_XPD_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `ENCKINIT_XTAL_32K` reader - Applies an internal clock to help the 32 kHz crystal to start."]
pub type ENCKINIT_XTAL_32K_R = crate::BitReader<bool>;
#[doc = "Field `ENCKINIT_XTAL_32K` writer - Applies an internal clock to help the 32 kHz crystal to start."]
pub type ENCKINIT_XTAL_32K_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `DBUF_XTAL_32K` reader - 0: single-end buffer 1: differential buffer"]
pub type DBUF_XTAL_32K_R = crate::BitReader<bool>;
#[doc = "Field `DBUF_XTAL_32K` writer - 0: single-end buffer 1: differential buffer"]
pub type DBUF_XTAL_32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `DGM_XTAL_32K` reader - xtal_32k gm control"]
pub type DGM_XTAL_32K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DGM_XTAL_32K` writer - xtal_32k gm control"]
pub type DGM_XTAL_32K_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXT_XTL_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `DRES_XTAL_32K` reader - DRES_XTAL_32K"]
pub type DRES_XTAL_32K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRES_XTAL_32K` writer - DRES_XTAL_32K"]
pub type DRES_XTAL_32K_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXT_XTL_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `XPD_XTAL_32K` reader - XPD_XTAL_32K"]
pub type XPD_XTAL_32K_R = crate::BitReader<bool>;
#[doc = "Field `XPD_XTAL_32K` writer - XPD_XTAL_32K"]
pub type XPD_XTAL_32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `DAC_XTAL_32K` reader - DAC_XTAL_32K"]
pub type DAC_XTAL_32K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_XTAL_32K` writer - DAC_XTAL_32K"]
pub type DAC_XTAL_32K_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXT_XTL_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_STATE` reader - Stores the status of the 32 kHz watchdog."]
pub type WDT_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTAL32K_GPIO_SEL` reader - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
pub type XTAL32K_GPIO_SEL_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_GPIO_SEL` writer - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
pub type XTAL32K_GPIO_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTL_EXT_CTR_LV` reader - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
pub type XTL_EXT_CTR_LV_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_LV` writer - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
pub type XTL_EXT_CTR_LV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTL_EXT_CTR_EN` reader - Enables the GPIO to power down the crystal oscillator."]
pub type XTL_EXT_CTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_EN` writer - Enables the GPIO to power down the crystal oscillator."]
pub type XTL_EXT_CTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable the 32 kHz crystal watchdog."]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&self) -> XTAL32K_WDT_EN_R {
        XTAL32K_WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to FPU the 32 kHz crystal watchdog clock."]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&self) -> XTAL32K_WDT_CLK_FO_R {
        XTAL32K_WDT_CLK_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset the 32 kHz crystal watchdog by SW."]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&self) -> XTAL32K_WDT_RESET_R {
        XTAL32K_WDT_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to FPU the external clock of 32 kHz crystal."]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&self) -> XTAL32K_EXT_CLK_FO_R {
        XTAL32K_EXT_CLK_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&self) -> XTAL32K_AUTO_BACKUP_R {
        XTAL32K_AUTO_BACKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&self) -> XTAL32K_AUTO_RESTART_R {
        XTAL32K_AUTO_RESTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
    #[inline(always)]
    pub fn xtal32k_auto_return(&self) -> XTAL32K_AUTO_RETURN_R {
        XTAL32K_AUTO_RETURN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&self) -> XTAL32K_XPD_FORCE_R {
        XTAL32K_XPD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Applies an internal clock to help the 32 kHz crystal to start."]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&self) -> ENCKINIT_XTAL_32K_R {
        ENCKINIT_XTAL_32K_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&self) -> DBUF_XTAL_32K_R {
        DBUF_XTAL_32K_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&self) -> DGM_XTAL_32K_R {
        DGM_XTAL_32K_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Stores the status of the 32 kHz watchdog."]
    #[inline(always)]
    pub fn wdt_state(&self) -> WDT_STATE_R {
        WDT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
    #[inline(always)]
    pub fn xtal32k_gpio_sel(&self) -> XTAL32K_GPIO_SEL_R {
        XTAL32K_GPIO_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the GPIO to power down the crystal oscillator."]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable the 32 kHz crystal watchdog."]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&mut self) -> XTAL32K_WDT_EN_W<0> {
        XTAL32K_WDT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to FPU the 32 kHz crystal watchdog clock."]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&mut self) -> XTAL32K_WDT_CLK_FO_W<1> {
        XTAL32K_WDT_CLK_FO_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset the 32 kHz crystal watchdog by SW."]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&mut self) -> XTAL32K_WDT_RESET_W<2> {
        XTAL32K_WDT_RESET_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to FPU the external clock of 32 kHz crystal."]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&mut self) -> XTAL32K_EXT_CLK_FO_W<3> {
        XTAL32K_EXT_CLK_FO_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to switch to the backup clock when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&mut self) -> XTAL32K_AUTO_BACKUP_W<4> {
        XTAL32K_AUTO_BACKUP_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to restart the 32 kHz crystal automatically when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&mut self) -> XTAL32K_AUTO_RESTART_W<5> {
        XTAL32K_AUTO_RESTART_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to switch back to 32 kHz crystal when the 32 kHz crystal is restarted."]
    #[inline(always)]
    pub fn xtal32k_auto_return(&mut self) -> XTAL32K_AUTO_RETURN_W<6> {
        XTAL32K_AUTO_RETURN_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to allow the software to FPD the 32 kHz crystal. Set 0 to allow the FSM to FPD the 32 kHz crystal. (R/W)"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&mut self) -> XTAL32K_XPD_FORCE_W<7> {
        XTAL32K_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 8 - Applies an internal clock to help the 32 kHz crystal to start."]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&mut self) -> ENCKINIT_XTAL_32K_W<8> {
        ENCKINIT_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 9 - 0: single-end buffer 1: differential buffer"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&mut self) -> DBUF_XTAL_32K_W<9> {
        DBUF_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 10:12 - xtal_32k gm control"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&mut self) -> DGM_XTAL_32K_W<10> {
        DGM_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 13:15 - DRES_XTAL_32K"]
    #[inline(always)]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W<13> {
        DRES_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 16 - XPD_XTAL_32K"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W<16> {
        XPD_XTAL_32K_W::new(self)
    }
    #[doc = "Bits 17:19 - DAC_XTAL_32K"]
    #[inline(always)]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W<17> {
        DAC_XTAL_32K_W::new(self)
    }
    #[doc = "Bit 23 - Selects the 32 kHz crystal clock. 0: selects the external 32 kHz clock. 1: selects clock from the RTC GPIO X32P_C."]
    #[inline(always)]
    pub fn xtal32k_gpio_sel(&mut self) -> XTAL32K_GPIO_SEL_W<23> {
        XTAL32K_GPIO_SEL_W::new(self)
    }
    #[doc = "Bit 30 - 0: powers down XTAL at high level 1: powers down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W<30> {
        XTL_EXT_CTR_LV_W::new(self)
    }
    #[doc = "Bit 31 - Enables the GPIO to power down the crystal oscillator."]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W<31> {
        XTL_EXT_CTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 kHz crystal oscillator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_xtl_conf](index.html) module"]
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_xtl_conf::R](R) reader structure"]
impl crate::Readable for EXT_XTL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](W) writer structure"]
impl crate::Writable for EXT_XTL_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_XTL_CONF to value 0x0006_6c80"]
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0006_6c80
    }
}