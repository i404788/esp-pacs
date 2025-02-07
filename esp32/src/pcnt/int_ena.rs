#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THR_EVENT_U0_INT_ENA` reader - This is the interrupt enable bit for channel0 event."]
pub type CNT_THR_EVENT_U0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U0_INT_ENA` writer - This is the interrupt enable bit for channel0 event."]
pub type CNT_THR_EVENT_U0_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U1_INT_ENA` reader - This is the interrupt enable bit for channel1 event."]
pub type CNT_THR_EVENT_U1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U1_INT_ENA` writer - This is the interrupt enable bit for channel1 event."]
pub type CNT_THR_EVENT_U1_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U2_INT_ENA` reader - This is the interrupt enable bit for channel2 event."]
pub type CNT_THR_EVENT_U2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U2_INT_ENA` writer - This is the interrupt enable bit for channel2 event."]
pub type CNT_THR_EVENT_U2_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U3_INT_ENA` reader - This is the interrupt enable bit for channel3 event."]
pub type CNT_THR_EVENT_U3_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U3_INT_ENA` writer - This is the interrupt enable bit for channel3 event."]
pub type CNT_THR_EVENT_U3_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U4_INT_ENA` reader - This is the interrupt enable bit for channel4 event."]
pub type CNT_THR_EVENT_U4_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U4_INT_ENA` writer - This is the interrupt enable bit for channel4 event."]
pub type CNT_THR_EVENT_U4_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U5_INT_ENA` reader - This is the interrupt enable bit for channel5 event."]
pub type CNT_THR_EVENT_U5_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U5_INT_ENA` writer - This is the interrupt enable bit for channel5 event."]
pub type CNT_THR_EVENT_U5_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U6_INT_ENA` reader - This is the interrupt enable bit for channel6 event."]
pub type CNT_THR_EVENT_U6_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U6_INT_ENA` writer - This is the interrupt enable bit for channel6 event."]
pub type CNT_THR_EVENT_U6_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U7_INT_ENA` reader - This is the interrupt enable bit for channel7 event."]
pub type CNT_THR_EVENT_U7_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U7_INT_ENA` writer - This is the interrupt enable bit for channel7 event."]
pub type CNT_THR_EVENT_U7_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_ena(&self) -> CNT_THR_EVENT_U0_INT_ENA_R {
        CNT_THR_EVENT_U0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_ena(&self) -> CNT_THR_EVENT_U1_INT_ENA_R {
        CNT_THR_EVENT_U1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_ena(&self) -> CNT_THR_EVENT_U2_INT_ENA_R {
        CNT_THR_EVENT_U2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_ena(&self) -> CNT_THR_EVENT_U3_INT_ENA_R {
        CNT_THR_EVENT_U3_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_ena(&self) -> CNT_THR_EVENT_U4_INT_ENA_R {
        CNT_THR_EVENT_U4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_ena(&self) -> CNT_THR_EVENT_U5_INT_ENA_R {
        CNT_THR_EVENT_U5_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_ena(&self) -> CNT_THR_EVENT_U6_INT_ENA_R {
        CNT_THR_EVENT_U6_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_ena(&self) -> CNT_THR_EVENT_U7_INT_ENA_R {
        CNT_THR_EVENT_U7_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_ena(&mut self) -> CNT_THR_EVENT_U0_INT_ENA_W<0> {
        CNT_THR_EVENT_U0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_ena(&mut self) -> CNT_THR_EVENT_U1_INT_ENA_W<1> {
        CNT_THR_EVENT_U1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_ena(&mut self) -> CNT_THR_EVENT_U2_INT_ENA_W<2> {
        CNT_THR_EVENT_U2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_ena(&mut self) -> CNT_THR_EVENT_U3_INT_ENA_W<3> {
        CNT_THR_EVENT_U3_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_ena(&mut self) -> CNT_THR_EVENT_U4_INT_ENA_W<4> {
        CNT_THR_EVENT_U4_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_ena(&mut self) -> CNT_THR_EVENT_U5_INT_ENA_W<5> {
        CNT_THR_EVENT_U5_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_ena(&mut self) -> CNT_THR_EVENT_U6_INT_ENA_W<6> {
        CNT_THR_EVENT_U6_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_ena(&mut self) -> CNT_THR_EVENT_U7_INT_ENA_W<7> {
        CNT_THR_EVENT_U7_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
