#[doc = "Register `DOUT_NUM` reader"]
pub struct R(crate::R<DOUT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT_NUM` writer"]
pub struct W(crate::W<DOUT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT_NUM_SPEC>;
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
impl From<crate::W<DOUT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT0_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT0_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT0_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT1_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT1_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT1_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT1_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT2_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT2_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT2_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT2_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT3_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT3_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT3_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT3_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT4_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT4_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT4_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT4_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT5_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT5_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT5_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT5_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT6_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT6_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT6_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT6_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
#[doc = "Field `DOUT7_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT7_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUT7_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT7_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUT_NUM_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&self) -> DOUT0_NUM_R {
        DOUT0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&self) -> DOUT1_NUM_R {
        DOUT1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&self) -> DOUT2_NUM_R {
        DOUT2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&self) -> DOUT3_NUM_R {
        DOUT3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&self) -> DOUT4_NUM_R {
        DOUT4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&self) -> DOUT5_NUM_R {
        DOUT5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&self) -> DOUT6_NUM_R {
        DOUT6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&self) -> DOUT7_NUM_R {
        DOUT7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&mut self) -> DOUT0_NUM_W<0> {
        DOUT0_NUM_W::new(self)
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&mut self) -> DOUT1_NUM_W<2> {
        DOUT1_NUM_W::new(self)
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&mut self) -> DOUT2_NUM_W<4> {
        DOUT2_NUM_W::new(self)
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&mut self) -> DOUT3_NUM_W<6> {
        DOUT3_NUM_W::new(self)
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&mut self) -> DOUT4_NUM_W<8> {
        DOUT4_NUM_W::new(self)
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&mut self) -> DOUT5_NUM_W<10> {
        DOUT5_NUM_W::new(self)
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&mut self) -> DOUT6_NUM_W<12> {
        DOUT6_NUM_W::new(self)
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&mut self) -> DOUT7_NUM_W<14> {
        DOUT7_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI output delay number configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_num](index.html) module"]
pub struct DOUT_NUM_SPEC;
impl crate::RegisterSpec for DOUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout_num::R](R) reader structure"]
impl crate::Readable for DOUT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout_num::W](W) writer structure"]
impl crate::Writable for DOUT_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUT_NUM to value 0"]
impl crate::Resettable for DOUT_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
