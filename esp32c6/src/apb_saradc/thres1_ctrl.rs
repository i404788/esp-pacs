#[doc = "Register `THRES1_CTRL` reader"]
pub struct R(crate::R<THRES1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES1_CTRL` writer"]
pub struct W(crate::W<THRES1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES1_CTRL_SPEC>;
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
impl From<crate::W<THRES1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_THRES1_CHANNEL` reader - configure thres1 to adc channel"]
pub type APB_SARADC_THRES1_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB_SARADC_THRES1_CHANNEL` writer - configure thres1 to adc channel"]
pub type APB_SARADC_THRES1_CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES1_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `APB_SARADC_THRES1_HIGH` reader - saradc thres1 monitor thres"]
pub type APB_SARADC_THRES1_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APB_SARADC_THRES1_HIGH` writer - saradc thres1 monitor thres"]
pub type APB_SARADC_THRES1_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES1_CTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `APB_SARADC_THRES1_LOW` reader - saradc thres1 monitor thres"]
pub type APB_SARADC_THRES1_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APB_SARADC_THRES1_LOW` writer - saradc thres1 monitor thres"]
pub type APB_SARADC_THRES1_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, THRES1_CTRL_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:3 - configure thres1 to adc channel"]
    #[inline(always)]
    pub fn apb_saradc_thres1_channel(&self) -> APB_SARADC_THRES1_CHANNEL_R {
        APB_SARADC_THRES1_CHANNEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:17 - saradc thres1 monitor thres"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high(&self) -> APB_SARADC_THRES1_HIGH_R {
        APB_SARADC_THRES1_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 18:30 - saradc thres1 monitor thres"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low(&self) -> APB_SARADC_THRES1_LOW_R {
        APB_SARADC_THRES1_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - configure thres1 to adc channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_channel(&mut self) -> APB_SARADC_THRES1_CHANNEL_W<0> {
        APB_SARADC_THRES1_CHANNEL_W::new(self)
    }
    #[doc = "Bits 5:17 - saradc thres1 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_high(&mut self) -> APB_SARADC_THRES1_HIGH_W<5> {
        APB_SARADC_THRES1_HIGH_W::new(self)
    }
    #[doc = "Bits 18:30 - saradc thres1 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_low(&mut self) -> APB_SARADC_THRES1_LOW_W<18> {
        APB_SARADC_THRES1_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres1_ctrl](index.html) module"]
pub struct THRES1_CTRL_SPEC;
impl crate::RegisterSpec for THRES1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres1_ctrl::R](R) reader structure"]
impl crate::Readable for THRES1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres1_ctrl::W](W) writer structure"]
impl crate::Writable for THRES1_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES1_CTRL to value 0x0003_ffed"]
impl crate::Resettable for THRES1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_ffed;
}
