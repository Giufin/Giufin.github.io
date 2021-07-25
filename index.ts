enum AtkType {
    yang,
    yin,
}

function to_stat(a: AtkType): Stat {
    if (a == AtkType.yang) {
        return Stat.yang_atk;
    } else {
        return Stat.yin_atk;
    }
}


enum Stat {
    health,
    agility,
    yang_atk,
    yang_def,
    yin_atk,
    yin_def,
}

enum Modifier {
    acc,
    crit_acc,
    crit_atk,
}

class Stats {
    health: number;
    agility: number;
    yang_atk: number;
    yang_def: number;
    yin_atk: number;
    yin_def: number;

    sum(): number {
        return this.health + this.agility + this.yang_atk + this.yang_def + this.yin_atk + this.yin_def
    }

    constructor(
        health: number = 0,
        agility: number = 0,
        yang_atk: number = 0,
        yang_def: number = 0,
        yin_atk: number = 0,
        yin_def: number = 0,
    ) {
        this.health = health;
        this.agility = agility;
        this.yang_atk = yang_atk;
        this.yang_def = yang_def;
        this.yin_atk = yin_atk;
        this.yin_def = yin_def;
    }

    mul(second: Stats): Stats {
        return new Stats(
            this.health * second.health,
            this.agility * second.agility,
            this.yang_atk * second.yang_atk,
            this.yang_def * second.yang_def,
            this.yin_atk * second.yin_atk,
            this.yin_def * second.yin_def)
    }

    mul_all(second: number): Stats {
        return new Stats(
            this.health * second,
            this.agility * second,
            this.yang_atk * second,
            this.yang_def * second,
            this.yin_atk * second,
            this.yin_def * second)
    }

    add(second: Stats): Stats {
        return new Stats(
            this.health + second.health,
            this.agility + second.agility,
            this.yang_atk + second.yang_atk,
            this.yang_def + second.yang_def,
            this.yin_atk + second.yin_atk,
            this.yin_def + second.yin_def)
    }
}

function stats_from_stat(
    b: Stat, c: number
): Stats {

    if (b == Stat.health) {
        return new Stats(c, 0, 0, 0, 0, 0);
    }
    else if (b == Stat.agility) {
        return new Stats(0, c, 0, 0, 0, 0);
    }
    else if (b == Stat.yang_atk) {
        return new Stats(0, 0, c, 0, 0, 0);
    }
    else if (b == Stat.yang_def) {
        return new Stats(0, 0, 0, c, 0, 0);
    }
    else if (b == Stat.yin_atk) {
        return new Stats(0, 0, 0, 0, c, 0);
    }
    else {
        return new Stats(0, 0, 0, 0, 0, c);
    }

}


class Modifiers {
    acc: number;
    crit_acc: number;
    crit_atk: number;

    constructor(
        accs: number = 0,
        crit_accs: number = 0,
        crit_atks: number = 0,
    ) {
        this.acc = accs;
        this.crit_acc = crit_accs;
        this.crit_atk = crit_atks;
    }
}

function modifiers_from_modifier(
    b: Modifier, c: number
): Modifiers {

    if (b == Modifier.acc) {
        return new Modifiers(c, 0, 0);
    }
    else if (b == Modifier.crit_acc) {
        return new Modifiers(0, c, 0);
    }
    else {
        return new Modifiers(0, 0, c);
    }

}


class Effect {
    stat: Stat | Modifier
    lvl: number
    chance: number

    constructor(stat: Stat | Modifier, lvl: number, chance: number) {
        this.stat = stat;
        this.lvl = lvl;
        this.chance = chance;
    }
}

class GoesOff {
    goesoff: Stat;
    goesoffpercent: number;

    constructor(stat: Stat, percentage: number) {
        this.goesoff = stat;
        this.goesoffpercent = percentage;
    }

    to_stats(): Stats {

        const stats = new Stats();

        // gotta love the fact that the switch statement in TS uses the same syntax as c, designed 50 years ago to make assembly jump maps more ergonomic! :)
        if (this.goesoff == Stat.health) {
            stats.health = this.goesoffpercent;
        } else if (this.goesoff == Stat.agility) {
            stats.agility = this.goesoffpercent;
        } else if (this.goesoff == Stat.yang_atk) {
            stats.yang_atk = this.goesoffpercent;
        } else if (this.goesoff == Stat.yang_def) {
            stats.yang_def = this.goesoffpercent;
        } else if (this.goesoff == Stat.yin_atk) {
            stats.yin_atk = this.goesoffpercent;
        } else if (this.goesoff == Stat.yin_def) {
            stats.yin_def = this.goesoffpercent;
        }

        return stats;

    }
}


