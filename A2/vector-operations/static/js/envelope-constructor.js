let baseUrl = 'http://127.0.0.1:8080/api';

let saved_points = []
let boundings = []

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
    for (let point of saved_points) {

        textSize(14);

        let strokeColor = [100, 100, 100, 100]
        let fillColor = [100, 100, 100, 100]


        stroke(...strokeColor)
        fill(...fillColor)
        circle(point.x, point.y, 2)
    }
}





function randomPoints() {
    let new_points = []
    for (let i = 0; i < 3; i++) {
        // Generate a random base integer within the range of -300 to 300
        const baseY = Math.floor(Math.random() * 601) - 300;
        const baseX = Math.floor(Math.random() * 801) - 400;
        const variance = 200
        for (let i = 0; i < 3; i++) {
            let valX = baseX + Math.floor(Math.random() * variance) - 1
            valX = Math.max(-width / 2, Math.min(width / 2, valX))
            let valY = baseY + Math.floor(Math.random() * variance) - 1
            valY = Math.max(-height / 2, Math.min(height / 2, valY))
            new_points.push(
                createVector(valX, valY)
            )
        }
    }

    saved_points.push(...new_points)
    updateEnvelope(new_points)

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
            drawer: (p) => rect(...p),
        },
        "Sphere": {
            constructor: (sphere) => {
                return [
                    sphere.center.dimensions[0], sphere.center.dimensions[1],
                    sphere.radius * 2, sphere.radius * 2
                ]
            },
            drawer: (p) => {
                ellipse(...p)
            }
        },
        "OBB": {
            constructor: (obb) => {
                return [obb.center, obb.axes, obb.half_sizes, obb.points]

            },
            drawer: (p) => {
                beginShape();
                for (let v of p[3]) {
                    vertex(v.dimensions[0], v.dimensions[1]);
                }
                endShape(CLOSE);
            }
        },
    }

    constructor(type, params) {
        this.requestOBJ = params

        this.drawer = this.methods[type].drawer
        this.params = this.methods[type].constructor(params)
        this.color = 3
        this.type = type
    }
    draw() {
        colore(this.color * 5, 0, 0, this.color * 5 + 10)
        this.drawer(this.params)
    }

}


async function updateEnvelope(new_points) {
    const checkedIds = [...document.querySelectorAll('input[type="checkbox"]:checked')]
        .map(checkbox => checkbox.id)

    for (let point of new_points) {
        let data = []
        data.push(
            { dimensions: [point.x, point.y] }
        )
    }
    const slicer = 3
    const numCalls = Math.floor(new_points.length / slicer);
    for (let i = 0; i < numCalls; i++) {
        const intSubset = new_points.slice(i * slicer, (i + 1) * slicer);
        let data = []
        for (let point of intSubset) {
            data.push(
                { dimensions: [point.x, point.y] }
            )
        }
        const bounding = checkedIds[Math.floor(Math.random() * checkedIds.length)];
        let res = await makeEnvelope(data, bounding)
        boundings.push(
            new Bounding(bounding, res)
        )
    }

    updateCollisions()
}

async function makeEnvelope(data, type) {
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

function drawBoundings() {
    for (let index in boundings) {
        boundings[index].draw()
    }
}




async function updateCollisions() {

    let data = {
        aabb: [],
        obb: [],
        sphere: [],
    }
    for (let index in boundings) {
        switch (boundings[index].type) {
            case 'AABB':
                data.aabb.push([boundings[index].requestOBJ, parseInt(index)])
                break;
            case 'OBB':
                data.obb.push([boundings[index].requestOBJ, parseInt(index)])
                break;
            case 'Sphere':
                data.sphere.push([boundings[index].requestOBJ, parseInt(index)])
                break;
            default: break;
        }
    }
    let res = await checkCollisions(data)


    return res
}

async function checkCollisions(data) {
    let res = await fetch('http://127.0.0.1:8080/api/envoltorios-intersectam', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data)
    })

    res = await res.json()
    console.log(res)

    let intersecting_boundings = res
    for (let index in boundings) {
        intersecting_boundings
            .filter(intersec => intersec[0] == parseInt(index))
            .forEach(_ => boundings[index].color += 5)
    }
    return res
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
        saved_points.push(
            createVector(mouseXC, mouseYC)
        )
        updateEnvelope()
    }
}

function mouseReleased() {

}

function clearPoints() {
    saved_points = []
    boundings = []
}
