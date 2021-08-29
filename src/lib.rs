mod effects;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cmp::min, collections::HashMap, convert::TryInto, usize};

use ordered_float::NotNan;
use wasm_bindgen::prelude::*;

use std::panic;

use crate::effects::{DefaultEffectMultiWorld, DefaultMultiEffectState, EffectState};

#[wasm_bindgen(module = "/md.js")]
extern "C" {
    pub fn start_drawing(idx: usize);
    pub fn end_drawing();
    pub fn move_to(x: f64, y: f64);
    pub fn debug(a: JsValue);
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn sim_char(data: String, effects: String, debufs: String, power: usize, slot: usize) {
    let zro = NotNan::new(0.0).unwrap();
    debug_str(
        &serde_json::to_string(&BulletGroup {
            acc: zro,
            amount: 0,
            crit: zro,
            scales_off: Stats::zeroed(),
            typ: Atktyp::Yang,
            effects_self: vec![],
            effects_enem: vec![],
        })
        .unwrap(),
    );

    panic::set_hook(Box::new(|x| debug_str(&format!("{}", x))));

    let effects: Vec<RawEffect> = match serde_json::from_str(&effects) {
        Ok(a) => a,
        Err(a) => {
            debug_str(&a.to_string());
            panic!();
        }
    };

    let effects: Vec<_> = effects
        .into_iter()
        .filter_map(|x| match x {
            RawEffect::Stat(x) => Some(x),
            _ => None,
        })
        .collect();

    //fix the code duplcation here
    let debufs: Vec<RawEffect> = match serde_json::from_str(&debufs) {
        Ok(a) => a,
        Err(a) => {
            debug_str(&a.to_string());
            panic!();
        }
    };

    let debufs: Vec<_> = debufs
        .into_iter()
        .filter_map(|x| match x {
            RawEffect::Stat(x) => Some(x),
            _ => None,
        })
        .collect();

    let char = match serde_json::from_str(&data) {
        Ok(a) => a,
        Err(a) => {
            debug_str(&a.to_string());
            panic!();
        }
    };
    let default_stats = Stats::new(
        NotNan::new(1000.0).unwrap(),
        NotNan::new(1000.0).unwrap(),
        NotNan::new(1000.0).unwrap(),
        NotNan::new(1000.0).unwrap(),
        NotNan::new(1000.0).unwrap(),
        NotNan::new(1000.0).unwrap(),
    );

    let res = sim(
        &char,
        power,
        &effects,
        &Enemy {
            stats: default_stats.clone(),
        },
        &debufs,
    );
    debug_str(&format!("{:?}", debufs));
    unsafe {
        start_drawing(slot);

        let max_y = NotNan::new_unchecked(7_000.0);
        for (y, x) in res {
            move_to(*x, *min(NotNan::new_unchecked(1.0), y / max_y));
        }

        end_drawing();
    }
}

/*
#[wasm_bindgen]
pub fn serialize_bullet_group(
    atk_type: String,
    acc: f64,
    crit: f64,
    amount: usize,
    scaling: String,
    factor: f64,
) -> String {
    let atk_type = serde_json::from_str(&atk_type)
    .map_err(|x| format!("{}", x))
    .unwrap_throw();
    //let scaling = serde_json::from_str(scaling).unwrap();""

    let bg = BulletGroup {
        typ: atk_type,
        acc: acc
        .try_into()
        .map_err(|x: ordered_float::FloatIsNan| format!("{}", x))
        .unwrap_throw(),
        amount,
        crit: crit
        .try_into()
        .map_err(|x: ordered_float::FloatIsNan| format!("{}", x))
        .unwrap_throw(),
        scales_off: Stats::zeroed(),
    };

    let res = serde_json::to_string(&bg)
    .map_err(|x| format!("{}", x))
    .unwrap_throw();
    res
}
*/

fn debug_str(a: &str) {
    #[allow(unused_unsafe)]
    unsafe {
        debug(JsValue::from_str(a));
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
enum Atktyp {
    Yang,
    Yin,
}
#[allow(unused)]
fn to_stat(a: Atktyp) -> Stat {
    match a {
        Atktyp::Yang => Stat::YangAtk,
        Atktyp::Yin => Stat::YinAtk,
    }
}
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]

enum BarrierAnomaly {
    Poison,
    Burn,
    Paralize,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Stat {
    Health,
    Agility,
    YangAtk,
    YangDef,
    YinAtk,
    YinDef,
}
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum CombatStat {
    Acc,
    CritAcc,
    CritAtk,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Stats {
    health: NotNan<f64>,
    agility: NotNan<f64>,
    yang_atk: NotNan<f64>,
    yang_def: NotNan<f64>,
    yin_atk: NotNan<f64>,
    yin_def: NotNan<f64>,
}

#[allow(unused)]
impl Stats {
    fn zeroed() -> Self {
        Self::new(
            NotNan::new(0.0).expect("definitely not a nan"),
            NotNan::new(0.0).expect("definitely not a nan"),
            NotNan::new(0.0).expect("definitely not a nan"),
            NotNan::new(0.0).expect("definitely not a nan"),
            NotNan::new(0.0).expect("definitely not a nan"),
            NotNan::new(0.0).expect("definitely not a nan"),
        )
    }

    fn new(
        health: NotNan<f64>,
        agility: NotNan<f64>,
        yang_atk: NotNan<f64>,
        yang_def: NotNan<f64>,
        yin_atk: NotNan<f64>,
        yin_def: NotNan<f64>,
    ) -> Self {
        Self {
            health,
            agility,
            yang_atk,
            yang_def,
            yin_atk,
            yin_def,
        }
    }

    fn sum(&self) -> NotNan<f64> {
        self.health + self.agility + self.yang_atk + self.yang_def + self.yin_atk + self.yin_def
    }

    fn mul(&self, second: &Stats) -> Stats {
        Stats::new(
            self.health * second.health,
            self.agility * second.agility,
            self.yang_atk * second.yang_atk,
            self.yang_def * second.yang_def,
            self.yin_atk * second.yin_atk,
            self.yin_def * second.yin_def,
        )
    }

    fn mul_all(&self, second: NotNan<f64>) -> Stats {
        return Stats::new(
            self.health * second,
            self.agility * second,
            self.yang_atk * second,
            self.yang_def * second,
            self.yin_atk * second,
            self.yin_def * second,
        );
    }

    fn add(&self, second: &Stats) -> Stats {
        return Stats::new(
            self.health + second.health,
            self.agility + second.agility,
            self.yang_atk + second.yang_atk,
            self.yang_def + second.yang_def,
            self.yin_atk + second.yin_atk,
            self.yin_def + second.yin_def,
        );
    }

    fn from_stat(b: &Stat, c: NotNan<f64>) -> Stats {
        let zero = NotNan::new(0.0).expect("definitely not a nan");
        match b {
            Stat::Health => Stats::new(c, zero, zero, zero, zero, zero),
            Stat::Agility => Stats::new(zero, c, zero, zero, zero, zero),
            Stat::YangAtk => Stats::new(zero, zero, c, zero, zero, zero),
            Stat::YangDef => Stats::new(zero, zero, zero, c, zero, zero),
            Stat::YinAtk => Stats::new(zero, zero, zero, zero, c, zero),
            Stat::YinDef => Stats::new(zero, zero, zero, zero, zero, c),
        }
    }

    fn form_array(a: [NotNan<f64>; 6]) -> Self {
        Stats::new(a[0], a[1], a[2], a[3], a[4], a[5])
    }
}

#[derive(Debug, Clone)]
struct Modifiers {
    acc: NotNan<f64>,
    crit_acc: NotNan<f64>,
    crit_atk: NotNan<f64>,
}

impl Modifiers {
    fn new(acc: NotNan<f64>, crit_acc: NotNan<f64>, crit_atk: NotNan<f64>) -> Self {
        Self {
            acc,
            crit_acc,
            crit_atk,
        }
    }
    #[allow(unused)]
    fn zeroed() -> Self {
        let zero = NotNan::new(0.0).unwrap();
        Self::new(zero, zero, zero)
    }
    #[allow(unused)]

    fn add(&self, other: &Modifiers) -> Modifiers {
        Self::new(
            self.acc + other.acc,
            self.crit_acc + other.crit_acc,
            self.crit_atk + other.crit_atk,
        )
    }

    #[allow(unused)]
    fn mul(&self, other: &Modifiers) -> Modifiers {
        Self::new(
            self.acc * other.acc,
            self.crit_acc * other.crit_acc,
            self.crit_atk * other.crit_atk,
        )
    }
    #[allow(unused)]

    fn from_modifier(b: &CombatStat, c: NotNan<f64>) -> Modifiers {
        match b {
            CombatStat::Acc => Modifiers::new(
                c,
                NotNan::new(0.0).expect("definitely not a nan"),
                NotNan::new(0.0).expect("definitely not a nan"),
            ),
            CombatStat::CritAcc => Modifiers::new(
                NotNan::new(0.0).expect("definitely not a nan"),
                c,
                NotNan::new(0.0).expect("definitely not a nan"),
            ),
            CombatStat::CritAtk => Modifiers::new(
                NotNan::new(0.0).expect("definitely not a nan"),
                NotNan::new(0.0).expect("definitely not a nan"),
                c,
            ),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct RawStatEffect {
    stat: AllEffect,
    lvl: i64,
    chance: NotNan<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
enum RawEffect {
    Stat(RawStatEffect),
    PercentIncrease(String, NotNan<f64>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
enum AllEffect {
    Normal(Stat),
    Combat(CombatStat),
    Barrier(BarrierAnomaly),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GoesOff {
    stat: Stat,
    multiplier: NotNan<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BulletGroup {
    typ: Atktyp,

    acc: NotNan<f64>,
    crit: NotNan<f64>,

    amount: usize,

    scales_off: Stats,

    effects_self: Vec<RawStatEffect>,
    effects_enem: Vec<RawStatEffect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Atack {
    bullets: [Vec<BulletGroup>; 4],
}

#[derive(Serialize, Deserialize, Debug)]
struct Enemy {
    stats: Stats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Character {
    name: String,
    stats: Stats,
    lw: Atack,
}

/// returns ordered amount of instances of either event1, event2 (or implied neither)
/// damage2 must be higher than damage1
fn bulletgroup_possibilities(
    count: usize,
    _damage1: NotNan<f64>,
    _damage2: NotNan<f64>,
) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let ret_count = count + 1;
    ret.reserve(ret_count * (ret_count / 2 + 1));

    for a in 0..ret_count {
        for b in a..ret_count {
            ret.push((a, b - a));
        }
    }

    ret
}

fn bulletlines_to_percentiles(
    bullet_group: BulletGroup,
    accbuff: NotNan<f64>,
    crit_acc_buff: NotNan<f64>,
    crit_atk_buff: NotNan<f64>,
) -> Vec<(Stats, NotNan<f64>)> {
    let one = NotNan::new(1.0).expect("definitely not a nan");
    //let zero = NotNan::new(0.0).expect("definitely not a nan");

    let factorials = {
        let mut factorials = [0 as i128; 30]; //remember to set an upper limit on the amount of bullets
        factorials[0] = 1;
        for a in 1..30 {
            factorials[a] = factorials[a - 1] * a as i128;
        }
        factorials
    };

    let hit_chance = min(bullet_group.acc * accbuff, one);
    let crit_chance = min(bullet_group.crit * crit_acc_buff, one);

    let crit_chance = crit_chance * hit_chance;
    let hit_chance = hit_chance - crit_chance;
    let neither_chance = one - (hit_chance + crit_chance);

    let hit_damage = bullet_group.scales_off.clone();
    let crit_damage = bullet_group.scales_off.mul_all(crit_atk_buff);
    let possibilities = bulletgroup_possibilities(bullet_group.amount, one, crit_atk_buff);

    possibilities
        .into_iter()
        .map(move |(hits, crits)| {
            let empties = bullet_group.amount - (hits + crits);

            let base_chance = factorials[bullet_group.amount];

            let base_chance = base_chance as f64;

            let combinations = neither_chance.powi(empties as i32)
                * hit_chance.powi(hits as i32)
                * crit_chance.powi(crits as i32);

            let total_chance = base_chance * combinations
                / (factorials[hits] * factorials[empties] * factorials[crits]) as f64;

            let total_damage = crit_damage
                .mul_all(NotNan::new(crits as f64).expect("uhh"))
                .add(&hit_damage.mul_all(NotNan::new(hits as f64).expect("uhh")));

            (total_damage, NotNan::new(total_chance).expect("eugh"))
        })
        .collect()
}

// Vec<Chance<(Stats, (i64, i64, i64)>>
// Vec<(Stats, (i64, i64, i64), NotNan<f64>)>

fn sim(
    char: &Character,
    power: usize,
    effects: &[RawStatEffect],
    enemy: &Enemy,
    enemy_effects: &[RawStatEffect],
) -> Vec<(NotNan<f64>, NotNan<f64>)> {
    let one = NotNan::new(1.0).expect("definitely not a nan");
    let zero = NotNan::new(0.0).expect("definitely not a nan");

    let char_stats = char.stats.clone();
    let enem_stats = enemy.stats.clone();

    debug_str(&format!("{:?}", effects));

    type EffectsType = DefaultEffectMultiWorld<Vec<(NotNan<f64>, NotNan<f64>)>>;
    let mut effect_world = EffectsType::new(vec![(zero, one)]);

    for eff in effects {
        effect_world.apply_char_effects(eff.clone());
    }

    for eff in enemy_effects {
        effect_world.apply_enemy_effects(eff.clone());
    }

    let lines = char.lw.bullets[0..power + 1]
        .iter()
        .map(|x| x.iter())
        .flatten();
    //debug_str(&format!("{:?}", char_stats));

    for line in lines {
        for eff_self in line.effects_self.iter().cloned() {
            effect_world.apply_char_effects(eff_self);
        }

        for eff_enem in line.effects_enem.iter().cloned() {
            effect_world.apply_enemy_effects(eff_enem);
        }

        effect_world.states = effect_world
            .states
            .drain()
            .map(|((state, damages), chance)| {
                let (char_stat_mul, enem_stat_mul, combat_stats) = state.get_stats();
                let char_stats = char_stats.mul(&char_stat_mul);
                let enem_stats = enem_stats.mul(&enem_stat_mul);

                let divby = match line.typ {
                    Atktyp::Yang => enem_stats.yang_def,
                    Atktyp::Yin => enem_stats.yin_def,
                };
                let next_hits = bulletlines_to_percentiles(
                    line.clone(),
                    combat_stats[0],
                    combat_stats[1],
                    combat_stats[2],
                );
                let damages_result = damages
                    .into_iter()
                    .map(|(damage, chance)| {
                        let char_stats = &char_stats;
                        next_hits.clone().into_iter().map(move |(stats, chance2)| {
                            (
                                stats.mul(char_stats).sum() / &divby + damage.clone(), //damage
                                chance * chance2,                                      //chance
                            )
                        })
                    })
                    .flatten()
                    .collect_vec();

                ((state, damages_result), chance)
            })
            .collect();
    }

    effect_world
        .states
        .into_iter()
        .map(|((_, damages), eff_chance)| {
            damages
                .into_iter()
                .map(move |(damage, chance)| (damage, chance * eff_chance))
        })
        .flatten()
        .collect()
}
