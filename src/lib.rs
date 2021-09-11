mod effects;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cmp::min, usize};

use ordered_float::NotNan;
use wasm_bindgen::prelude::*;

use std::panic;

use crate::effects::DefaultEffectMultiWorld;

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
            panic!("{}", data);
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

    debug_str(&serde_json::to_string(&char).unwrap());
    debug_str(&serde_json::to_string(&effects).unwrap());



    debug_str(&format!("{:?}", debufs));
    #[allow(unused_unsafe)]
    unsafe {
        start_drawing(slot);

        for (y, x) in res {
            move_to(*x, *y);
        }

        end_drawing();
    }
}

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

pub enum BarrierAnomaly {
    Poison,
    Burn,
    Paralize,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Stat {
    Health,
    Agility,
    YangAtk,
    YangDef,
    YinAtk,
    YinDef,
}
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum CombatStat {
    Acc,
    CritAcc,
    CritAtk,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Stats {
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
pub struct RawStatEffect {
    stat: AllEffect,
    lvl: i64,
    chance: NotNan<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum RawEffect {
    Stat(RawStatEffect),
    PercentIncrease(String, NotNan<f64>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum AllEffect {
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


fn bulletlines_to_percentiles(
    bullet_group: BulletGroup,
    accbuff: NotNan<f64>,
    crit_acc_buff: NotNan<f64>,
    crit_atk_buff: NotNan<f64>,
) -> Vec<[(Stats, NotNan<f64>); 3]> {
    let one = NotNan::new(1.0).expect("definitely not a nan");

    let hit_chance = min(bullet_group.acc * accbuff, one);
    let crit_chance = min(bullet_group.crit * crit_acc_buff, one);

    let crit_chance = crit_chance * hit_chance;
    let hit_chance = hit_chance - crit_chance;
    let neither_chance = one - (hit_chance + crit_chance);

    let neither_damage = Stats::zeroed();
    let hit_damage = bullet_group.scales_off.clone();
    let crit_damage = bullet_group.scales_off.mul_all(crit_atk_buff);

    std::array::IntoIter::new([[
        (neither_damage, neither_chance),
        (hit_damage, hit_chance),
        (crit_damage, crit_chance),
    ]])
    .cycle()
    .take(bullet_group.amount)
    .collect()
}


fn sim(
    char: &Character,
    power: usize,
    effects: &[RawStatEffect],
    enemy: &Enemy,
    enemy_effects: &[RawStatEffect],
) -> Vec<(NotNan<f64>, NotNan<f64>)> {
    const QUANT: usize = 100000;

    let zero = NotNan::new(0.0).expect("definitely not a nan");
    let one = NotNan::new(1.0).expect("definitely not a nan");

    let cutoff = NotNan::new(0.00000000001).expect("definitely not a nan");

    let scale = one * 1.0;

    let variance = NotNan::new(0.84).unwrap();

    let mut buf_temp = vec![zero; QUANT];

    let mut buf1 = vec![zero; QUANT];
    buf1[0] = one;

    let char_stats = char.stats.clone();
    let enem_stats = enemy.stats.clone();

    type EffectsType = DefaultEffectMultiWorld<Vec<NotNan<f64>>>;
    let mut effect_world = EffectsType::new(buf1);

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
            .map(|((state, mut damages), chance)| {
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

                for hits in next_hits {
                    for (dmg, chance) in hits {
                        let dmg_s = (dmg.mul(&char_stats).sum() / divby) / scale;
                        let dmg_l = dmg_s * variance;
                        let dmg_qh = dmg_s.round() as usize + 1;
                        let dmg_ql = dmg_l.round() as usize;

                        let divby = NotNan::new((dmg_qh - dmg_ql) as f64).unwrap();
                        for i in 0..damages.len() {
                            let chance_prev = damages[i];
                            if chance_prev != zero {
                                let chance_next = chance_prev * chance / divby;
                                *buf_temp.get_mut(i + dmg_ql).unwrap() += chance_next;

                                *buf_temp.get_mut(i + dmg_qh).unwrap() -= chance_next;
                            }
                        }
                    }

                    let mut chance = zero;
                    for el in &mut buf_temp {
                        chance += *el;
                        if chance < cutoff {
                            chance = zero;
                        }

                        *el = chance;
                    }

                    std::mem::swap(&mut buf_temp, &mut damages);
                    buf_temp.fill(zero);
                }

                ((state, damages), chance)
            })
            .collect();
    }

    debug_str("here");

    let mut unsorted = effect_world
        .states
        .into_iter()
        .map(|((_, damages), eff_chance)| {
            damages
                .into_iter()
                .enumerate()
                .filter(|(_, chance)| chance != &zero)
                .map(move |(damage, chance)| {
                    (
                        NotNan::new(damage as f64).unwrap() * scale,
                        chance * eff_chance,
                    )
                })
        })
        .flatten()
        .collect_vec();
    debug_str("here");

    unsorted.sort();

    for i in 1..unsorted.len() {
        unsorted[i].1 = unsorted[i - 1].1 + unsorted[i].1;
    }
    debug_str("here");

    unsorted.retain(|(_, chance)| chance > &zero);

    unsorted
}
