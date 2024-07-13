export const prefixes = {
    quetta: 30,
    ronna: 27,
    yotta: 24,
    zetta: 21,
    exa: 18,
    peta: 15,
    tera: 12,
    giga: 9,
    mega: 6,
    kilo: 3,
    hecto: 2,
    deca: 1,
    deci: -1,
    centi: -2,
    milli: -3,
    micro: -6,
    nano: -9,
    pico: -12,
    femto: -15,
    atto: -18,
    zepto: -21,
    yocto: -24,
    ronto: -27,
    quecto: -30
}

type UnitConversion = number;

type UnitDescription = {
    /** NOT THE SAME AS SI BASE UNITS */
    base: {
        [unit: string]: {
            /**
             * SEMANTIC equivalence. E.g. grays and sieverts have the same SI base units,
             * but semantically they are different, so they go in the base section.
             */
            equiv?: Record<string, UnitConversion>,
            conversions?: Record<string, UnitConversion>,
        }
    }

    extra: {
        [unit: string]: {
            usePrefix?: false,
        }
    }

    aliases: {
        [unit: string]: {
            usePrefix?: false,
            equiv: {
                [index: string]: UnitConversion,
            } & {
                scaling?: number
            }
        }
    }
}

export const siUnits = {
    base: {
        // SI base (except kilogram)
        seconds: {},
        meters: {},
        grams: {},
        amperes: {},
        kelvin: {},
        moles: {},
        candelas: {},

        // not actually si base unit but w/e
        byte: {},
    },

    extra: {
        // SI derived
        radians: {
            usePrefix: false
        },
        steradians: {
            usePrefix: false
        },
        celsius: {
            usePrefix: false
        },
        minutes: {
            usePrefix: false
        },
        hours: {
            usePrefix: false
        },
        days: {
            usePrefix: false
        },
        astronomicalUnits: {
            usePrefix: false
        },
        degrees: {
            usePrefix: false
        },
        arcminutes: {
            usePrefix: false
        },
        arcseconds: {
            usePrefix: false
        },
        ares: {},
        liters: {},
        daltons: {},
        electronvolts: {},
        nepers: {},
        bels: {},
        atmospheres: {},
        bars: {},
        parsec: {},
        millimetersOfMercury: {
            usePrefix: false
        }
    },

    aliases: {
        micron: {
            usePrefix: false,
            equiv: {
                micrometers: 1,
            }
        },
        fermi: {
            usePrefix: false,
            equiv: {
                femtometers: 1,
            }
        },
        metricTon: {
            usePrefix: false,
            equiv: {
                megagrams: 1
            }
        },
        hertz: {
            equiv: {
                seconds: -1
            }
        },
        newtons: {
            equiv: {
                kilograms: 1,
                meters: 1,
                seconds: -2
            }
        },
        pascals: {
            equiv: {
                kilograms: 1,
                meters: -1,
                seconds: -2
            }
        },
        joules: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -2
            }
        },
        watts: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -3
            }
        },
        coulombs: {
            equiv: {
                seconds: 1,
                amperes: 1
            }
        },
        volts: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -3,
                amperes: -1
            }
        },
        farads: {
            equiv: {
                kilograms: -1,
                meters: -2,
                seconds: 4,
                amperes: 2
            }
        },
        ohms: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -3,
                amperes: -2
            }
        },
        siemens: {
            equiv: {
                kilograms: -1,
                meters: -2,
                seconds: 3,
                amperes: 2
            }
        },
        webers: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -2,
                amperes: -1
            }
        },
        teslas: {
            equiv: {
                kilograms: 1,
                seconds: -2,
                amperes: -1
            }
        },
        henries: {
            equiv: {
                kilograms: 1,
                meters: 2,
                seconds: -2,
                amperes: -2
            }
        },
        lumens: {
            equiv: {
                candelas: 1
            }
        },
        lux: {
            equiv: {
                candelas: 1,
                meters: -2
            }
        },
        becquerels: {
            equiv: {
                seconds: -1
            }
        },
        grays: {
            equiv: {
                meters: 2,
                seconds: -2
            }
        },
        sieverts: {
            equiv: {
                meters: 2,
                seconds: -2
            }
        },
        katals: {
            equiv: {
                seconds: -1,
                moles: 1
            }
        }
    }
} satisfies UnitDescription