#[doc = "Register `U%s_CONF0` reader"]
pub struct R(crate::R<U_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U%s_CONF0` writer"]
pub struct W(crate::W<U_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U_CONF0_SPEC>;
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
impl From<crate::W<U_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_THRES_U` reader - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub type FILTER_THRES_U_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FILTER_THRES_U` writer - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub type FILTER_THRES_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u16, u16, 10, O>;
#[doc = "Field `FILTER_EN_U` reader - This is the enable bit for unit %s's input filter."]
pub type FILTER_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_EN_U` writer - This is the enable bit for unit %s's input filter."]
pub type FILTER_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `THR_ZERO_EN_U` reader - This is the enable bit for unit %s's zero comparator."]
pub type THR_ZERO_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `THR_ZERO_EN_U` writer - This is the enable bit for unit %s's zero comparator."]
pub type THR_ZERO_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `THR_H_LIM_EN_U` reader - This is the enable bit for unit %s's thr_h_lim comparator."]
pub type THR_H_LIM_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `THR_H_LIM_EN_U` writer - This is the enable bit for unit %s's thr_h_lim comparator."]
pub type THR_H_LIM_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `THR_L_LIM_EN_U` reader - This is the enable bit for unit %s's thr_l_lim comparator."]
pub type THR_L_LIM_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `THR_L_LIM_EN_U` writer - This is the enable bit for unit %s's thr_l_lim comparator."]
pub type THR_L_LIM_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `THR_THRES0_EN_U` reader - This is the enable bit for unit %s's thres0 comparator."]
pub type THR_THRES0_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `THR_THRES0_EN_U` writer - This is the enable bit for unit %s's thres0 comparator."]
pub type THR_THRES0_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `THR_THRES1_EN_U` reader - This is the enable bit for unit %s's thres1 comparator."]
pub type THR_THRES1_EN_U_R = crate::BitReader<bool>;
#[doc = "Field `THR_THRES1_EN_U` writer - This is the enable bit for unit %s's thres1 comparator."]
pub type THR_THRES1_EN_U_W<'a, const O: u8> = crate::BitWriter<'a, u32, U_CONF0_SPEC, bool, O>;
#[doc = "Field `CH0_NEG_MODE_U` reader - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
pub type CH0_NEG_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_NEG_MODE_U` writer - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
pub type CH0_NEG_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH0_POS_MODE_U` reader - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
pub type CH0_POS_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_POS_MODE_U` writer - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
pub type CH0_POS_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH0_HCTRL_MODE_U` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH0_HCTRL_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_HCTRL_MODE_U` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH0_HCTRL_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH0_LCTRL_MODE_U` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH0_LCTRL_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_LCTRL_MODE_U` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH0_LCTRL_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1_NEG_MODE_U` reader - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
pub type CH1_NEG_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_NEG_MODE_U` writer - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
pub type CH1_NEG_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1_POS_MODE_U` reader - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
pub type CH1_POS_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_POS_MODE_U` writer - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
pub type CH1_POS_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1_HCTRL_MODE_U` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH1_HCTRL_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_HCTRL_MODE_U` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH1_HCTRL_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1_LCTRL_MODE_U` reader - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH1_LCTRL_MODE_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_LCTRL_MODE_U` writer - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
pub type CH1_LCTRL_MODE_U_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, U_CONF0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres_u(&self) -> FILTER_THRES_U_R {
        FILTER_THRES_U_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u(&self) -> FILTER_EN_U_R {
        FILTER_EN_U_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u(&self) -> THR_ZERO_EN_U_R {
        THR_ZERO_EN_U_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator."]
    #[inline(always)]
    pub fn thr_h_lim_en_u(&self) -> THR_H_LIM_EN_U_R {
        THR_H_LIM_EN_U_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator."]
    #[inline(always)]
    pub fn thr_l_lim_en_u(&self) -> THR_L_LIM_EN_U_R {
        THR_L_LIM_EN_U_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u(&self) -> THR_THRES0_EN_U_R {
        THR_THRES0_EN_U_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u(&self) -> THR_THRES1_EN_U_R {
        THR_THRES1_EN_U_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch0_neg_mode_u(&self) -> CH0_NEG_MODE_U_R {
        CH0_NEG_MODE_U_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch0_pos_mode_u(&self) -> CH0_POS_MODE_U_R {
        CH0_POS_MODE_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&self) -> CH0_HCTRL_MODE_U_R {
        CH0_HCTRL_MODE_U_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&self) -> CH0_LCTRL_MODE_U_R {
        CH0_LCTRL_MODE_U_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch1_neg_mode_u(&self) -> CH1_NEG_MODE_U_R {
        CH1_NEG_MODE_U_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch1_pos_mode_u(&self) -> CH1_POS_MODE_U_R {
        CH1_POS_MODE_U_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&self) -> CH1_HCTRL_MODE_U_R {
        CH1_HCTRL_MODE_U_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&self) -> CH1_LCTRL_MODE_U_R {
        CH1_LCTRL_MODE_U_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres_u(&mut self) -> FILTER_THRES_U_W<0> {
        FILTER_THRES_U_W::new(self)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en_u(&mut self) -> FILTER_EN_U_W<10> {
        FILTER_EN_U_W::new(self)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en_u(&mut self) -> THR_ZERO_EN_U_W<11> {
        THR_ZERO_EN_U_W::new(self)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator."]
    #[inline(always)]
    pub fn thr_h_lim_en_u(&mut self) -> THR_H_LIM_EN_U_W<12> {
        THR_H_LIM_EN_U_W::new(self)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator."]
    #[inline(always)]
    pub fn thr_l_lim_en_u(&mut self) -> THR_L_LIM_EN_U_W<13> {
        THR_L_LIM_EN_U_W::new(self)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en_u(&mut self) -> THR_THRES0_EN_U_W<14> {
        THR_THRES0_EN_U_W::new(self)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en_u(&mut self) -> THR_THRES1_EN_U_W<15> {
        THR_THRES1_EN_U_W::new(self)
    }
    #[doc = "Bits 16:17 - This register sets the behavior when the signal input of channel 0 detects a negative edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch0_neg_mode_u(&mut self) -> CH0_NEG_MODE_U_W<16> {
        CH0_NEG_MODE_U_W::new(self)
    }
    #[doc = "Bits 18:19 - This register sets the behavior when the signal input of channel 0 detects a positive edge. 1: Increase the counter;2: Decrease the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch0_pos_mode_u(&mut self) -> CH0_POS_MODE_U_W<18> {
        CH0_POS_MODE_U_W::new(self)
    }
    #[doc = "Bits 20:21 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u(&mut self) -> CH0_HCTRL_MODE_U_W<20> {
        CH0_HCTRL_MODE_U_W::new(self)
    }
    #[doc = "Bits 22:23 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u(&mut self) -> CH0_LCTRL_MODE_U_W<22> {
        CH0_LCTRL_MODE_U_W::new(self)
    }
    #[doc = "Bits 24:25 - This register sets the behavior when the signal input of channel 1 detects a negative edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch1_neg_mode_u(&mut self) -> CH1_NEG_MODE_U_W<24> {
        CH1_NEG_MODE_U_W::new(self)
    }
    #[doc = "Bits 26:27 - This register sets the behavior when the signal input of channel 1 detects a positive edge. 1: Increment the counter;2: Decrement the counter;0, 3: No effect on counter"]
    #[inline(always)]
    pub fn ch1_pos_mode_u(&mut self) -> CH1_POS_MODE_U_W<26> {
        CH1_POS_MODE_U_W::new(self)
    }
    #[doc = "Bits 28:29 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is high. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u(&mut self) -> CH1_HCTRL_MODE_U_W<28> {
        CH1_HCTRL_MODE_U_W::new(self)
    }
    #[doc = "Bits 30:31 - This register configures how the CH%s_POS_MODE/CH%s_NEG_MODE settings will be modified when the control signal is low. 0: No modification;1: Invert behavior (increase -> decrease, decrease -> increase);2, 3: Inhibit counter modification"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u(&mut self) -> CH1_LCTRL_MODE_U_W<30> {
        CH1_LCTRL_MODE_U_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 0 for unit %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_conf0](index.html) module"]
pub struct U_CONF0_SPEC;
impl crate::RegisterSpec for U_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_conf0::R](R) reader structure"]
impl crate::Readable for U_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u_conf0::W](W) writer structure"]
impl crate::Writable for U_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U%s_CONF0 to value 0x3c10"]
impl crate::Resettable for U_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c10
    }
}