class BulletGroup {
    type: AtkType;

    acc: number;
    crit: number;

    amount: number;

    goes_off: GoesOff[];

    constructor(
        type: AtkType,
        acc: number,
        crit: number,
        amount: number,
        damage: number,
        goes: Stat | null = null,
        mult: number | null = null,
    ) {
        this.type = type;
        this.acc = acc;
        this.crit = crit;
        this.amount = amount;
        this.goes_off = [];

        this.goes_off.push(new GoesOff(to_stat(type), damage))
        if (goes !== null && mult !== null) {
            this.goes_off.push(new GoesOff(goes, damage * mult))
        }

    }

}

class Atack {
    bullets: BulletGroup[][];

    constructor(bullets: BulletGroup[][]) {
        this.bullets = bullets;
    }
}

class StatBoost {
    chances: Map<number, number>;

    constructor(chances: Map<number, number>) {
        this.chances = chances;
    }
}

class StatBoosts {
    inner: Map<Stat, StatBoost>;
    constructor(inner: Map<Stat, StatBoost>) {
        this.inner = inner;
    }
}

class Enemy {
    stats: Stats;
    constructor(stats: Stats) {
        this.stats = stats
    }
}

class StatArray {
    lvl: number
    chances: number[]

    constructor(lvl: number, chances: number[]) {
        this.lvl = lvl;
        this.chances = chances;
    }
}

class Character {
    name: string
    stats: Stats
    lw: Atack

    constructor(name: string,
        stats: Stats,
        lw: Atack,
    ) {
        this.name = name;
        this.stats = stats;
        this.lw = lw;
    }
}

class Hit {
    chance: number;
    yang: Stats;
    yin: Stats;

    constructor(
        chance: number,
        yang: Stats,
        yin: Stats,
    ) {
        this.chance = chance;
        this.yang = yang;
        this.yin = yin;
    }
}



function bulletgroups_to_percentiles(atk: Atack, power: number, accbuff: number, crit_acc_buff: number, crit_atk_buff: number): [Map<Stats, number>, Map<Stats, number>] {

    const bullet_groups = atk.bullets.slice(0, power + 1).flat();
    const bullets = <[BulletGroup]>bullet_groups.map(x => new Array(x.amount).fill(x)).flat();

    let hits_yang = new Map<Stats, number>();
    hits_yang.set(new Stats(), 1.0);

    let hits_yin = new Map<Stats, number>();
    hits_yin.set(new Stats(), 1.0);


    let temp_yang = new Map<Stats, number>();
    let temp_yin = new Map<Stats, number>();



    bullets.forEach(bullet => {

        const hit_chance = Math.min(bullet.acc * (1 + 0.2 * accbuff), 1);
        const crit_chance = Math.min(bullet.crit * (1 + 0.2 * crit_acc_buff), 1);

        const crit_damage = Math.min(bullet.crit * (2 + 0.3 * crit_acc_buff), 5);


        const [all_hits, temp_hits] = (bullet.type == AtkType.yang) ? [hits_yang, temp_yang] : [hits_yin, temp_yin];
        temp_hits.clear()

        all_hits.forEach((chance, damage) => {

            let total_damage = new Stats()

            bullet.goes_off.forEach(dmg => {
                total_damage = total_damage.add(dmg.to_stats());
            });


            //no crit hit
            {
                const key = damage.add(total_damage);
                const val = temp_hits.get(key) ?? 0.0;
                temp_hits.set(key, val + hit_chance * (1 - crit_chance) * chance)
            }

            //crit hit
            {
                const key = damage.add(total_damage).mul_all(crit_damage);
                const val = temp_hits.get(key) ?? 0.0;
                temp_hits.set(key, val + hit_chance * (crit_chance) * chance)
            }

            //no hit
            {
                const key = damage;
                const val = temp_hits.get(key) ?? 0.0;
                temp_hits.set(key, val + (1 - hit_chance) * chance)
            }

        })

        //[all_hits, temp_hits] = [temp_hits, all_hits] does not work

        //loving the ownership model not allowing me to do something like this now: all_hits = temp_hits;
        if (bullet.type == AtkType.yang) {
            hits_yang = temp_hits;
            temp_yang = new Map<Stats, number>();
        } else {
            hits_yin = temp_hits;
            temp_yin = new Map<Stats, number>();
        }



    });

    return [hits_yang, hits_yin];

}

