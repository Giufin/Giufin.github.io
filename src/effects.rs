use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, convert::TryInto, fmt::Debug, hash::Hash, marker::PhantomData,
    usize,
};

use ordered_float::{Float, NotNan};

use crate::{AllEffect, BarrierAnomaly, CombatStat, RawAllEffect, Stat, Stats};

fn mult_from_lvl(mult: NotNan<f64>, level: i64) -> NotNan<f64> {
    let one = NotNan::new(1.0).expect("definitely not a nan");
    if level < 0 {
        one / (one + NotNan::new((-level) as f64).unwrap() * mult)
    } else {
        one * (one + NotNan::new(level as f64).unwrap() * mult)
    }
}

pub trait MultiEffect<T>
where
    T: MultiEffectIdx,
    Self: Sized + std::fmt::Debug + Clone,
{
    const SIZE: usize;
    fn data(idx: usize) -> T;
}

pub trait MultiEffectIdx
where
    Self: Sized + std::fmt::Debug + Clone,
{
    type Colle: MultiEffect<Self>;
    type StateData: Clone;
    fn idx(&self) -> usize;
    fn apply(idx: usize, lvl: i64, target: EffectState<Self>) -> EffectState<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Effect<T: MultiEffectIdx> {
    stat: T,
    lvl: i64,
    chance: NotNan<f64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EffectState<T: MultiEffectIdx> {
    _phantom: PhantomData<T>,
    effects: Box<[i64]>,
    data: T::StateData,
}

impl<T: MultiEffectIdx> EffectState<T>
where
    T: std::fmt::Debug,
{
    pub fn new(default_data: T::StateData) -> Self {
        Self {
            _phantom: Default::default(),
            data: default_data,
            effects: (0..T::Colle::SIZE)
                .into_iter()
                .map(|_| 0)
                .collect::<Vec<_>>()
                .try_into()
                .expect(
                    "because this is an iterator over a range of the same size this cannot panic",
                ),
        }
    }

    fn apply_in_place(&mut self, lvl: i64, typ: T) {
        *self = T::apply(typ.idx(), lvl, self.clone());
    }
}

impl MultiEffectIdx for Stat {
    type Colle = StatMultiEffect;
    type StateData = ();
    fn idx(&self) -> usize {
        match self {
            Stat::Health => 0,
            Stat::Agility => 1,
            Stat::YangAtk => 2,
            Stat::YangDef => 3,
            Stat::YinAtk => 4,
            Stat::YinDef => 5,
        }
    }

    fn apply(idx: usize, lvl: i64, mut target: EffectState<Self>) -> EffectState<Self> {
        target.effects[idx] = (target.effects[idx] + lvl).clamp(-10, 10);
        target
    }
}

#[derive(Clone, Debug)]
pub struct StatMultiEffect {}

impl MultiEffect<Stat> for StatMultiEffect {
    const SIZE: usize = 6;

    fn data(idx: usize) -> Stat {
        match idx {
            0 => Stat::Health,
            1 => Stat::Agility,
            2 => Stat::YangAtk,
            3 => Stat::YangDef,
            4 => Stat::YinAtk,
            5 => Stat::YinDef,
            _ => panic!("bad stat index"),
        }
    }
}

impl MultiEffectIdx for BarrierAnomaly {
    type Colle = AnomallyMultiEffect;
    type StateData = i64;
    fn idx(&self) -> usize {
        //hack to get them all to share a cap
        match self {
            BarrierAnomaly::Poison => 0,
            BarrierAnomaly::Burn => 1,
            BarrierAnomaly::Paralize => 2,
        }
    }

    fn apply(idx: usize, lvl: i64, mut target: EffectState<Self>) -> EffectState<Self> {
        if lvl < 0 {
            todo!()
        }

        let tot: i64 = target.effects.iter().sum();
        let lvl = lvl.min(target.data - tot);

        target.effects[idx] += lvl;
        target
    }
}

#[derive(Clone, Debug)]
pub struct AnomallyMultiEffect {}

impl MultiEffect<BarrierAnomaly> for AnomallyMultiEffect {
    const SIZE: usize = 3;

    fn data(idx: usize) -> BarrierAnomaly {
        match idx {
            0 => BarrierAnomaly::Poison,
            1 => BarrierAnomaly::Burn,
            2 => BarrierAnomaly::Paralize,

            _ => panic!("bad stat index"),
        }
    }
}

impl MultiEffectIdx for CombatStat {
    type Colle = CombatMultiEffect;
    type StateData = ();

    fn idx(&self) -> usize {
        //hack to get them all to share a cap
        match self {
            CombatStat::Acc => 0,
            CombatStat::CritAcc => 1,
            CombatStat::CritAtk => 2,
        }
    }

    fn apply(idx: usize, lvl: i64, mut target: EffectState<Self>) -> EffectState<Self> {
        target.effects[idx] = (target.effects[idx] + lvl).clamp(-10, 10);
        target
    }
}

#[derive(Clone, Debug)]
pub struct CombatMultiEffect {}

impl MultiEffect<CombatStat> for CombatMultiEffect {
    const SIZE: usize = 3;

    fn data(idx: usize) -> CombatStat {
        match idx {
            0 => CombatStat::Acc,
            1 => CombatStat::CritAcc,
            2 => CombatStat::CritAtk,

            _ => panic!("bad stat index"),
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CharacterEffectState {
    barriers: EffectState<BarrierAnomaly>,
    stats: EffectState<Stat>,
    combat: EffectState<CombatStat>,
    // modifier: potentially todo, add bullet buffs here
}

impl CharacterEffectState {
    fn new(barrier_count: usize) -> Self {
        CharacterEffectState {
            barriers: EffectState::new(barrier_count as i64),
            stats: EffectState::new(()),
            combat: EffectState::new(()),
        }
    }

    fn get_stats(&self) -> Stats {
        let stats_data = &self.stats.effects;
        let anomallies_data = &self.barriers.effects;

        let mut res = (0..6)
            .into_iter()
            .map(|a| {
                Stats::from_stat(
                    &StatMultiEffect::data(a),
                    mult_from_lvl(NotNan::new(0.3).unwrap(), stats_data[a]),
                )
            })
            .reduce(|l, r| l.add(&r))
            .unwrap();

        let yang_mul = NotNan::new(0.85)
            .unwrap()
            .powi(anomallies_data[0 /* poison */] as i32);

        let yin_mul = NotNan::new(0.85)
            .unwrap()
            .powi(anomallies_data[1 /* burn */] as i32);

        res.yang_atk *= yang_mul;
        res.yang_def *= yang_mul;

        res.yin_atk *= yin_mul;
        res.yin_def *= yin_mul;

        res
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefaultMultiEffectState {
    states: Vec<CharacterEffectState>,
}

impl DefaultMultiEffectState {
    pub fn new(barrier_count: usize, char_count: i64) -> Self {
        let states = (0..=char_count)
            .map(|_| CharacterEffectState::new(barrier_count))
            .collect();

        DefaultMultiEffectState { states }
    }

    fn add_effect(&mut self, eff: AllEffect, lvl: i64, to: i64) {
        let to = &mut self.states[to as usize];
        match eff {
            AllEffect::Barrier(barrier) => to.barriers.apply_in_place(lvl, barrier),
            AllEffect::Combat(combatstat) => to.combat.apply_in_place(lvl, combatstat),
            AllEffect::Normal(stat) => to.stats.apply_in_place(lvl, stat),
            AllEffect::Resource(stat) => (),

        }
    }

    pub fn get_stats(&self, idx: i64) -> (Stats, Stats, [NotNan<f64>; 3]) {
        let one = NotNan::new(1.0).unwrap();
        let point_two = NotNan::new(0.2).unwrap();
        let point_three = NotNan::new(0.3).unwrap();
        let point_eighty_five = NotNan::new(0.85).unwrap();

        let char = &self.states[idx as usize];
        let enemy = &self.states[0];

        let char_stats = char.get_stats();
        let enemy_stats = enemy.get_stats();

        let combat = char
            .combat
            .effects
            .iter()
            .zip(enemy.combat.effects.iter())
            .map(|(char, enem)| (char + enem))
            .collect::<Vec<_>>();

        let acc = mult_from_lvl(point_two, combat[0])
            * point_eighty_five.powi(enemy.barriers.effects[1] as i32);

        let criacc = mult_from_lvl(point_two, combat[1]);
        let criatk = one + mult_from_lvl(point_three, combat[2]);

        (char_stats, enemy_stats, [acc, criacc, criatk])
    }
}
#[derive(Debug)]
pub struct DefaultEffectMultiWorld<T: Clone + Debug + std::ops::AddAssign<T>> {
    pub states: HashMap<DefaultMultiEffectState, (T, NotNan<f64>)>,
    default: T,
}

impl<T: Clone + Debug + std::ops::AddAssign<T>> DefaultEffectMultiWorld<T> {
    pub fn new(default: T, char_count: i64) -> Self {
        let mut states = HashMap::new();
        states.insert(
            DefaultMultiEffectState::new(6, char_count),
            (default.clone(), NotNan::new(1.0).unwrap()),
        );
        DefaultEffectMultiWorld { default, states }
    }

    fn filter_zeros(&mut self) {
        self.states
            .retain(|_, (_, chance)| chance > &mut NotNan::new(0.0).unwrap())
    }

    pub fn apply(&mut self, eff: RawAllEffect, to: i64) {
        let zero = NotNan::new(0.0).unwrap();
        {
            let eff = &eff;
            let effstat = &eff.stat;
            for (data, chance) in std::mem::replace(&mut self.states, HashMap::new())
                .into_iter()
                .map(move |(state, (data, chance))| {
                    eff.chances.iter().map(move |(efflvl, effchance)| {
                        let mut stateclone = state.clone(); //on fail

                        stateclone.add_effect(effstat.clone(), *efflvl, to);
                        ((stateclone, data.clone()), chance * effchance)
                    })
                })
                .flatten()
            {
                let r = self
                    .states
                    .entry(data.0)
                    .or_insert((self.default.clone(), zero));
                (*r).0 += data.1;
                (*r).1 += chance;
            }
        }
        self.filter_zeros()
    }
}
