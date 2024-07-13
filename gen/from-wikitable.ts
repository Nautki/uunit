const table = `
| [[hertz]]
| align = "center"| Hz
| [[frequency]]
| align = "center"| 1/s
| align = "center"| s<sup>&minus;1</sup>
|-
| [[radian]]
| align = "center"| rad
| [[angle]]
| align = "center"| m/m
| align="center" | 1
|-
| [[steradian]]
| align = "center"| sr
| [[solid angle]]
| align = "center"| m<sup>2</sup>/m<sup>2</sup>
| align="center" | 1
|-
| [[newton (unit)|newton]]
| align = "center"| N
| [[force]], [[weight]]
| align = "center"| kg⋅m/s<sup>2</sup>
| align = "center"| kg⋅m⋅s<sup>&minus;2</sup>
|-
| [[pascal (unit)|pascal]]
| align = "center"| Pa
| [[pressure]], [[Stress (physics)|stress]]
| align = "center"| N/m<sup>2</sup>
| align = "center"| kg⋅m<sup>&minus;1</sup>⋅s<sup>&minus;2</sup>
|-
| [[joule]]
| align = "center"| J
| [[energy]], [[Mechanical work|work]], [[heat]]
| align = "center"| m⋅N, C⋅V, W⋅s
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;2</sup>
|-
| [[watt]]
| align = "center"| W
| [[Power (physics)|power]], [[radiant flux]]
| align = "center"| J/s, V⋅A
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;3</sup>
|-
| [[coulomb]]
| align = "center"| C
| [[electric charge]] or [[quantity of electricity]]
| align = "center"| s⋅A, F⋅V
| align = "center"| s⋅A
|-
| [[volt]]
| align = "center"| V
| [[voltage]], [[electrical potential difference]], [[electromotive force]]
| align = "center"| W/A, J/C 
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;3</sup>⋅A<sup>&minus;1</sup> 
|-
| [[farad]]
| align = "center"| F
| [[capacitance|electrical capacitance]]
| align = "center"| C/V, s/Ω
| align = "center"| kg<sup>&minus;1</sup>⋅m<sup>&minus;2</sup>⋅s<sup>4</sup>⋅A<sup>2</sup> 
|-
| [[ohm (unit)|ohm]]
| align = "center"| Ω
| [[electrical resistance and conductance|electrical resistance]], [[electrical impedance|impedance]], [[Reactance (electronics)|reactance]]
| align = "center"| 1/S, V/A
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;3</sup>⋅A<sup>&minus;2</sup>
|-
| [[Siemens (unit)|siemens]]
| align = "center"| S
| [[electrical conductance]]
| align = "center"| 1/Ω, A/V
| align = "center"| kg<sup>&minus;1</sup>⋅m<sup>&minus;2</sup>⋅s<sup>3</sup>⋅A<sup>2</sup>
|-
| [[Weber (unit)|weber]]
| align = "center"| Wb
| [[magnetic flux]]
| align = "center"| J/A, T⋅m<sup>2</sup>,V⋅s
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;2</sup>⋅A<sup>&minus;1</sup>
|-
| [[tesla (unit)|tesla]]
| align = "center"| T
| [[magnetic field|magnetic induction]], [[magnetic flux density]]
| align = "center"| V⋅s/m<sup>2</sup>, Wb/m<sup>2</sup>, N/(A⋅m)  
| align = "center"| kg⋅s<sup>&minus;2</sup>⋅A<sup>&minus;1</sup>
|-
| [[henry (unit)|henry]]
| align = "center"| H
| [[inductance|electrical inductance]]
| align = "center"| V⋅s/A, Ω⋅s, Wb/A 
| align = "center"| kg⋅m<sup>2</sup>⋅s<sup>&minus;2</sup>⋅A<sup>&minus;2</sup>
|-
| [[Celsius|degree Celsius]]
| align = "center"| °C
| [[temperature]] relative to [[Absolute zero|273.15 K]]
| align = "center"| K 
| align = "center"| K
|-
| [[lumen]]
| align = "center"| lm
| [[luminous flux]]  
| align = "center"| cd⋅sr
| align = "center"| cd
|-
| [[lux]]
| align = "center"| lx
| [[illuminance]]
| align = "center"| lm/m<sup>2</sup>
| align = "center"| cd⋅m<sup>&minus;2</sup>
|-
| [[becquerel]]
| align = "center"| Bq
| [[radioactivity]] (decays per unit time)
| align = "center"| 1/s
| align = "center"| s<sup>&minus;1</sup>
|-
| [[gray (unit)|gray]]
| align = "center"| Gy
| [[absorbed dose]] (of [[ionizing radiation]])
| align = "center"| J/kg 
| align = "center"| m<sup>2</sup>⋅s<sup>&minus;2</sup>
|-
| [[sievert]]
| align = "center"| Sv
| [[equivalent dose]] (of [[ionizing radiation]])
| align = "center"| J/kg 
| align = "center"| m<sup>2</sup>⋅s<sup>&minus;2</sup>
|-
| [[katal]]
| align = "center"| kat
| [[catalytic activity]]
| align = "center"| mol/s 
| align = "center"| s<sup>&minus;1</sup>⋅mol
`;

const abbr = {
    kg: 'kilograms',
    m: 'meters',
    s: 'seconds',
    A: 'amperes',
    cd: 'candelas',
    K: 'kelvins',
    '1': '1',
}

let units = table
    .split('|-')
    .map(s => s
        .split('\n')
        .map(s => s
            .trim()
            .replace(/\[\[(?:[^|]*\|)?(.*?)\]\]/, '$1')
            .replace(/(?:(?:\| [^\|]*))*(?:\| (.*))/, '$1')
            .replace(/&minus;/gm, '-')
        )
        .filter(s => s)
    )
    .reduce((acc, curr) => ({
        ...acc,
        [curr[0]]: {
            equiv: curr[4]
                .split('⋅')
                .reduce((acc, s) => {
                    let [_, unit, pow] = s.match(/(\w+)(?:<sup>([\w-]+)<\/sup>)?/)!;
                    return {
                        ...acc,
                        [abbr[unit]]: +(pow ?? 1)
                    }
                }, {})
            }
    }), {});

console.log(JSON.stringify(units, null, 4).replaceAll('"', ''));