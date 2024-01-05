use crate::data::enums::StarType;
use crate::data::random::DspRandom;
use crate::data::star::Star;

fn gen_random_name(seed: i32, star: &Star) -> String {
    let mut rand = DspRandom::new(seed);
    let seed1 = rand.next_seed();
    let num1 = rand.next_f64();
    if star.star_type == StarType::GiantStar {
        let num2 = rand.next_f64();
        if num2 < 0.4 {
            random_giant_star_name_from_raw_names(seed1)
        } else if num2 < 0.7 {
            random_giant_star_name_with_constellation_alpha(seed1)
        } else {
            random_giant_star_name_with_format(seed1)
        }
    } else if star.star_type == StarType::NeutronStar {
        random_neutron_star_name_with_format(seed1)
    } else if star.star_type == StarType::BlackHole {
        random_black_hole_name_with_format(seed1)
    } else if num1 < 0.6 {
        random_star_name_from_raw_names(seed1)
    } else if num1 < 0.93 {
        random_star_name_with_constellation_alpha(seed1)
    } else {
        random_star_name_with_constellation_number(seed1)
    }
}

pub fn random_name<'a>(
    seed: i32,
    star: &Star,
    mut names: impl Iterator<Item = &'a &'a str>,
) -> String {
    let mut rand = DspRandom::new(seed);
    for _ in 0..256 {
        let str = gen_random_name(rand.next_seed(), star);
        if names.all(|s| *s != str) {
            return str;
        }
    }
    return "XStar".to_owned();
}

fn random_giant_star_name_with_constellation_alpha(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_i32(15, 26);
    let num3 = rand.next_i32(0, 26);
    let index = num1 % CONSTELLATIONS.len();
    (130 + num2 + num3).to_string() + " " + CONSTELLATIONS[index]
}

fn random_giant_star_name_with_format(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_i32(0, 10000);
    let num3 = rand.next_i32(0, 100);
    let index = num1 % GIANT_NAME_FORMATS.len();
    GIANT_NAME_FORMATS[index](num2, num3)
}

fn random_neutron_star_name_with_format(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_i32(0, 24);
    let num3 = rand.next_i32(0, 60);
    let num4 = rand.next_i32(0, 60);
    let index = num1 % NEUTRON_STAR_NAME_FORMATS.len();
    NEUTRON_STAR_NAME_FORMATS[index](num2, num3, num4)
}

fn random_black_hole_name_with_format(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_i32(0, 24);
    let num3 = rand.next_i32(0, 60);
    let num4 = rand.next_i32(0, 60);
    let index = num1 % BLACK_HOLE_NAME_FORMATS.len();
    BLACK_HOLE_NAME_FORMATS[index](num2, num3, num4)
}

fn random_star_name_from_raw_names(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let index = (rand.next_usize()) % RAW_STAR_NAMES.len();
    RAW_STAR_NAMES[index].to_owned()
}

fn random_star_name_with_constellation_alpha(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_usize();
    let index1 = num1 % CONSTELLATIONS.len();
    let index2 = num2 % ALPHABETA.len();
    let constellation = CONSTELLATIONS[index1];
    if constellation.len() > 10 {
        ALPHABETA_LETTER[index2].to_owned() + " " + constellation
    } else {
        ALPHABETA[index2].to_owned() + " " + constellation
    }
}

fn random_star_name_with_constellation_number(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let num1 = rand.next_usize();
    let num2 = rand.next_i32(27, 75);
    let index = num1 % CONSTELLATIONS.len();
    num2.to_string() + " " + CONSTELLATIONS[index]
}

fn random_giant_star_name_from_raw_names(seed: i32) -> String {
    let mut rand = DspRandom::new(seed);
    let index = (rand.next_usize()) % RAW_GIANT_NAMES.len();
    RAW_GIANT_NAMES[index].to_owned()
}

type Fn2 = fn(i1: i32, i2: i32) -> String;
type Fn3 = fn(i1: i32, i2: i32, i3: i32) -> String;

const GIANT_NAME_FORMATS: &'static [Fn2] = &[
    |i1, i2| format!("HD {:04}{:02}", i1, i2),
    |i1, i2| format!("HDE {:04}{:02}", i1, i2),
    |i1, _| format!("HR {:04}", i1),
    |i1, _| format!("HV {:04}", i1),
    |i1, i2| format!("LBV {:04}-{:02}", i1, i2),
    |i1, _| format!("NSV {:04}", i1),
    |i1, i2| format!("YSC {:04}-{:02}", i1, i2),
];

