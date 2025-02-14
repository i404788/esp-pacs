#[doc = "Register `HP_MODEM_SYSCLK` reader"]
pub struct R(crate::R<HP_MODEM_SYSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_MODEM_SYSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_MODEM_SYSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_MODEM_SYSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_MODEM_SYSCLK` writer"]
pub struct W(crate::W<HP_MODEM_SYSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_MODEM_SYSCLK_SPEC>;
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
impl From<crate::W<HP_MODEM_SYSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_MODEM_SYSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_NO_DIV` reader - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_NO_DIV_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_NO_DIV` writer - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_NO_DIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_MODEM_SYSCLK_SPEC, bool, O>;
#[doc = "Field `HP_MODEM_ICG_SYS_CLOCK_EN` reader - need_des"]
pub type HP_MODEM_ICG_SYS_CLOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM_ICG_SYS_CLOCK_EN` writer - need_des"]
pub type HP_MODEM_ICG_SYS_CLOCK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_MODEM_SYSCLK_SPEC, bool, O>;
#[doc = "Field `HP_MODEM_SYS_CLK_SLP_SEL` reader - need_des"]
pub type HP_MODEM_SYS_CLK_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM_SYS_CLK_SLP_SEL` writer - need_des"]
pub type HP_MODEM_SYS_CLK_SLP_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_MODEM_SYSCLK_SPEC, bool, O>;
#[doc = "Field `HP_MODEM_ICG_SLP_SEL` reader - need_des"]
pub type HP_MODEM_ICG_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `HP_MODEM_ICG_SLP_SEL` writer - need_des"]
pub type HP_MODEM_ICG_SLP_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_MODEM_SYSCLK_SPEC, bool, O>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_SEL` reader - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_SEL` writer - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_MODEM_SYSCLK_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_sys_clk_no_div(&self) -> HP_MODEM_DIG_SYS_CLK_NO_DIV_R {
        HP_MODEM_DIG_SYS_CLK_NO_DIV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_icg_sys_clock_en(&self) -> HP_MODEM_ICG_SYS_CLOCK_EN_R {
        HP_MODEM_ICG_SYS_CLOCK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_modem_sys_clk_slp_sel(&self) -> HP_MODEM_SYS_CLK_SLP_SEL_R {
        HP_MODEM_SYS_CLK_SLP_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_icg_slp_sel(&self) -> HP_MODEM_ICG_SLP_SEL_R {
        HP_MODEM_ICG_SLP_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_sys_clk_sel(&self) -> HP_MODEM_DIG_SYS_CLK_SEL_R {
        HP_MODEM_DIG_SYS_CLK_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_sys_clk_no_div(&mut self) -> HP_MODEM_DIG_SYS_CLK_NO_DIV_W<26> {
        HP_MODEM_DIG_SYS_CLK_NO_DIV_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_icg_sys_clock_en(&mut self) -> HP_MODEM_ICG_SYS_CLOCK_EN_W<27> {
        HP_MODEM_ICG_SYS_CLOCK_EN_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_sys_clk_slp_sel(&mut self) -> HP_MODEM_SYS_CLK_SLP_SEL_W<28> {
        HP_MODEM_SYS_CLK_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_icg_slp_sel(&mut self) -> HP_MODEM_ICG_SLP_SEL_W<29> {
        HP_MODEM_ICG_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_sys_clk_sel(&mut self) -> HP_MODEM_DIG_SYS_CLK_SEL_W<30> {
        HP_MODEM_DIG_SYS_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_modem_sysclk](index.html) module"]
pub struct HP_MODEM_SYSCLK_SPEC;
impl crate::RegisterSpec for HP_MODEM_SYSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_modem_sysclk::R](R) reader structure"]
impl crate::Readable for HP_MODEM_SYSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_modem_sysclk::W](W) writer structure"]
impl crate::Writable for HP_MODEM_SYSCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_SYSCLK to value 0"]
impl crate::Resettable for HP_MODEM_SYSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
