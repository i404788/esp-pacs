#[doc = "Register `BLK0_WDATA1` reader"]
pub struct R(crate::R<BLK0_WDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA1` writer"]
pub struct W(crate::W<BLK0_WDATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA1_SPEC>;
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
impl From<crate::W<BLK0_WDATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_MAC_CRC_LOW` reader - program for low 32bit WIFI_MAC_Address"]
pub type WIFI_MAC_CRC_LOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WIFI_MAC_CRC_LOW` writer - program for low 32bit WIFI_MAC_Address"]
pub type WIFI_MAC_CRC_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK0_WDATA1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - program for low 32bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn wifi_mac_crc_low(&self) -> WIFI_MAC_CRC_LOW_R {
        WIFI_MAC_CRC_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for low 32bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn wifi_mac_crc_low(&mut self) -> WIFI_MAC_CRC_LOW_W<0> {
        WIFI_MAC_CRC_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata1](index.html) module"]
pub struct BLK0_WDATA1_SPEC;
impl crate::RegisterSpec for BLK0_WDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata1::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata1::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK0_WDATA1 to value 0"]
impl crate::Resettable for BLK0_WDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
