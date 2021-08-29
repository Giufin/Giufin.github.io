
const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

const colors = [
  "#8b4513",
  "#191970",
  "#3cb371",
  "#ffd700",
  "#00ff00",
  "#00bfff",
  "#0000ff",
  "#ff1493",
];


var color_idx = 0;

var prev_y = canvas.height;
function next_color() {

  if (color_idx == 10) {
    const message = "Error, maximal number of charaters possible to display at once should be capped at 10";

    alert(message); // for easier diagnosis just in case
    throw new Error(message);
  }
  context.strokeStyle = colors[color_idx];
  color_idx += 1;

}
var values = [[], [], [], [], [], [], [], [], [], []];
var next = null;

export function move_to(x_f, y_f) {
  const y = y_f;
  const x = x_f;

  //if (next.length == 0 || next[next.length - 1][0] + 1 < x) { //
  next.push([x, y]);
  //}
}

export function start_drawing(val) {
  next = values[val];
  next.length = 0;
}

export function end_drawing() {
  context.clearRect(0, 0, canvas.width, canvas.height);
  var max = 0;
  for (const el of values) {
    if (el.length != 0) {
      if (max < el[el.length - 1][1]) {
        max = el[el.length - 1][1];
      }
    }
  }

  for (var i = 0; i < 10; i++) {

    const next1 = values[i];
    if (next1.length == 0) {
      continue;
    }



    context.beginPath();

    let prev = 0;
    context.lineTo(0, canvas.height);


    context.strokeStyle = colors[i];

    for (const [x, y] of next1) {
      const nextY = (1 - y / max) * canvas.height;
      const nextX = x * canvas.width;
      context.lineTo(prev, nextY);
      context.lineTo(nextX, nextY);

      prev = nextX;
    }
    context.stroke();

  }



}

export function clear() {
  context.clearRect(0, 0, canvas.width, canvas.height);
}

export function debug(str) {
  console.log(str)
}

