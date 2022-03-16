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
    if v_recursion(usize::MAX, h, &mut v_jh_dict, &mkt) < v_recursion(1, h, &mut v_jh_dict, &mkt) {
        xs.push(1);
    }

    return (xs, v);
}