function stat_index(stat: Stat | Modifier): number {
    switch (stat) {
        case Stat.health: return 0; //break;
        case Stat.agility: return 1; //break;
        case Stat.yang_atk: return 2; //break;
        case Stat.yang_def: return 3; //break;
        case Stat.yin_atk: return 4; //break;
        case Stat.yin_def: return 5; //break;

        case Modifier.acc: return 6; // break;
        case Modifier.crit_acc: return 7; // break;
        case Modifier.crit_atk: return 8; // break;

    }
}

function apply_effects(
    stats: Stats,
    effects_ex: Effect[],
): [Stats, Modifiers, number][] {

    //so elegant
    const effects = <Array<[number, Array<number>]>>new Array(9).fill(null).map(_ => [0, []]);

    // we will treat the stats and their respective chance to proc + 1 as an array of 9 tuples of guaranteed level as well as the chance, because doing this in a type safe way would be a chore, and so is using maps indexed by the stat's respective enum
    effects_ex.forEach(eff => {
        const index = stat_index(eff.stat);
        effects[index][0] += eff.lvl;
        effects[index][1].push(eff.chance);
    });

    function get_dict_for_stat(stat_level: number, stat_chances: Array<number>): Map<number, number> {
        let dict = new Map<number, number>();
        dict.set(stat_level, 1);

        stat_chances.forEach(chance1 => {
            const temp = new Map<number, number>();

            dict.forEach((chance, lvl) => {
                const key = lvl;
                const next_key = Math.min(key + 1, 10);
                const val = temp.get(key) ?? 0;
                const val_plus_one = temp.get(next_key) ?? 0;

                temp.set(key, val + chance * (1 - chance1));
                temp.set(next_key, val_plus_one + chance * chance1);

            });

            dict = temp
        });

        return dict;
    }


    //taken from https://gist.github.com/ssippe/1f92625532eef28be6974f898efb23ef, this needs to be profiled however due to the number of conditional buffs being very low this wont be an issue

    const chances = effects.map(([x, y]) => get_dict_for_stat(x, y));

    function cartesianProduct<T>(...allEntries: T[][]): T[][] {
        return allEntries.reduce<T[][]>(
            (results, entries) =>
                results
                    .map(result => entries.map(entry => [...result, entry]))
                    .reduce((subResults, result) => [...subResults, ...result], []),
            [[]]
        )
    }
    //map every intry to list of levels -> propabilities 
    const arr = chances.map(x => Array.from(x.entries()));
    const cart = cartesianProduct(...arr)

    return cart.map(x => {
        //this looks very chaotic but seems easier to do and more performant than writing a function to generate Stats and Modifiers from arrays
        const ret_stats = stats.mul(new Stats(
            1 + 0.3 * x[0][0],
            1 + 0.3 * x[1][0],
            1 + 0.3 * x[2][0],
            1 + 0.3 * x[3][0],
            1 + 0.3 * x[4][0],
            1 + 0.3 * x[5][0]
        ));
        const ret_modifiers = new Modifiers(x[6][0], x[7][0], x[8][0]);
        const ret_chance = x.map(([_, b]) => b).reduce((a, b) => a * b);

        return [ret_stats, ret_modifiers, ret_chance]
    })

}


function sim(
    char: Character,
    power: number,
    effects: Array<Effect>,
    enemy: Enemy,
    enemy_effects: Array<Effect>,
): Array<[number, number]> {

    const char_stats = apply_effects(char.stats, effects);
    const enemy_stats = apply_effects(enemy.stats, enemy_effects);

    const unsorted = new Array<[number, number]>();

    for (const [f_stats, f_mod, f_chance] of char_stats) {
        for (const [e_stats, e_mod, e_chance] of enemy_stats) {

            const chance = f_chance * e_chance

            const [res_yang, res_yin] = bulletgroups_to_percentiles(char.lw, power, f_mod.acc + e_mod.acc, f_mod.crit_acc + e_mod.crit_acc, f_mod.crit_atk + e_mod.crit_atk);

            for (const [damage_yang, chance_yang] of Array.from(res_yang)) {
                for (const [damage_yin, chance_yin] of Array.from(res_yin)) {
                    const final_damage =
                        damage_yin.mul(f_stats).sum() / e_stats.yin_def +
                        damage_yang.mul(f_stats).sum() / e_stats.yang_def

                    unsorted.push([final_damage, chance_yang * chance_yin * chance])
                }
            }

        }
    }


    unsorted.sort((x, y) => { return x[0] - y[0] }); // not anymore :)

    for (let i = 1; i < unsorted.length; i++) {
        unsorted[i][1] = unsorted[i - 1][1] + unsorted[i][1]
    }

    return unsorted;

}