const NEUTRON_STAR_NAME_FORMATS: &'static [Fn3] = &[
    |i1, i2, i3| format!("NTR J{:02}{:02}+{:02}", i1, i2, i3),
    |i1, i2, i3| format!("NTR J{:02}{:02}-{:02}", i1, i2, i3),
];

const BLACK_HOLE_NAME_FORMATS: &'static [Fn3] = &[
    |i1, i2, i3| format!("DSR J{:02}{:02}+{:02}", i1, i2, i3),
    |i1, i2, i3| format!("DSR J{:02}{:02}-{:02}", i1, i2, i3),
];

const RAW_GIANT_NAMES: &'static [&'static str] = &[
    "AH Scorpii",
    "Aldebaran",
    "Alpha Herculis",
    "Antares",
    "Arcturus",
    "AV Persei",
    "BC Cygni",
    "Betelgeuse",
    "BI Cygni",
    "BO Carinae",
    "Canopus",
    "CE Tauri",
    "CK Carinae",
    "CW Leonis",
    "Deneb",
    "Epsilon Aurigae",
    "Eta Carinae",
    "EV Carinae",
    "IX Carinae",
    "KW Sagittarii",
    "KY Cygni",
    "Mira",
    "Mu Cephei",
    "NML Cygni",
    "NR Vulpeculae",
    "PZ Cassiopeiae",
    "R Doradus",
    "R Leporis",
    "Rho Cassiopeiae",
    "Rigel",
    "RS Persei",
    "RT Carinae",
    "RU Virginis",
    "RW Cephei",
    "S Cassiopeiae",
    "S Cephei",
    "S Doradus",
    "S Persei",
    "SU Persei",
    "TV Geminorum",
    "U Lacertae",
    "UY Scuti",
    "V1185 Scorpii",
    "V354 Cephei",
    "V355 Cepheus",
    "V382 Carinae",
    "V396 Centauri",
    "V437 Scuti",
    "V509 Cassiopeiae",
    "V528 Carinae",
    "V602 Carinae",
    "V648 Cassiopeiae",
    "V669 Cassiopeiae",
    "V838 Monocerotis",
    "V915 Scorpii",
    "VV Cephei",
    "VX Sagittarii",
    "VY Canis Majoris",
    "WOH G64",
    "XX Persei",
];

const ALPHABETA: &'static [&'static str] = &[
    "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta", "Iota", "Kappa", "Lambda",
];

const ALPHABETA_LETTER: &'static [&'static str] =
    &["α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ"];

const CONSTELLATIONS: &'static [&'static str] = &[
    "Andromedae",
    "Antliae",
    "Apodis",
    "Aquarii",
    "Aquilae",
    "Arae",
    "Arietis",
    "Aurigae",
    "Bootis",
    "Caeli",
    "Camelopardalis",
    "Cancri",
    "Canum Venaticorum",
    "Canis Majoris",
    "Canis Minoris",
    "Capricorni",
    "Carinae",
    "Cassiopeiae",
    "Centauri",
    "Cephei",
    "Ceti",
    "Chamaeleontis",
    "Circini",
    "Columbae",
    "Comae Berenices",
    "Coronae Australis",
    "Coronae Borealis",
    "Corvi",
    "Crateris",
    "Crucis",
    "Cygni",
    "Delphini",
    "Doradus",
    "Draconis",
    "Equulei",
    "Eridani",
    "Fornacis",
    "Geminorum",
    "Gruis",
    "Herculis",
    "Horologii",
    "Hydrae",
    "Hydri",
    "Indi",
    "Lacertae",
    "Leonis",
    "Leonis Minoris",
    "Leporis",
    "Librae",
    "Lupi",
    "Lyncis",
    "Lyrae",
    "Mensae",
    "Microscopii",
    "Monocerotis",
    "Muscae",
    "Normae",
    "Octantis",
    "Ophiuchii",
    "Orionis",
    "Pavonis",
    "Pegasi",
    "Persei",
    "Phoenicis",
    "Pictoris",
    "Piscium",
    "Piscis Austrini",
    "Puppis",
    "Pyxidis",
    "Reticuli",
    "Sagittae",
    "Sagittarii",
    "Scorpii",
    "Sculptoris",
    "Scuti",
    "Serpentis",
    "Sextantis",
    "Tauri",
    "Telescopii",
    "Trianguli",
    "Trianguli Australis",
    "Tucanae",
    "Ursae Majoris",
    "Ursae Minoris",
    "Velorum",
    "Virginis",
    "Volantis",
    "Vulpeculae",
];

