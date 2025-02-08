let baseUrl = 'http://127.0.0.1:8080/api';

let segments = []
let intersectingSegments = []
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


    if (drawingMode && startPoint) {
        let previewVector = createVector(mouseXC, mouseYC);
        stroke(0, 0, 0)
        line(
            startPoint.x, startPoint.y,
            previewVector.x, previewVector.y,
            color(150)
        );
    }


    drawSegments()


}

function drawSegments() {
    for (let segmentIndex in segments) {

        textSize(14);
        let midX = (segments[segmentIndex].s.x + segments[segmentIndex].e.x) / 2;
        let midY = (segments[segmentIndex].s.y + segments[segmentIndex].e.y) / 2;

        let strokeColor = null
        let fillColor = null
        if (intersectingSegments[segmentIndex]) {
            strokeColor = [200, 100, 100]
            fillColor = [200, 100, 100]

            for (let intersecPoint of intersectingSegments[segmentIndex]) {
                stroke(...strokeColor)
                fill(...fillColor)
                circle(intersecPoint.dimensions[0], intersecPoint.dimensions[1],
                    5)
            }
        }
        else {
            strokeColor = [100, 100, 100, 100]
            fillColor = [100, 100, 100, 100]
        }

        stroke(...strokeColor)
        fill(...fillColor)
        line(
            segments[segmentIndex].s.x,
            segments[segmentIndex].s.y,
            segments[segmentIndex].e.x,
            segments[segmentIndex].e.y
        )
    }
}

async function findIntersected() {
    let data = []
    for (let segment of segments) {
        data.push(
            [
                { dimensions: [segment.s.x, segment.s.y] },
                { dimensions: [segment.e.x, segment.e.y] }
            ]
        )
    }
    return fetch('http://127.0.0.1:8080/api/intersectam', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ segments: data })
    });
}

function randomSegments() {
    for (let i = 0; i < 5; i++) {
        segments.push(
            new Segment(
                p5.Vector.random2D().mult(random(50, 300)),
                p5.Vector.random2D().mult(random(50, 300))
            )
        );
    }
    findIntersected()
        .then(res => {
            return res.json()
        })
        .then(res => {
            intersectingSegments = res
        })
        .catch(err => console.log(err))
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
        let newSegemnt = new Segment(endPoint, startPoint);
        segments.push(newSegemnt);

        findIntersected()
            .then(res => {
                return res.json()
            })
            .then(res => {
                intersectingSegments = res
            })
            .catch(err => console.log(err))


        drawingMode = false;
        startPoint = null;
    }
}

function clearSegments() {
    segments = []
    intersectingSegments = []
}
