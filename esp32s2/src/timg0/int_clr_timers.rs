#[doc = "Register `INT_CLR_TIMERS` writer"]
pub struct W(crate::W<INT_CLR_TIMERS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_TIMERS_SPEC>;
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
impl From<crate::W<INT_CLR_TIMERS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_TIMERS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_INT_CLR` writer - Set this bit to clear the TIMG_T0_INT interrupt."]
pub type T0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_TIMERS_SPEC, bool, O>;
#[doc = "Field `T1_INT_CLR` writer - Set this bit to clear the TIMG_T1_INT interrupt."]
pub type T1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_TIMERS_SPEC, bool, O>;
#[doc = "Field `WDT_INT_CLR` writer - Set this bit to clear the TIMG_WDT_INT interrupt."]
pub type WDT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_TIMERS_SPEC, bool, O>;
#[doc = "Field `LACT_INT_CLR` writer - Set this bit to clear the TIMG_LACT_INT interrupt."]
pub type LACT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_TIMERS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_clr(&mut self) -> T0_INT_CLR_W<0> {
        T0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_clr(&mut self) -> T1_INT_CLR_W<1> {
        T1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<2> {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the TIMG_LACT_INT interrupt."]
    #[inline(always)]
    pub fn lact_int_clr(&mut self) -> LACT_INT_CLR_W<3> {
        LACT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_timers](index.html) module"]
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_timers::W](W) writer structure"]
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR_TIMERS to value 0"]
impl crate::Resettable for INT_CLR_TIMERS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
