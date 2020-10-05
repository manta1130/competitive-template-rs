#[allow(clippy::many_single_char_names)]
pub fn fft(ar: &mut Vec<f64>, ai: &mut Vec<f64>, inv: bool) {
    assert_eq!(ar.len(), ai.len());

    let mut n = ar.len();
    //配列の要素数が2のべき乗でない場合は調整する
    while ar.len().count_ones() != 1 {
        n += 1;
        ar.push(0.0);
        ai.push(0.0);
    }

    let (mut wr, mut wi, mut xr, mut xi): (f64, f64, f64, f64);
    let w = if inv { -1.0 } else { 1.0 } * 2.0 * std::f64::consts::PI / (n as f64);

    let mut i = 0;
    for j in 1..n - 1 {
        let mut k = n >> 1;
        loop {
            i ^= k;
            if k <= i {
                break;
            }
            k >>= 1
        }

        if j < i {
            xr = ar[j];
            xi = ai[j];
            ar[j] = ar[i];
            ai[j] = ai[i];
            ar[i] = xr;
            ai[i] = xi;
        }
    }

    let mut mh = 1;
    loop {
        let m = mh << 1;
        if m > n {
            break;
        }
        let mut irev = 0;

        i = 0;
        while i < n {
            wr = (w * irev as f64).cos();
            wi = (w * irev as f64).sin();
            let mut k = n >> 2;
            loop {
                irev ^= k;
                if k <= irev {
                    break;
                }
                k >>= 1;
            }

            for j in i..mh + i {
                k = j + mh;
                xr = ar[j] - ar[k];
                xi = ai[j] - ai[k];
                ar[j] += ar[k];
                ai[j] += ai[k];
                ar[k] = wr * xr - wi * xi;
                ai[k] = wr * xi + wi * xr;
            }

            i += m;
        }
        mh = m;
    }
    if inv {
        for i in ar.iter_mut() {
            *i /= n as f64;
        }
        for i in ai.iter_mut() {
            *i /= n as f64;
        }
    }
}
