#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE0` writer"]
pub struct W(crate::W<STATE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE0_SPEC>;
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
impl From<crate::W<STATE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CPU_INT` writer - Sends a SW RTC interrupt to CPU."]
pub type SW_CPU_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_CAUSE_CLR` writer - Clears the RTC reject-to-sleep cause."]
pub type SLP_REJECT_CAUSE_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub type APB2RTC_BRIDGE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub type APB2RTC_BRIDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
#[doc = "Field `SDIO_ACTIVE_IND` reader - Indicates the SDIO is active."]
pub type SDIO_ACTIVE_IND_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP` reader - Sleep wakeup bit."]
pub type SLP_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP` writer - Sleep wakeup bit."]
pub type SLP_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT` reader - Sleep reject bit."]
pub type SLP_REJECT_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT` writer - Sleep reject bit."]
pub type SLP_REJECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
#[doc = "Field `SLEEP_EN` reader - Sends the chip to sleep."]
pub type SLEEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_EN` writer - Sends the chip to sleep."]
pub type SLEEP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates the SDIO is active."]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Sleep wakeup bit."]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Sleep reject bit."]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sends the chip to sleep."]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sends a SW RTC interrupt to CPU."]
    #[inline(always)]
    pub fn sw_cpu_int(&mut self) -> SW_CPU_INT_W<0> {
        SW_CPU_INT_W::new(self)
    }
    #[doc = "Bit 1 - Clears the RTC reject-to-sleep cause."]
    #[inline(always)]
    pub fn slp_reject_cause_clr(&mut self) -> SLP_REJECT_CAUSE_CLR_W<1> {
        SLP_REJECT_CAUSE_CLR_W::new(self)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W<22> {
        APB2RTC_BRIDGE_SEL_W::new(self)
    }
    #[doc = "Bit 29 - Sleep wakeup bit."]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<29> {
        SLP_WAKEUP_W::new(self)
    }
    #[doc = "Bit 30 - Sleep reject bit."]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<30> {
        SLP_REJECT_W::new(self)
    }
    #[doc = "Bit 31 - Sends the chip to sleep."]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<31> {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the sleep / reject / wakeup state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state0::W](W) writer structure"]
impl crate::Writable for STATE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
