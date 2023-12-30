export enum StarType {
    MainSeqStar = "MainSeqStar",
    GiantStar = "GiantStar",
    WhiteDwarf = "WhiteDwarf",
    NeutronStar = "NeutronStar",
    BlackHole = "BlackHole",
}

export enum SpectrType {
    M = "M",
    K = "K",
    G = "G",
    F = "F",
    A = "A",
    B = "B",
    O = "O",
    X = "X",
}

export enum PlanetType {
    None = "None",
    Vocano = "Vocano",
    Ocean = "Ocean",
    Desert = "Desert",
    Ice = "Ice",
    Gas = "Gas",
}

export enum VeinType {
    None = "None",
    Iron = "Iron",
    Copper = "Copper",
    Silicium = "Silicium",
    Titanium = "Titanium",
    Stone = "Stone",
    Coal = "Coal",
    Oil = "Oil",
    Fireice = "Fireice",
    Diamond = "Diamond",
    Fractal = "Fractal",
    Crysrub = "Crysrub",
    Grat = "Grat",
    Bamboo = "Bamboo",
    Mag = "Mag",
}

export enum RuleType {
    And = "And",
    Or = "Or",
    Luminosity = "Luminosity",
    DysonRadius = "DysonRadius",
    AverageVeinAmount = "AverageVeinAmount",
    Spectr = "Spectr",
    TidalLockCount = "TidalLockCount",
    OceanType = "OceanType",
    StarType = "StarType",
    GasCount = "GasCount",
    SatelliteCount = "SatelliteCount",
    Birth = "Birth",
    ThemeId = "ThemeId",
    PlanetCount = "PlanetCount",
}

export enum ConditionType {
    Eq = "Eq",
    Neq = "Neq",
    Lt = "Lt",
    Lte = "Lte",
    Gt = "Gt",
    Gte = "Gte",
    Between = "Between",
    NotBetween = "NotBetween",
}

export enum GasType {
    Fireice = 1011,
    Hydrogen = 1120,
    Deuterium = 1121,
}

export enum OceanType {
    None = 0,
    Ice = -2,
    Lava = -1,
    Water = 1000,
    Sulfur = 1116,
}
