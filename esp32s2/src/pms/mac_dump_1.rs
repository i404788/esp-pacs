#[doc = "Register `MAC_DUMP_1` reader"]
pub struct R(crate::R<MAC_DUMP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_DUMP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_DUMP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_DUMP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_DUMP_1` writer"]
pub struct W(crate::W<MAC_DUMP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_DUMP_1_SPEC>;
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
impl From<crate::W<MAC_DUMP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_DUMP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_DUMP_CONNECT` reader - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAC_DUMP_CONNECT` writer - Configure MAC dump connection."]
pub type MAC_DUMP_CONNECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_DUMP_1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Configure MAC dump connection."]
    #[inline(always)]
    pub fn mac_dump_connect(&self) -> MAC_DUMP_CONNECT_R {
        MAC_DUMP_CONNECT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configure MAC dump connection."]
    #[inline(always)]
    pub fn mac_dump_connect(&mut self) -> MAC_DUMP_CONNECT_W<0> {
        MAC_DUMP_CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC dump permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_dump_1](index.html) module"]
pub struct MAC_DUMP_1_SPEC;
impl crate::RegisterSpec for MAC_DUMP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_dump_1::R](R) reader structure"]
impl crate::Readable for MAC_DUMP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_dump_1::W](W) writer structure"]
impl crate::Writable for MAC_DUMP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_DUMP_1 to value 0xe4"]
impl crate::Resettable for MAC_DUMP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe4
    }
}
