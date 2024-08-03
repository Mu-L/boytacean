//! Game Boy licensee vendors information and static enumerations.

use std::fmt::{self, Display, Formatter};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Licensee {
    Unknown,
    None,
    Nintendo,
    Capcom,
    HOTB,
    Jaleco,
    CoconutsJapan,
    EliteSystems,
    EAElectronicArts,
    HudsonSoft,
    ITCEntertainment,
    Yanoman,
    JapanClary,
    VirginGamesLtd,
    PCMComplete,
    SanX,
    Kemco,
    SETACorporation,
    Infogrames,
    Bandai,
    NewLicensee,
    Konami,
    HectorSoft,
    Banpresto,
    EntertainmentI,
    Gremlin,
    UbiSoft,
    Atlus,
    MalibuInteractive,
    Angel,
    SpectrumHoloby,
    Irem,
    USGold,
    Absolute,
    AcclaimEntertainment,
    Activision,
    SammyUSACorporation,
    GameTek,
    ParkPlace,
    LJN,
    Matchbox,
    MiltonBradleyCompany,
    Mindscape,
    Romstar,
    NaxatSoft,
    Tradewest,
    TitusInteractive,
    OceanSoftware,
    EliteSystems2,
    ElectroBrain,
    InterplayEntertainment,
    Broderbund,
    SculpturedSoftware,
    TheSalesCurveLimited,
    THQ,
    Accolade,
    TriffixEntertainment,
    Microprose,
    MisawaEntertainment,
    Lozc,
    TokumaShoten,
    BulletProofSoftware,
    VicTokai,
    Ape,
    IMax,
    Chunsoft,
    VideoSystem,
    TsubarayaProductions,
    Varie,
    YonezawaSpal,
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
    ASCIICorporationOrNexsoft,
    SquareEnix,
    HALLaboratory,
    SNK,
    PonyCanyon,
    CultureBrain,
    Sunsoft,
    SonyImagesoft,
    SammyCorporation,
    Taito,
    Square,
    DataEast,
    Tonkinhouse,
    Koei,
    UFL,
    Ultra,
    Vap,
    UseCorporation,
    Meldac,
    PoneyCanyon,
    Sofel,
    Quest,
    SigmaEnterprises,
    ASKKodanshaCo,
    CopyaSystem,
    Tomy,
    NCS,
    Human,
    Altron,
    TowaChiki,
    Yutaka,
    Epcoh,
    Athena,
    AsmikAceEntertainment,
    Natsume,
    KingRecords,
    EpicSonyRecords,
    IGS,
    AWave,
    ExtremeEntertainment,

    // new licensee only codes (use 0x33 as old code)
    NintendoResearchDevelopment1,
    BAI,
    KSS,
    PlanningOfficeWADA,
    Viacom,
    HiTechExpressions,
    Mattel,
    LucasfilmGames,
    TsukudaOriginal,
    ChunsoftCo,
    OceanSoftwareAcclaimEntertainment,
    Kaneko,
    PackInVideo,
    BottomUp,
    KonamiYuGiOh,
    MTO,
    Kodansha,
}

