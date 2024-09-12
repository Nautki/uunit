import fs from 'fs-extra';
import { prefixes, siUnits } from './units';

let outfiles = {
    lib: fs.readFileSync(__dirname + '/lib.rs').toString()
}

const dir = __dirname + '/../src';
fs.emptyDirSync(dir);
/*
let baseUnits: {
    [unit: string]: {
        name: string,
        unitIdent: string,
        module: string,
        feat?: string[]
    }
} = {};
*/
const capitalize = (s: string) => {
    return s[0].toUpperCase() + s.substring(1)
}

const snake = (s: string) => {
    return s.replace(/[A-Z]+/gm, (rep) => '_' + rep.toLowerCase()).replace(/^_*/, '')
}

/*
const uintTypenum = (n: number) => {
    if (n == 0) {
        return 'UTerm'
    }

    let bit = n % 2;
    let rest = n >> 1;
    return `UInt<${uintTypenum(rest)}, B${bit}>`
}*/

const typenum = (n: number) => {
    if (n < 0n) {
        return `N${-n}`
    } else if (n > 0n) {
        return `P${n}`
    } else {
        return `Z0`
    }
}

const out = (module: string, s: string) => {
    s = s.trim();

    if (!(module in outfiles)) {
        outfiles[module] = `use crate::*;`;
    }

    outfiles[module] += '\n' + s;
}
/*
const _dim = (_units: { [unit: string]: bigint }) => {
    return `DimensionStruct<${typenum(_units.e10 ?? 1n)}, ${Object.keys(baseUnits).map(u => typenum(_units[u as any] ?? 0n)).join(', ')}>`
}

const addBaseUnit = (module: string, s: string, data: {
    feat?: string[]
} = {}) => {
    const unitIdent = 'Unit' + capitalize(s);
    baseUnits[s] = {
        name: capitalize(s),
        unitIdent,
        module,
        ...data
    }
    return baseUnits[s]
}

for (const item of [siUnits.base, siUnits.extra]) {
    for (const [s, data] of Object.entries(item)) {
        let module = snake(s);

        addBaseUnit(module, s, data as any);
    }
}


for (const [unit, data] of Object.entries(baseUnits) as any) {
    out('lib', `
pub mod ${data.module};
pub use ${data.module}::*;
    `);

    out(data.module, `
pub type ${data.unitIdent} = ${_dim({ [unit]: 1n })};
pub type ${data.name}<T> = Quantity<T, ${data.unitIdent}>;
    `);

    units.push({ name: capitalize(unit) });

    if (!('usePrefix' in data) || data.usePrefix) {
        for (const [prefix, pow] of Object.entries(prefixes)) {
            const name = capitalize(prefix + unit);
            out(data.module, `
#[cfg(feature = "si_${prefix}")]
pub type Unit${name} = ${_dim({ e10: BigInt(pow), [unit]: 1n })};
#[cfg(feature = "si_${prefix}")]
pub type ${name}<T> = Quantity<T, Unit${name}>;
            `);
            units.push({ name, prefix, scaling: pow });
        }
    }
}*/



type UnitKind = 'base' | 'prefixed' | 'alias';
type Unit = {
    module: string,
    name: string,
    upperName: string,
    snakeName: string,
    features: string[],
    kind: UnitKind ,
    prefix?: string,
    dim: Dim,
};
let units: {
    [lowerCamelName: string]: Unit,
} & {
    map: Unit[]['map'],
    base: Unit[]
} & Iterable<Unit> = {} as any;

Object.defineProperties(units, {
    map: {
        enumerable: false,
        get: () => {
            return (...args) => (Object.values(units) as any).map(...args);
        }
    },
    [Symbol.iterator]: {
        enumerable: false,
        get: () => {
            return () => Object.values(units)[Symbol.iterator]();
        }
    },
    base: {
        enumerable: false,
        get: () => {
            return Object.values(units).filter(u => u.kind == 'base')
        }
    }
})

type Dim = {
    [index: string]: number
}

const normalizeDim = (dim: Dim) => {
    let acc: Dim = {
        scaling: 0
    };

    for (const [name, outerPow] of Object.entries(dim)) {
        if (name == 'scaling') {
            acc[name] += outerPow;
            continue;
        }

        const unit = units[name];

        if (!unit) {
            throw new Error(`Unknown unit ${name}`)
        }

        for (const [name, innerPow] of Object.entries(unit.dim)) {
            if (!(name in acc)) {
                acc[name] = innerPow * outerPow;
            } else {
                acc[name] += innerPow * outerPow;
            }
        }
    }

    return acc;
}

