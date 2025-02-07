#[doc = "Register `SUS_STATUS` reader"]
pub struct R(crate::R<SUS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUS_STATUS` writer"]
pub struct W(crate::W<SUS_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUS_STATUS_SPEC>;
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
impl From<crate::W<SUS_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUS_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_SUS` reader - The status of flash suspend, only used in SPI1."]
pub type FLASH_SUS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_SUS` writer - The status of flash suspend, only used in SPI1."]
pub type FLASH_SUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `WAIT_PESR_CMD_2B` reader - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
pub type WAIT_PESR_CMD_2B_R = crate::BitReader<bool>;
#[doc = "Field `WAIT_PESR_CMD_2B` writer - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
pub type WAIT_PESR_CMD_2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `FLASH_HPM_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type FLASH_HPM_DLY_128_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_HPM_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
pub type FLASH_HPM_DLY_128_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `FLASH_RES_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type FLASH_RES_DLY_128_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_RES_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
pub type FLASH_RES_DLY_128_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `FLASH_DP_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type FLASH_DP_DLY_128_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_DP_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
pub type FLASH_DP_DLY_128_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `FLASH_PER_DLY_128` reader - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type FLASH_PER_DLY_128_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PER_DLY_128` writer - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
pub type FLASH_PER_DLY_128_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `FLASH_PES_DLY_128` reader - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type FLASH_PES_DLY_128_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PES_DLY_128` writer - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
pub type FLASH_PES_DLY_128_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
#[doc = "Field `SPI0_LOCK_EN` reader - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
pub type SPI0_LOCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI0_LOCK_EN` writer - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
pub type SPI0_LOCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUS_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The status of flash suspend, only used in SPI1."]
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&self) -> WAIT_PESR_CMD_2B_R {
        WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&self) -> FLASH_HPM_DLY_128_R {
        FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    pub fn flash_res_dly_128(&self) -> FLASH_RES_DLY_128_R {
        FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    pub fn flash_dp_dly_128(&self) -> FLASH_DP_DLY_128_R {
        FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    pub fn flash_per_dly_128(&self) -> FLASH_PER_DLY_128_R {
        FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    pub fn flash_pes_dly_128(&self) -> FLASH_PES_DLY_128_R {
        FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
    #[inline(always)]
    pub fn spi0_lock_en(&self) -> SPI0_LOCK_EN_R {
        SPI0_LOCK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The status of flash suspend, only used in SPI1."]
    #[inline(always)]
    pub fn flash_sus(&mut self) -> FLASH_SUS_W<0> {
        FLASH_SUS_W::new(self)
    }
    #[doc = "Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&mut self) -> WAIT_PESR_CMD_2B_W<1> {
        WAIT_PESR_CMD_2B_W::new(self)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&mut self) -> FLASH_HPM_DLY_128_W<2> {
        FLASH_HPM_DLY_128_W::new(self)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    #[inline(always)]
    pub fn flash_res_dly_128(&mut self) -> FLASH_RES_DLY_128_W<3> {
        FLASH_RES_DLY_128_W::new(self)
    }
    #[doc = "Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    #[inline(always)]
    pub fn flash_dp_dly_128(&mut self) -> FLASH_DP_DLY_128_W<4> {
        FLASH_DP_DLY_128_W::new(self)
    }
    #[doc = "Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    #[inline(always)]
    pub fn flash_per_dly_128(&mut self) -> FLASH_PER_DLY_128_W<5> {
        FLASH_PER_DLY_128_W::new(self)
    }
    #[doc = "Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    #[inline(always)]
    pub fn flash_pes_dly_128(&mut self) -> FLASH_PES_DLY_128_W<6> {
        FLASH_PES_DLY_128_W::new(self)
    }
    #[doc = "Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
    #[inline(always)]
    pub fn spi0_lock_en(&mut self) -> SPI0_LOCK_EN_W<7> {
        SPI0_LOCK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sus_status](index.html) module"]
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sus_status::R](R) reader structure"]
impl crate::Readable for SUS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sus_status::W](W) writer structure"]
impl crate::Writable for SUS_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUS_STATUS to value 0"]
impl crate::Resettable for SUS_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
