#[doc = "Register `ICACHE_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_AUTOLOAD_CTRL` writer"]
pub struct W(crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>;
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
impl From<crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the first section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the first section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT0_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHE_AUTOLOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the second section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the second section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT1_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHE_AUTOLOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `ICACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
pub type ICACHE_AUTOLOAD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
pub type ICACHE_AUTOLOAD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHE_AUTOLOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `ICACHE_AUTOLOAD_DONE` reader - The bit is used to indicate autoload operation is finished."]
pub type ICACHE_AUTOLOAD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type ICACHE_AUTOLOAD_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type ICACHE_AUTOLOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHE_AUTOLOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `ICACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type ICACHE_AUTOLOAD_RQST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type ICACHE_AUTOLOAD_RQST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICACHE_AUTOLOAD_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct0_ena(&self) -> ICACHE_AUTOLOAD_SCT0_ENA_R {
        ICACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct1_ena(&self) -> ICACHE_AUTOLOAD_SCT1_ENA_R {
        ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn icache_autoload_ena(&self) -> ICACHE_AUTOLOAD_ENA_R {
        ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate autoload operation is finished."]
    #[inline(always)]
    pub fn icache_autoload_done(&self) -> ICACHE_AUTOLOAD_DONE_R {
        ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn icache_autoload_order(&self) -> ICACHE_AUTOLOAD_ORDER_R {
        ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn icache_autoload_rqst(&self) -> ICACHE_AUTOLOAD_RQST_R {
        ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct0_ena(&mut self) -> ICACHE_AUTOLOAD_SCT0_ENA_W<0> {
        ICACHE_AUTOLOAD_SCT0_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct1_ena(&mut self) -> ICACHE_AUTOLOAD_SCT1_ENA_W<1> {
        ICACHE_AUTOLOAD_SCT1_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn icache_autoload_ena(&mut self) -> ICACHE_AUTOLOAD_ENA_W<2> {
        ICACHE_AUTOLOAD_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn icache_autoload_order(&mut self) -> ICACHE_AUTOLOAD_ORDER_W<4> {
        ICACHE_AUTOLOAD_ORDER_W::new(self)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn icache_autoload_rqst(&mut self) -> ICACHE_AUTOLOAD_RQST_W<5> {
        ICACHE_AUTOLOAD_RQST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_autoload_ctrl](index.html) module"]
pub struct ICACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_autoload_ctrl::R](R) reader structure"]
impl crate::Readable for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_autoload_ctrl::W](W) writer structure"]
impl crate::Writable for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_AUTOLOAD_CTRL to value 0x08"]
impl crate::Resettable for ICACHE_AUTOLOAD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
