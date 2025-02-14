#[doc = "Register `LSTIMER%s_CONF` reader"]
pub struct R(crate::R<LSTIMER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER%s_CONF` writer"]
pub struct W(crate::W<LSTIMER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER_CONF_SPEC>;
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
impl From<crate::W<LSTIMER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_RES` reader - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
pub type DUTY_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUTY_RES` writer - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
pub type DUTY_RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSTIMER_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `DIV_NUM` reader - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIV_NUM` writer - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LSTIMER_CONF_SPEC, u32, u32, 18, O>;
#[doc = "Field `PAUSE` reader - This bit is used to pause the counter in low speed timer0."]
pub type PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE` writer - This bit is used to pause the counter in low speed timer0."]
pub type PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTIMER_CONF_SPEC, bool, O>;
#[doc = "Field `RST` reader - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTIMER_CONF_SPEC, bool, O>;
#[doc = "Field `TICK_SEL` reader - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub type TICK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TICK_SEL` writer - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub type TICK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTIMER_CONF_SPEC, bool, O>;
#[doc = "Field `PARA_UP` reader - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub type PARA_UP_R = crate::BitReader<bool>;
#[doc = "Field `PARA_UP` writer - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub type PARA_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTIMER_CONF_SPEC, bool, O>;
#[doc = "Field `LIM` reader - "]
pub type LIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIM` writer - "]
pub type LIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSTIMER_CONF_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn para_up(&self) -> PARA_UP_R {
        PARA_UP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lim(&self) -> LIM_R {
        LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn duty_res(&mut self) -> DUTY_RES_W<0> {
        DUTY_RES_W::new(self)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num(&mut self) -> DIV_NUM_W<5> {
        DIV_NUM_W::new(self)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W<23> {
        PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<24> {
        RST_W::new(self)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<25> {
        TICK_SEL_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn para_up(&mut self) -> PARA_UP_W<26> {
        PARA_UP_W::new(self)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lim(&mut self) -> LIM_W<31> {
        LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer_conf](index.html) module"]
pub struct LSTIMER_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer_conf::R](R) reader structure"]
impl crate::Readable for LSTIMER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer_conf::W](W) writer structure"]
impl crate::Writable for LSTIMER_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER%s_CONF to value 0x0100_0000"]
impl crate::Resettable for LSTIMER_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
