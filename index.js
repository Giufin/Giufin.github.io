
import init, * as wasm from "./pkg/tlw_cal_rewrite_number_2.js";

const colors = [
  "#006400",
  "#ff0000",
  "#ffd700",
  "#c71585",
  "#00ff00",
  "#00ffff",
  "#0000ff",
  "#1e90ff",
];
const camelToSnakeCase = str => str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
const PascalToSnakeCase = str => str.split(/(?=[A-Z])/).join('_').toLowerCase();

const convert_json_bulletgroup_to_internal = (bulletgroup) => {


  let scaling =
  {

    "yin_atk": 0,
    "yang_atk": 0,

    "yin_def": 0,
    "yang_def": 0,

    "health": 0,
    "agility": 0,

  };


  if (bulletgroup.atktype == "Yin") {
    scaling.yin_atk = bulletgroup.power;
  } else {
    scaling.yang_atk = bulletgroup.power;
  }


  if (bulletgroup.addscaling !== "none") {
    //will cause an error because of name differences

    scaling[PascalToSnakeCase(bulletgroup.addscaling)] = bulletgroup.addscaling_multi * bulletgroup.power
  }


  return {
    "typ": bulletgroup.atktype,
    "acc": bulletgroup.acc,
    "crit": bulletgroup.crit,
    "scales_off": scaling,
    "amount": bulletgroup.amount,
  }
}

const convert_json_bulletgroups_to_internal = (bulletgroups) => {
  let groups = [[], [], [], []]


  bulletgroups.Bulletgroups.forEach(function (a) {
    groups[a.boostlvl].push(convert_json_bulletgroup_to_internal(a));
  })

  return groups
}

const runWasm = async (attack, char, effects, dbf, power, idx) => {

  await init("./pkg/tlw_cal_rewrite_number_2_bg.wasm");
  wasm.greet();

  console.log(effects);

  let groups = convert_json_bulletgroups_to_internal(attack);

  let stats;
  if (char.stats == null) {
    stats = {
      "health": 1000,
      "agility": 1000,
      "yang_atk": 1000,
      "yang_def": 1000,
      "yin_atk": 1000,
      "yin_def": 1000
    }
  } else {
    stats = char.stats
  }

  const char_ex = {
    "name": char.name,
    "stats": stats,
    "lw": { "bullets": groups }
  }

  console.log(char_ex);


  wasm.sim_char(JSON.stringify(char_ex), JSON.stringify(effects), JSON.stringify(dbf), power, idx)


};

//a limitation? of no compile step is that the files present here must be named individually
let character_names = [
  "alice",
  "youmu",
  "medicine",
  "patchy",
  "yuyuko",
  "ran",
  "reimu"
];

let characters = [];

for (let k of character_names) {
  let text = await fetch(`characters/${k}.json`).then(response => response.text());
  characters.push([JSON.parse(text), text]);
}

let story_cards = JSON.parse(await fetch(`characters/storycard.json`).then(response => response.text()));

let selections = []
let selection_node = document.getElementById("char_0");
let selection_list = document.getElementById("char_select");
let selection_filter_text = "";

let cards = []
let card_node = document.getElementById("card_0");
let card_list = document.getElementById("card_list");
let card_add = document.getElementById("card_add");


let selection_filter = document.getElementById("char_select_text_input");
selection_filter.onkeyup = ((_) => { selection_filter_text = selection_filter.value; update_chars() });



function update_chars() {

  for (const el of selections) {
    //no true way of case insensitivity
    if (el.el.filter.toUpperCase().includes(selection_filter_text.toUpperCase())) {
      el.node.style.display = "block";
    } else {
      el.node.style.display = "none";
    }

  }

}


function ondatacomplete(data, idx) {

  data.attack = JSON.parse(JSON.stringify(data.attack )); //thank you javascript... very cool

  let effs = [];
  let dbf = [];

  let all = [];
  for (let i = 0; i < 3; i++) {
    if (data.skills[i]) {
      let h = data.char.skills[i];

      all.push(...h.effect);

    }
  }

  if (data.card != null) {

    for (let a of data.card.Effect) {
      if (a.Stat != undefined) {
        all.push(a);
      } else {
        //for now the effetcs will be multiplicative, not additive; TODO: fix it
        let ef = a.Buff;
        if (ef.type.element != undefined) {

          for (let el of data.attack.Bulletgroups) {

            if (el.element == ef.type.element) {
              el.power *= 1 + (ef.value);
            }
          }

        } else {
          for (let el of data.attack.Bulletgroups) {

            if (el.bullettype == ef.type.bullettype) {
              el.power *= 1 + (ef.value);
            }

          }
        }
      }
    }
  }

  all.push(...data.attack.pre)


  for (let i = 0; i < data.power; i++) {
    all.push(...data.char.power)
  }

  for (let i = 0; i < data.graze; i++) {
    all.push(...data.char.graze)
  }

  for (let a of all) {
    if (a.Stat.target == "All" || a.Stat.target == "Solo") {
      dbf.push(a)
    } else {
      effs.push(a)
    }
  }

  console.log(effs);
  console.log(dbf);


  runWasm(data.attack, data.char, effs, dbf, data.power, idx)
}

