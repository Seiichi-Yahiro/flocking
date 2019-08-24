const canvas = document.getElementById('canvas');
const context = canvas.getContext('2d');

canvas.width = Number(canvas.getBoundingClientRect().width);
canvas.height = Number(canvas.getBoundingClientRect().height);
//context.scale(2, 2);

export function clear_canvas() {
    context.clearRect(0, 0, canvas.width, canvas.height);
}

export function draw_boid(x, y, angle) {
    context.save();
    context.translate(x, y);
    context.rotate(angle);

    context.beginPath();

    [[5, 0], [-5, -3], [-5, 3]].forEach(([x, y], index) => (index === 0 ? context.moveTo(x, y) : context.lineTo(x, y)));

    context.closePath();
    context.stroke();
    context.restore();
}
