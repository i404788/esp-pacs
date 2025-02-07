#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT_THR_EVENT_U0_INT_ST` reader - This is the interrupt status bit for channel0 event."]
pub type CNT_THR_EVENT_U0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U1_INT_ST` reader - This is the interrupt status bit for channel1 event."]
pub type CNT_THR_EVENT_U1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U2_INT_ST` reader - This is the interrupt status bit for channel2 event."]
pub type CNT_THR_EVENT_U2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U3_INT_ST` reader - This is the interrupt status bit for channel3 event."]
pub type CNT_THR_EVENT_U3_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U4_INT_ST` reader - This is the interrupt status bit for channel4 event."]
pub type CNT_THR_EVENT_U4_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U5_INT_ST` reader - This is the interrupt status bit for channel5 event."]
pub type CNT_THR_EVENT_U5_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U6_INT_ST` reader - This is the interrupt status bit for channel6 event."]
pub type CNT_THR_EVENT_U6_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CNT_THR_EVENT_U7_INT_ST` reader - This is the interrupt status bit for channel7 event."]
pub type CNT_THR_EVENT_U7_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This is the interrupt status bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0_int_st(&self) -> CNT_THR_EVENT_U0_INT_ST_R {
        CNT_THR_EVENT_U0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt status bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1_int_st(&self) -> CNT_THR_EVENT_U1_INT_ST_R {
        CNT_THR_EVENT_U1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt status bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2_int_st(&self) -> CNT_THR_EVENT_U2_INT_ST_R {
        CNT_THR_EVENT_U2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt status bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3_int_st(&self) -> CNT_THR_EVENT_U3_INT_ST_R {
        CNT_THR_EVENT_U3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt status bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4_int_st(&self) -> CNT_THR_EVENT_U4_INT_ST_R {
        CNT_THR_EVENT_U4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt status bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5_int_st(&self) -> CNT_THR_EVENT_U5_INT_ST_R {
        CNT_THR_EVENT_U5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt status bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6_int_st(&self) -> CNT_THR_EVENT_U6_INT_ST_R {
        CNT_THR_EVENT_U6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt status bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7_int_st(&self) -> CNT_THR_EVENT_U7_INT_ST_R {
        CNT_THR_EVENT_U7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
