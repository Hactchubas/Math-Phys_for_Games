

let baseUrl = 'http://127.0.0.1:8080/api';


let character = {
    pos: null,
    vel: null
}

class Wall {
    constructor(s, e) {
        this.s = s
        this.e = e
        getNormal(s, e)
            .then(res => {
                try {
                    if (res.status >= 200 && res.status < 300) {
                        return res.json()
                    }
                } catch (e) {
                    console.error('Error parsing responde from server:', e);
                }
            })
            .then(res => {
                this.n = createVector(...res.dimensions)
                this.seta
            })
            .catch(e => {
                console.log(e)
            })

    }

    drawNormal() {
        return this.n ? [
            (this.s[0] + this.e[0]) / 2,
            (this.s[1] + this.e[1]) / 2,
            (this.s[0] + this.e[0]) / 2 + 10 * this.n.x,
            (this.s[1] + this.e[1]) / 2 + 10 * this.n.y
        ] : [
            (this.s[0] + this.e[0]) / 2,
            (this.s[1] + this.e[1]) / 2,
            (this.s[0] + this.e[0]) / 2,
            (this.s[1] + this.e[1]) / 2,
        ]
    }
}

/// guardam a posição do mouse no plano cartesiano
var mouseXC, mouseYC = 0

let edges = []
let edgesSize = 0
let drawed = []
let walls = []
let intersectingSegments = []
let last = null

var mouseXC, mouseYC = 0
function setup() {
    let myCanvas = createCanvas(800, 600);
    myCanvas.parent("#canvas-destination");
    frameRate(20);
    textAlign(CENTER, CENTER);
    let w2 = width / 2 - 32
    let h2 = height / 2 - 32
    edges.push(
        new Wall([-w2, h2], [w2, h2]),
        new Wall([w2, h2], [w2, -h2]),
        new Wall([w2, -h2], [-w2, -h2]),
        new Wall([-w2, -h2], [-w2, h2])
    )

    character.pos = createVector(0, 0)
    character.vel = createVector(15, 20)

    for (let wall of edges) {
        walls.push(
            [
                { dimensions: [wall.s[0], wall.s[1]] },
                { dimensions: [wall.e[0], wall.e[1]] }
            ]
        )
    }
    edgesSize = edges.length

}
var stopDraw = false;
var lastEdge = null;
function draw() {
    goCartesian()
    drawCharacter()
    updateVel(character)

    if (drawingMode && startPoint) {
        let previewVector = createVector(mouseXC, mouseYC);
        stroke(0, 0, 0)
        line(
            startPoint.x, startPoint.y,
            previewVector.x, previewVector.y,
        );
    }


}

function updatePos({ vel, pos }) {
    if (pos) {
        const vel_req = { dimensions: [vel.x, vel.y] };
        const pos_req = { dimensions: [pos.x, pos.y] };
        updateCharacterPos(vel_req, pos_req)
            .then(res => {
                try {
                    if (res.status >= 200 && res.status < 300) {
                        return res.json()
                    }
                } catch (e) {
                    console.error('Error parsing responde from server:', e);
                }
            })
            .then(res => {
                character.pos = createVector(...res.dimensions)
            })
            .catch(e => {
                console.log(e)
            })

    }
}
function updateVel({ vel, pos }) {
    if (vel) {
        const vel_req = { dimensions: [vel.x, vel.y] };
        const pos_req = { dimensions: [pos.x, pos.y] };
        fetch(
            baseUrl + '/colisao-refletida',
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    walls: walls,
                    vector: [pos_req, vel_req]
                })
            }
        ).then(res => res.json())
            .then(res => {
                let reflected = res.findIndex(value => value)
                if (reflected != -1 && reflected != lastEdge) {
                    character.vel = createVector(res[reflected].dimensions[0], res[reflected].dimensions[1])
                    lastEdge = reflected
                }
            }).then(_ => {
                updatePos(character)
            })
            .catch(e => {
                console.log(e)
            })
    }
}

async function updateCharacterPos(u, v) {
    return fetch(baseUrl + '/soma', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            v1: u,
            v2: v,
            scalar: null
        })
    });
}


function drawCharacter() {
    let { pos, vel } = character
    ellipse(pos.x, pos.y, 10)
    // seta(
    //     pos.x, pos.y,
    //     pos.x + vel.x, pos.y + vel.y
    // )

    if (last) {
        seta(...last)
    }
}

async function getNormal(s, e) {
    let body = {
        segment: [
            { dimensions: [...s] },
            { dimensions: [...e] }
        ]
    }
    return fetch(baseUrl + '/normal', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(body)
    })
}


