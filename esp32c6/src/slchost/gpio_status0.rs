#[doc = "Register `GPIO_STATUS0` reader"]
pub struct R(crate::R<GPIO_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_SDIO_INT0` reader - *******Description***********"]
pub type GPIO_SDIO_INT0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn gpio_sdio_int0(&self) -> GPIO_SDIO_INT0_R {
        GPIO_SDIO_INT0_R::new(self.bits)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status0](index.html) module"]
pub struct GPIO_STATUS0_SPEC;
impl crate::RegisterSpec for GPIO_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_status0::R](R) reader structure"]
impl crate::Readable for GPIO_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_STATUS0 to value 0"]
impl crate::Resettable for GPIO_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
