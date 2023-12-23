import type {
    SpectrType,
    StarType,
    PlanetType,
    VeinType,
    ConditionType,
    RuleType,
    GasType,
} from "./enums"

declare global {
    declare type integer = number
    declare type float = number

    declare interface GameDesc {
        seed: integer
        starCount?: integer
        resourceMultiplier?: float
    }

    declare interface Galaxy {
        seed: integer
        stars: Star[]
    }

    declare interface Star {
        id: integer
        position: [float, float, float]
        name: string
        mass: float
        lifetime: float
        age: float
        temperature: float
        type: StarType
        spectr: SpectrType
        luminosity: float
        radius: float
        dysonRadius: float
        planets: Planet[]
    }

    declare interface Planet {
        id: integer
        index: integer
        orbitAround: integer
        orbitIndex: integer
        name: string
        isBirth: bool
        orbitRadius: float
        orbitInclination: float
        orbitLongitude: float
        orbitalPeriod: float
        orbitPhase: float
        obliquity: float
        rotationPeriod: float
        rotationPhase: float
        sunDistance: float
        planetType: PlanetType
        habitableBias: float
        temperatureBias: float
        themeProto: ThemeProto
        veins: Vein[]
        gases: [itemId: GasType, rate: float][]
    }

    declare interface ThemeProto {
        id: integer
        name: string
    }

    declare interface Vein {
        veinType: VeinType
        minGroup: integer
        maxGroup: integer
        minPatch: integer
        maxPatch: integer
        minAmount: integer
        maxAmount: integer
    }

    declare namespace Condition {
        export type Eq = { type: ConditionType.Eq; value: float }
        export type Neq = { type: ConditionType.Neq; value: float }
        export type Lt = { type: ConditionType.Lt; value: float }
        export type Lte = { type: ConditionType.Lte; value: float }
        export type Gt = { type: ConditionType.Gt; value: float }
        export type Gte = { type: ConditionType.Gte; value: float }
        export type Between = {
            type: ConditionType.Between
            value: [float, float]
        }
        export type NotBetween = {
            type: ConditionType.NotBetween
            value: [float, float]
        }
    }

    declare type Condition =
        | Condition.Eq
        | Condition.Neq
        | Condition.Lt
        | Condition.Lte
        | Condition.Gt
        | Condition.Gte

    declare namespace Rule {
        export type And = { type: RuleType.And; rules: Rule[] }
        export type Or = { type: RuleType.Or; rules: Rule[] }
        export type Luminosity = {
            type: RuleType.Luminosity
            condition: Condition
        }
        export type DysonRadius = {
            type: RuleType.DysonRadius
            condition: Condition
        }
        export type AverageVeinAmount = {
            type: RuleType.AverageVeinAmount
            vein: VeinType
            condition: Condition
        }
        export type AverageVeinPatch = {
            type: RuleType.AverageVeinPatch
            vein: VeinType
            condition: Condition
        }
        export type Spectr = {
            type: RuleType.Spectr
            spectr: SpectrType[]
        }
        export type TidalLockCount = {
            type: RuleType.TidalLockCount
            condition: Condition
        }
        export type OceanType = {
            type: RuleType.OceanType
            oceanType: integer[]
        }
        export type StarType = {
            type: RuleType.StarType
            starType: StarType[]
        }
        export type GasCount = {
            type: RuleType.GasCount
            condition: Condition
        }
        export type SatelliteCount = {
            type: RuleType.SatelliteCount
            condition: Condition
        }
        export type Birth = {
            type: RuleType.Birth
        }
    }

    declare type Rule =
        | Rule.And
        | Rule.Or
        | Rule.Luminosity
        | Rule.DysonRadius
        | Rule.AverageVeinAmount
        | Rule.AverageVeinPatch
        | Rule.Spectr
        | Rule.TidalLockCount
        | Rule.OceanType
        | Rule.StarType
        | Rule.GasCount
        | Rule.SatelliteCount
        | Rule.Birth

    declare interface WorldGen {
        generate(gameDesc: GameDesc): Promise<Galaxy>
        find(options: {
            gameDesc: Omit<GameDesc, "seed">
            range: [number, number]
            rule: Rule
            concurrency: integer
        }): AsyncGenerator<Galaxy>
        stop(): void
    }

    declare interface Store {
        settings: Settings
        modals: {
            settings: boolean
        }
        selects: {
            resourceMultiplier: boolean
        }
    }

    declare interface Settings {
        concurrency: integer
        darkMode: boolean
        nativeMode: boolean
        starCount: integer
        resourceMultiplier: float
    }
}
