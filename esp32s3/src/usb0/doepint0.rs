#[doc = "Register `DOEPINT0` reader"]
pub struct R(crate::R<DOEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT0` writer"]
pub struct W(crate::W<DOEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT0_SPEC>;
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
impl From<crate::W<DOEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL0` reader - "]
pub type XFERCOMPL0_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL0` writer - "]
pub type XFERCOMPL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `EPDISBLD0` reader - "]
pub type EPDISBLD0_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLD0` writer - "]
pub type EPDISBLD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `AHBERR0` reader - "]
pub type AHBERR0_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR0` writer - "]
pub type AHBERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `SETUP0` reader - "]
pub type SETUP0_R = crate::BitReader<bool>;
#[doc = "Field `SETUP0` writer - "]
pub type SETUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `OUTTKNEPDIS0` reader - "]
pub type OUTTKNEPDIS0_R = crate::BitReader<bool>;
#[doc = "Field `OUTTKNEPDIS0` writer - "]
pub type OUTTKNEPDIS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `STSPHSERCVD0` reader - "]
pub type STSPHSERCVD0_R = crate::BitReader<bool>;
#[doc = "Field `STSPHSERCVD0` writer - "]
pub type STSPHSERCVD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `BACK2BACKSETUP0` reader - "]
pub type BACK2BACKSETUP0_R = crate::BitReader<bool>;
#[doc = "Field `BACK2BACKSETUP0` writer - "]
pub type BACK2BACKSETUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `OUTPKTERR0` reader - "]
pub type OUTPKTERR0_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTERR0` writer - "]
pub type OUTPKTERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `BNAINTR0` reader - "]
pub type BNAINTR0_R = crate::BitReader<bool>;
#[doc = "Field `BNAINTR0` writer - "]
pub type BNAINTR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `PKTDRPSTS0` reader - "]
pub type PKTDRPSTS0_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS0` writer - "]
pub type PKTDRPSTS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `BBLEERR0` reader - "]
pub type BBLEERR0_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERR0` writer - "]
pub type BBLEERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `NAKINTRPT0` reader - "]
pub type NAKINTRPT0_R = crate::BitReader<bool>;
#[doc = "Field `NAKINTRPT0` writer - "]
pub type NAKINTRPT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `NYEPINTRPT0` reader - "]
pub type NYEPINTRPT0_R = crate::BitReader<bool>;
#[doc = "Field `NYEPINTRPT0` writer - "]
pub type NYEPINTRPT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
#[doc = "Field `STUPPKTRCVD0` reader - "]
pub type STUPPKTRCVD0_R = crate::BitReader<bool>;
#[doc = "Field `STUPPKTRCVD0` writer - "]
pub type STUPPKTRCVD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl0(&self) -> XFERCOMPL0_R {
        XFERCOMPL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld0(&self) -> EPDISBLD0_R {
        EPDISBLD0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr0(&self) -> AHBERR0_R {
        AHBERR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup0(&self) -> SETUP0_R {
        SETUP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis0(&self) -> OUTTKNEPDIS0_R {
        OUTTKNEPDIS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd0(&self) -> STSPHSERCVD0_R {
        STSPHSERCVD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup0(&self) -> BACK2BACKSETUP0_R {
        BACK2BACKSETUP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr0(&self) -> OUTPKTERR0_R {
        OUTPKTERR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr0(&self) -> BNAINTR0_R {
        BNAINTR0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts0(&self) -> PKTDRPSTS0_R {
        PKTDRPSTS0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr0(&self) -> BBLEERR0_R {
        BBLEERR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt0(&self) -> NAKINTRPT0_R {
        NAKINTRPT0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt0(&self) -> NYEPINTRPT0_R {
        NYEPINTRPT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd0(&self) -> STUPPKTRCVD0_R {
        STUPPKTRCVD0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl0(&mut self) -> XFERCOMPL0_W<0> {
        XFERCOMPL0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld0(&mut self) -> EPDISBLD0_W<1> {
        EPDISBLD0_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr0(&mut self) -> AHBERR0_W<2> {
        AHBERR0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup0(&mut self) -> SETUP0_W<3> {
        SETUP0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis0(&mut self) -> OUTTKNEPDIS0_W<4> {
        OUTTKNEPDIS0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd0(&mut self) -> STSPHSERCVD0_W<5> {
        STSPHSERCVD0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup0(&mut self) -> BACK2BACKSETUP0_W<6> {
        BACK2BACKSETUP0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr0(&mut self) -> OUTPKTERR0_W<8> {
        OUTPKTERR0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr0(&mut self) -> BNAINTR0_W<9> {
        BNAINTR0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts0(&mut self) -> PKTDRPSTS0_W<11> {
        PKTDRPSTS0_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr0(&mut self) -> BBLEERR0_W<12> {
        BBLEERR0_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt0(&mut self) -> NAKINTRPT0_W<13> {
        NAKINTRPT0_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt0(&mut self) -> NYEPINTRPT0_W<14> {
        NYEPINTRPT0_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd0(&mut self) -> STUPPKTRCVD0_W<15> {
        STUPPKTRCVD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint0](index.html) module"]
pub struct DOEPINT0_SPEC;
impl crate::RegisterSpec for DOEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint0::R](R) reader structure"]
impl crate::Readable for DOEPINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint0::W](W) writer structure"]
impl crate::Writable for DOEPINT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT0 to value 0"]
impl crate::Resettable for DOEPINT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