function push_card(idx) {
  let next = card_node.cloneNode(true);
  next.style.display = "block";
  let data = { node: next, char: null, attack: null, power: 0, graze: 0, skills: [false, false, false], card: null };

  let colorbox = next.querySelector(".color_box");
  colorbox.style.border = `5px solid ${colors[idx]}`


  let internal = next.querySelector(".internal_data");
  let header = next.querySelector(".header");
  header.onclick = function (ev) {

    if (internal.style.display == "block") {
      for (let card of cards) {
        card.node.querySelector(".internal_data").style.display = "none";
      }
    } else {
      for (let card of cards) {
        card.node.querySelector(".internal_data").style.display = "none";
      }

      internal.style.display = "block";
    }

  }
  let char = internal.querySelector(".character");
  let spell = internal.querySelector(".spell");


  let pnode = internal.querySelector(".power");

  pnode.onclick = function (ev) {
    if (pnode.innerText == "0P") {
      pnode.innerText = "1P"
      data.power = 1;
    } else if (pnode.innerText == "1P") {
      pnode.innerText = "2P"
      data.power = 2;
    } else if (pnode.innerText == "2P") {
      pnode.innerText = "3P"
      data.power = 3;
    } else if (pnode.innerText == "3P") {
      pnode.innerText = "0P"
      data.power = 0;
    }
  }

  let gnode = internal.querySelector(".graze");

  gnode.onclick = function (ev) {
    console.log("here")
    if (gnode.innerText == "0G") {
      gnode.innerText = "1G"
      data.graze = 1;
    } else if (gnode.innerText == "1G") {
      gnode.innerText = "2G"
      data.graze = 2;
    } else if (gnode.innerText == "2G") {
      gnode.innerText = "3G"
      data.graze = 3;
    } else if (gnode.innerText == "3G") {
      gnode.innerText = "0G"
      data.graze = 0;
    }
  }

  let cardnode = internal.querySelector(".card");

  cardnode.onclick = function (ev) {

    let cardmapped = story_cards.map(function (el) {
      return new Selection(JSON.stringify(el), el["Card name"], el);
    });

    selection(cardmapped,
      ev.clientX, ev.clientY, function (a) {
        console.log("a")
        cardnode.innerText = a.data["Card name"]
        data.card = a.data;
      });
  }

  for (let i = 1; i < 4; i++) {


    let skillnode = internal.querySelector(`.s${i}`);
    let on = false;

    skillnode.onclick = function (ev) {
      if (!on) {
        skillnode.style.backgroundColor = "black";
        data.skills[i - 1] = true;
      } else {
        skillnode.style.backgroundColor = "white";
        data.skills[i - 1] = false;
      }
      on = !on;
    }



  }

  char.onclick = function (ev) {


    let charmapped = characters.map(function (el) {
      return new Selection(el[1], el[0].name, el[0]);
    });

    selection(charmapped,
      ev.clientX, ev.clientY, function (a) {

        char.textContent = a.text;
        data.char = a.data;

        spell.textContent = "choose an attack"
        data.attack = null;
        spell.onclick = function (ev2) { //the lint saying shadowing is bad is so useful 😍 mistakes are impossible now

          let attacksmapped = data.char.spellcards.map(function (x) {
            return new Selection("", `${x.cardtype}: ${x.cardname}`, x);
          })

          selection(attacksmapped,
            ev2.clientX, ev2.clientY, function (a2) {
              data.attack = a2.data;
              ondatacomplete(data, idx);
            }
          )


        }
      })

  }

  cards.push(data);
  card_list.insertBefore(next, card_add);

  return next;
}

for (let i = 0; i < 8; i++) {
  push_card(i)
}

class Selection {

  filter;
  text;
  data;

  constructor(filter_, text_, data_) {
    this.filter = filter_;
    this.text = text_;
    this.data = data_;
    return this;
  }


}

function selection(between, x, y, callback) {

  selection_list.querySelectorAll(".char").forEach(function (e) { e.remove(); });
  selection_list.style.display = "block";

  selection_list.style.left = `${x}px`;
  selection_list.style.top = `${y}px`;

  selection_filter.value = "";

  between.map(function (el) {
    let moved_el = el; //to fix the fact that idx is overriden each loop

    let clone = selection_node.cloneNode();
    clone.textContent = el.text;
    clone.style.display = "block";

    selection_list.appendChild(clone);

    selections.push({ el, node: clone });

    clone.onclick = function () {
      selection_list.style.display = "none";

      callback(moved_el);
    }
  })
}


let import_text = document.querySelector("#import_text");
let import_button = document.querySelector("#import_button");

import_button.onclick = function () {
  let text = import_text.value;
  import_text.value = "";

  characters.push([JSON.parse(text), text]);
}

let close_button = document.querySelector("#close_button");

close_button.onclick = function () {
  selection_list.style.display = "none";
}

//selection([{ text: "abc", filter: "abc" }, { text: "efg", filter: "efg" }], 520, 50, function (e) { console.log(e) });\\



