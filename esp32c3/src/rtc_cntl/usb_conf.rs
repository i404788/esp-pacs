#[doc = "Register `USB_CONF` reader"]
pub struct R(crate::R<USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CONF` writer"]
pub struct W(crate::W<USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONF_SPEC>;
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
impl From<crate::W<USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - disable io_mux reset"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - disable io_mux reset"]
pub type IO_MUX_RESET_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - disable io_mux reset"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - disable io_mux reset"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<18> {
        IO_MUX_RESET_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_conf](index.html) module"]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_conf::R](R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_conf::W](W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
