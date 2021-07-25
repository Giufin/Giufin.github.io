use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cmp::min, collections::HashMap, convert::TryInto, usize};

use ordered_float::NotNan;
use wasm_bindgen::prelude::*;

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
pub fn greet() {
    let fifty = NotNan::new(0.5).unwrap();
    let ef1 = RawEffect::Stat(Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Normal(Stat::YinAtk),
    });
    let ef2 = RawEffect::Stat(Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Normal(Stat::YinDef),
    });
    let ef3 = RawEffect::Stat(Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Normal(Stat::YangAtk),
    });
    let ef4 = RawEffect::Stat(Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Normal(Stat::YinAtk),
    });
    let ef5 = RawEffect::Stat(Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Normal(Stat::Agility),
    });

    let ef6 = Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Combat(CombatStat::Acc),
    };
    let ef7 = Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Combat(CombatStat::CritAcc),
    };
    let ef8 = Effect {
        chance: fifty,
        lvl: 1,
        stat: BothStat::Combat(CombatStat::CritAtk),
    };

    debug_str(&format!(
        "{:?} \n {:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n",
        serde_json::to_string_pretty(&ef1).unwrap(),
        serde_json::to_string_pretty(&ef2).unwrap(),
        serde_json::to_string_pretty(&ef3).unwrap(),
        serde_json::to_string_pretty(&ef4).unwrap(),
        serde_json::to_string_pretty(&ef5).unwrap(),
        serde_json::to_string_pretty(&ef6).unwrap(),
        serde_json::to_string_pretty(&ef7).unwrap(),
        serde_json::to_string_pretty(&ef8).unwrap(),
    ));
}

