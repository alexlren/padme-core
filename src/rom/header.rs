#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CgbMode {
    None,
    Both,
    Cgb,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
    Unknown,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Licensee {
    None,
    Capcom,
    ElectronicArts,
    HudsonSoft,
    BAi,
    Kss,
    Pow,
    PcmComplete,
    SanX,
    KemcoJapan,
    Seta,
    Viacom,
    Nintendo,
    Bandai,
    OceanAcclaim,
    Konami,
    Hector,
    Taito,
    Hudson,
    Banpresto,
    Ubisoft,
    Atlus,
    Malibu,
    Angel,
    BulletProof,
    Irem,
    Absolute,
    Acclaim,
    Activision,
    AmericanSammy,
    HitechEntertainment,
    Ljn,
    Matchbox,
    Mattel,
    MiltonBradley,
    Titus,
    Virgin,
    LucasArts,
    Ocean,
    Infogrames,
    Interplay,
    Broderbund,
    Sculptured,
    Sci,
    Thq,
    Accolade,
    Misawa,
    Lozc,
    TokumaShotenIntermedia,
    TsukudaOriginal,
    Chunsoft,
    VideoSystem,
    Varie,
    YonezawaSpal,
    Kaneko,
    PackInSoft,
    EliteSystems,
    ItcEntertainment,
    Yanoman,
    Clary,
    KotobukiSystems,
    EntertainmentI,
    Gremlin,
    SpectrumHoloby,
    UsGold,
    Gametek,
    ParkPlace,
    Mindscape,
    Tradewest,
    ElectroBrain,
    SculpteredSoft,
    TheSalesCurve,
    TriffixEntertainment,
    Microprose,
    Kemco,
    TokumaShoten,
    VicTokai,
    Ape,
    Imax,
    ChunSoft,
    Tsuburava,
    Arc,
    NihonBussan,
    Tecmo,
    Imagineer,
    Nova,
    HoriElectric,
    Kawada,
    Takara,
    TechnosJapan,
    ToeiAnimation,
    Toho,
    Namco,
    Nexoft,
    Enix,
    Hal,
    Snk,
    PonyCanyon,
    CultureBrain,
    Sunsoft,
    Sony,
    Sammy,
    SquareSoft,
    DataEast,
    TonkinHouse,
    Koei,
    Ufl,
    Ultra,
    Vap,
    Use,
    Meldac,
    Sofel,
    Quest,
    Sigma,
    AskKodansha,
    NaxatSoft,
    CopyaSystems,
    Tomy,
    Ncs,
    Human,
    Altron,
    Jaleco,
    Towachiki,
    Uutaka,
    Epoch,
    Athena,
    Asmik,
    Natsume,
    KingRecords,
    EpicSonyRecords,
    Igs,
    AWave,
    ExtremeEntertainment,
    Romstar,
    Unknown,
}