let startPoint = null;
let drawingMode = false;
function goCartesian() {
    background(255)

    mouseXC = mouseX - width / 2
    mouseYC = height / 2 - mouseY

    colore(0, 0, 0, 100)
    seta(0, height / 2, width, height / 2)
    seta(width / 2, height, width / 2, 0)

    translate(width / 2, height / 2)
    scale(1, -1, 1)

    for (let edge of Object.values(edges)) {
        strokeWeight(5)
        stroke(0, 0, 0)
        line(edge.s[0], edge.s[1], edge.e[0], edge.e[1])
        strokeWeight(2)
        stroke(256, 0, 0)
        seta(...edge.drawNormal())
    }
    stroke(0, 0, 0)
    strokeWeight(2)
}

/// Atualiza as variáveis globais com as coordenadas do mouse no plano cartesiano
function grabMouse() {
    mouseXC = mouseX - width / 2
    mouseYC = height / 2 - mouseY
}


function mousePressed() {
    if (mouseX > 0 && mouseX < width && mouseY > 0 && mouseY < height) {
        drawingMode = true;
        startPoint = createVector(mouseXC, mouseYC);
        line(
            startPoint.x, startPoint.y,
        )
    }
}

function mouseReleased() {
    if (drawingMode && startPoint) {
        let endPoint = createVector(mouseXC, mouseYC);
        edges.push(
            new Wall([startPoint.x, startPoint.y], [endPoint.x, endPoint.y])
        );
        if (edges.length != edgesSize) {
            for (let wall of edges) {
                walls.push(
                    [
                        { dimensions: [wall.s[0], wall.s[1]] },
                        { dimensions: [wall.e[0], wall.e[1]] }
                    ]
                )
            }
        }
        drawingMode = false;
        startPoint = null;
    }
}

/** Renderiza texto corretamente no plano cartesiano
 *  @param str Texto a ser escrito
 *  @param x Posição horizontal do canto inferior esquerdo texto
 *  @param y Posição vertical do canto inferior esquerdo texto
 */
function texto(str, x, y) {
    push()
    translate(x, y)
    scale(1, -1)
    translate(-x, -y)

    // desenha o texto normalmente
    text(str, x, y)
    pop()
}


/* Define as cores de preenchimento e de contorno com o mesmo valor.
 * Há várias opções de trabalho em RGB nesse caso:
 *  - caso c1,c2,c3 e c4 sejam passados, o efeito padrão é uma cor RGBA
 *  - caso c1,c2 e c3 sejam passados, tem-se uma cor RGB.
 *  - caso c1 e c2 sejam passados, c1 é um tom de cinza e c2 é opacidade.
 *  - caso apenas c1 seja passado, c1 é um tom de cinza.
 */
function colore(c1, c2, c3, c4) {
    if (c4 != null) {
        fill(c1, c2, c3, c4)
        stroke(c1, c2, c3, c4)
        return
    }
    if (c3 != null) {
        fill(c1, c2, c3)
        stroke(c1, c2, c3)
        return
    }

    if (c2 == null) {
        fill(c1)
        stroke(c1)
    }
    else {
        fill(c1, c1, c1, c2)
        stroke(c1, c1, c1, c2)
    }
}

/* Desenha um segmento de reta com seta do ponto (x1,y1) para (x2,y2)
 */
function seta(x1, y1, x2, y2) {
    // o segmento de reta
    line(x1, y1, x2, y2)
    var dx = x2 - x1, dy = y2 - y1
    var le = sqrt(dx * dx + dy * dy) // comprimento do vetor
    // o vetor v é unitário paralelo ao segmento, com mesmo sentido
    var vx = dx / le, vy = dy / le
    // o vetor u é unitário e perpendicular ao segmento
    var ux = -vy
    var uy = vx
    // a cabeça triangular
    triangle(x2, y2,
        x2 - 5 * vx + 2 * ux, y2 - 5 * vy + 2 * uy,
        x2 - 5 * vx - 2 * ux, y2 - 5 * vy - 2 * uy)
}


function resetCharacter() {

    character.pos = createVector(0, 0)
    character.vel = createVector(15, 20)
}

function clearWalls() {
    let w2 = width / 2 - 32
    let h2 = height / 2 - 32
    edges = [
        new Wall([-w2, h2], [w2, h2]),
        new Wall([w2, h2], [w2, -h2]),
        new Wall([w2, -h2], [-w2, -h2]),
        new Wall([-w2, -h2], [-w2, h2])
    ]

    walls = []
    for (let wall of edges) {
        walls.push(
            [
                { dimensions: [wall.s[0], wall.s[1]] },
                { dimensions: [wall.e[0], wall.e[1]] }
            ]
        )
    }
}