const RAW_STAR_NAMES: &'static [&'static str] = &[
    "Acamar",
    "Achernar",
    "Achird",
    "Acrab",
    "Acrux",
    "Acubens",
    "Adhafera",
    "Adhara",
    "Adhil",
    "Agena",
    "Aladfar",
    "Albaldah",
    "Albali",
    "Albireo",
    "Alchiba",
    "Alcor",
    "Alcyone",
    "Alderamin",
    "Aldhibain",
    "Aldib",
    "Alfecca",
    "Alfirk",
    "Algedi",
    "Algenib",
    "Algenubi",
    "Algieba",
    "Algjebbath",
    "Algol",
    "Algomeyla",
    "Algorab",
    "Alhajoth",
    "Alhena",
    "Alifa",
    "Alioth",
    "Alkaid",
    "Alkalurops",
    "Alkaphrah",
    "Alkes",
    "Alkhiba",
    "Almach",
    "Almeisan",
    "Almuredin",
    "AlNa'ir",
    "Alnasl",
    "Alnilam",
    "Alnitak",
    "Alniyat",
    "Alphard",
    "Alphecca",
    "Alpheratz",
    "Alrakis",
    "Alrami",
    "Alrescha",
    "AlRijil",
    "Alsahm",
    "Alsciaukat",
    "Alshain",
    "Alshat",
    "Alshemali",
    "Alsuhail",
    "Altair",
    "Altais",
    "Alterf",
    "Althalimain",
    "AlTinnin",
    "Aludra",
    "AlulaAustralis",
    "AlulaBorealis",
    "Alwaid",
    "Alwazn",
    "Alya",
    "Alzirr",
    "AmazonStar",
    "Ancha",
    "Anchat",
    "AngelStern",
    "Angetenar",
    "Ankaa",
    "Anser",
    "Antecanis",
    "Apollo",
    "Arich",
    "Arided",
    "Arietis",
    "Arkab",
    "ArkebPrior",
    "Arneb",
    "Arrioph",
    "AsadAustralis",
    "Ascella",
    "Aschere",
    "AsellusAustralis",
    "AsellusBorealis",
    "AsellusPrimus",
    "Ashtaroth",
    "Asmidiske",
    "Aspidiske",
    "Asterion",
    "Asterope",
    "Asuia",
    "Athafiyy",
    "Atik",
    "Atlas",
    "Atria",
    "Auva",
    "Avior",
    "Azelfafage",
    "Azha",
    "Azimech",
    "BatenKaitos",
    "Becrux",
    "Beid",
    "Bellatrix",
    "Benatnasch",
    "Biham",
    "Botein",
    "Brachium",
    "Bunda",
    "Cajam",
    "Calbalakrab",
    "Calx",
    "Canicula",
    "Capella",
    "Caph",
    "Castor",
    "Castula",
    "Cebalrai",
    "Ceginus",
    "Celaeno",
    "Chara",
    "Chertan",
    "Choo",
    "Clava",
    "CorCaroli",
    "CorHydrae",
    "CorLeonis",
    "Cornu",
    "CorScorpii",
    "CorSepentis",
    "CorTauri",
    "Coxa",
    "Cursa",
    "Cymbae",
    "Cynosaura",
    "Dabih",
    "DenebAlgedi",
    "DenebDulfim",
    "DenebelOkab",
    "DenebKaitos",
    "DenebOkab",
    "Denebola",
    "Dhalim",
    "Dhur",
    "Diadem",
    "Difda",
    "DifdaalAuwel",
    "Dnoces",
    "Dubhe",
    "Dziban",
    "Dzuba",
    "Edasich",
    "ElAcola",
    "Elacrab",
    "Electra",
    "Elgebar",
    "Elgomaisa",
    "ElKaprah",
    "ElKaridab",
    "Elkeid",
    "ElKhereb",
    "Elmathalleth",
    "Elnath",
    "ElPhekrah",
    "Eltanin",
    "Enif",
    "Erakis",
    "Errai",
    "FalxItalica",
    "Fidis",
    "Fomalhaut",
    "Fornacis",
    "FumAlSamakah",
    "Furud",
    "Gacrux",
    "Gallina",
    "GarnetStar",
    "Gemma",
    "Genam",
    "Giausar",
    "GiedePrime",
    "Giedi",
    "Gienah",
    "Gienar",
    "Gildun",
    "Girtab",
    "Gnosia",
    "Gomeisa",
    "Gorgona",
    "Graffias",
    "Hadar",
    "Hamal",
    "Haris",
    "Hasseleh",
    "Hastorang",
    "Hatysa",
    "Heka",
    "Hercules",
    "Heze",
    "Hoedus",
    "Homam",
    "HyadumPrimus",
    "Icalurus",
    "Iclarkrav",
    "Izar",
    "Jabbah",
    "Jewel",
    "Jugum",
    "Juza",
    "Kabeleced",
    "Kaff",
    "Kaffa",
    "Kaffaljidma",
    "Kaitain",
    "KalbalAkrab",
    "Kat",
    "KausAustralis",
    "KausBorealis",
    "KausMedia",
    "Keid",
    "KeKouan",
    "Kelb",
    "Kerb",
    "Kerbel",
    "KiffaBoraelis",
    "Kitalpha",
    "Kochab",
    "Kornephoros",
    "Kraz",
    "Ksora",
    "Kuma",
    "Kurhah",
    "Kursa",
    "Lesath",
    "Maasym",
    "Maaz",
    "Mabsuthat",
    "Maia",
    "Marfik",
    "Markab",
    "Marrha",
    "Matar",
    "Mebsuta",
    "Megres",
    "Meissa",
    "Mekbuda",
    "Menkalinan",
    "Menkar",
    "Menkent",
    "Menkib",
    "Merak",
    "Meres",
    "Merga",
    "Meridiana",
    "Merope",
    "Mesartim",
    "Metallah",
    "Miaplacidus",
    "Mimosa",
    "Minelauva",
    "Minkar",
    "Mintaka",
    "Mirac",
    "Mirach",
    "Miram",
    "Mirfak",
    "Mirzam",
    "Misam",
    "Mismar",
    "Mizar",
    "Muhlifain",
    "Muliphein",
    "Muphrid",
    "Muscida",
    "NairalSaif",
    "NairalZaurak",
    "Naos",
    "Nash",
    "Nashira",
    "Navi",
    "Nekkar",
    "Nicolaus",
    "Nihal",
    "Nodus",
    "Nunki",
    "Nusakan",
    "OculusBoreus",
    "Okda",
    "Osiris",
    "OsPegasi",
    "Palilicium",
    "Peacock",
    "Phact",
    "Phecda",
    "Pherkad",
    "PherkadMinor",
    "Pherkard",
    "Phoenice",
    "Phurad",
    "Pishpai",
    "Pleione",
    "Polaris",
    "Pollux",
    "Porrima",
    "Postvarta",
    "Praecipua",
    "Procyon",
    "Propus",
    "Protrygetor",
    "Pulcherrima",
    "Rana",
    "RanaSecunda",
    "Rasalas",
    "Rasalgethi",
    "Rasalhague",
    "Rasalmothallah",
    "RasHammel",
    "Rastaban",
    "Reda",
    "Regor",
    "Regulus",
    "Rescha",
    "RigilKentaurus",
    "RiglalAwwa",
    "Rotanen",
    "Ruchba",
    "Ruchbah",
    "Rukbat",
    "Rutilicus",
    "Saak",
    "Sabik",
    "Sadachbia",
    "Sadalbari",
    "Sadalmelik",
    "Sadalsuud",
    "Sadatoni",
    "Sadira",
    "Sadr",
    "Saidak",
    "Saiph",
    "Salm",
    "Sargas",
    "Sarin",
    "Sartan",
    "Sceptrum",
    "Scheat",
    "Schedar",
    "Scheddi",
    "Schemali",
    "Scutulum",
    "SeatAlpheras",
    "Segin",
    "Seginus",
    "Shaula",
    "Shedir",
    "Sheliak",
    "Sheratan",
    "Singer",
    "Sirius",
    "Sirrah",
    "Situla",
    "Skat",
    "Spica",
    "Sterope",
    "Subra",
    "Suha",
    "Suhail",
    "SuhailHadar",
    "SuhailRadar",
    "Suhel",
    "Sulafat",
    "Superba",
    "Svalocin",
    "Syrma",
    "Tabit",
    "Tais",
    "Talitha",
    "TaniaAustralis",
    "TaniaBorealis",
    "Tarazed",
    "Tarf",
    "TaTsun",
    "Taygeta",
    "Tegmen",
    "Tejat",
    "TejatPrior",
    "Terebellum",
    "Theemim",
    "Thuban",
    "Tolimann",
    "Tramontana",
    "Tsih",
    "Tureis",
    "Unukalhai",
    "Vega",
    "Venabulum",
    "Venator",
    "Vendemiatrix",
    "Vespertilio",
    "Vildiur",
    "Vindemiatrix",
    "Wasat",
    "Wazn",
    "YedPosterior",
    "YedPrior",
    "Zaniah",
    "Zaurak",
    "Zavijava",
    "ZenithStar",
    "Zibel",
    "Zosma",
    "Zubenelakrab",
    "ZubenElgenubi",
    "Zubeneschamali",
    "ZubenHakrabi",
    "Zubra",
];