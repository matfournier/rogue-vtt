
export type Glyph = {
    c: string,
    x: number,
    y: number,
}

export class TextRender {
    readonly px;
    readonly tileSize;
    readonly fontFace;
    readonly font;
    readonly delta_x;
    readonly delta_y;


    constructor(px: number, tileSize: number, fontFace: string) {
        this.px = px
        this.tileSize = tileSize
        this.fontFace = fontFace
        this.font = `${px}pt ${this.font}`

        this.delta_x = tileSize - (tileSize - px);
        this.delta_y = tileSize - px;
    }

    // can we avoid setting the font every time 
    // need to take in more than a token, but not sure whaat
    // token and list of hexes 
    // will also need to filter tokens to only show what is visible in the camera 
    // only renders the main canvas 
    render(context: CanvasRenderingContext2D, colour: string, tokens: Array<Glyph>): void {
        let originalFillStyle = context.fillStyle;
        context.font = this.font
        tokens.forEach(token => {
            // ideally I'm not calculating this every frame, but it's static until the camera moves? 
            context.fillText(token.c, (token.x * this.tileSize) + this.delta_x, (token.y * this.tileSize) + this.delta_x)
        })
    }

    // need to have a way to render a sidebar 
}