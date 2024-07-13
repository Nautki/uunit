import fs from 'fs-extra';
import { prefixes, siUnits } from './units';

let outfiles = {
    lib: fs.readFileSync(__dirname + '/lib.rs').toString()
}

const dir = __dirname + '/../src';
fs.emptyDirSync(dir);

let baseUnits: {
    [unit: string]: {
        name: string,
        unitIdent: string,
        module: string,
        feat?: string[]
    }
} = {};

const capitalize = (s: string) => {
    return s[0].toUpperCase() + s.substring(1)
}

const snake = (s: string) => {
    return s.replace(/[A-Z]+/gm, (rep) => '_' + rep.toLowerCase()).replace(/^_*/, '')
}

const uintTypenum = (n: bigint) => {
    if (n == 0n) {
        return 'UTerm'
    }

    let bit = n % 2n;
    let rest = n >> 1n;
    return `UInt<${uintTypenum(rest)}, B${bit}>`
}

const typenum = (n: bigint) => {
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

const dim = (_units: { [unit: string]: bigint }) => {
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

const generics = ['TScaling', ...Object.values(baseUnits).map(u => `T${u.name}`)];
const assoc = ['Scaling', ...Object.values(baseUnits).map(u => u.name)];

const dimTypeGenericsDef = generics.map(u => `${u}: Integer`).join(', ');
const dimTypeGenerics = generics.join(', ');
const dimTypeDef = `DimensionStruct<${dimTypeGenericsDef}>`;
out('lib', `
#[derive(Clone, Copy)]
pub struct ${dimTypeDef} {
${assoc.map(u => `    ${snake(u)}: PhantomData<T${u}>`).join(',\n')}
}
impl <${dimTypeGenericsDef}> DimensionStruct<${dimTypeGenerics}> {
    pub fn new() -> Self {
        Self {
${assoc.map(u => `            ${snake(u)}: PhantomData`).join(',\n')}
        }
    }
}
pub trait Dimension {
${assoc.map(u => `    type ${u}: Integer;`).join('\n')}
}
impl <${dimTypeGenericsDef}> Dimension for DimensionStruct<${dimTypeGenerics}> {
${assoc.map(u => `    type ${u} = T${u};`).join('\n')}
}
`)

const outUnit = (module: string, unit: string, feat: string[] = []) => {
    let unitIdent = `Unit${capitalize(unit)}`;
    let name = capitalize(unit);

    let cfg = '';
    if (feat?.length! > 0) {
        cfg = `#[cfg(${feat!.map(f => `feature = "${f}"`).join(', ')})]\n`;
    }

    out(module, `
${cfg}pub type ${unitIdent} = DimensionStruct<P1, ${dim({ [unit]: 1n })}>;
${cfg}pub type ${name}<T> = Quantity<T, Unit${name}>;
    `);
}

let allUnits: {
    name: string,
    prefix?: string,
    scaling?: number,
}[] = [];

for (const [unit, data] of Object.entries(baseUnits) as any) {
    out('lib', `
pub mod ${data.module};
pub use ${data.module}::*;
    `);

    out(data.module, `
pub type ${data.unitIdent} = ${dim({ [unit]: 1n })};
pub type ${data.name}<T> = Quantity<T, ${data.unitIdent}>;
    `);

    allUnits.push({ name: capitalize(unit) });

    if (!('usePrefix' in data) || data.usePrefix) {
        for (const [prefix, pow] of Object.entries(prefixes)) {
            const name = capitalize(prefix + unit);
            out(data.module, `
#[cfg(feature = "si_${prefix}")]
pub type Unit${name} = ${dim({ e10: BigInt(pow), [unit]: 1n })};
#[cfg(feature = "si_${prefix}")]
pub type ${name}<T> = Quantity<T, Unit${name}>;
            `);
            allUnits.push({ name, prefix, scaling: pow });
        }
    }
}

out('lib', `
pub trait DimExt: Copy {
${allUnits.map(u => `
${u.prefix ? `    #[cfg(feature = "si_${u.prefix}")]` : ''}
    fn as_${snake(u.name)}(self) -> ${u.name}<Self> {
        Quantity::new(self, Unit${u.name}::new())
    }`).join('\n')}
}
`)

const prims = `i8, i16, i32, i64, i128, isize,
u8, u16, u32, u64, u128, usize,
f32, f64`.split(',').map(s => s.trim());

prims.forEach(p => out('lib', `impl DimExt for ${p} {}`));


out('lib', `
impl <T, D: Dimension> Quantity<T, D> {
${allUnits.map(u => `
${u.prefix ? `    #[cfg(feature = "si_${u.prefix}")]` : ''}
    fn as_${snake(u.name)}(self) -> ${u.name}<T> {
        Quantity::new(self.value, Unit${u.name}::new())
    }`).join('\n')}
}
`)

for (const [path, data] of Object.entries(outfiles)) {
    fs.writeFileSync(__dirname + '/../src/' + path.split('::').join('/') + '.rs', data)
}