#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub struct W(crate::W<INT_ENA_RTC_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_W1TC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA_W1TC` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `WDT_INT_ENA_W1TC` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA_W1TC` writer - enable touch scan done interrupt"]
pub type TOUCH_SCAN_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `ULP_CP_INT_ENA_W1TC` writer - enable ULP-coprocessor interrupt"]
pub type ULP_CP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_DONE_INT_ENA_W1TC` writer - enable touch done interrupt"]
pub type TOUCH_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_ACTIVE_INT_ENA_W1TC` writer - enable touch active interrupt"]
pub type TOUCH_ACTIVE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_INACTIVE_INT_ENA_W1TC` writer - enable touch inactive interrupt"]
pub type TOUCH_INACTIVE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TC` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TC` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SARADC1_INT_ENA_W1TC` writer - enable saradc1 interrupt"]
pub type SARADC1_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TSENS_INT_ENA_W1TC` writer - enable tsens interrupt"]
pub type TSENS_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `COCPU_INT_ENA_W1TC` writer - enable riscV cocpu interrupt"]
pub type COCPU_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SARADC2_INT_ENA_W1TC` writer - enable saradc2 interrupt"]
pub type SARADC2_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SWD_INT_ENA_W1TC` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TC` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `COCPU_TRAP_INT_ENA_W1TC` writer - enable cocpu trap interrupt"]
pub type COCPU_TRAP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA_W1TC` writer - enable touch timeout interrupt"]
pub type TOUCH_TIMEOUT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `GLITCH_DET_INT_ENA_W1TC` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC` writer - enbale touch approach_loop done interrupt"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W<0> {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W<1> {
        SLP_REJECT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle_int_ena_w1tc(&mut self) -> SDIO_IDLE_INT_ENA_W1TC_W<2> {
        SDIO_IDLE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt_int_ena_w1tc(&mut self) -> WDT_INT_ENA_W1TC_W<3> {
        WDT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    pub fn touch_scan_done_int_ena_w1tc(&mut self) -> TOUCH_SCAN_DONE_INT_ENA_W1TC_W<4> {
        TOUCH_SCAN_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn ulp_cp_int_ena_w1tc(&mut self) -> ULP_CP_INT_ENA_W1TC_W<5> {
        ULP_CP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    pub fn touch_done_int_ena_w1tc(&mut self) -> TOUCH_DONE_INT_ENA_W1TC_W<6> {
        TOUCH_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    pub fn touch_active_int_ena_w1tc(&mut self) -> TOUCH_ACTIVE_INT_ENA_W1TC_W<7> {
        TOUCH_ACTIVE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    pub fn touch_inactive_int_ena_w1tc(&mut self) -> TOUCH_INACTIVE_INT_ENA_W1TC_W<8> {
        TOUCH_INACTIVE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out_int_ena_w1tc(&mut self) -> BROWN_OUT_INT_ENA_W1TC_W<9> {
        BROWN_OUT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer_int_ena_w1tc(&mut self) -> MAIN_TIMER_INT_ENA_W1TC_W<10> {
        MAIN_TIMER_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    pub fn saradc1_int_ena_w1tc(&mut self) -> SARADC1_INT_ENA_W1TC_W<11> {
        SARADC1_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    pub fn tsens_int_ena_w1tc(&mut self) -> TSENS_INT_ENA_W1TC_W<12> {
        TSENS_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    pub fn cocpu_int_ena_w1tc(&mut self) -> COCPU_INT_ENA_W1TC_W<13> {
        COCPU_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    pub fn saradc2_int_ena_w1tc(&mut self) -> SARADC2_INT_ENA_W1TC_W<14> {
        SARADC2_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd_int_ena_w1tc(&mut self) -> SWD_INT_ENA_W1TC_W<15> {
        SWD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena_w1tc(&mut self) -> XTAL32K_DEAD_INT_ENA_W1TC_W<16> {
        XTAL32K_DEAD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    pub fn cocpu_trap_int_ena_w1tc(&mut self) -> COCPU_TRAP_INT_ENA_W1TC_W<17> {
        COCPU_TRAP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    pub fn touch_timeout_int_ena_w1tc(&mut self) -> TOUCH_TIMEOUT_INT_ENA_W1TC_W<18> {
        TOUCH_TIMEOUT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn glitch_det_int_ena_w1tc(&mut self) -> GLITCH_DET_INT_ENA_W1TC_W<19> {
        GLITCH_DET_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 20 - enbale touch approach_loop done interrupt"]
    #[inline(always)]
    pub fn touch_approach_loop_done_int_ena_w1tc(
        &mut self,
    ) -> TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<20> {
        TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "oneset clr rtc interrupt enable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc_w1tc](index.html) module"]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc_w1tc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
