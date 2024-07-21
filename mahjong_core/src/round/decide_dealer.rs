use crate::{Wind, WINDS_ROUND_ORDER};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

// https://stackoverflow.com/a/24257996
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecideDealerWinds(Option<[Wind; 4]>);

#[derive(Debug, EnumIter, Eq, PartialEq, Clone)]
pub enum SetInitialWindsError {
    Duplicate,
}

const FACTORIALS: [usize; 4] = [6, 2, 1, 1];

// Uses lehmer codes to map from permutation to number
impl DecideDealerWinds {
    pub fn new(list: Option<[Wind; 4]>) -> Result<Self, SetInitialWindsError> {
        if list.is_none() {
            return Ok(Self(None));
        }

        let mut picked_winds: FxHashSet<Wind> = FxHashSet::default();
        let winds_list = list.unwrap();
        for wind in winds_list.iter() {
            if picked_winds.contains(wind) {
                return Err(SetInitialWindsError::Duplicate);
            }
            picked_winds.insert(wind.clone());
        }

        Ok(Self(Some(winds_list)))
    }

    pub fn from_number(n: Option<u8>) -> Self {
        if n.is_none() {
            return Self(None);
        }

        let mut lehmer_codes = Vec::new();
        let mut n = n.unwrap();

        for factorial in FACTORIALS {
            let remainder = n % (factorial as u8);
            let base_number = n - remainder;
            let lehmer_code = (base_number / (factorial as u8)) as usize;
            lehmer_codes.push(lehmer_code);
            n = remainder;
        }

        let mut winds = WINDS_ROUND_ORDER.clone().to_vec();
        let mut winds_list = Vec::new();
        for lehmer_code in lehmer_codes {
            let wind = winds.remove(lehmer_code);
            winds_list.push(wind);
        }

        Self(Some(winds_list.try_into().unwrap()))
    }

    pub fn to_number(&self) -> Option<u8> {
        self.0.as_ref()?;

        let mut lehmer_codes = Vec::new();
        let mut winds = self.0.as_ref().unwrap().to_vec();
        let mut ordered_winds = WINDS_ROUND_ORDER.clone().to_vec();

        for _ in 0..4 {
            let wind = winds.remove(0);
            let idx = ordered_winds.iter().position(|w| w == &wind).unwrap();
            ordered_winds.remove(idx);
            lehmer_codes.push(idx);
        }

        let mut n = 0;
        for i in 0..4 {
            n += lehmer_codes[i] * FACTORIALS[i];
        }
        Some(n as u8)
    }

    pub fn iter(&self, func: impl FnMut((usize, &Wind))) {
        self.0.as_ref().unwrap().iter().enumerate().for_each(func);
    }
}
