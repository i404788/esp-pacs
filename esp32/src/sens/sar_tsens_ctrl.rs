#[doc = "Register `SAR_TSENS_CTRL` reader"]
pub struct R(crate::R<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TSENS_CTRL` writer"]
pub struct W(crate::W<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TSENS_CTRL_SPEC>;
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
impl From<crate::W<SAR_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_XPD_WAIT` reader - "]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - "]
pub type TSENS_XPD_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL_SPEC, u16, u16, 12, O>;
#[doc = "Field `TSENS_XPD_FORCE` reader - "]
pub type TSENS_XPD_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_XPD_FORCE` writer - "]
pub type TSENS_XPD_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_CLK_INV` reader - "]
pub type TSENS_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLK_INV` writer - "]
pub type TSENS_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_CLK_GATED` reader - "]
pub type TSENS_CLK_GATED_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLK_GATED` writer - "]
pub type TSENS_CLK_GATED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_IN_INV` reader - invert temperature sensor data"]
pub type TSENS_IN_INV_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_IN_INV` writer - invert temperature sensor data"]
pub type TSENS_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `TSENS_POWER_UP` reader - temperature sensor power up"]
pub type TSENS_POWER_UP_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_POWER_UP` writer - temperature sensor power up"]
pub type TSENS_POWER_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_POWER_UP_FORCE` reader - 1: dump out & power up controlled by SW 0: by FSM"]
pub type TSENS_POWER_UP_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_POWER_UP_FORCE` writer - 1: dump out & power up controlled by SW 0: by FSM"]
pub type TSENS_POWER_UP_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
#[doc = "Field `TSENS_DUMP_OUT` reader - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type TSENS_DUMP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_DUMP_OUT` writer - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type TSENS_DUMP_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_gated(&self) -> TSENS_CLK_GATED_R {
        TSENS_CLK_GATED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: dump out & power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<0> {
        TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<12> {
        TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<13> {
        TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_gated(&mut self) -> TSENS_CLK_GATED_W<14> {
        TSENS_CLK_GATED_W::new(self)
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<15> {
        TSENS_IN_INV_W::new(self)
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<16> {
        TSENS_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W<24> {
        TSENS_POWER_UP_W::new(self)
    }
    #[doc = "Bit 25 - 1: dump out & power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W<25> {
        TSENS_POWER_UP_FORCE_W::new(self)
    }
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W<26> {
        TSENS_DUMP_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl](index.html) module"]
pub struct SAR_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tsens_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL to value 0x0006_6002"]
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0006_6002
    }
}
