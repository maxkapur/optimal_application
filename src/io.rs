// v.is_sorted() is an unstable feature so I rolled my own
fn is_sorted(t: &[usize]) -> bool {
    for i in 1..t.len() - 1 {
        if t[i] > t[i+1] {return false;}
    }

    return true;
}


#[derive(Debug)]
pub struct SameCostsMarket {
    f:      Vec<f64>,
    t:      Vec<usize>,
    omf:    Vec<f64>,
    ft:     Vec<f64>,
    h:      usize,
    m:      usize,
}


impl SameCostsMarket {
    pub fn new(f: &[f64], t: &[usize], h: usize) -> Self {
        let m: usize = f.len();
        assert_eq!(m, t.len());
        assert!(0 < h);
        assert!(h <= m);
        assert!(is_sorted(t));
        for &i in f.iter() {assert!(0.0 < i && i <= 1.0);}

        let omf: Vec<f64> = f.iter().map(|&f| 1.0 - f).collect();
        let ft: Vec<f64> = f.iter().zip(t.iter()).map(|(&f, &t)| f * t as f64).collect();

        return SameCostsMarket {
            f: f.to_vec(),
            t: t.to_vec(),
            omf,
            ft,
            h,
            m
        };
    }
}




#[derive(Debug)]
pub struct VariedCostsMarket {
    f:      Vec<f64>,
    t:      Vec<usize>,
    g:      Vec<usize>,
    omf:    Vec<f64>,
    ft:     Vec<f64>,
    h_big:  usize,
    m:      usize,
}


impl VariedCostsMarket {
    pub fn new(f: &[f64], t: &[usize], g: &[usize], h_big: usize) -> Self {
        let m: usize = f.len();
        assert_eq!(m, t.len());
        assert_eq!(m, g.len());
        assert!(0 < h_big);
        assert!(h_big <= g.iter().fold(0, |acc, &gi| acc + gi));
        assert!(is_sorted(t));
        for &i in f.iter() {assert!(0.0 < i && i <= 1.0);}

        let omf: Vec<f64> = f.iter().map(|&f| 1.0 - f).collect();
        let ft: Vec<f64> = f.iter().zip(t.iter()).map(|(&f, &t)| f * t as f64).collect();

        return VariedCostsMarket {
            f: f.to_vec(),
            t: t.to_vec(),
            g: g.to_vec(),
            omf,
            ft,
            h_big,
            m
        };
    }
}