#[wasm_bindgen]
pub fn sim_char(data: String, effects: String, debufs: String, power: usize, slot: usize) {
    debug_str(&data);

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
enum Stat {
    Health,
    Agility,
    YangAtk,
    YangDef,
    YinAtk,
    YinDef,
}

impl Stat {
    fn get_index(&self) -> usize {
        match self {
            Stat::Health => 0,
            Stat::Agility => 1,
            Stat::YangAtk => 2,
            Stat::YangDef => 3,
            Stat::YinAtk => 4,
            Stat::YinDef => 5,
        }
    }
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
enum RawEffect {
    Stat(Effect),
    PercentIncrease(String, NotNan<f64>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
enum BothStat {
    Normal(Stat),
    Combat(CombatStat),
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct Effect {
    stat: BothStat,
    lvl: i64,
    chance: NotNan<f64>,
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

fn apply_effects(
    effects_ex: &[Effect],
) -> (
    Vec<([i64; 6], NotNan<f64>)>,
    Vec<((i64, i64, i64), NotNan<f64>)>,
) {
    let mut effects: HashMap<BothStat, (i64, Vec<NotNan<f64>>)> = HashMap::new();

    // we will treat the stats and their respective chance to proc + 1 as an array of 9 tuples of guaranteed level as well as the chance, because doing self in a typ safe way would be a chore, and so is using HashMaps indexed by the stat's respective enum
    for eff in effects_ex {
        let re = effects
            .entry(eff.stat.clone())
            .or_insert_with(|| (0, Vec::new()));
        re.0 += eff.lvl;
        if eff.chance != NotNan::new(0.0).unwrap() {
            re.1.push(eff.chance);
        }
    }

    let all_stats = vec![
        BothStat::Normal(Stat::Health),
        BothStat::Normal(Stat::YinAtk),
        BothStat::Normal(Stat::YinDef),
        BothStat::Normal(Stat::YangAtk),
        BothStat::Normal(Stat::YangDef),
        BothStat::Normal(Stat::Agility),
        BothStat::Combat(CombatStat::Acc),
        BothStat::Combat(CombatStat::CritAcc),
        BothStat::Combat(CombatStat::CritAtk),
    ];

    for a in all_stats.into_iter() {
        effects.entry(a).or_insert((0, Vec::new()));
    }

    fn get_dict_for_stat(
        stat_level: i64,
        stat_chances: &[NotNan<f64>],
    ) -> HashMap<i64, NotNan<f64>> {
        let mut dict = HashMap::<i64, NotNan<f64>>::new();

        let one = NotNan::new(1.0).expect("definitely not a nan");
        let zero = NotNan::new(0.0).expect("definitely not a nan");

        dict.insert(stat_level, one);

        for chance1 in stat_chances {
            let mut temp = HashMap::<i64, NotNan<f64>>::new();

            for (lvl, chance) in dict {
                let key = lvl;
                *temp.entry(key).or_insert(zero) += chance * (one - chance1);

                let key = min(key + 1, 10);
                *temp.entry(key).or_insert(zero) += chance * chance1;
            }

            dict = temp
        }

        dict
    }

    let chances = effects
        .iter()
        .map(|(stat, (x, y))| (stat, get_dict_for_stat(*x, &y)));

    let one = NotNan::new(1.0).expect("definitely not a nan");

    let mut output1 = vec![([0; 6], one)];

    let mut output2 = vec![((0, 0, 0), one)];

    for (stat, chances) in chances {
        match stat {
            BothStat::Combat(x) => {
                let temp = std::mem::replace(&mut output2, Vec::new());

                for (previous, prevchance) in temp {
                    for (level, chance) in chances.clone() {
                        let level = level.clamp(-10, 10);
                        let newe = match x {
                            CombatStat::Acc => (((previous).0 + level, (previous).1, (previous).2)),

                            CombatStat::CritAcc => {
                                ((previous).0, (previous).1 + level, (previous).2)
                            }

                            CombatStat::CritAtk => {
                                ((previous).0, (previous).1, (previous).2 + level)
                            }
                        };

                        output2.push((newe, chance * prevchance));
                    }
                }
            }

            BothStat::Normal(x) => {
                let temp = std::mem::replace(&mut output1, Vec::new());

                for (mut previous, prevchance) in temp {
                    for (level, chance) in chances.clone() {
                        let level = level.clamp(-10, 10);
                        previous[x.get_index()] += level;

                        output1.push((previous, chance * prevchance));
                    }
                }
            }
        }
    }

    (output1, output2)
}

fn mult_from_lvl(mult: NotNan<f64>, level: i64) -> NotNan<f64> {
    let one = NotNan::new(1.0).expect("definitely not a nan");
    if level < 0 {
        one / (one + NotNan::new((-level) as f64).unwrap() * mult)
    } else {
        one * (one + NotNan::new(level as f64).unwrap() * mult)
    }
}

fn buffs_to_numbers(acc: i64, crit_acc: i64, crit_atk: i64) -> Modifiers {
    let acc = acc.clamp(-10, 10);
    let crit_acc = crit_acc.clamp(-10, 10);
    let crit_atk = crit_atk.clamp(-10, 10);

    let acc_mult = mult_from_lvl(NotNan::new(0.2).unwrap(), acc);
    let crit_acc_mult = mult_from_lvl(NotNan::new(0.2).unwrap(), crit_acc);
    let crit_atk_mult = mult_from_lvl(NotNan::new(0.3).unwrap(), crit_atk);

    Modifiers::new(
        acc_mult,
        crit_acc_mult,
        crit_atk_mult + NotNan::new(1.0).unwrap(),
    )
}

fn sim(
    char: &Character,
    power: usize,
    effects: &[Effect],
    enemy: &Enemy,
    enemy_effects: &[Effect],
) -> Vec<(NotNan<f64>, NotNan<f64>)> {
    let char_stats = char.stats.clone();
    let enem_stats = enemy.stats.clone();

    let (char_stat_effects, char_combat_effects) = apply_effects(effects);
    let (enemy_stat_effects, enemy_combat_effects) = apply_effects(enemy_effects);

    let one = NotNan::new(1.0).expect("definitely not a nan");
    let zero = NotNan::new(0.0).expect("definitely not a nan");

    //debug_str(&format!("{:?}", char_stats));

    let combat_effects_in_play: Vec<_> = char_combat_effects // mutable because in the future effects will be pushed from bulletlines
        .into_iter()
        .cartesian_product(enemy_combat_effects.into_iter())
        .map(|(a, b)| {
            (
                buffs_to_numbers((a.0).0 + (b.0).0, (a.0).1 + (b.0).1, (a.0).2 + (b.0).2),
                a.1 * b.1,
            )
        })
        .filter(|(_, c)| *c != zero)
        .collect();

    let mut hits: Vec<_> = combat_effects_in_play
        .into_iter()
        .map(|x| (x, Vec::new()))
        .collect();

    for line in char.lw.bullets[0..power + 1]
        .iter()
        .map(|x| x.iter())
        .flatten()
    {
        for ((modifiers, chance), mut damage) in std::mem::replace(&mut hits, Vec::new()) {
            let mut unsorted = Vec::new();

            let (acc, crit_acc, crit_atk) = (modifiers.acc, modifiers.crit_acc, modifiers.crit_atk);

            let res = bulletlines_to_percentiles(line.clone(), acc, crit_acc, crit_atk);

            for (damage, bullet_chance) in res {
                unsorted.push((damage, bullet_chance * chance));
            }

            damage.push((unsorted, line.typ.clone()));
            // now here if there is a line effect we double the damage, and push one version for each condition, in the case where there are no effects attached we just push a single element
            hits.push(((modifiers, chance), damage));
        }
    }

    // I wonder how well/not well can this be optimized
    let groups: Vec<(Vec<_>, _)> = hits
        .into_iter()
        .map(|(_, line)| line)
        .reduce(|prev, next| {
            prev.into_iter()
                .zip(next.into_iter())
                .map(|((mut a, _), (b, typ))| {
                    a.extend(b.into_iter());
                    (a, typ)
                })
                .collect()
        })
        .unwrap();

    // log(5, 1e6) = ~8.5, which means that if we are about to overflow on data the safe
    // lower bound for a single group is around 8, although that is a way too little data, the safest aproach would be to assume
    // that there will be no units with more bullets than yuuka in the game (25, 5, 5, 5, 5, 5), then we can first multiply out
    // half the data (390k from the first 4 groups, 78k from the following ones) we can fold each of these to about a thousand

    let mut combined_stat_effects = char_stat_effects
        .into_iter()
        .cartesian_product(enemy_stat_effects.into_iter())
        .map(|x| (x, Vec::new()))
        .collect();

    groups.into_iter().for_each(|(group, scales_against)| {
        for (
            ((char_stat_effect, char_eff_chance), (enemy_stat_effect, enemy_eff_chance)),
            mut damages,
        ) in std::mem::replace(&mut combined_stat_effects, Vec::new())
        {
            let char_stats = char_stats.mul(&Stats::form_array(
                std::array::IntoIter::new(char_stat_effect)
                    .map(|x| mult_from_lvl(NotNan::new(0.3).unwrap(), x))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ));

            let enemy_stats = enem_stats.mul(&Stats::form_array(
                std::array::IntoIter::new(enemy_stat_effect)
                    .map(|x| mult_from_lvl(NotNan::new(0.3).unwrap(), x))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ));

            let mut data = Vec::new();
            for (damage, chance) in &group {
                let divby = match scales_against {
                    Atktyp::Yang => &enemy_stats.yang_def,
                    Atktyp::Yin => &enemy_stats.yin_def,
                };
                data.push((damage.mul(&char_stats).sum() / divby, chance.clone()));
            }
            damages.push(data);

            //same as above, if there is a line efffect we push multiple, if not we push one
            combined_stat_effects.push((
                (
                    (char_stat_effect, char_eff_chance),
                    (enemy_stat_effect, enemy_eff_chance),
                ),
                damages,
            ))
        }

        debug_str(&format!("stat eff len {}", combined_stat_effects.len()));
    });
    let mut unsorted = combined_stat_effects
        .into_iter()
        .map(|(((_, chance_a), (_, chance_b)), affected)| {
            //here we can create affected ex and shadow affected with it; with f.in. only 2 groups. The following code should not depend on the ammount of groups

            const MAX_SIM: usize = 1_000_000;
            let total_len = affected.iter().fold(1, |a, b| a * b.len());
            debug_str(&format!("total: {:?}", total_len));
            for a in affected.clone() {
                debug_str(&format!("part: {:?}", a.len()));
            }
            if total_len > MAX_SIM {
                //todo!("more then 1mil data entries")
            }

            affected
                .into_iter()
                .fold(
                    vec![(zero, one)],
                    |prev: Vec<(NotNan<f64>, NotNan<f64>)>, next| {
                        prev.into_iter()
                            .cartesian_product(next.into_iter())
                            .map(|((x_dmg, x_chance), (y_dmg, y_chance))| {
                                (x_dmg + y_dmg, x_chance * y_chance)
                            })
                            .collect()
                    },
                )
                .into_iter()
                .map(|x| (x.0, x.1 * chance_a * chance_b))
                .collect()
        })
        .reduce(|mut a: Vec<_>, b| {
            a.extend(b.into_iter());
            a
        })
        .unwrap();

    unsorted.sort(); // not anymore :)

    //debug_str(&format!("unsorted: {:?}", unsorted));

    for i in 1..unsorted.len() {
        unsorted[i].1 = unsorted[i - 1].1 + unsorted[i].1;
    }
    unsorted
}

/*

fn draw_data(data: [(NotNan<f64>, NotNan<f64>)]) {
    let canvas = document.getElementById("canvas") as
        HTMLCanvasElement;

    let mut context = canvas.getContext("2d");
    if (context == null) {
        throw "temp"
    }

    let line_to = (x: NotNan<f64>, y: NotNan<f64>) => {
        if (context == null) {
            throw "temp"
        }

        context.lineTo(x, canvas.height - y);
    }

    context.beginPath();
    line_to(0, 0);


    var prev_y = 0;

    for (let [y_f, x_f] of data) {
        let y = (y_f * canvas.height) / 6NotNan::new(0.0).expect("definitely not a nan");
        let x = x_f * canvas.width;

        line_to(x, prev_y);
        line_to(x, y);
        prev_y = y;
    }




    context.stroke();
}


let mut dummy = Enemy(new Stats::new(1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan")))
let mut test_char = Character::new(
    "test",
    Stats::new(1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan"), 1NotNan::new(0.0).expect("definitely not a nan")),
    Atack::new([
        [BulletGroup::new(Atktyp::yang, NotNan::new(0.0).expect("definitely not a nan")05, 1.0, 1, 1NotNan::new(0.0).expect("definitely not a nan"), Stat::yang_def, NotNan::new(0.0).expect("definitely not a nan"))],
        [BulletGroup::new(Atktyp::yang, 0.50, NotNan::new(0.0).expect("definitely not a nan")5, 1, 4NotNan::new(0.0).expect("definitely not a nan"), Stat::yang_def, NotNan::new(0.0).expect("definitely not a nan"))],
        [BulletGroup::new(Atktyp::yin, 0.50, NotNan::new(0.0).expect("definitely not a nan")5, 1, 16NotNan::new(0.0).expect("definitely not a nan"), Stat::yin_def, NotNan::new(0.0).expect("definitely not a nan"))],
        [BulletGroup::new(Atktyp::yang, 0.50, NotNan::new(0.0).expect("definitely not a nan")5, 1, 64NotNan::new(0.0).expect("definitely not a nan"), Stat::yang_def, NotNan::new(0.0).expect("definitely not a nan"))],
    ]),
)


let chen = Character::new(
    "Chen",
    Stats::new(4500, 1200, 1220, 985, 880, 815),
    Atack::new([
        [BulletGroup::new(Atktyp::yang, 0.5, 0.12, 16, 11.25, Stat::health, NotNan::new(0.0).expect("definitely not a nan"))],
        [BulletGroup::new(Atktyp::yang, 0.5, 0.12, 3, 20.57, Stat::agility, 0.20)],
        [
            BulletGroup::new(Atktyp::yang, 0.5, 0.12, 3, 22.29, Stat::agility, 0.20),
            BulletGroup::new(Atktyp::yang, 0.5, 0.12, 2, 36.0, Stat::agility, 0.20),
        ],
        [
            BulletGroup::new(Atktyp::yang, 0.5, 0.12, 2, 38.57, Stat::agility, 0.20),
            BulletGroup::new(Atktyp::yang, 0.5, 0.12, 2, 41.14, Stat::agility, 0.20),
        ],
    ]),
)

let yori = Character::new(
    "Yorihime",
    Stats::new(5550, 1370, 1NotNan::new(0.0).expect("definitely not a nan"), 925, 1420, 1075),
    Atack::new([
        [
            BulletGroup::new(Atktyp::yin, 0.80, 0.5, 18, 5.71, Stat::agility, 0.25)
        ],
        [
            BulletGroup::new(Atktyp::yin, 0.80, 0.5, 3, 13.71, Stat::agility, 0.25),
        ],
        [
            BulletGroup::new(Atktyp::yin, 0.80, 0.5, 3, 16.00, Stat::agility, 0.30),
        ],
        [BulletGroup::new(Atktyp::yin, 0.80, 0.5, 3, 18.29, Stat::agility, 0.35),
        BulletGroup::new(Atktyp::yin, 0.80, 0.5, 3, 20.57, Stat::agility, 0.35),
        BulletGroup::new(Atktyp::yin, 0.80, 0.5, 3, 22.86, Stat::agility, 0.35)],
    ]),
)

let tenshi = Character::new(
    "Tenshi",
    Stats::new(5850, 810, 1540, 1450, 1140, 790),
    Atack::new([
        [
            BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 10, 10.29 * 1.6, Stat::yang_def, 1.10)
        ],
        [
            BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 2, 14.63 * 1.3, Stat::yang_def, 1.10),
            BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 2, 13.41 * 1.6, Stat::yang_def, 1.10),
        ],
        [
            BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 2, 18.29 * 1.6, Stat::health, NotNan::new(0.0).expect("definitely not a nan")),
            BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 2, 16.46 * 1.3, Stat::health, NotNan::new(0.0).expect("definitely not a nan")),
        ],
        [BulletGroup::new(Atktyp::yang, 0.75, NotNan::new(0.0).expect("definitely not a nan")5, 2, 29.26 * 1.6, Stat::health, NotNan::new(0.0).expect("definitely not a nan"))],
    ]),
)
draw_data(sim(test_char, 1, [], dummy, []))


*/
