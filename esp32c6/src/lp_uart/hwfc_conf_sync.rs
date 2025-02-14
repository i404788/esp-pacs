#[doc = "Register `HWFC_CONF_SYNC` reader"]
pub struct R(crate::R<HWFC_CONF_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWFC_CONF_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWFC_CONF_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWFC_CONF_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWFC_CONF_SYNC` writer"]
pub struct W(crate::W<HWFC_CONF_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWFC_CONF_SYNC_SPEC>;
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
impl From<crate::W<HWFC_CONF_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWFC_CONF_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HWFC_CONF_SYNC_SPEC, u8, u8, 5, O>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for UART receiver."]
pub type RX_FLOW_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for UART receiver."]
pub type RX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWFC_CONF_SYNC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 3:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<3> {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<8> {
        RX_FLOW_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware flow-control configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwfc_conf_sync](index.html) module"]
pub struct HWFC_CONF_SYNC_SPEC;
impl crate::RegisterSpec for HWFC_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwfc_conf_sync::R](R) reader structure"]
impl crate::Readable for HWFC_CONF_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwfc_conf_sync::W](W) writer structure"]
impl crate::Writable for HWFC_CONF_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWFC_CONF_SYNC to value 0"]
impl crate::Resettable for HWFC_CONF_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
