#[derive(Clone)]
struct College {
    j: usize,
    f: f64,
    t: f64,
    omf: f64,
    ft: f64
}


const DUMMY_COLLEGE: College = College{j: 0, f: 1.0, t: -1.0, omf: 0.0, ft: -1.0};


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
    
    let mut xs: Vec<usize> = Vec::with_capacity(mkt.h);
    let mut vs: Vec<f64> = Vec::with_capacity(mkt.h);

    let mut v: f64 = 0.0;

    for _j in 0..mkt.h {
        if let Some(best_c) = cs.peek(){
            xs.push(best_c.j);
            v += best_c.ft;
            vs.push(v);

            cs = BinaryHeap::from_iter(
                cs.clone().drain().filter(|c| c.j != best_c.j).map(|c| 
                    if c.j <= best_c.j {
                        College{j: c.j, f: c.f, t: c.t * best_c.omf, omf: c.omf, ft: c.ft * best_c.omf}
                    } else {
                        College{j: c.j, f: c.f, t: c.t - best_c.ft, omf: c.omf, ft: c.ft - c.f * best_c.ft}
                    }
                )
            );
        }
    }

    return (xs, vs);
}


fn arg_max(cs: &Vec<College>) -> Option<usize> {
    if cs.len() == 0 {return None};

    let mut best_idx: usize = 0;

    for idx in 1..cs.len() {
        if cs[idx].ft > cs[best_idx].ft {
            best_idx = idx;
        }
    }

    return Some(best_idx);
}


pub fn application_order_vec(mkt: &SameCostsMarket) -> (Vec<usize>, Vec<f64>) {
    let mut cs: Vec<College> =
        (0..mkt.m).map(|j| College{j, f: mkt.f[j], t: mkt.t[j] as f64, omf: mkt.omf[j], ft: mkt.ft[j]}).collect();

    let mut xs: Vec<usize> = Vec::with_capacity(mkt.h);
    let mut vs: Vec<f64> = Vec::with_capacity(mkt.h);

    let mut v: f64 = 0.0;

    if let Some(mut best_idx) = arg_max(&cs) {
        let mut best_c = cs[best_idx].clone();

        for _j in 0..mkt.h {
            xs.push(best_c.j);
            v += best_c.ft;
            vs.push(v);

            let mut next_best_c:College = DUMMY_COLLEGE;
            let mut next_best_idx:usize = usize::MAX;

            for i in 0..best_idx {
                cs[i] = College {
                    j: cs[i].j,
                    f: cs[i].f,
                    t: cs[i].t * best_c.omf,
                    omf: cs[i].omf,
                    ft: cs[i].ft * best_c.omf
                };

                if next_best_c.ft <= cs[i].ft {
                    next_best_c = cs[i].clone();
                    next_best_idx = i;
                }
            }

            for i in best_idx + 1..cs.len() {
                cs[i-1] = College {
                    j: cs[i].j,
                    f: cs[i].f,
                    t: cs[i].t - best_c.ft,
                    omf: cs[i].omf,
                    ft: cs[i].ft - cs[i].f * best_c.ft
                };                
                
                if next_best_c.ft <= cs[i-1].ft {
                    next_best_c = cs[i-1].clone();
                    next_best_idx = i-1;
                }
            }

            (best_c, best_idx) = (next_best_c, next_best_idx);

            cs.pop();
        }
    }

    return (xs, vs);
}
