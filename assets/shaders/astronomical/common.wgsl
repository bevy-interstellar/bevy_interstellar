#define_import_path astro::common
#ifndef IMPORT_ASTRO_COMMON
#define IMPORT_ASTRO_COMMON

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
        g = 0.3900816 * log(t / 100.0) - 0.6318414;
    } else {
        g = pow(0.011298909 * (t - 6000.0), -0.07551485);
    }

    // blue 
    if t < 1900.0 {
        b = 0.0;
    } else if t < 6600.0 {
        b = -0.5432068 * log(t / 100.0 - 10.0) - 1.1962541;
    } else {
        b = 1.0;
    }

    var color = vec3(r, g, b);
    color = min(vec3(1.0), color);
    color = max(vec3(0.0), color);

    return vec3(r, g, b);
}

#endif
