#[doc = "Register `BLKSIZ` reader"]
pub type R = crate::R<BlksizSpec>;
#[doc = "Register `BLKSIZ` writer"]
pub type W = crate::W<BlksizSpec>;
#[doc = "Field `DBLOCKSIZE` reader - desc DBLOCKSIZE"]
pub type DblocksizeR = crate::FieldReader<u16>;
#[doc = "Field `DBLOCKSIZE` writer - desc DBLOCKSIZE"]
pub type DblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DblocksizeR {
        DblocksizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DblocksizeW<'_, BlksizSpec> {
        DblocksizeW::new(self, 0)
    }
}
#[doc = "desc BLKSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`blksiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizSpec;
impl crate::RegisterSpec for BlksizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksiz::R`](R) reader structure"]
impl crate::Readable for BlksizSpec {}
#[doc = "`write(|w| ..)` method takes [`blksiz::W`](W) writer structure"]
impl crate::Writable for BlksizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKSIZ to value 0"]
impl crate::Resettable for BlksizSpec {}
