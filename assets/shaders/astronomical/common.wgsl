#define_import_path astro::common
#ifndef IMPORT_ASTRO_COMMON
#define IMPORT_ASTRO_COMMON

#import noise::prelude

fn astro_surface_noise(position: vec3<f32>, time: f32) -> f32 {
    let v = vec4(position, time);

    // fractal brownian motion algorithm
    let lacu = 1.3;
    let gain = 0.7;

    var res = 0.0;
    var ampl = 1.0 - gain; // ensure -1.0 < res < 1.0 
    var freq = 1.0;
    for (var i = 0; i < 8; i++) {
        res += ampl * noise_simplex_vec4f(freq * v);
        freq *= lacu;
        ampl *= gain;
    }

    res = 0.5 + res * 0.5;
    return 80.0 * pow(res, 3.0);
}

fn astro_kelvin_to_rgb(t: f32) -> vec3<f32> {
    var r: f32;
    var g: f32;
    var b: f32;

    // red
    if t < 6600.0 {
        r = 1.0;
    } else {
        r = pow(0.012929362 * (t - 6000.0), -0.1332048);
    }

    // green
    if t < 6600.0 {
        g = 0.3900816 * log(t) - 2.4282336;
    } else {
        g = pow(0.011298909 * (t - 6000.0), -0.07551485);
    }

    // blue 
    if t < 1900.0 {
        b = 0.0;
    } else if t < 6600.0 {
        b = 0.5432068 * log(t - 1000.0) - 3.6978139;
    } else {
        b = 1.0;
    }

    var color = vec3(r, g, b);
    color = min(vec3(1.0), color);
    color = max(vec3(0.0), color);

    return vec3(r, g, b);
}

#endif