const dim = (dim: Dim) => {
    return `DimensionStruct<${typenum(dim.scaling ?? 0)}, ${units.base.map(u => typenum(dim[u.name] ?? 0)).join(', ')}>`
}

const addUnit = (module: string, name: string, kind: UnitKind, dim: Dim, options: Partial<Unit> = {}) => {
    let unit: Unit = {
        module,
        name,
        upperName: capitalize(name),
        snakeName: snake(name),
        features: [],
        kind,
        dim: kind == 'base' ? dim : normalizeDim(dim),
        ...options
    }

    units[name] = unit;

    return unit;
}

for (const item of [siUnits.base, siUnits.extra, siUnits.aliases]) {
    for (const [name, data] of Object.entries(item)) {
        let module = snake(name);
        const kind = name in siUnits.aliases ? 'alias' : 'base';

        addUnit(module, name, kind, kind == 'base' ? {
            [name]: 1
        } : (data as any).equiv);

        if (!('usePrefix' in data) || data.usePrefix) {
            for (const [prefix, scaling] of Object.entries(prefixes)) {
                const prefixedName = prefix + name;

                addUnit(module, prefixedName, 'prefixed', {
                    scaling,
                    [name]: 1,
                });
            }
        }
    }
}

const prims = `i8, i16, i32, i64, i128, isize,
u8, u16, u32, u64, u128, usize,
f32, f64`.split(',').map(s => s.trim());

prims.forEach(p => out('lib', `impl WithUnits for ${p} {
    type Output<D: Dimension> = Quantity<${p}, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}`));


const generics = ['Scaling', ...units.base.map(u => `${u.upperName}`)];
const aGenerics = generics.map(u => `A${u}`);
const aGenericsDef = generics.map(u => `A${u}: Integer`);
const bGenerics = generics.map(u => `B${u}`);
const bGenericsDef = generics.map(u => `B${u}: Integer`);

const dimTypeGenericsDef = generics.map(u => `${u}: Integer`).join(', ');
const dimTypeGenerics = generics.join(', ');
const dimTypeDef = `DimensionStruct<${dimTypeGenericsDef}>`;
out('lib', `
#[derive(Clone, Copy)]
pub struct ${dimTypeDef} {
${generics.map(u => `    ${snake(u)}: PhantomData<${u}>`).join(',\n')}
}
impl <${dimTypeGenericsDef}> DimensionStruct<${dimTypeGenerics}> {
    pub fn new() -> Self {
        Self {
${generics.map(u => `            ${snake(u)}: PhantomData`).join(',\n')}
        }
    }
}
pub trait Dimension {
${generics.map(u => `    type ${u}: Integer;`).join('\n')}
}
impl <${dimTypeGenericsDef}> Dimension for DimensionStruct<${dimTypeGenerics}> {
${generics.map(u => `    type ${u} = ${u};`).join('\n')}
}
`);

const genDimOp = (op: string, powOp: string) => {
    out('lib',`  
impl <${
    generics.map((g, i) => `${aGenerics[i]}: Integer + ${capitalize(powOp)}<${bGenerics[i]}>`).join(', ')
}, ${
    bGenericsDef
}> ${capitalize(op)}<DimensionStruct<${bGenerics.join(', ')}>> for DimensionStruct<${aGenerics.join(', ')}>
where ${
    generics.map((g, i) => `<${aGenerics[i]} as ${capitalize(powOp)}<${bGenerics[i]}>>::Output: Integer`).join(', ')
} {
    type Output = DimensionStruct<${
        generics.map((g, i) => `<${aGenerics[i]} as ${capitalize(powOp)}<${bGenerics[i]}>>::Output`)
    }>;

    fn ${snake(op)}(self, rhs: DimensionStruct<${bGenerics.join(', ')}>) -> Self::Output {
        DimensionStruct::new()
    }
}`);
}

genDimOp('mul', 'add');
genDimOp('div', 'sub');

[...new Set(units.map(u => u.module))].map(mod => {
    out('lib', `
pub mod ${mod};
pub use ${mod}::*;
    `);
})

for (const unit of units) {
    console.log(unit);

    let cfg = '';
    if (unit.features?.length! > 0) {
        cfg = `#[cfg(${unit.features!.map(f => `feature = "${f}"`).join(', ')})]\n`;
    }

    out(unit.module, `
${cfg}pub type Unit${unit.upperName} = ${dim(unit.dim)};
${cfg}pub type ${unit.upperName}<T> = Quantity<T, Unit${unit.upperName}>;
    `);
}

for (const [path, data] of Object.entries(outfiles)) {
    fs.writeFileSync(__dirname + '/../src/' + path.split('::').join('/') + '.rs', data)
}