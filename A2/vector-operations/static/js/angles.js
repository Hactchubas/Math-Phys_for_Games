
let baseUrl = 'http://127.0.0.1:8080/api';

let vectors = [];
let angles = []
let resultVector = null;
let drawingMode = false;
let endPoint = null;
let gridSize = 50;



var mouseXC, mouseYC = 0

function setup() {
    let myCanvas = createCanvas(900, 600);
    myCanvas.parent("#canvas-destination");
    frameRate(60);
    textAlign(CENTER, CENTER);
}

function draw() {
    goCartesian()

    if (drawingMode && endPoint) {
        let previewVector = createVector(mouseXC, mouseYC);
        stroke(0, 0, 0)
        seta(
            0, 0,
            previewVector.x, previewVector.y,
            color(150)
        );
    }

    drawVectors()
    drawAngles()
}




function drawAngles() {

    for (let [i, angle] of angles.entries()) {
        let closers = angle
            .filter(angleTo => angleTo[0].Ok && (angleTo[0].Ok >= 1 || angleTo[0].Ok <= -1))
            .reduce(([neg, pos], angleTo) => {
                if (angleTo[0].Ok < 0 && (neg === null || Math.abs(angleTo[0].Ok) < Math.abs(neg[0].Ok))) {
                    neg = angleTo;  // Atualiza o negativo mais próximo de zero
                } else if (angleTo[0].Ok > 0 && (pos === null || Math.abs(angleTo[0].Ok) < Math.abs(pos[0].Ok))) {
                    pos = angleTo;  // Atualiza o positivo mais próximo de zero
                }
                return [neg, pos];
            }, [null, null]);
        for (let [j, closer] of closers.entries()) {
            if(!closer) continue
            let other = createVector(...closer[1].dimensions)
            let angleBetween = closer[0].Ok
            let selfAngle = vectors[i].heading()
            let otherAngle = other.heading()
            let distParam = vectors[i].x  + vectors[i].y + other.x + other.y
            noFill();
            let hue = map(i, 0, vectors.length, 0, 360);
            colorMode(HSB);
            stroke(hue, 80, 80);
            let dist = map(distParam, -1000, 1000, 50, -50, true) + 200
            if (angleBetween < 0) {
                let midAngle = selfAngle - radians(angleBetween)/2
                let textPos = p5.Vector.fromAngle(midAngle).mult(dist - 50);
                textSize(10)
                strokeWeight(1)
                texto(Math.abs(angleBetween.toFixed(2)) + "º", textPos.x, textPos.y)
                arc(0, 0, dist , dist, selfAngle, selfAngle - radians(angleBetween));
            } else {
                arc(0, 0, dist , dist, otherAngle, otherAngle + radians(angleBetween));
            }
            colorMode(RGB);
        }

        

    }
}

function findAngles() {
    let data = []
    for (let vector of vectors) {
        data.push(
            { dimensions: [vector.x, vector.y] }
        )
    }
    return fetch('http://127.0.0.1:8080/api/angulos', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ vectors: data, method: 1 })
    });
}

function drawVectors() {
    for (let [i, vector] of vectors.entries()) {
        let hue = map(i, 0, vectors.length, 0, 360);
        colorMode(HSB)
        stroke(hue, 80, 80);
        seta(
            0, 0,
            vector.x, vector.y

        )
        colorMode(RGB);
    }
}
function mousePressed() {
    if (mouseX > 0 && mouseX < width && mouseY > 0 && mouseY < height) {
        drawingMode = true;
        endPoint = createVector(mouseXC, mouseYC);
    }
}
function mouseReleased() {
    if (drawingMode && endPoint) {
        let endPoint = createVector(mouseXC, mouseYC);
        vectors.push(endPoint);
        drawingMode = false;
        endPoint = null;

        findAngles()
            .then(res => {
                return res.json()
            })
            .then(res => {
                angles = res
            })
            .catch(err => console.log(err))

    }
}

function clearVectors() {
    vectors = []
    angles = []
}




function goCartesian() {
    background(255)
    stroke(200);


    mouseXC = mouseX - width / 2
    mouseYC = height / 2 - mouseY

    strokeWeight(2);
    colore(0, 0, 0, 100)
    seta(0, height / 2, width, height / 2)
    seta(width / 2, height, width / 2, 0)

    translate(width / 2, height / 2)
    let w2 = width * 5 / 30
    let h2 = height * 5 / 20
    line(-w2, h2, w2, h2)
    line(w2, h2, w2, -h2)
    line(w2, -h2, -w2, -h2)
    line(-w2, -h2, -w2, h2)


    strokeWeight(0.5);
    for (let i = -width / 2, j = -height / 2; i <= width / 2; i += width / 30, j += height / 20) {
        line(i, -height / 2, i, height / 2);
        line(-width / 2, j, width / 2, j);
    }
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

