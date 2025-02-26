
let vectors = [];
let resultVector = null;
let drawingMode = false;
let startPoint = null;
let shuffledOrder = [];
let gridSize = 50;

function setup() {
    let myCanvas = createCanvas(800, 600);
    myCanvas.parent("#canvas-destination");
    frameRate(60);
    textAlign(CENTER, CENTER);
}

function draw() {
    background(240);
    push();
    translate(width / 2, height / 2);

    drawGridAndAxes();

    if (vectors.length > 0) {
        let currentPos = createVector(0, 0);
        for (let i = 0; i < vectors.length; i++) {
            let idx = shuffledOrder[i] !== undefined ? shuffledOrder[i] : i;
            let v = vectors[idx];
            let label = String.fromCharCode(65 + idx); // Convert 0->A, 1->B, etc.
            drawVector(currentPos, v, getVectorColor(idx), 1, label);
            currentPos.add(v);
        }

        if (resultVector) {
            drawVector(createVector(0, 0), resultVector, color(255, 0, 0), 2, "R");
        }
    }

    if (drawingMode && startPoint) {
        let mouseVec = createVector(mouseX - width / 2, mouseY - height / 2);
        let previewVector = p5.Vector.sub(mouseVec, startPoint);
        drawVector(startPoint, previewVector, color(150));
    }

    pop();
    drawLegend();
}

function drawGridAndAxes() {
    stroke(200);
    strokeWeight(1);
    for (let i = -width / 2; i <= width / 2; i += gridSize) {
        line(i, -height / 2, i, height / 2);
        line(-width / 2, i, width / 2, i);
    }

    stroke(0);
    strokeWeight(2);
    line(-width / 2, 0, width / 2, 0);
    line(0, -height / 2, 0, height / 2);

    textAlign(CENTER, CENTER);
    stroke(0);
    fill(0);
    strokeWeight(1);
    textSize(8);

    for (let i = -width / 2; i <= width / 2; i += gridSize) {
        if (i !== 0) {
            text(i * 50 / gridSize, i, 20);
        }
    }

    for (let i = -height / 2; i <= height / 2; i += gridSize) {
        if (i !== 0) {
            text(-i * 50 / gridSize, -20, i);
        }
    }

    textSize(14);
    text("X", width / 2 - 20, 35);
    text("Y", -35, -height / 2 + 20);
}

function mousePressed() {
    if (mouseX > 0 && mouseX < width && mouseY > 0 && mouseY < height) {
        drawingMode = true;
        startPoint = createVector(mouseX - width / 2, mouseY - height / 2);
    }
}

function mouseReleased() {
    if (drawingMode && startPoint) {
        let endPoint = createVector(mouseX - width / 2, mouseY - height / 2);
        let newVector = p5.Vector.sub(endPoint, startPoint);
        vectors.push(newVector);
        updateResultVector();
        shuffledOrder = Array.from({ length: vectors.length }, (_, i) => i);
        drawingMode = false;
        startPoint = null;
    }
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
        // White background for label
        noStroke();
        fill(255);
        rect(midX - 10, midY - 10, 20, 20);
        // Label text
        fill(vectorColor);
        text(label, midX, midY);
    }

    pop();
}

function drawLegend() {
    if (vectors.length < 1) return
    let legendX = 0;
    let legendY = 0;
    let legendSpacing = 20;

    textAlign(LEFT, CENTER);
    textSize(12);
    for (let i = 0; i < vectors.length; i++) {
        let label = String.fromCharCode(65 + i);
        fill(getVectorColor(i));
        text(`${label}: (${vectors[i].x}, ${vectors[i].y})`, legendX + 10, legendY + (i + 1) * legendSpacing);
    }

    if (resultVector) {
        fill(255, 0, 0);
        text(`Soma (R): (${resultVector.x}, ${resultVector.y})`, legendX + 10, legendY + (vectors.length + 1) * legendSpacing);
    }
}


function getVectorColor(index) {
    let hue = map(index, 0, vectors.length, 0, 360);
    colorMode(HSB);
    let c = color(hue, 80, 80);
    colorMode(RGB);
    return c;
}

async function updateResultVector() {
    if (vectors.length === 0) {
        resultVector = null;
        return;
    }

    try {
        let v1 = {
            dimensions: [vectors[0].x, vectors[0].y]
        };

        for (let i = 1; i < vectors.length; i++) {
            let v2 = {
                dimensions: [vectors[i].x, vectors[i].y]
            };

            const response = await fetch('http://127.0.0.1:8080/api/soma', {
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
            v1 = result;
        }

        resultVector = createVector(v1.dimensions[0], v1.dimensions[1]);
    } catch (error) {
        console.error('Error calculating sum:', error);
    }
}

function randomVectors() {
    vectors = [];
    for (let i = 0; i < 5; i++) {
        let v = p5.Vector.random2D();
        v.mult(random(50, 150));
        vectors.push(v);
    }
    shuffledOrder = Array.from({ length: vectors.length }, (_, i) => i);

    updateResultVector();
}

function shuffleVectors() {
    shuffledOrder = Array.from({ length: vectors.length }, (_, i) => i);
    for (let i = shuffledOrder.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [shuffledOrder[i], shuffledOrder[j]] = [shuffledOrder[j], shuffledOrder[i]];
    }
}

function clearVectors() {
    vectors = [];
    resultVector = null;
    shuffledOrder = [];
}
