use bitmaps::Bitmap;

const BASE: usize = 128;
#[derive(Debug, PartialEq, Clone, Copy)]
struct BitmapOff {
    bm: Bitmap<BASE>,
    off: usize,
}

impl BitmapOff {
    pub fn new(off: usize) -> Self {
        BitmapOff {
            bm: Bitmap::new(),
            off,
        }
    }

    pub fn init_1(off: usize) -> Self {
        let mut bm = Bitmap::new();
        bm.invert();
        BitmapOff { bm, off }
    }

    pub fn set(&mut self, idx: usize) {
        self.bm.set(idx - self.off, true);
    }

    pub fn reset(&mut self, idx: usize) {
        self.bm.set(idx - self.off, false);
    }

    pub fn is_empty(&self) -> bool {
        self.bm.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bm.len()
    }

    pub fn pick_first(&mut self) -> u32 {
        for i in 0..BASE {
            if self.bm.get(i) == true {
                self.bm.set(i, false);
                return (i + self.off) as u32;
            }
        }
        unreachable!();
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueSet {
    set: Vec<BitmapOff>,
    size: usize,
}

impl ValueSet {
    pub fn init(size: usize) -> Self {
        let mut set = Vec::with_capacity(size / BASE);
        for i in 0..(size / BASE) {
            set.push(BitmapOff::init_1(i * BASE));
        }
        if size % BASE != 0 {
            let off = (size / BASE) * BASE;
            let mut bm = BitmapOff::init_1(off);
            for v in (size % BASE)..BASE {
                bm.reset(v + off);
            }
            set.push(bm);
        }

        ValueSet { set, size }
    }

    pub fn intersection(&self, other: &ValueSet) -> Self {
        let mut it1 = self.set.iter();
        let mut it2 = other.set.iter();
        let mut b1 = it1.next();
        let mut b2 = it2.next();
        let mut set = Vec::new();
        let mut size = 0;
        loop {
            match (b1, b2) {
                (Some(v1), Some(v2)) => {
                    if v1.off > v2.off {
                        b2 = it2.next();
                    } else if v1.off < v2.off {
                        b1 = it1.next();
                    } else {
                        b1 = it1.next();
                        b2 = it2.next();
                        let bmo = BitmapOff {
                            bm: v1.bm & v2.bm,
                            off: v1.off,
                        };
                        if !bmo.is_empty() {
                            size += bmo.len();
                            set.push(bmo);
                        }
                    }
                }
                _ => break,
            };
        }
        Self { set, size }
    }

    pub fn difference(&self, other: &ValueSet) -> Self {
        let mut it1 = self.set.iter();
        let mut it2 = other.set.iter();
        let mut b1 = it1.next();
        let mut b2 = it2.next();
        let mut set = Vec::new();
        let mut size = 0;
        loop {
            match (b1, b2) {
                (Some(v1), Some(v2)) => {
                    if v1.off > v2.off {
                        b2 = it2.next();
                    } else if v1.off < v2.off {
                        let bmo = *v1;
                        b1 = it1.next();
                        if !bmo.is_empty() {
                            size += bmo.len();
                            set.push(bmo);
                        }
                    } else {
                        b1 = it1.next();
                        b2 = it2.next();
                        let bmo = BitmapOff {
                            bm: v1.bm & (!v2.bm),
                            off: v1.off,
                        };
                        if !bmo.is_empty() {
                            size += bmo.len();
                            set.push(bmo);
                        }
                    }
                }
                (Some(v1), None) => {
                    let bmo = *v1;
                    b1 = it1.next();
                    if !bmo.is_empty() {
                        size += bmo.len();
                        set.push(bmo);
                    }
                }
                _ => break,
            };
        }
        Self { set, size }
    }

    pub fn union(&self, other: &ValueSet) -> Self {
        let mut it1 = self.set.iter();
        let mut it2 = other.set.iter();
        let mut b1 = it1.next();
        let mut b2 = it2.next();
        let mut set = Vec::new();
        let mut size = 0;
        loop {
            let bmo;
            match (b1, b2) {
                (Some(v1), Some(v2)) => {
                    if v1.off > v2.off {
                        bmo = *v2;
                        b2 = it2.next();
                    } else if v1.off < v2.off {
                        bmo = *v1;
                        b1 = it1.next();
                    } else {
                        b1 = it1.next();
                        b2 = it2.next();
                        bmo = BitmapOff {
                            bm: v1.bm | v2.bm,
                            off: v1.off,
                        };
                    }
                }
                (Some(v1), None) => {
                    bmo = *v1;
                    b1 = it1.next();
                }
                (None, Some(v2)) => {
                    bmo = *v2;
                    b2 = it2.next();
                }
                (None, None) => break,
            };
            if !bmo.is_empty() {
                size += bmo.len();
                set.push(bmo);
            }
        }
        Self { set, size }
    }

    pub fn random_pick(&mut self) -> u32 {
        let len = self.set.len();
        let choice_idx = (rand::random::<f32>() * (len as f32)) as usize;
        assert_eq!(self.set[choice_idx].is_empty(), false);
        let res = self.set[choice_idx].pick_first();
        if self.set[choice_idx].is_empty() {
            self.set.remove(choice_idx);
        }
        res
    }

    pub fn reset(&mut self, v: usize) {
        match self.set.binary_search_by_key(&(v / BASE), |&bm| bm.off) {
            Ok(idx) => {
                self.set[idx].reset(v);
                if self.set[idx].is_empty() {
                    self.set.remove(idx);
                }
            }
            Err(_) => unreachable!(format!("try reset {:} \n", v)),
        }
        self.size -= 1;
    }

    pub fn set(&mut self, v: usize) {
        match self.set.binary_search_by_key(&(v / BASE), |&bm| bm.off) {
            Ok(idx) => self.set[idx].set(v),
            Err(idx) => {
                let mut bm = BitmapOff::new((v / BASE) * BASE);
                bm.set(v);
                self.set.insert(idx, bm);
            }
        }
        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let l = ValueSet::init(129);
        let r = ValueSet::init(128);
        let mut v = ValueSet::init(0);
        v.set(128);
        assert_eq!(l.difference(&r), v);
    }

    #[test]
    fn test_reset() {
        let l = ValueSet::init(129);
        let r = ValueSet::init(128);
        let mut v = ValueSet::init(129);
        for i in 0..128 {
            v.reset(i);
        }
        assert_eq!(l.difference(&r), v);
    }

    #[test]
    fn init() {
        let size = 781u32;
        let mut v = ValueSet::init(size as usize);
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(v.random_pick());
        }
        vec.sort();
        assert_eq!(vec, (0..size).collect::<Vec<u32>>());
    }
    #[test]
    fn test_itersection() {
        let l = ValueSet::init(129);
        let r = ValueSet::init(125);
        assert_eq!(l.intersection(&r), r);
    }

    #[test]
    fn test_union() {
        let l = ValueSet::init(129);
        let r = ValueSet::init(125);
        assert_eq!(l.union(&r), l);
    }

    #[test]
    fn test_difference() {
        let l = ValueSet::init(129);
        let r = ValueSet::init(128);
        let mut v = ValueSet::init(129);
        for i in 0..128 {
            v.reset(i);
        }
        assert_eq!(l.difference(&r), v);
    }
}
