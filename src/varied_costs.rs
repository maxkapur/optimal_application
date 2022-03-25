use num::clamp;


fn v_recursion(j: usize, h: usize, v_jh_dict: &mut HashMap<(usize, usize),f64>, mkt: &VariedCostsMarket) -> f64 {
    // I.e. if j == -1
    if j == usize::MAX || h == 0 {
        return 0.0;
    }

    let jmo = if j == 0 {usize::MAX} else {j - 1};

    if let Some(v) = v_jh_dict.get(&(j, h)) {
        return *v;
    } else if h < mkt.g[j] {
        let v_jmo = v_recursion(jmo, h, v_jh_dict, &mkt);
        v_jh_dict.insert((j, h), v_jmo);
        return 0.0;
    } else {
        let v_max = {
            let v0 = v_recursion(jmo, h, v_jh_dict, &mkt);
            let v1 = mkt.omf[j] * v_recursion(jmo, h - mkt.g[j], v_jh_dict, &mkt) + mkt.ft[j];
            if v0 > v1 {v0} else {v1}
        };

        v_jh_dict.insert((j, h), v_max);
        return v_max;
    }
}


pub fn optimal_portfolio_dynamic(mkt: &VariedCostsMarket) -> (Vec<usize>, f64) {
    let mut v_jh_dict = HashMap::<(usize, usize),f64>::new();

    let mut h = mkt.h_big;
    let mut xs = Vec::new();

    let v = v_recursion(mkt.m - 1, mkt.h_big, &mut v_jh_dict, &mkt);

    for j in (1..mkt.m).rev() {
        if v_recursion(j-1, h, &mut v_jh_dict, &mkt) < v_recursion(j, h, &mut v_jh_dict, &mkt) {
            xs.push(j);
            h -= mkt.g[j];
        }
    }

    // j = 0 case
    if v_recursion(usize::MAX, h, &mut v_jh_dict, &mkt) < v_recursion(0, h, &mut v_jh_dict, &mkt) {
        xs.push(0);
    }

    return (xs, v);
}


#[derive(Debug)]
pub struct ScaleParams{
    prec:   u16,
    t:      Vec<usize>,
    ft:     Vec<f64>,
    u_bar:  usize,
    infty:  usize,
}


impl ScaleParams {
    pub fn new(mkt: &VariedCostsMarket, eps: f64) -> Self {
        assert!(0.0 < eps && eps < 1.0);
        let prec: u16 = 1 + ((mkt.m * mkt.m) as f64 / (eps * mkt.ft.iter().fold(0.0, |acc, &x| acc + x))).log2() as u16;
        let t: Vec<usize> = Vec::from_iter(mkt.t.iter().map(|&t| t * 2usize.pow(prec as u32)));
        let ft: Vec<f64> = Vec::from_iter(t.iter().zip(mkt.f.iter()).map(|(&t, &f)| f * (t as f64)));
        let u_bar = 1 + ft.iter().fold(0.0, |acc, &x| acc + x) as usize;
        let infty = 1 + mkt.g.iter().fold(0, |acc, &x| acc + x);

        return ScaleParams{prec, t, ft, u_bar, infty};
    }
}


fn g_recursion(j: usize, v: isize, g_jv_dict: &mut HashMap<(usize,isize), usize>, mkt: &VariedCostsMarket, sp: &ScaleParams) -> usize {
    if let Some(w) = g_jv_dict.get(&(j, v)) {return *w;}
    
    if v <= 0 {
        return 0;
    } else if j == usize::MAX || (sp.t[j] as isize) < v || v >= (sp.u_bar as isize) {
        return sp.infty;
    } else {
        let jmo = if j == 0 {usize::MAX} else {j - 1};

        if mkt.f[j] < 1.0 {
            let w_min = {
                let v_minus_delta = clamp(
                    (v as f64 - sp.ft[j]) / mkt.omf[j],
                    -1.0,
                    sp.u_bar as f64
                ).floor() as isize;

                let w0 = g_recursion(jmo, v, g_jv_dict, &mkt, &sp);
                let w1 = mkt.g[j] + g_recursion(jmo, v_minus_delta, g_jv_dict, &mkt, &sp);
                if w0 < w1 {w0} else {w1}
            };

            g_jv_dict.insert((j, v), w_min);
            return w_min;
        } else {
            let w_min = {
                let w0 = g_recursion(jmo, v, g_jv_dict, &mkt, &sp);
                if w0 < mkt.g[j] {w0} else {mkt.g[j]}
            };

            g_jv_dict.insert((j, v), w_min);
            return w_min;
        }
    }
}


pub fn optimal_portfolio_fptas(mkt: &VariedCostsMarket, eps: f64) -> (Vec<usize>, f64) {
    let sp = ScaleParams::new(&mkt, eps);

    let mut g_jv_dict = HashMap::<(usize, isize), usize>::new();

    // Binary search
    let mut v: isize = 0;
    let mut v_ub = sp.u_bar as isize;
    let mut mid: isize;
    let moo = mkt.m - 1;

    while v + 1 < v_ub {
        mid = (v + v_ub) / 2;

        println!("mid = {}", mid);
        println!("g = {}", g_recursion(moo, mid, &mut g_jv_dict, &mkt, &sp));

        if g_recursion(moo, mid, &mut g_jv_dict, &mkt, &sp) > mkt.h_big {
            v_ub = mid;
        } else {
            v = mid;
            if g_recursion(moo, v + 1, &mut g_jv_dict, &mkt, &sp) > mkt.h_big {
                // Found it
                println!("v = {}", v as f64 / (2.0_f64).powf(sp.prec as f64));
                println!("g = {}", g_recursion(moo, v, &mut g_jv_dict, &mkt, &sp));
                break;
            }
        }
    }

    let mut xs: Vec<usize> = Vec::new();


    let mut v_mut = v;
    for j in (1..mkt.m).rev() {
        if g_recursion(j, v_mut, &mut g_jv_dict, &mkt, &sp) < g_recursion(j - 1, v_mut, &mut g_jv_dict, &mkt, &sp) {
            xs.push(j);
            v_mut =
                clamp(
                    (v as f64 - sp.ft[j]) / mkt.omf[j],
                    -1.0,
                    sp.u_bar as f64
                ).floor() as isize;
        }
    }

    // j = 0 case
    if g_recursion(0, v_mut, &mut g_jv_dict, &mkt, &sp) < g_recursion(usize::MAX, v_mut, &mut g_jv_dict, &mkt, &sp) {
        xs.push(0);
    }

    println!("sp = {:?}", sp);

    // returned value is an estimate
    return (xs, v as f64 / (2.0_f64).powf(sp.prec as f64));
}
