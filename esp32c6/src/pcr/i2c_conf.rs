#[doc = "Register `I2C_CONF` reader"]
pub struct R(crate::R<I2C_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CONF` writer"]
pub struct W(crate::W<I2C_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CONF_SPEC>;
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
impl From<crate::W<I2C_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_CLK_EN` reader - Set 1 to enable i2c apb clock"]
pub type I2C_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_CLK_EN` writer - Set 1 to enable i2c apb clock"]
pub type I2C_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONF_SPEC, bool, O>;
#[doc = "Field `I2C_RST_EN` reader - Set 0 to reset i2c module"]
pub type I2C_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_RST_EN` writer - Set 0 to reset i2c module"]
pub type I2C_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable i2c apb clock"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset i2c module"]
    #[inline(always)]
    pub fn i2c_rst_en(&self) -> I2C_RST_EN_R {
        I2C_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable i2c apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<0> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset i2c module"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rst_en(&mut self) -> I2C_RST_EN_W<1> {
        I2C_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_conf](index.html) module"]
pub struct I2C_CONF_SPEC;
impl crate::RegisterSpec for I2C_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_conf::R](R) reader structure"]
impl crate::Readable for I2C_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_conf::W](W) writer structure"]
impl crate::Writable for I2C_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CONF to value 0x01"]
impl crate::Resettable for I2C_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
