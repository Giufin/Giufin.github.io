mod effects;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::OneOrMany;
use std::{cmp::min, collections::HashSet, usize};

use ordered_float::NotNan;
use wasm_bindgen::prelude::*;

use std::panic;

use crate::effects::DefaultEffectMultiWorld;

const QUANT: usize = 10000;

#[wasm_bindgen(module = "/md.js")]
extern "C" {
    pub fn start_drawing(idx: usize);
    pub fn end_drawing();
    pub fn move_to(x: f64, y: f64);
    pub fn debug(a: JsValue);
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]

enum Tag {
    Human,
    Youkai,
    Fairy,
    Gensokyo, //todo rest
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]

enum BulletType {
    Light,
    Sharp,
    Heavy,
    Slash,
    Body,
    Normal,
    Ofuda,
    Laser,
    Missile, //todo rest
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
enum Element {
    No,
    Moon,
    Sun,
    Star,
    Wood,
    Water,
    Fire,
    Earth,
    Metal,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum AtackCalc {
    Chart,
    Buff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AtackType {
    Lw,
    Sc1,
    Sc2,
    Sc3,
    Sc4,
    Ss,
    Fs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OneToThree {
    //common enough to deserve it
    One,
    Two,
    Three,
}

impl OneToThree {
    fn to_int(&self) -> i64 {
        match self {
            OneToThree::One => 0,
            OneToThree::Two => 1,
            OneToThree::Three => 2,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawAttack {
    typ: AtackType,
    graze: i64,
    power: i64,
    display: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ActionType {
    Skill(OneToThree),
    Attack(RawAttack),
    TurnEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

struct Action {
    act: ActionType,
    by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

struct SimData {
    chars: Vec<Character>,
    actions: Vec<Action>,
    enemy: Enemy,
}

#[wasm_bindgen]
pub fn sim_char(data: String, slot: usize) {
    panic::set_hook(Box::new(|x| debug_str(&format!("{}", x))));
    let one: NotNan<f64> = NotNan::new(1.0).expect("definitely not a nan");
    let zero: NotNan<f64> = NotNan::new(0.0).expect("definitely not a nan");

    let testdata = SimData {
        chars: vec![Character {
            name: "aaaaaaaaaa".into(),
            cards: [None, None, None, None, None],
            stats: Stats::new(one, one, one, one, one, one),
            lw: Attack {
                bullets: [
                    vec![BulletLine {
                        acc: one,
                        amount: 1,
                        bullet_type: BulletType::Normal,
                        crit: zero,
                        effects_self: vec![RawAllEffect {
                            stat: AllEffect::Normal(Stat::YangDef),
                            target: Target::Solo,
                            chances: vec![(1, one)],
                        }],
                        element: Element::No,
                        killer: HashSet::new(),
                        scales_off: Stats {
                            agility: zero,
                            health: zero,
                            yang_atk: one,
                            yang_def: zero,
                            yin_atk: zero,
                            yin_def: zero,
                        },
                        typ: Atktyp::Yang,
                        sure_hit: false,
                        precise: false,
                        elastic: false,
                        explosive: false,
                    }],
                    vec![],
                    vec![],
                    vec![],
                ],
            },
            sc1: Attack {
                bullets: [vec![], vec![], vec![], vec![]],
            },
            sc2: Attack {
                bullets: [vec![], vec![], vec![], vec![]],
            },
            skills: [
                Skill {
                    effs: vec![RawEffect::All(RawAllEffect {
                        chances: vec![(1, one)],
                        stat: AllEffect::Normal(Stat::Health),
                        target: Target::All,
                    })],
                },
                Skill { effs: vec![] },
                Skill { effs: vec![] },
            ],
        }],
        actions: vec![
            Action {
                by: 1,
                act: ActionType::Attack(RawAttack {
                    display: true,
                    graze: 0,
                    power: 0,
                    typ: AtackType::Lw,
                }),
            },
            Action {
                by: 1,
                act: ActionType::Skill(OneToThree::One),
            },
        ],
        enemy: Enemy {
            stats: Stats::new(one, one, one, one, one, one),
            skills: Vec::new(),
        },
    };

    let str = serde_json::to_string(&testdata).unwrap();
    debug_str(&str);
    let data: SimData = match serde_json::from_str(&data) {
        Ok(a) => a,
        Err(a) => {
            debug_str(&a.to_string());
            panic!();
        }
    };

    let mut buf1 = vec![zero; QUANT];
    buf1[0] = one;
    let mut effs = EffectsType::new(DamageState(buf1, zero), data.chars.len() as i64);

    for action in data.actions {
        let char = &data.chars[action.by as usize - 1];
        let enemy = &data.enemy;
        match action.act {
            ActionType::Attack(at) => {
                let (atk, sc) = match at.typ {
                    AtackType::Lw => (&char.lw, &char.cards[4]),
                    _ | AtackType::Fs | AtackType::Ss => todo!(),
                };

                let lines = atk.bullets[(0..=(at.power as usize))].iter().flatten();

                for line in lines {
                    effs = sim(line, &char.stats, &enemy.stats, action.by, effs);
                }
            }
            ActionType::Skill(idx) => {
                let skill = &char.skills[idx.to_int() as usize];
                for eff in &skill.effs {
                    match eff {
                        RawEffect::BulletModif(typ, ammount) => todo!(),
                        RawEffect::ElemModif(elem, ammount) => todo!(),
                        RawEffect::All(eff) => match eff.target {
                            Target::All | Target::Solo => {
                                effs.apply(eff.clone(), 0);
                            }
                            Target::Selff => {
                                effs.apply(eff.clone(), idx.to_int() + 1);
                            }
                            Target::Team => {
                                for i in 1..=data.chars.len() {
                                    effs.apply(eff.clone(), i as i64);
                                }
                            }
                        },
                    }
                }
            }
            ActionType::TurnEnd => todo!(), //perform random enemy action, take down bullet effects and such
        }
    }

    let res = exctract(effs);
    debug_str(&format!("{:?}", res));
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RawAllEffect {
    stat: AllEffect,
    target: Target,
    chances: Vec<(i64, NotNan<f64>)>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
enum Target {
    Solo,
    All,
    #[serde(rename = "Self")]
    Selff, //Self is a keyword
    Team,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
enum RawEffect {
    All(RawAllEffect),
    BulletModif(BulletType, NotNan<f64>),
    ElemModif(Element, NotNan<f64>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AllEffect {
    Normal(Stat),
    Combat(CombatStat),
    Barrier(BarrierAnomaly),
    Resource(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GoesOff {
    stat: Stat,
    multiplier: NotNan<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BulletLine {
    typ: Atktyp,

    acc: NotNan<f64>,
    crit: NotNan<f64>,

    amount: usize,
    scales_off: Stats,

    effects_self: Vec<RawAllEffect>,

    bullet_type: BulletType,
    element: Element,

    killer: HashSet<Tag>,
    sure_hit: bool,

    explosive: bool,
    precise: bool,
    elastic: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attack {
    bullets: [Vec<BulletLine>; 4],
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Skill {
    effs: Vec<RawEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

struct Enemy {
    stats: Stats,
    skills: Vec<Skill>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Character {
    name: String,
    stats: Stats,
    lw: Attack,
    sc1: Attack,
    sc2: Attack,
    skills: [Skill; 3],
    cards: [Option<Spellcard>; 5],
}
#[derive(Serialize, Deserialize, Debug, Clone)]

struct Spellcard {
    stats: Stats,
    effect: Vec<RawEffect>,
}

fn bulletlines_to_percentiles(
    bullet_group: BulletLine,
    accbuff: NotNan<f64>,
    crit_acc_buff: NotNan<f64>,
    crit_atk_buff: NotNan<f64>,
) -> Vec<[(Stats, NotNan<f64>); 3]> {
    let one: NotNan<f64> = NotNan::new(1.0).expect("definitely not a nan");

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
#[derive(Serialize, Deserialize, Debug, Clone)]

struct DamageState(Vec<NotNan<f64>>, NotNan<f64>);
impl std::ops::AddAssign<DamageState> for DamageState {
    fn add_assign(&mut self, rhs: DamageState) {
        let quant_not_nan = NotNan::new(QUANT as f64).expect("definitely not a nan");
        let lb = self.0.last().unwrap() * self.1;
        let rb = rhs.0.last().unwrap() * rhs.1;

        let (lb, rb) = if lb > rb {
            (lb, lb / rb)
        } else {
            (rb / lb, rb)
        };
        todo!();
    }
}
type EffectsType = DefaultEffectMultiWorld<DamageState>; //todo: move to a struct

fn sim(
    line: &BulletLine,
    char_stats: &Stats,
    enem_stats: &Stats,
    char_idx: i64,
    mut effects: EffectsType,
) -> EffectsType {
    let zero: NotNan<f64> = NotNan::new(0.0).expect("definitely not a nan");
    let quant_not_nan = NotNan::new(QUANT as f64).expect("definitely not a nan");

    let variance = NotNan::new(0.84).unwrap();

    let mut buf_temp = vec![zero; QUANT];

    effects.states = effects
        .states
        .drain()
        .map(|(state, (DamageState(mut damages, mut mult), chance))| {
            let (char_stat_mul, enem_stat_mul, combat_stats) = state.get_stats(char_idx);

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
                let previous_max_damage = mult * quant_not_nan;
                let current_max_damage = hits[2].0.mul(&char_stats).sum() / divby;
                let new_max_damage = current_max_damage + previous_max_damage;

                let divby2 = if previous_max_damage != zero {
                    new_max_damage / previous_max_damage
                } else {
                    new_max_damage
                };

                for i in 0..QUANT {
                    let ifloat = NotNan::new(i as f64).unwrap();
                    let i_adjusted = ifloat / divby2;
                    let i_round = i_adjusted.round() as usize;
                    let cur = damages[i];
                    damages[i] = zero;
                    damages[i_round] += cur;
                }

                let current_mult = new_max_damage / quant_not_nan;
                mult = current_mult;

                for (dmg, chance) in hits {
                    let dmg_s = (dmg.mul(&char_stats).sum() / divby) / current_mult;
                    let dmg_l = dmg_s * variance;
                    let dmg_qh = dmg_s.round() as usize + 1;
                    let dmg_ql = dmg_l.round() as usize;

                    let divby = NotNan::new((dmg_qh - dmg_ql) as f64).unwrap();
                    for i in 0..QUANT {
                        let chance_prev = damages[i];
                        if chance_prev != zero {
                            let chance_next = chance_prev * chance / divby;
                            *buf_temp.get_mut(i + dmg_ql).unwrap() += chance_next;

                            let highroll = buf_temp.get_mut(i + dmg_qh);
                            match highroll {
                                Some(a) => *a -= chance_next,
                                None => (),
                            }
                        }
                    }
                }

                let mut chance = zero;
                for el in &mut buf_temp {
                    chance += *el;
                    *el = chance;
                }

                std::mem::swap(&mut buf_temp, &mut damages);
                buf_temp.fill(zero);
            }

            (state, (DamageState(damages, mult), chance))
        })
        .collect();

    effects
}

fn exctract(effs: EffectsType) -> Vec<(NotNan<f64>, NotNan<f64>)> {
    let zero = NotNan::new(0.0).expect("definitely not a nan");

    let mut unsorted = effs
        .states
        .into_iter()
        .map(|(_, (DamageState(damages, mult), eff_chance))| {
            damages
                .into_iter()
                .enumerate()
                .filter(|(_, chance)| chance != &zero)
                .map(move |(damage, chance)| {
                    (
                        NotNan::new(damage as f64).unwrap() * mult,
                        chance * eff_chance,
                    )
                })
        })
        .flatten()
        .collect_vec();

    unsorted.sort();

    for i in 1..unsorted.len() {
        unsorted[i].1 = unsorted[i - 1].1 + unsorted[i].1;
    }

    unsorted.retain(|(_, chance)| chance > &zero);

    unsorted
}
