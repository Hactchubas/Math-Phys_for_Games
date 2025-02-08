// Initialize variables to store slider values
let alfa = 1;
let beta = 1;


let alfaSlider
let betaSlider
let alfaValueDisplay
let betaValueDisplay
window.addEventListener('load', function () {
    alfaSlider = document.getElementById('alfa');
    betaSlider = document.getElementById('beta');
    alfaValueDisplay = document.getElementById('alfaValue');
    betaValueDisplay = document.getElementById('betaValue');

    alfaSlider.addEventListener('input', function () {
        alfa = parseFloat(alfaSlider.value);
        alfaValueDisplay.textContent = alfa.toFixed(2);  // Display the alfa value with two decimal places
    });

    betaSlider.addEventListener('input', function () {
        beta = parseFloat(betaSlider.value);
        betaValueDisplay.textContent = beta.toFixed(2);  // Display the beta value with two decimal places
    });
}, false);



let baseUrl = 'http://127.0.0.1:8080/api';

let incidentVector = null;
let normalVector = null;
let reflectedVector = null;

let vp = null;
let vn = null;
let vp_r = null;
let vn_r = null;
let reac_result = null;

let block = {
    x: -200,
    y: -200,
    w: 2000,
    h: 10,
    offsetX: 0,
    offsetY: 0
};

function setup() {
    let myCanvas = createCanvas(800, 600);
    myCanvas.parent("#canvas-destination");
    textAlign(CENTER, CENTER);
    normalVector = createVector(0, 75);
}

function draw() {
    background(240);
    translate(width / 2, height / 2);

    drawGridAndAxes();
    drawVector(createVector(block.x, block.y), normalVector, color(0, 0, 255), 2, "N");

    incidentVector = createVector(mouseX - width / 2, mouseY - height / 2);
    drawVector(createVector(0, 0), incidentVector, color(0, 255, 0), 2, "I");

    updateDecompositionVectors(incidentVector);
    if (vp && vn) {
        drawVector(createVector(0, 0), vp, color(255, 0, 0), 2, "Vn");
        drawVector(createVector(0, 0), vn, color(0, 0, 255), 2, "Vp");
    }

    reactionVector(incidentVector, normalVector)
        .then(res => {
            if (res) {
                reac_result = res
            }
        })
        .catch(error => {
            console.error('Error drawing vectors:', error);
        });
    if (reac_result) {
        drawVector(createVector(mouseX - width / 2, mouseY - height / 2), reac_result, color(255, 0, 0), 2, "R");
    }

    drawBlock();
}

function drawGridAndAxes() {
    stroke(200);
    strokeWeight(1);
    for (let i = -width / 2; i <= width / 2; i += 50) {
        line(i, -height / 2, i, height / 2);
        line(-width / 2, i, width / 2, i);
    }

    stroke(0);
    strokeWeight(2);
    line(-width / 2, 0, width / 2, 0);
    line(0, -height / 2, 0, height / 2);
}

function drawBlock() {
    rectMode(CENTER);
    fill(128, 128, 128, 160);
    noStroke();
    rect(block.x, block.y, block.w, block.h);
    rectMode(CORNER);
}

function drawVector(start, vector, vectorColor, weight = 1, label = "") {
    push();
    stroke(vectorColor);
    strokeWeight(weight);
    fill(vectorColor);

    // Draw line
    line(start.x, start.y, start.x + vector.x, start.y + vector.y);

    // Draw arrow head
    push();
    translate(start.x + vector.x, start.y + vector.y);
    rotate(vector.heading());
    let arrowSize = 10;
    triangle(0, arrowSize / 2, 0, -arrowSize / 2, arrowSize, 0);
    pop();

    // Draw label
    if (label) {
        textSize(14);
        let midX = start.x + vector.x / 2;
        let midY = start.y + vector.y / 2;
        noStroke();
        fill(255);
        rect(midX - 10, midY - 10, 20, 20);
        fill(vectorColor);
        text(label, midX, midY);
    }

    pop();
}

async function updateDecompositionVectors(vector) {
    try {
        let v1 = {
            dimensions: [vector.x, vector.y]
        };
        let v2 = {
            dimensions: [normalVector.x, normalVector.y]
        };

        const response = await fetch(baseUrl + '/decomposicao', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                v1: v1,
                v2: v2,
                scalar: null
            })
        });

        const result = await response.json();
        [vn, vp] = result;
        vp = createVector(vp.dimensions[0], vp.dimensions[1]);
        vn = createVector(vn.dimensions[0], vn.dimensions[1]);

    } catch (error) {
        console.error('Error calculating:', error);
    }
}

async function reactionVector(incident, normal) {

    if (incident && normal) {
        const v1 = { dimensions: [incident.x, incident.y] };
        const v2 = { dimensions: [normal.x, normal.y] };

        try {
            const response = await requestReaction(v1, v2);
            const result = await response.json();
            const reac_vector = createVector(result.dimensions[0], result.dimensions[1]);
            return reac_vector // Retorna os vetores de reação
        } catch (error) {
            console.error('Error calculating reaction vector:', error);
        }
    }
}

function requestReaction(v1, v2) {
    return fetch(baseUrl + '/reacao', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            v1: v1,
            v2: v2,
            alfa: alfa,
            beta: beta
        })
    });
}

function resetVectors() {
    reflectedVector = null;
}

function mousePressed() {
    let xpos = mouseX - width / 2;
    let ypos = mouseY - height / 2;
    
}

function mouseReleased() {
    block.dragging = false;
}

