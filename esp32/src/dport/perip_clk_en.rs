#[doc = "Register `PERIP_CLK_EN` reader"]
pub struct R(crate::R<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN` writer"]
pub struct W(crate::W<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS_CLK_EN` reader - "]
pub type TIMERS_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERS_CLK_EN` writer - "]
pub type TIMERS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `SPI01_CLK_EN` reader - "]
pub type SPI01_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI01_CLK_EN` writer - "]
pub type SPI01_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UART_CLK_EN` reader - "]
pub type UART_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CLK_EN` writer - "]
pub type UART_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `WDG_CLK_EN` reader - "]
pub type WDG_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDG_CLK_EN` writer - "]
pub type WDG_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `I2S0_CLK_EN` reader - "]
pub type I2S0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S0_CLK_EN` writer - "]
pub type I2S0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UART1_CLK_EN` reader - "]
pub type UART1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_CLK_EN` writer - "]
pub type UART1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `SPI2_CLK_EN` reader - "]
pub type SPI2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_CLK_EN` writer - "]
pub type SPI2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `I2C0_EXT0_CLK_EN` reader - "]
pub type I2C0_EXT0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C0_EXT0_CLK_EN` writer - "]
pub type I2C0_EXT0_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UHCI0_CLK_EN` reader - "]
pub type UHCI0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UHCI0_CLK_EN` writer - "]
pub type UHCI0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `RMT_CLK_EN` reader - "]
pub type RMT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `RMT_CLK_EN` writer - "]
pub type RMT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `PCNT_CLK_EN` reader - "]
pub type PCNT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PCNT_CLK_EN` writer - "]
pub type PCNT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `LEDC_CLK_EN` reader - "]
pub type LEDC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_CLK_EN` writer - "]
pub type LEDC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UHCI1_CLK_EN` reader - "]
pub type UHCI1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UHCI1_CLK_EN` writer - "]
pub type UHCI1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - "]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - "]
pub type TIMERGROUP_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `EFUSE_CLK_EN` reader - "]
pub type EFUSE_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_CLK_EN` writer - "]
pub type EFUSE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - "]
pub type TIMERGROUP1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - "]
pub type TIMERGROUP1_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `SPI3_CLK_EN` reader - "]
pub type SPI3_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_CLK_EN` writer - "]
pub type SPI3_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `PWM0_CLK_EN` reader - "]
pub type PWM0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM0_CLK_EN` writer - "]
pub type PWM0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `I2C_EXT1_CLK_EN` reader - "]
pub type I2C_EXT1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EXT1_CLK_EN` writer - "]
pub type I2C_EXT1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `TWAI_CLK_EN` reader - "]
pub type TWAI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TWAI_CLK_EN` writer - "]
pub type TWAI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `PWM1_CLK_EN` reader - "]
pub type PWM1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM1_CLK_EN` writer - "]
pub type PWM1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `I2S1_CLK_EN` reader - "]
pub type I2S1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S1_CLK_EN` writer - "]
pub type I2S1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `SPI_DMA_CLK_EN` reader - "]
pub type SPI_DMA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_CLK_EN` writer - "]
pub type SPI_DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UART2_CLK_EN` reader - "]
pub type UART2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART2_CLK_EN` writer - "]
pub type UART2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `UART_MEM_CLK_EN` reader - "]
pub type UART_MEM_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART_MEM_CLK_EN` writer - "]
pub type UART_MEM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `PWM2_CLK_EN` reader - "]
pub type PWM2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM2_CLK_EN` writer - "]
pub type PWM2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
#[doc = "Field `PWM3_CLK_EN` reader - "]
pub type PWM3_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM3_CLK_EN` writer - "]
pub type PWM3_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIP_CLK_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c0_ext0_clk_en(&self) -> I2C0_EXT0_CLK_EN_R {
        I2C0_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_dma_clk_en(&self) -> SPI_DMA_CLK_EN_R {
        SPI_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W<0> {
        TIMERS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W<1> {
        SPI01_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<2> {
        UART_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W<3> {
        WDG_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W<4> {
        I2S0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<5> {
        UART1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<6> {
        SPI2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c0_ext0_clk_en(&mut self) -> I2C0_EXT0_CLK_EN_W<7> {
        I2C0_EXT0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W<8> {
        UHCI0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<9> {
        RMT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<10> {
        PCNT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<11> {
        LEDC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W<12> {
        UHCI1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W<13> {
        TIMERGROUP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W<14> {
        EFUSE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W<15> {
        TIMERGROUP1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W<16> {
        SPI3_CLK_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W<17> {
        PWM0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W<18> {
        I2C_EXT1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W<19> {
        TWAI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W<20> {
        PWM1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W<21> {
        I2S1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_dma_clk_en(&mut self) -> SPI_DMA_CLK_EN_W<22> {
        SPI_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<23> {
        UART2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W<24> {
        UART_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W<25> {
        PWM2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W<26> {
        PWM3_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en](index.html) module"]
pub struct PERIP_CLK_EN_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf9c1_e06f
    }
}
