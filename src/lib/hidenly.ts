import AmauiLz77 from '@amaui/lz77';

function str2bin(text: string): string {
    const bin: string[] = [];
    for (let i = 0; i < text.length; i++) {
        bin.push(text.charCodeAt(i).toString(2).padStart(8, '0'));
    }
    return bin.join(' ');
}

function wrap(string: string): string {
    return "\uFEFF" + string + "\uFEFF";
}

function unwrap(string: string): string {
    const tmp: string[] = string.split('\uFEFF');
    if (tmp.length == 1) return string;
    return tmp[1];
}

function bin2str(bin: string): string {
    const text: string[] = [];
    const binArr: string[] = bin.split(' ');
    for (let i = 0; i < binArr.length; i++) {
        text.push(String.fromCharCode(parseInt(binArr[i], 2)));
    }
    return text.join('');
}

// function bin2hidden(str: string): string {
//     return str
//         .replace(/ /g, "\u2060")
//         .replace(/0/g, "\u200B")
//         .replace(/1/g, "\u200C");
// }

// function hidden2bin(str: string): string {
//     return str
//         .replace(/\u2060/g, " ")
//         .replace(/\u200B/g, "0")
//         .replace(/\u200C/g, "1");
// }

const hexCharMap: { [key: string]: string } = {
    ' ': "\u200B\u200B",
    '0': "\u200C\u200C",
    '1': "\u200D\u200D",
    '2': "\u200E\u200E",
    '3': "\u200F\u200F",
    '4': "\u2060\u2060",
    '5': "\u202B\u202B",
    '6': "\u202C\u202C",
    '7': "\u202D\u202D",
    '8': "\u200B\u202C",
    '9': "\u200B\u200D",
    "A": "\u200B\u200E",
    "B": "\u200B\u200F",
    "C": "\u200B\u2060",
    "D": "\u200B\u202B",
    "E": "\u200B\u202C",
    "F": "\u200B\u200D",
};

function stringToHex(str: string): string {
    return str.split('').map(c => c.charCodeAt(0).toString(16).padStart(2, "0")).join('');
}

function hexToString(str: string): string {
    return str.split('')
        .filter(p => !!p)
        .map(c => String.fromCharCode(parseInt(c, 16)))
        .join('');
}

function hexToUnicode(str: string): string {
    const hex: string = stringToHex();
    return hex.split('').map(c => hexCharMap[c] || '').join('');
}

function unicodeToHex(hex: string): string {
    let str = '';
    for (let i = 0; i < hex.length; i += 2) {
        const hexCode = hex.substr(i, 2);
        const char = Object.keys(hexCharMap).find((key) => hexCharMap[key] === hexCode);
        if (char) {
            str += String.fromCharCode(parseInt(char, 16));
        }
    }
    return str;
}

function bin2hidden(str: string): string {
    return str
        .replace(/ /g, "\u200B")
        .replace(/0/g, "\u200C")
        .replace(/1/g, "\u200D");
}

function hidden2bin(str: string): string {
    return str
        .replace(/\u200B/g, " ")
        .replace(/\u200C/g, "0")
        .replace(/\u200D/g, "1");
}

export function hideMessage(shown: string, hidden: string): string {
    if (!hidden) return shown;

    const hiddenWrapped = wrap(hexToUnicode(hidden));
    console.log("HiddenWrapped: '", [...hiddenWrapped], "'");

    if (shown.length === 1) {
        return String([shown[0], hiddenWrapped].join(""));
    }

    const half = shown.length / 2;
    const resultString = [shown.slice(0, half), hiddenWrapped, shown.slice(half)].join("");
    return String(resultString);
}

export function unhideMessage(encoded: string): string {
    if (!encoded) return encoded;
    const encodedUnwrapped = unwrap(encoded);
    const message = unicodeToHex(encodedUnwrapped);
    return String(message);
}