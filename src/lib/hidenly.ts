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
    const tmp: string[] = string.split("\uFEFF");
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

function bin2hidden(str: string): string {
    return str
        .replace(/ /g, "\u2060")
        .replace(/0/g, "\u200B")
        .replace(/1/g, "\u200C");
}

function hidden2bin(str: string): string {
    return str
        .replace(/\u2060/g, " ")
        .replace(/\u200B/g, "0")
        .replace(/\u200C/g, "1");
}

export function hideMessage(shown: string, hidden: string): string {
    if (!hidden) return shown;

    const hiddenBin = str2bin(hidden);
    const hiddenWrapped = wrap(bin2hidden(hiddenBin));

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
    const message = bin2str(hidden2bin(encodedUnwrapped))
    return String(message);
}