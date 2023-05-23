
export class Alphabet {
    private alphabet: Set<string>
    private taken: Set<string>
    constructor() {
        const alpha = Array.from(Array(52)).map((_, i) => String.fromCharCode(i + 65));
        alpha.forEach(c => {
            this.alphabet.add(c)
            console.log(c)
        })
    }

    assign(x: string): void {
        if (this.alphabet.has(x)) {
            this.taken.add(x)
        }
    }

    free(): Set<string> {
        const _difference = new Set(this.alphabet);
        for (const elem of this.taken) {
            _difference.delete(elem);
        }
        return _difference;
    }

}

// "players" , some hex colour, the symbols they can be. 
// "enemies", some hex colour, the symbols they can be 
export class Entities {
    readonly id: string
    colour: string
    symbols: Alphabet

    constructor(id: string, colour: string) {
        this.id = id;
        this.colour = colour;
        this.symbols = new Alphabet();
    }

    free(): Set<string> { return this.symbols.free() }

}
