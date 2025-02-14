#[doc = "Register `LCD_CTRL` reader"]
pub struct R(crate::R<LCD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CTRL` writer"]
pub struct W(crate::W<LCD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CTRL_SPEC>;
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
impl From<crate::W<LCD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_HB_FRONT` reader - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_HB_FRONT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_HB_FRONT` writer - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
pub type LCD_HB_FRONT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL_SPEC, u16, u16, 11, O>;
#[doc = "Field `LCD_VA_HEIGHT` reader - It is the vertical active height of a frame. Can be configured in CONF state."]
pub type LCD_VA_HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_VA_HEIGHT` writer - It is the vertical active height of a frame. Can be configured in CONF state."]
pub type LCD_VA_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL_SPEC, u16, u16, 10, O>;
#[doc = "Field `LCD_VT_HEIGHT` reader - It is the vertical total height of a frame. Can be configured in CONF state."]
pub type LCD_VT_HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_VT_HEIGHT` writer - It is the vertical total height of a frame. Can be configured in CONF state."]
pub type LCD_VT_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL_SPEC, u16, u16, 10, O>;
#[doc = "Field `LCD_MODE_EN` reader - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
pub type LCD_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_MODE_EN` writer - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
pub type LCD_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> LCD_HB_FRONT_R {
        LCD_HB_FRONT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_va_height(&self) -> LCD_VA_HEIGHT_R {
        LCD_VA_HEIGHT_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> LCD_VT_HEIGHT_R {
        LCD_VT_HEIGHT_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_mode_en(&self) -> LCD_MODE_EN_R {
        LCD_MODE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_hb_front(&mut self) -> LCD_HB_FRONT_W<0> {
        LCD_HB_FRONT_W::new(self)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_va_height(&mut self) -> LCD_VA_HEIGHT_W<11> {
        LCD_VA_HEIGHT_W::new(self)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_vt_height(&mut self) -> LCD_VT_HEIGHT_W<21> {
        LCD_VT_HEIGHT_W::new(self)
    }
    #[doc = "Bit 31 - 1: Enable LCD mode output vsync, hsync, de. 0: Disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lcd_mode_en(&mut self) -> LCD_MODE_EN_W<31> {
        LCD_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD frame control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ctrl](index.html) module"]
pub struct LCD_CTRL_SPEC;
impl crate::RegisterSpec for LCD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_ctrl::R](R) reader structure"]
impl crate::Readable for LCD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_ctrl::W](W) writer structure"]
impl crate::Writable for LCD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_CTRL to value 0"]
impl crate::Resettable for LCD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
