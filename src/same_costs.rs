#[derive(Clone)]
struct College {
    j: usize,
    f: f64,
    t: f64,
    omf: f64,
    ft: f64
}


impl PartialEq for College {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.t == other.t
    }
}


impl Eq for College {}


impl Ord for College {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ft.partial_cmp(&other.ft).unwrap().then_with(|| self.j.cmp(&other.j))
    }
}


impl PartialOrd for College {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ft.partial_cmp(&other.ft)
    }
}


pub fn application_order_heap(mkt: &SameCostsMarket) -> (Vec<usize>, Vec<f64>) {
    let mut cs: BinaryHeap<College> = 
        BinaryHeap::from_iter((0..mkt.m).map(|j| College{j, f: mkt.f[j], t: mkt.t[j] as f64, omf: mkt.omf[j], ft: mkt.ft[j]}));
    
    let mut xs: Vec<usize> = Vec::new();
    let mut vs: Vec<f64> = Vec::new();

    let mut v = 0.0;

    for _j in 0..mkt.h {
        // Identify the best school
        let best_c = cs.peek().unwrap();

        xs.push(best_c.j);
        v += best_c.ft;
        vs.push(v);

        cs = BinaryHeap::from_iter(
            cs.clone().drain().filter(|c| c.j != best_c.j).map(|c| 
                if c.t <= best_c.t {
                    College{j: c.j, f: c.f, t: c.t * best_c.omf, omf: c.omf, ft: c.ft * best_c.omf}
                } else {
                    College{j: c.j, f: c.f, t: c.t - best_c.ft, omf: c.omf, ft: c.ft - c.f * best_c.ft}
                }
            )
        );
    }

    return (xs, vs);
}


pub fn application_order_vec(mkt: &SameCostsMarket) -> (Vec<usize>, Vec<f64>) {
    let mut cs: Vec<College> =
        (0..mkt.m).map(|j| College{j, f: mkt.f[j], t: mkt.t[j] as f64, omf: mkt.omf[j], ft: mkt.ft[j]}).collect();

    let mut xs: Vec<usize> = Vec::new();
    let mut vs: Vec<f64> = Vec::new();

    let mut v = 0.0;

    for _j in 0..mkt.h {
        // Identify the best school
        let mut best_idx: usize = 0;

        for idx in 0..cs.len() {
            if cs[idx].ft > cs[best_idx].ft {
                best_idx = idx;
            }
        }
        
        let best_c = cs[best_idx].clone();
        cs.remove(best_idx);
        xs.push(best_c.j);

        v += best_c.ft;
        vs.push(v);

        for c in cs.iter_mut() {
            if c.t <= best_c.t {
                c.t *= best_c.omf;
                c.ft *= best_c.omf;
            } else {
                c.t -= best_c.ft;
                c.ft = c.f * c.t;
            }
        }
    }

    return (xs, vs);
}
