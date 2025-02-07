#[doc = "Register `OPTIONS0` reader"]
pub struct R(crate::R<OPTIONS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS0` writer"]
pub struct W(crate::W<OPTIONS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS0_SPEC>;
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
impl From<crate::W<OPTIONS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_PROCPU_C0` reader - {reg_sw_stall_procpu_c1\\[5:0\\], reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub type SW_STALL_PROCPU_C0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_STALL_PROCPU_C0` writer - {reg_sw_stall_procpu_c1\\[5:0\\], reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub type SW_STALL_PROCPU_C0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTIONS0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SW_PROCPU_RST` reader - PRO CPU SW reset"]
pub type SW_PROCPU_RST_R = crate::BitReader<bool>;
#[doc = "Field `SW_PROCPU_RST` writer - PRO CPU SW reset"]
pub type SW_PROCPU_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BB_I2C_FORCE_PD` reader - BB_I2C force power down"]
pub type BB_I2C_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `BB_I2C_FORCE_PD` writer - BB_I2C force power down"]
pub type BB_I2C_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BB_I2C_FORCE_PU` reader - BB_I2C force power up"]
pub type BB_I2C_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `BB_I2C_FORCE_PU` writer - BB_I2C force power up"]
pub type BB_I2C_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BBPLL_I2C_FORCE_PD` reader - BB_PLL _I2C force power down"]
pub type BBPLL_I2C_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_I2C_FORCE_PD` writer - BB_PLL _I2C force power down"]
pub type BBPLL_I2C_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BBPLL_I2C_FORCE_PU` reader - BB_PLL_I2C force power up"]
pub type BBPLL_I2C_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_I2C_FORCE_PU` writer - BB_PLL_I2C force power up"]
pub type BBPLL_I2C_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BBPLL_FORCE_PD` reader - BB_PLL force power down"]
pub type BBPLL_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_FORCE_PD` writer - BB_PLL force power down"]
pub type BBPLL_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `BBPLL_FORCE_PU` reader - BB_PLL force power up"]
pub type BBPLL_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `BBPLL_FORCE_PU` writer - BB_PLL force power up"]
pub type BBPLL_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `XTL_FORCE_PD` reader - crystall force power down"]
pub type XTL_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `XTL_FORCE_PD` writer - crystall force power down"]
pub type XTL_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `XTL_FORCE_PU` reader - crystall force power up"]
pub type XTL_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `XTL_FORCE_PU` writer - crystall force power up"]
pub type XTL_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `XTL_EN_WAIT` reader - wait bias_sleep and current source wakeup"]
pub type XTL_EN_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTL_EN_WAIT` writer - wait bias_sleep and current source wakeup"]
pub type XTL_EN_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTIONS0_SPEC, u8, u8, 4, O>;
#[doc = "Field `XTL_EXT_CTR_SEL` reader - Need add desc"]
pub type XTL_EXT_CTR_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTL_EXT_CTR_SEL` writer - Need add desc"]
pub type XTL_EXT_CTR_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTIONS0_SPEC, u8, u8, 3, O>;
#[doc = "Field `ANALOG_FORCE_ISO` reader - Need add desc"]
pub type ANALOG_FORCE_ISO_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_FORCE_ISO` writer - Need add desc"]
pub type ANALOG_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `ANALOG_FORCE_NOISO` reader - Need add desc"]
pub type ANALOG_FORCE_NOISO_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_FORCE_NOISO` writer - Need add desc"]
pub type ANALOG_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `DG_WRAP_FORCE_RST` reader - digital wrap force reset in deep sleep"]
pub type DG_WRAP_FORCE_RST_R = crate::BitReader<bool>;
#[doc = "Field `DG_WRAP_FORCE_RST` writer - digital wrap force reset in deep sleep"]
pub type DG_WRAP_FORCE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `DG_WRAP_FORCE_NORST` reader - digital core force no reset in deep sleep"]
pub type DG_WRAP_FORCE_NORST_R = crate::BitReader<bool>;
#[doc = "Field `DG_WRAP_FORCE_NORST` writer - digital core force no reset in deep sleep"]
pub type DG_WRAP_FORCE_NORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
#[doc = "Field `SW_SYS_RST` reader - SW system reset"]
pub type SW_SYS_RST_R = crate::BitReader<bool>;
#[doc = "Field `SW_SYS_RST` writer - SW system reset"]
pub type SW_SYS_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\], reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&self) -> SW_PROCPU_RST_R {
        SW_PROCPU_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - wait bias_sleep and current source wakeup"]
    #[inline(always)]
    pub fn xtl_en_wait(&self) -> XTL_EN_WAIT_R {
        XTL_EN_WAIT_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&self) -> XTL_EXT_CTR_SEL_R {
        XTL_EXT_CTR_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 25 - Need add desc"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn sw_sys_rst(&self) -> SW_SYS_RST_R {
        SW_SYS_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\], reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W<2> {
        SW_STALL_PROCPU_C0_W::new(self)
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W<5> {
        SW_PROCPU_RST_W::new(self)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W<6> {
        BB_I2C_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W<7> {
        BB_I2C_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W<8> {
        BBPLL_I2C_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W<9> {
        BBPLL_I2C_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W<10> {
        BBPLL_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W<11> {
        BBPLL_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W<12> {
        XTL_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W<13> {
        XTL_FORCE_PU_W::new(self)
    }
    #[doc = "Bits 14:17 - wait bias_sleep and current source wakeup"]
    #[inline(always)]
    pub fn xtl_en_wait(&mut self) -> XTL_EN_WAIT_W<14> {
        XTL_EN_WAIT_W::new(self)
    }
    #[doc = "Bits 20:22 - Need add desc"]
    #[inline(always)]
    pub fn xtl_ext_ctr_sel(&mut self) -> XTL_EXT_CTR_SEL_W<20> {
        XTL_EXT_CTR_SEL_W::new(self)
    }
    #[doc = "Bit 25 - Need add desc"]
    #[inline(always)]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W<25> {
        ANALOG_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W<28> {
        ANALOG_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W<29> {
        DG_WRAP_FORCE_RST_W::new(self)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W<30> {
        DG_WRAP_FORCE_NORST_W::new(self)
    }
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W<31> {
        SW_SYS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options0](index.html) module"]
pub struct OPTIONS0_SPEC;
impl crate::RegisterSpec for OPTIONS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options0::R](R) reader structure"]
impl crate::Readable for OPTIONS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options0::W](W) writer structure"]
impl crate::Writable for OPTIONS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTIONS0 to value 0x1000_a000"]
impl crate::Resettable for OPTIONS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_a000
    }
}