impl Licensee {
    pub fn from_data(old_code: u8, new_code: &[u8]) -> Licensee {
        let mut licensee = match old_code {
            0x00 => Licensee::None,
            0x01 => Licensee::Nintendo,
            0x08 => Licensee::Capcom,
            0x09 => Licensee::HOTB,
            0x0a => Licensee::Jaleco,
            0x0b => Licensee::CoconutsJapan,
            0x0c => Licensee::EliteSystems,
            0x13 => Licensee::EAElectronicArts,
            0x18 => Licensee::HudsonSoft,
            0x19 => Licensee::ITCEntertainment,
            0x1a => Licensee::Yanoman,
            0x1d => Licensee::JapanClary,
            0x1f => Licensee::VirginGamesLtd,
            0x24 => Licensee::PCMComplete,
            0x25 => Licensee::SanX,
            0x28 => Licensee::Kemco,
            0x29 => Licensee::SETACorporation,
            0x30 => Licensee::Infogrames,
            0x31 => Licensee::Nintendo,
            0x32 => Licensee::Bandai,
            0x33 => Licensee::NewLicensee,
            0x34 => Licensee::Konami,
            0x35 => Licensee::HectorSoft,
            0x38 => Licensee::Capcom,
            0x39 => Licensee::Banpresto,
            0x3c => Licensee::EntertainmentI,
            0x3e => Licensee::Gremlin,
            0x41 => Licensee::UbiSoft,
            0x42 => Licensee::Atlus,
            0x44 => Licensee::MalibuInteractive,
            0x46 => Licensee::Angel,
            0x47 => Licensee::SpectrumHoloby,
            0x49 => Licensee::Irem,
            0x4a => Licensee::VirginGamesLtd,
            0x4d => Licensee::MalibuInteractive,
            0x4f => Licensee::USGold,
            0x50 => Licensee::Absolute,
            0x51 => Licensee::AcclaimEntertainment,
            0x52 => Licensee::Activision,
            0x53 => Licensee::SammyUSACorporation,
            0x54 => Licensee::GameTek,
            0x55 => Licensee::ParkPlace,
            0x56 => Licensee::LJN,
            0x57 => Licensee::Matchbox,
            0x59 => Licensee::MiltonBradleyCompany,
            0x5a => Licensee::Mindscape,
            0x5b => Licensee::Romstar,
            0x5c => Licensee::NaxatSoft,
            0x5d => Licensee::Tradewest,
            0x60 => Licensee::TitusInteractive,
            0x61 => Licensee::VirginGamesLtd,
            0x67 => Licensee::OceanSoftware,
            0x69 => Licensee::EAElectronicArts,
            0x6e => Licensee::EliteSystems2,
            0x6f => Licensee::ElectroBrain,
            0x70 => Licensee::Infogrames,
            0x71 => Licensee::InterplayEntertainment,
            0x72 => Licensee::Broderbund,
            0x73 => Licensee::SculpturedSoftware,
            0x75 => Licensee::TheSalesCurveLimited,
            0x78 => Licensee::THQ,
            0x79 => Licensee::Accolade,
            0x7a => Licensee::TriffixEntertainment,
            0x7c => Licensee::Microprose,
            0x7f => Licensee::Kemco,
            0x80 => Licensee::MisawaEntertainment,
            0x83 => Licensee::Lozc,
            0x86 => Licensee::TokumaShoten,
            0x8b => Licensee::BulletProofSoftware,
            0x8c => Licensee::VicTokai,
            0x8e => Licensee::Ape,
            0x8f => Licensee::IMax,
            0x91 => Licensee::Chunsoft,
            0x92 => Licensee::VideoSystem,
            0x93 => Licensee::TsubarayaProductions,
            0x95 => Licensee::Varie,
            0x96 => Licensee::YonezawaSpal,
            0x97 => Licensee::Kemco,
            0x99 => Licensee::Arc,
            0x9a => Licensee::NihonBussan,
            0x9b => Licensee::Tecmo,
            0x9c => Licensee::Imagineer,
            0x9d => Licensee::Banpresto,
            0x9f => Licensee::Nova,
            0xa1 => Licensee::HoriElectric,
            0xa2 => Licensee::Bandai,
            0xa4 => Licensee::Konami,
            0xa6 => Licensee::Kawada,
            0xa7 => Licensee::Takara,
            0xa9 => Licensee::TechnosJapan,
            0xaa => Licensee::Broderbund,
            0xac => Licensee::ToeiAnimation,
            0xad => Licensee::Toho,
            0xaf => Licensee::Namco,
            0xb0 => Licensee::AcclaimEntertainment,
            0xb1 => Licensee::ASCIICorporationOrNexsoft,
            0xb2 => Licensee::Bandai,
            0xb4 => Licensee::SquareEnix,
            0xb6 => Licensee::HALLaboratory,
            0xb7 => Licensee::SNK,
            0xb9 => Licensee::PonyCanyon,
            0xba => Licensee::CultureBrain,
            0xbb => Licensee::Sunsoft,
            0xbd => Licensee::SonyImagesoft,
            0xbf => Licensee::SammyCorporation,
            0xc0 => Licensee::Taito,
            0xc2 => Licensee::Kemco,
            0xc3 => Licensee::Square,
            0xc4 => Licensee::TokumaShoten,
            0xc5 => Licensee::DataEast,
            0xc6 => Licensee::Tonkinhouse,
            0xc8 => Licensee::Koei,
            0xc9 => Licensee::UFL,
            0xca => Licensee::Ultra,
            0xcb => Licensee::Vap,
            0xcc => Licensee::UseCorporation,
            0xcd => Licensee::Meldac,
            0xce => Licensee::PonyCanyon,
            0xcf => Licensee::Angel,
            0xd0 => Licensee::Taito,
            0xd1 => Licensee::Sofel,
            0xd2 => Licensee::Quest,
            0xd3 => Licensee::SigmaEnterprises,
            0xd4 => Licensee::ASKKodanshaCo,
            0xd6 => Licensee::NaxatSoft,
            0xd7 => Licensee::CopyaSystem,
            0xd9 => Licensee::Banpresto,
            0xda => Licensee::Tomy,
            0xdb => Licensee::LJN,
            0xdd => Licensee::NCS,
            0xde => Licensee::Human,
            0xdf => Licensee::Altron,
            0xe0 => Licensee::Jaleco,
            0xe1 => Licensee::TowaChiki,
            0xe2 => Licensee::Yutaka,
            0xe3 => Licensee::Varie,
            0xe5 => Licensee::Epcoh,
            0xe7 => Licensee::Athena,
            0xe8 => Licensee::AsmikAceEntertainment,
            0xe9 => Licensee::Natsume,
            0xea => Licensee::KingRecords,
            0xeb => Licensee::Atlus,
            0xec => Licensee::EpicSonyRecords,
            0xee => Licensee::IGS,
            0xf0 => Licensee::AWave,
            0xf3 => Licensee::ExtremeEntertainment,
            0xff => Licensee::LJN,
            _ => Licensee::Unknown,
        };
        if licensee == Licensee::NewLicensee {
            let new_code_s = std::str::from_utf8(new_code).unwrap_or("");
            licensee = match new_code_s {
                "00" => Licensee::None,
                "01" => Licensee::NintendoResearchDevelopment1,
                "08" => Licensee::Capcom,
                "13" => Licensee::EAElectronicArts,
                "18" => Licensee::HudsonSoft,
                "19" => Licensee::BAI,
                "20" => Licensee::KSS,
                "22" => Licensee::PlanningOfficeWADA,
                "24" => Licensee::PCMComplete,
                "25" => Licensee::SanX,
                "28" => Licensee::Kemco,
                "29" => Licensee::SETACorporation,
                "30" => Licensee::Viacom,
                "31" => Licensee::Nintendo,
                "32" => Licensee::Bandai,
                "33" => Licensee::OceanSoftwareAcclaimEntertainment,
                "34" => Licensee::Konami,
                "35" => Licensee::HectorSoft,
                "37" => Licensee::Taito,
                "38" => Licensee::HudsonSoft,
                "39" => Licensee::Banpresto,
                "41" => Licensee::UbiSoft,
                "42" => Licensee::Atlus,
                "44" => Licensee::MalibuInteractive,
                "46" => Licensee::Angel,
                "47" => Licensee::BulletProofSoftware,
                "49" => Licensee::Irem,
                "50" => Licensee::Absolute,
                "51" => Licensee::AcclaimEntertainment,
                "52" => Licensee::Activision,
                "53" => Licensee::SammyUSACorporation,
                "54" => Licensee::Konami,
                "55" => Licensee::HiTechExpressions,
                "56" => Licensee::LJN,
                "57" => Licensee::Matchbox,
                "58" => Licensee::Mattel,
                "59" => Licensee::MiltonBradleyCompany,
                "60" => Licensee::TitusInteractive,
                "61" => Licensee::VirginGamesLtd,
                "64" => Licensee::LucasfilmGames,
                "67" => Licensee::OceanSoftware,
                "69" => Licensee::EAElectronicArts,
                "70" => Licensee::Infogrames,
                "71" => Licensee::InterplayEntertainment,
                "72" => Licensee::Broderbund,
                "73" => Licensee::SculpturedSoftware,
                "75" => Licensee::TheSalesCurveLimited,
                "78" => Licensee::THQ,
                "79" => Licensee::Accolade,
                "80" => Licensee::MisawaEntertainment,
                "83" => Licensee::Lozc,
                "86" => Licensee::TokumaShoten,
                "87" => Licensee::TsukudaOriginal,
                "91" => Licensee::Chunsoft,
                "92" => Licensee::VideoSystem,
                "93" => Licensee::OceanSoftwareAcclaimEntertainment,
                "95" => Licensee::Varie,
                "96" => Licensee::YonezawaSpal,
                "97" => Licensee::Kaneko,
                "99" => Licensee::PackInVideo,
                "9H" => Licensee::BottomUp,
                "A4" => Licensee::KonamiYuGiOh,
                "BL" => Licensee::MTO,
                "DK" => Licensee::Kodansha,
                _ => Licensee::Unknown,
            }
        }
        licensee
    }