function draw_data(data: Array<[number, number]>) {
    const canvas = document.getElementById("canvas") as
        HTMLCanvasElement;

    let context = canvas.getContext("2d");
    if (context == null) {
        throw "temp"
    }

    const line_to = (x: number, y: number) => {
        if (context == null) {
            throw "temp"
        }

        context.lineTo(x, canvas.height - y);
    }

    context.beginPath();
    line_to(0, 0);


    var prev_y = 0;

    for (const [y_f, x_f] of data) {
        const y = (y_f * canvas.height) / 6000;
        const x = x_f * canvas.width;

        line_to(x, prev_y);
        line_to(x, y);
        prev_y = y;
    }




    context.stroke();
}


let dummy = new Enemy(new Stats(1000, 1000, 1000, 1000, 1000, 1000))
let test_char = new Character(
    "test",
    new Stats(1000, 1000, 1000, 1000, 1000, 1000),
    new Atack([
        [new BulletGroup(AtkType.yang, 0.005, 1.0, 1, 10.0, Stat.yang_def, 0.0)],
        [new BulletGroup(AtkType.yang, 0.50, 0.05, 1, 40.0, Stat.yang_def, 0.0)],
        [new BulletGroup(AtkType.yin, 0.50, 0.05, 1, 160.0, Stat.yin_def, 0.0)],
        [new BulletGroup(AtkType.yang, 0.50, 0.05, 1, 640.0, Stat.yang_def, 0.0)],
    ]),
)


const chen = new Character(
    "Chen",
    new Stats(4500, 1200, 1220, 985, 880, 815),
    new Atack([
        [new BulletGroup(AtkType.yang, 0.5, 0.12, 16, 11.25, Stat.health, 0.0)],
        [new BulletGroup(AtkType.yang, 0.5, 0.12, 3, 20.57, Stat.agility, 0.20)],
        [
            new BulletGroup(AtkType.yang, 0.5, 0.12, 3, 22.29, Stat.agility, 0.20),
            new BulletGroup(AtkType.yang, 0.5, 0.12, 2, 36.0, Stat.agility, 0.20),
        ],
        [
            new BulletGroup(AtkType.yang, 0.5, 0.12, 2, 38.57, Stat.agility, 0.20),
            new BulletGroup(AtkType.yang, 0.5, 0.12, 2, 41.14, Stat.agility, 0.20),
        ],
    ]),
)

const yori = new Character(
    "Yorihime",
    new Stats(5550, 1370, 1000, 925, 1420, 1075),
    new Atack([
        [
            new BulletGroup(AtkType.yin, 0.80, 0.5, 18, 5.71, Stat.agility, 0.25)
        ],
        [
            new BulletGroup(AtkType.yin, 0.80, 0.5, 3, 13.71, Stat.agility, 0.25),
        ],
        [
            new BulletGroup(AtkType.yin, 0.80, 0.5, 3, 16.00, Stat.agility, 0.30),
        ],
        [new BulletGroup(AtkType.yin, 0.80, 0.5, 3, 18.29, Stat.agility, 0.35),
        new BulletGroup(AtkType.yin, 0.80, 0.5, 3, 20.57, Stat.agility, 0.35),
        new BulletGroup(AtkType.yin, 0.80, 0.5, 3, 22.86, Stat.agility, 0.35)],
    ]),
)

const tenshi = new Character(
    "Tenshi",
    new Stats(5850, 810, 1540, 1450, 1140, 790),
    new Atack([
        [
            new BulletGroup(AtkType.yang, 0.75, 0.05, 10, 10.29 * 1.6, Stat.yang_def, 1.10)
        ],
        [
            new BulletGroup(AtkType.yang, 0.75, 0.05, 2, 14.63 * 1.3, Stat.yang_def, 1.10),
            new BulletGroup(AtkType.yang, 0.75, 0.05, 2, 13.41 * 1.6, Stat.yang_def, 1.10),
        ],
        [
            new BulletGroup(AtkType.yang, 0.75, 0.05, 2, 18.29 * 1.6, Stat.health, 0.0),
            new BulletGroup(AtkType.yang, 0.75, 0.05, 2, 16.46 * 1.3, Stat.health, 0.0),
        ],
        [new BulletGroup(AtkType.yang, 0.75, 0.05, 2, 29.26 * 1.6, Stat.health, 0.0)],
    ]),
)
draw_data(sim(test_char, 1, [], dummy, []))
