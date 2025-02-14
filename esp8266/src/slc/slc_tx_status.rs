#[doc = "Register `SLC_TX_STATUS` reader"]
pub struct R(crate::R<SLC_TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_TX_STATUS` writer"]
pub struct W(crate::W<SLC_TX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_TX_STATUS_SPEC>;
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
impl From<crate::W<SLC_TX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_TX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TX_FULL` reader - "]
pub type SLC_TX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_FULL` writer - "]
pub type SLC_TX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLC_TX_STATUS_SPEC, bool, O>;
#[doc = "Field `SLC_TX_EMPTY` reader - "]
pub type SLC_TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SLC_TX_EMPTY` writer - "]
pub type SLC_TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLC_TX_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_tx_full(&self) -> SLC_TX_FULL_R {
        SLC_TX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_tx_empty(&self) -> SLC_TX_EMPTY_R {
        SLC_TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_tx_full(&mut self) -> SLC_TX_FULL_W<0> {
        SLC_TX_FULL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_tx_empty(&mut self) -> SLC_TX_EMPTY_W<1> {
        SLC_TX_EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_TX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_tx_status](index.html) module"]
pub struct SLC_TX_STATUS_SPEC;
impl crate::RegisterSpec for SLC_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_tx_status::R](R) reader structure"]
impl crate::Readable for SLC_TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_tx_status::W](W) writer structure"]
impl crate::Writable for SLC_TX_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLC_TX_STATUS to value 0"]
impl crate::Resettable for SLC_TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
