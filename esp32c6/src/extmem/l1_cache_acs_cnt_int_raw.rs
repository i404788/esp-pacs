#[doc = "Register `L1_CACHE_ACS_CNT_INT_RAW` reader"]
pub struct R(crate::R<L1_CACHE_ACS_CNT_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_ACS_CNT_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_ACS_CNT_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_ACS_CNT_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1_IBUS0_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1_IBUS1_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2."]
pub type L1_IBUS2_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3."]
pub type L1_IBUS3_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1_BUS0_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1_BUS1_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache."]
pub type L1_DBUS2_OVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `L1_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache."]
pub type L1_DBUS3_OVF_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_raw(&self) -> L1_IBUS0_OVF_INT_RAW_R {
        L1_IBUS0_OVF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_raw(&self) -> L1_IBUS1_OVF_INT_RAW_R {
        L1_IBUS1_OVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2."]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_raw(&self) -> L1_IBUS2_OVF_INT_RAW_R {
        L1_IBUS2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3."]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_raw(&self) -> L1_IBUS3_OVF_INT_RAW_R {
        L1_IBUS3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_bus0_ovf_int_raw(&self) -> L1_BUS0_OVF_INT_RAW_R {
        L1_BUS0_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_bus1_ovf_int_raw(&self) -> L1_BUS1_OVF_INT_RAW_R {
        L1_BUS1_OVF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_raw(&self) -> L1_DBUS2_OVF_INT_RAW_R {
        L1_DBUS2_OVF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_raw(&self) -> L1_DBUS3_OVF_INT_RAW_R {
        L1_DBUS3_OVF_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_acs_cnt_int_raw](index.html) module"]
pub struct L1_CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_acs_cnt_int_raw::R](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
