#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CMDINDEX` reader - desc CMDINDEX"]
pub type CmdindexR = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - desc CMDINDEX"]
pub type CmdindexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WAITRESP` reader - desc WAITRESP"]
pub type WaitrespR = crate::BitReader;
#[doc = "Field `WAITRESP` writer - desc WAITRESP"]
pub type WaitrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPLEN` reader - desc RESPLEN"]
pub type ResplenR = crate::BitReader;
#[doc = "Field `RESPLEN` writer - desc RESPLEN"]
pub type ResplenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECKRESPCRC` reader - desc CHECKRESPCRC"]
pub type CheckrespcrcR = crate::BitReader;
#[doc = "Field `CHECKRESPCRC` writer - desc CHECKRESPCRC"]
pub type CheckrespcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEXPECT` reader - desc DEXPECT"]
pub type DexpectR = crate::BitReader;
#[doc = "Field `DEXPECT` writer - desc DEXPECT"]
pub type DexpectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - desc DTMODE"]
pub type DtmodeR = crate::BitReader;
#[doc = "Field `DTMODE` writer - desc DTMODE"]
pub type DtmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTOP` reader - desc AUTOSTOP"]
pub type AutostopR = crate::BitReader;
#[doc = "Field `AUTOSTOP` writer - desc AUTOSTOP"]
pub type AutostopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPEND` reader - desc WAITPEND"]
pub type WaitpendR = crate::BitReader;
#[doc = "Field `WAITPEND` writer - desc WAITPEND"]
pub type WaitpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORTCMD` reader - desc ABORTCMD"]
pub type AbortcmdR = crate::BitReader;
#[doc = "Field `ABORTCMD` writer - desc ABORTCMD"]
pub type AbortcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOINIT` reader - desc AUTOINIT"]
pub type AutoinitR = crate::BitReader;
#[doc = "Field `AUTOINIT` writer - desc AUTOINIT"]
pub type AutoinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSYNC` reader - desc REGSYNC"]
pub type RegsyncR = crate::BitReader;
#[doc = "Field `REGSYNC` writer - desc REGSYNC"]
pub type RegsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATACMD` reader - desc ATACMD"]
pub type AtacmdR = crate::BitReader;
#[doc = "Field `ATACMD` writer - desc ATACMD"]
pub type AtacmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEN` reader - desc IEN"]
pub type IenR = crate::BitReader;
#[doc = "Field `IEN` writer - desc IEN"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTEN` reader - desc BOOTEN"]
pub type BootenR = crate::BitReader;
#[doc = "Field `BOOTEN` writer - desc BOOTEN"]
pub type BootenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACK` reader - desc BOOTACK"]
pub type BootackR = crate::BitReader;
#[doc = "Field `BOOTACK` writer - desc BOOTACK"]
pub type BootackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTDIS` reader - desc BOOTDIS"]
pub type BootdisR = crate::BitReader;
#[doc = "Field `BOOTDIS` writer - desc BOOTDIS"]
pub type BootdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTMODE` reader - desc BOOTMODE"]
pub type BootmodeR = crate::BitReader;
#[doc = "Field `BOOTMODE` writer - desc BOOTMODE"]
pub type BootmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTCMD` reader - desc STARTCMD"]
pub type StartcmdR = crate::BitReader;
#[doc = "Field `STARTCMD` writer - desc STARTCMD"]
pub type StartcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CmdindexR {
        CmdindexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WaitrespR {
        WaitrespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESPLEN"]
    #[inline(always)]
    pub fn resplen(&self) -> ResplenR {
        ResplenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CHECKRESPCRC"]
    #[inline(always)]
    pub fn checkrespcrc(&self) -> CheckrespcrcR {
        CheckrespcrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DEXPECT"]
    #[inline(always)]
    pub fn dexpect(&self) -> DexpectR {
        DexpectR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DtmodeR {
        DtmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&self) -> AutostopR {
        AutostopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WaitpendR {
        WaitpendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ABORTCMD"]
    #[inline(always)]
    pub fn abortcmd(&self) -> AbortcmdR {
        AbortcmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc AUTOINIT"]
    #[inline(always)]
    pub fn autoinit(&self) -> AutoinitR {
        AutoinitR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - desc REGSYNC"]
    #[inline(always)]
    pub fn regsync(&self) -> RegsyncR {
        RegsyncR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc ATACMD"]
    #[inline(always)]
    pub fn atacmd(&self) -> AtacmdR {
        AtacmdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BootenR {
        BootenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc BOOTACK"]
    #[inline(always)]
    pub fn bootack(&self) -> BootackR {
        BootackR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc BOOTDIS"]
    #[inline(always)]
    pub fn bootdis(&self) -> BootdisR {
        BootdisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BootmodeR {
        BootmodeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STARTCMD"]
    #[inline(always)]
    pub fn startcmd(&self) -> StartcmdR {
        StartcmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CmdindexW<'_, CmdSpec> {
        CmdindexW::new(self, 0)
    }
    #[doc = "Bit 6 - desc WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WaitrespW<'_, CmdSpec> {
        WaitrespW::new(self, 6)
    }
    #[doc = "Bit 7 - desc RESPLEN"]
    #[inline(always)]
    pub fn resplen(&mut self) -> ResplenW<'_, CmdSpec> {
        ResplenW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CHECKRESPCRC"]
    #[inline(always)]
    pub fn checkrespcrc(&mut self) -> CheckrespcrcW<'_, CmdSpec> {
        CheckrespcrcW::new(self, 8)
    }
    #[doc = "Bit 9 - desc DEXPECT"]
    #[inline(always)]
    pub fn dexpect(&mut self) -> DexpectW<'_, CmdSpec> {
        DexpectW::new(self, 9)
    }
    #[doc = "Bit 10 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CmdSpec> {
        DirW::new(self, 10)
    }
    #[doc = "Bit 11 - desc DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DtmodeW<'_, CmdSpec> {
        DtmodeW::new(self, 11)
    }
    #[doc = "Bit 12 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AutostopW<'_, CmdSpec> {
        AutostopW::new(self, 12)
    }
    #[doc = "Bit 13 - desc WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WaitpendW<'_, CmdSpec> {
        WaitpendW::new(self, 13)
    }
    #[doc = "Bit 14 - desc ABORTCMD"]
    #[inline(always)]
    pub fn abortcmd(&mut self) -> AbortcmdW<'_, CmdSpec> {
        AbortcmdW::new(self, 14)
    }
    #[doc = "Bit 15 - desc AUTOINIT"]
    #[inline(always)]
    pub fn autoinit(&mut self) -> AutoinitW<'_, CmdSpec> {
        AutoinitW::new(self, 15)
    }
    #[doc = "Bit 21 - desc REGSYNC"]
    #[inline(always)]
    pub fn regsync(&mut self) -> RegsyncW<'_, CmdSpec> {
        RegsyncW::new(self, 21)
    }
    #[doc = "Bit 22 - desc ATACMD"]
    #[inline(always)]
    pub fn atacmd(&mut self) -> AtacmdW<'_, CmdSpec> {
        AtacmdW::new(self, 22)
    }
    #[doc = "Bit 23 - desc IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IenW<'_, CmdSpec> {
        IenW::new(self, 23)
    }
    #[doc = "Bit 24 - desc BOOTEN"]
    #[inline(always)]
    pub fn booten(&mut self) -> BootenW<'_, CmdSpec> {
        BootenW::new(self, 24)
    }
    #[doc = "Bit 25 - desc BOOTACK"]
    #[inline(always)]
    pub fn bootack(&mut self) -> BootackW<'_, CmdSpec> {
        BootackW::new(self, 25)
    }
    #[doc = "Bit 26 - desc BOOTDIS"]
    #[inline(always)]
    pub fn bootdis(&mut self) -> BootdisW<'_, CmdSpec> {
        BootdisW::new(self, 26)
    }
    #[doc = "Bit 27 - desc BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BootmodeW<'_, CmdSpec> {
        BootmodeW::new(self, 27)
    }
    #[doc = "Bit 31 - desc STARTCMD"]
    #[inline(always)]
    pub fn startcmd(&mut self) -> StartcmdW<'_, CmdSpec> {
        StartcmdW::new(self, 31)
    }
}
#[doc = "desc CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
