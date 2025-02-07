#[doc = "Register `CACHE_STATE` reader"]
pub struct R(crate::R<CACHE_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICACHE_STATE` reader - The bit is used to indicate whether icache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
pub type ICACHE_STATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCACHE_STATE` reader - The bit is used to indicate whether dcache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
pub type DCACHE_STATE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - The bit is used to indicate whether icache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
    #[inline(always)]
    pub fn icache_state(&self) -> ICACHE_STATE_R {
        ICACHE_STATE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - The bit is used to indicate whether dcache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
    #[inline(always)]
    pub fn dcache_state(&self) -> DCACHE_STATE_R {
        DCACHE_STATE_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_state](index.html) module"]
pub struct CACHE_STATE_SPEC;
impl crate::RegisterSpec for CACHE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_state::R](R) reader structure"]
impl crate::Readable for CACHE_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_STATE to value 0"]
impl crate::Resettable for CACHE_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