    pub fn description(&self) -> &'static str {
        match self {
            Licensee::Unknown => "Unknown",
            Licensee::None => "None",
            Licensee::Nintendo => "Nintendo",
            Licensee::Capcom => "Capcom",
            Licensee::HOTB => "HOT-B",
            Licensee::Jaleco => "Jaleco",
            Licensee::CoconutsJapan => "Coconuts Japan",
            Licensee::EliteSystems => "Elite Systems",
            Licensee::EAElectronicArts => "EA (Electronic Arts)",
            Licensee::HudsonSoft => "Hudson Soft",
            Licensee::ITCEntertainment => "ITC Entertainment",
            Licensee::Yanoman => "Yanoman",
            Licensee::JapanClary => "Japan Clary",
            Licensee::VirginGamesLtd => "Virgin Games Ltd.",
            Licensee::PCMComplete => "PCM Complete",
            Licensee::SanX => "San-X",
            Licensee::Kemco => "Kemco",
            Licensee::SETACorporation => "SETA Corporation",
            Licensee::Infogrames => "Infogrames",
            Licensee::Bandai => "Bandai",
            Licensee::NewLicensee => "Indicates that the New licensee code should be used instead.",
            Licensee::Konami => "Konami",
            Licensee::HectorSoft => "HectorSoft",
            Licensee::Banpresto => "Banpresto",
            Licensee::EntertainmentI => ".Entertainment i",
            Licensee::Gremlin => "Gremlin",
            Licensee::UbiSoft => "Ubi Soft",
            Licensee::Atlus => "Atlus",
            Licensee::MalibuInteractive => "Malibu Interactive",
            Licensee::Angel => "Angel",
            Licensee::SpectrumHoloby => "Spectrum Holoby",
            Licensee::Irem => "Irem",
            Licensee::USGold => "U.S. Gold",
            Licensee::Absolute => "Absolute",
            Licensee::AcclaimEntertainment => "Acclaim Entertainment",
            Licensee::Activision => "Activision",
            Licensee::SammyUSACorporation => "Sammy USA Corporation",
            Licensee::GameTek => "GameTek",
            Licensee::ParkPlace => "Park Place",
            Licensee::LJN => "LJN",
            Licensee::Matchbox => "Matchbox",
            Licensee::MiltonBradleyCompany => "Milton Bradley Company",
            Licensee::Mindscape => "Mindscape",
            Licensee::Romstar => "Romstar",
            Licensee::NaxatSoft => "Naxat Soft",
            Licensee::Tradewest => "Tradewest",
            Licensee::TitusInteractive => "Titus Interactive",
            Licensee::OceanSoftware => "Ocean Software",
            Licensee::EliteSystems2 => "Elite Systems",
            Licensee::ElectroBrain => "Electro Brain",
            Licensee::InterplayEntertainment => "Interplay Entertainment",
            Licensee::Broderbund => "Broderbund",
            Licensee::SculpturedSoftware => "Sculptured Software",
            Licensee::TheSalesCurveLimited => "The Sales Curve Limited",
            Licensee::THQ => "THQ",
            Licensee::Accolade => "Accolade",
            Licensee::TriffixEntertainment => "Triffix Entertainment",
            Licensee::Microprose => "Microprose",
            Licensee::MisawaEntertainment => "Misawa Entertainment",
            Licensee::Lozc => "Lozc",
            Licensee::TokumaShoten => "Tokuma Shoten",
            Licensee::BulletProofSoftware => "Bullet-Proof Software",
            Licensee::VicTokai => "Vic Tokai",
            Licensee::Ape => "Ape",
            Licensee::IMax => "I’Max",
            Licensee::Chunsoft => "Chunsoft Co.",
            Licensee::VideoSystem => "Video System",
            Licensee::TsubarayaProductions => "Tsubaraya Productions",
            Licensee::Varie => "Varie",
            Licensee::YonezawaSpal => "Yonezawa/S’Pal",
            Licensee::Arc => "Arc",
            Licensee::NihonBussan => "Nihon Bussan",
            Licensee::Tecmo => "Tecmo",
            Licensee::Imagineer => "Imagineer",
            Licensee::Nova => "Nova",
            Licensee::HoriElectric => "Hori Electric",
            Licensee::Kawada => "Kawada",
            Licensee::Takara => "Takara",
            Licensee::TechnosJapan => "Technos Japan",
            Licensee::ToeiAnimation => "Toei Animation",
            Licensee::Toho => "Toho",
            Licensee::Namco => "Namco",
            Licensee::ASCIICorporationOrNexsoft => "ASCII Corporation or Nexsoft",
            Licensee::SquareEnix => "Square Enix",
            Licensee::HALLaboratory => "HAL Laboratory",
            Licensee::SNK => "SNK",
            Licensee::PonyCanyon => "Pony Canyon",
            Licensee::CultureBrain => "Culture Brain",
            Licensee::Sunsoft => "Sunsoft",
            Licensee::SonyImagesoft => "Sony Imagesoft",
            Licensee::SammyCorporation => "Sammy Corporation",
            Licensee::Taito => "Taito",
            Licensee::Square => "Square",
            Licensee::DataEast => "Data East",
            Licensee::Tonkinhouse => "Tonkinhouse",
            Licensee::Koei => "Koei",
            Licensee::UFL => "UFL",
            Licensee::Ultra => "Ultra",
            Licensee::Vap => "Vap",
            Licensee::UseCorporation => "Use Corporation",
            Licensee::Meldac => "Meldac",
            Licensee::PoneyCanyon => "Pony Canyon",
            Licensee::Sofel => "Sofel",
            Licensee::Quest => "Quest",
            Licensee::SigmaEnterprises => "Sigma Enterprises",
            Licensee::ASKKodanshaCo => "ASK Kodansha Co.",
            Licensee::CopyaSystem => "Copya System",
            Licensee::Tomy => "Tomy",
            Licensee::NCS => "NCS",
            Licensee::Human => "Human",
            Licensee::Altron => "Altron",
            Licensee::TowaChiki => "Towa Chiki",
            Licensee::Yutaka => "Yutaka",
            Licensee::Epcoh => "Epcoh",
            Licensee::Athena => "Athena",
            Licensee::AsmikAceEntertainment => "Asmik Ace Entertainment",
            Licensee::Natsume => "Natsume",
            Licensee::KingRecords => "King Records",
            Licensee::EpicSonyRecords => "Epic/Sony Records",
            Licensee::IGS => "IGS",
            Licensee::AWave => "A Wave",
            Licensee::ExtremeEntertainment => "Extreme Entertainment",
            Licensee::NintendoResearchDevelopment1 => "Nintendo Research & Development 1",
            Licensee::BAI => "B-AI",
            Licensee::KSS => "KSS",
            Licensee::PlanningOfficeWADA => "Planning Office WADA",
            Licensee::Viacom => "Viacom",
            Licensee::HiTechExpressions => "Hi Tech Expressions",
            Licensee::Mattel => "Mattel",
            Licensee::LucasfilmGames => "Lucasfilm Games",
            Licensee::TsukudaOriginal => "Tsukuda Original",
            Licensee::ChunsoftCo => "Chunsoft Co.",
            Licensee::OceanSoftwareAcclaimEntertainment => "Ocean Software/Acclaim Entertainment",
            Licensee::Kaneko => "Kaneko",
            Licensee::PackInVideo => "Pack-In-Video",
            Licensee::BottomUp => "Bottom Up",
            Licensee::KonamiYuGiOh => "Konami (Yu-Gi-Oh!)",
            Licensee::MTO => "MTO",
            Licensee::Kodansha => "Kodansha",
        }
    }
}

impl Display for Licensee {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}