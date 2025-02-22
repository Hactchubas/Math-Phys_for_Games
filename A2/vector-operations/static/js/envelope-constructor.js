let baseUrl = 'http://127.0.0.1:8080/api';

let points = []
let boundings = []
class Segment {
    constructor(s, e) {
        this.s = s
        this.e = e
    }
}

/// guardam a posição do mouse no plano cartesiano
var mouseXC, mouseYC = 0

function setup() {
    let myCanvas = createCanvas(800, 600);
    myCanvas.parent("#canvas-destination");
    frameRate(20);
    textAlign(CENTER, CENTER);
    let w2 = width / 15
    let h2 = height / 15


}


function draw() {
    goCartesian()

    drawPoints()

    if (boundings.length > 0) {
        drawBoundings()
    }
}

function drawPoints() {
    for (let point of points) {

        textSize(14);

        let strokeColor = [100, 100, 100, 100]
        let fillColor = [100, 100, 100, 100]


        stroke(...strokeColor)
        fill(...fillColor)
        circle(point.x, point.y, 5)
    }
}

async function makeEnvelope(type) {
    let data = []
    for (let point of points) {
        data.push(
            { dimensions: [point.x, point.y] }
        )
    }
    let res = await fetch('http://127.0.0.1:8080/api/envoltorios-construtor', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            points: data,
            bounding_type: type
        })
    })

    res = await res.json()
    return res
}

function randomPoints() {
    for (let i = 0; i < 5; i++) {
        points.push(
            p5.Vector.random2D().mult(random(0, 300))
        );
    }
    updateEnvelope()
}

class Bounding {
    methods = {
        "AABB": {
            constructor: (min_max) => {
                return [
                    min_max.min.dimensions[0], min_max.min.dimensions[1],
                    min_max.max.dimensions[0] - min_max.min.dimensions[0], min_max.max.dimensions[1] - min_max.min.dimensions[1]
                ]
            },
            drawer: rect,
            color: [200,0,0, 50]
        },
        "Sphere": {
            constructor: (sphere) =>{
                return [
                    sphere.center.dimensions[0], sphere.center.dimensions[1],
                    sphere.radius * 2, sphere.radius * 2
                ]
            },
            drawer: ellipse,
            color: [0,200,0, 50]
        }
    }
    constructor(type, params) {
        this.drawer = this.methods[type].drawer
        this.params = this.methods[type].constructor(params)
        this.color = this.methods[type].color
    }

    draw() {
        colore(...this.color)
        this.drawer(...this.params)
    }

}


async function updateEnvelope() {
    boundings = []
    const checkedIds = [...document.querySelectorAll('input[type="checkbox"]:checked')]
        .map(checkbox => checkbox.id)
        .forEach(async bounding => {
            let res = await makeEnvelope(bounding)
            boundings.push(
                new Bounding(bounding, res)
            )
        })
}

function drawBoundings() {
    for (let bounding of boundings) {
        bounding.draw()
    }
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

    strokeWeight(5)
    stroke(0, 0, 0)
    strokeWeight(2)
}

/// Atualiza as variáveis globais com as coordenadas do mouse no plano cartesiano
function grabMouse() {
    mouseXC = mouseX - width / 2
    mouseYC = height / 2 - mouseY
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

function mouseClicked() {
    if (mouseXC > -400 && mouseXC < 400 && mouseYC > -300 && mouseYC < 300) {
        console.log(mouseXC, mouseYC);
        points.push(
            createVector(mouseXC, mouseYC)
        )
        updateEnvelope()
    }
}

function mouseReleased() {

}

function clearPoints() {
    points = []
    boundings = []
}
