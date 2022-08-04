#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]


use core::ffi::*;

pub const C_AUDAY: f64 = 173.1446326846693;
pub const DEG2RAD: f64 = 0.017453292519943295;
pub const HOUR2RAD: f64 = 0.26179938779914946;
pub const RAD2DEG: f64 = 57.29577951308232;
pub const RAD2HOUR: f64 = 3.819718634205488;
pub const SUN_RADIUS_KM: f64 = 695700.0;
pub const MERCURY_EQUATORIAL_RADIUS_KM: f64 = 2440.5;
pub const MERCURY_POLAR_RADIUS_KM: f64 = 2438.3;
pub const VENUS_RADIUS_KM: f64 = 6051.8;
pub const EARTH_EQUATORIAL_RADIUS_KM: f64 = 6378.1366;
pub const EARTH_FLATTENING: f64 = 0.996647180302104;
pub const EARTH_POLAR_RADIUS_KM: f64 = 6356.751857971648;
pub const MOON_EQUATORIAL_RADIUS_KM: f64 = 1738.1;
pub const MOON_POLAR_RADIUS_KM: f64 = 1736.0;
pub const MARS_EQUATORIAL_RADIUS_KM: f64 = 3396.2;
pub const MARS_POLAR_RADIUS_KM: f64 = 3376.2;
pub const JUPITER_EQUATORIAL_RADIUS_KM: f64 = 71492.0;
pub const JUPITER_POLAR_RADIUS_KM: f64 = 66854.0;
pub const JUPITER_MEAN_RADIUS_KM: f64 = 69911.0;
pub const IO_RADIUS_KM: f64 = 1821.6;
pub const EUROPA_RADIUS_KM: f64 = 1560.8;
pub const GANYMEDE_RADIUS_KM: f64 = 2631.2;
pub const CALLISTO_RADIUS_KM: f64 = 2410.3;
pub const SATURN_EQUATORIAL_RADIUS_KM: f64 = 60268.0;
pub const SATURN_POLAR_RADIUS_KM: f64 = 54364.0;
pub const URANUS_EQUATORIAL_RADIUS_KM: f64 = 25559.0;
pub const URANUS_POLAR_RADIUS_KM: f64 = 24973.0;
pub const NEPTUNE_EQUATORIAL_RADIUS_KM: f64 = 24764.0;
pub const NEPTUNE_POLAR_RADIUS_KM: f64 = 24341.0;
pub const PLUTO_RADIUS_KM: f64 = 1188.3;
pub const TIME_TEXT_BYTES: u32 = 25;
pub type size_t = c_longlong;
pub type wchar_t = c_ushort;
pub type max_align_t = f64;
pub const astro_status_t_ASTRO_SUCCESS: astro_status_t = 0;
pub const astro_status_t_ASTRO_NOT_INITIALIZED: astro_status_t = 1;
pub const astro_status_t_ASTRO_INVALID_BODY: astro_status_t = 2;
pub const astro_status_t_ASTRO_NO_CONVERGE: astro_status_t = 3;
pub const astro_status_t_ASTRO_BAD_TIME: astro_status_t = 4;
pub const astro_status_t_ASTRO_BAD_VECTOR: astro_status_t = 5;
pub const astro_status_t_ASTRO_SEARCH_FAILURE: astro_status_t = 6;
pub const astro_status_t_ASTRO_EARTH_NOT_ALLOWED: astro_status_t = 7;
pub const astro_status_t_ASTRO_NO_MOON_QUARTER: astro_status_t = 8;
pub const astro_status_t_ASTRO_WRONG_MOON_QUARTER: astro_status_t = 9;
pub const astro_status_t_ASTRO_INTERNAL_ERROR: astro_status_t = 10;
pub const astro_status_t_ASTRO_INVALID_PARAMETER: astro_status_t = 11;
pub const astro_status_t_ASTRO_FAIL_APSIS: astro_status_t = 12;
pub const astro_status_t_ASTRO_BUFFER_TOO_SMALL: astro_status_t = 13;
pub const astro_status_t_ASTRO_OUT_OF_MEMORY: astro_status_t = 14;
pub const astro_status_t_ASTRO_INCONSISTENT_TIMES: astro_status_t = 15;
pub type astro_status_t = c_int;

#[repr(C)]
pub struct astro_time_t {
    pub ut: f64,
    pub tt: f64,
    pub psi: f64,
    pub eps: f64,
    pub st: f64,
}

#[repr(C)]
pub struct astro_utc_t {
    pub year: c_int,
    pub month: c_int,
    pub day: c_int,
    pub hour: c_int,
    pub minute: c_int,
    pub second: f64,
}

#[repr(C)]
pub struct astro_vector_t {
    pub status: astro_status_t,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: astro_time_t,
}

#[repr(C)]
pub struct astro_state_vector_t {
    pub status: astro_status_t,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub t: astro_time_t,
}

#[repr(C)]
pub struct astro_spherical_t {
    pub status: astro_status_t,
    pub lat: f64,
    pub lon: f64,
    pub dist: f64,
}

#[repr(C)]
pub struct astro_angle_result_t {
    pub status: astro_status_t,
    pub angle: f64,
}

pub const astro_body_t_BODY_INVALID: astro_body_t = -1;
pub const astro_body_t_BODY_MERCURY: astro_body_t = 0;
pub const astro_body_t_BODY_VENUS: astro_body_t = 1;
pub const astro_body_t_BODY_EARTH: astro_body_t = 2;
pub const astro_body_t_BODY_MARS: astro_body_t = 3;
pub const astro_body_t_BODY_JUPITER: astro_body_t = 4;
pub const astro_body_t_BODY_SATURN: astro_body_t = 5;
pub const astro_body_t_BODY_URANUS: astro_body_t = 6;
pub const astro_body_t_BODY_NEPTUNE: astro_body_t = 7;
pub const astro_body_t_BODY_PLUTO: astro_body_t = 8;
pub const astro_body_t_BODY_SUN: astro_body_t = 9;
pub const astro_body_t_BODY_MOON: astro_body_t = 10;
pub const astro_body_t_BODY_EMB: astro_body_t = 11;
pub const astro_body_t_BODY_SSB: astro_body_t = 12;
pub type astro_body_t = c_int;

#[repr(C)]
pub struct astro_observer_t {
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
}

#[repr(C)]
pub struct astro_equatorial_t {
    pub status: astro_status_t,
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
    pub vec: astro_vector_t,
}

#[repr(C)]
pub struct astro_ecliptic_t {
    pub status: astro_status_t,
    pub vec: astro_vector_t,
    pub elat: f64,
    pub elon: f64,
}

#[repr(C)]
pub struct astro_horizon_t {
    pub azimuth: f64,
    pub altitude: f64,
    pub ra: f64,
    pub dec: f64,
}

#[repr(C)]
pub struct astro_rotation_t {
    pub status: astro_status_t,
    pub rot: [[f64; 3usize]; 3usize],
}

pub const astro_refraction_t_REFRACTION_NONE: astro_refraction_t = 0;
pub const astro_refraction_t_REFRACTION_NORMAL: astro_refraction_t = 1;
pub const astro_refraction_t_REFRACTION_JPLHOR: astro_refraction_t = 2;
pub type astro_refraction_t = c_int;

#[repr(C)]
pub struct astro_search_result_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
}

#[repr(C)]
pub struct astro_seasons_t {
    pub status: astro_status_t,
    pub mar_equinox: astro_time_t,
    pub jun_solstice: astro_time_t,
    pub sep_equinox: astro_time_t,
    pub dec_solstice: astro_time_t,
}

#[repr(C)]
pub struct astro_moon_quarter_t {
    pub status: astro_status_t,
    pub quarter: c_int,
    pub time: astro_time_t,
}

#[repr(C)]
pub struct astro_func_result_t {
    pub status: astro_status_t,
    pub value: f64,
}

pub type astro_search_func_t = core::option::Option<
    unsafe extern "C" fn(context: *mut c_void, time: astro_time_t) -> astro_func_result_t,
>;

pub type astro_deltat_func = core::option::Option<unsafe extern "C" fn(ut: f64) -> f64>;
extern "C" {
    pub fn Astronomy_DeltaT_EspenakMeeus(ut: f64) -> f64;
    pub fn Astronomy_DeltaT_JplHorizons(ut: f64) -> f64;
    pub fn Astronomy_SetDeltaTFunction(func: astro_deltat_func);
}

pub const astro_visibility_t_VISIBLE_MORNING: astro_visibility_t = 0;
pub const astro_visibility_t_VISIBLE_EVENING: astro_visibility_t = 1;
pub type astro_visibility_t = c_int;

#[repr(C)]
pub struct astro_elongation_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub visibility: astro_visibility_t,
    pub elongation: f64,
    pub ecliptic_separation: f64,
}

#[repr(C)]
pub struct astro_hour_angle_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub hor: astro_horizon_t,
}

#[repr(C)]
pub struct astro_illum_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub mag: f64,
    pub phase_angle: f64,
    pub phase_fraction: f64,
    pub helio_dist: f64,
    pub ring_tilt: f64,
}

pub const astro_apsis_kind_t_APSIS_PERICENTER: astro_apsis_kind_t = 0;
pub const astro_apsis_kind_t_APSIS_APOCENTER: astro_apsis_kind_t = 1;
pub const astro_apsis_kind_t_APSIS_INVALID: astro_apsis_kind_t = 2;
pub type astro_apsis_kind_t = c_int;

#[repr(C)]
pub struct astro_apsis_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub kind: astro_apsis_kind_t,
    pub dist_au: f64,
    pub dist_km: f64,
}

pub const astro_eclipse_kind_t_ECLIPSE_NONE: astro_eclipse_kind_t = 0;
pub const astro_eclipse_kind_t_ECLIPSE_PENUMBRAL: astro_eclipse_kind_t = 1;
pub const astro_eclipse_kind_t_ECLIPSE_PARTIAL: astro_eclipse_kind_t = 2;
pub const astro_eclipse_kind_t_ECLIPSE_ANNULAR: astro_eclipse_kind_t = 3;
pub const astro_eclipse_kind_t_ECLIPSE_TOTAL: astro_eclipse_kind_t = 4;
pub type astro_eclipse_kind_t = c_int;

#[repr(C)]
pub struct astro_lunar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub peak: astro_time_t,
    pub sd_penum: f64,
    pub sd_partial: f64,
    pub sd_total: f64,
}

#[repr(C)]
pub struct astro_global_solar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub peak: astro_time_t,
    pub distance: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[repr(C)]
pub struct astro_eclipse_event_t {
    pub time: astro_time_t,
    pub altitude: f64,
}

#[repr(C)]
pub struct astro_local_solar_eclipse_t {
    pub status: astro_status_t,
    pub kind: astro_eclipse_kind_t,
    pub partial_begin: astro_eclipse_event_t,
    pub total_begin: astro_eclipse_event_t,
    pub peak: astro_eclipse_event_t,
    pub total_end: astro_eclipse_event_t,
    pub partial_end: astro_eclipse_event_t,
}

#[repr(C)]
pub struct astro_transit_t {
    pub status: astro_status_t,
    pub start: astro_time_t,
    pub peak: astro_time_t,
    pub finish: astro_time_t,
    pub separation: f64,
}

pub const astro_aberration_t_ABERRATION: astro_aberration_t = 0;
pub const astro_aberration_t_NO_ABERRATION: astro_aberration_t = 1;
pub type astro_aberration_t = c_int;
pub const astro_equator_date_t_EQUATOR_J2000: astro_equator_date_t = 0;
pub const astro_equator_date_t_EQUATOR_OF_DATE: astro_equator_date_t = 1;
pub type astro_equator_date_t = c_int;
pub const astro_direction_t_DIRECTION_RISE: astro_direction_t = 1;
pub const astro_direction_t_DIRECTION_SET: astro_direction_t = -1;
pub type astro_direction_t = c_int;

#[repr(C)]
pub struct astro_constellation_t {
    pub status: astro_status_t,
    pub symbol: *const c_char,
    pub name: *const c_char,
    pub ra_1875: f64,
    pub dec_1875: f64,
}

pub const astro_time_format_t_TIME_FORMAT_DAY: astro_time_format_t = 0;
pub const astro_time_format_t_TIME_FORMAT_MINUTE: astro_time_format_t = 1;
pub const astro_time_format_t_TIME_FORMAT_SECOND: astro_time_format_t = 2;
pub const astro_time_format_t_TIME_FORMAT_MILLI: astro_time_format_t = 3;
pub type astro_time_format_t = c_int;

#[repr(C)]
pub struct astro_libration_t {
    pub elat: f64,
    pub elon: f64,
    pub mlat: f64,
    pub mlon: f64,
    pub dist_km: f64,
    pub diam_deg: f64,
}

#[repr(C)]
pub struct astro_axis_t {
    pub status: astro_status_t,
    pub ra: f64,
    pub dec: f64,
    pub spin: f64,
    pub north: astro_vector_t,
}

#[repr(C)]
pub struct astro_jupiter_moons_t {
    pub io: astro_state_vector_t,
    pub europa: astro_state_vector_t,
    pub ganymede: astro_state_vector_t,
    pub callisto: astro_state_vector_t,
}

pub const astro_node_kind_t_INVALID_NODE: astro_node_kind_t = 0;
pub const astro_node_kind_t_ASCENDING_NODE: astro_node_kind_t = 1;
pub const astro_node_kind_t_DESCENDING_NODE: astro_node_kind_t = -1;
pub type astro_node_kind_t = c_int;

#[repr(C)]
pub struct astro_node_event_t {
    pub status: astro_status_t,
    pub time: astro_time_t,
    pub kind: astro_node_kind_t,
}

#[repr(C)]
pub struct astro_grav_sim_s {
    _unused: [u8; 0],
}

pub type astro_grav_sim_t = astro_grav_sim_s;
extern "C" {
    pub fn Astronomy_Reset();
    pub fn Astronomy_VectorLength(vector: astro_vector_t) -> f64;
    pub fn Astronomy_AngleBetween(a: astro_vector_t, b: astro_vector_t) -> astro_angle_result_t;
    pub fn Astronomy_BodyName(body: astro_body_t) -> *const c_char;
    pub fn Astronomy_BodyCode(name: *const c_char) -> astro_body_t;
    pub fn Astronomy_MakeObserver(latitude: f64, longitude: f64, height: f64) -> astro_observer_t;
    pub fn Astronomy_CurrentTime() -> astro_time_t;
    pub fn Astronomy_MakeTime(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_int,
        minute: c_int,
        second: f64,
    ) -> astro_time_t;
    pub fn Astronomy_TimeFromUtc(utc: astro_utc_t) -> astro_time_t;
    pub fn Astronomy_UtcFromTime(time: astro_time_t) -> astro_utc_t;
    pub fn Astronomy_FormatTime(
        time: astro_time_t,
        format: astro_time_format_t,
        text: *mut c_char,
        size: size_t,
    ) -> astro_status_t;
    pub fn Astronomy_TimeFromDays(ut: f64) -> astro_time_t;
    pub fn Astronomy_TerrestrialTime(tt: f64) -> astro_time_t;
    pub fn Astronomy_AddDays(time: astro_time_t, days: f64) -> astro_time_t;
    pub fn Astronomy_SiderealTime(time: *mut astro_time_t) -> f64;
    pub fn Astronomy_HelioDistance(body: astro_body_t, time: astro_time_t) -> astro_func_result_t;
    pub fn Astronomy_HelioVector(body: astro_body_t, time: astro_time_t) -> astro_vector_t;
    pub fn Astronomy_GeoVector(
        body: astro_body_t,
        time: astro_time_t,
        aberration: astro_aberration_t,
    ) -> astro_vector_t;
    pub fn Astronomy_GeoMoon(time: astro_time_t) -> astro_vector_t;
    pub fn Astronomy_EclipticGeoMoon(time: astro_time_t) -> astro_spherical_t;
    pub fn Astronomy_GeoMoonState(time: astro_time_t) -> astro_state_vector_t;
    pub fn Astronomy_GeoEmbState(time: astro_time_t) -> astro_state_vector_t;
    pub fn Astronomy_Libration(time: astro_time_t) -> astro_libration_t;
    pub fn Astronomy_BaryState(body: astro_body_t, time: astro_time_t) -> astro_state_vector_t;
    pub fn Astronomy_HelioState(body: astro_body_t, time: astro_time_t) -> astro_state_vector_t;
    pub fn Astronomy_MassProduct(body: astro_body_t) -> f64;
    pub fn Astronomy_LagrangePoint(
        point: c_int,
        time: astro_time_t,
        major_body: astro_body_t,
        minor_body: astro_body_t,
    ) -> astro_state_vector_t;
    pub fn Astronomy_LagrangePointFast(
        point: c_int,
        major_state: astro_state_vector_t,
        major_mass: f64,
        minor_state: astro_state_vector_t,
        minor_mass: f64,
    ) -> astro_state_vector_t;
    pub fn Astronomy_JupiterMoons(time: astro_time_t) -> astro_jupiter_moons_t;
    pub fn Astronomy_Equator(
        body: astro_body_t,
        time: *mut astro_time_t,
        observer: astro_observer_t,
        equdate: astro_equator_date_t,
        aberration: astro_aberration_t,
    ) -> astro_equatorial_t;
    pub fn Astronomy_ObserverVector(
        time: *mut astro_time_t,
        observer: astro_observer_t,
        equdate: astro_equator_date_t,
    ) -> astro_vector_t;
    pub fn Astronomy_ObserverState(
        time: *mut astro_time_t,
        observer: astro_observer_t,
        equdate: astro_equator_date_t,
    ) -> astro_state_vector_t;
    pub fn Astronomy_VectorObserver(
        vector: *mut astro_vector_t,
        equdate: astro_equator_date_t,
    ) -> astro_observer_t;
    pub fn Astronomy_ObserverGravity(latitude: f64, height: f64) -> f64;
    pub fn Astronomy_SunPosition(time: astro_time_t) -> astro_ecliptic_t;
    pub fn Astronomy_Ecliptic(equ: astro_vector_t) -> astro_ecliptic_t;
    pub fn Astronomy_EclipticLongitude(
        body: astro_body_t,
        time: astro_time_t,
    ) -> astro_angle_result_t;
    pub fn Astronomy_Horizon(
        time: *mut astro_time_t,
        observer: astro_observer_t,
        ra: f64,
        dec: f64,
        refraction: astro_refraction_t,
    ) -> astro_horizon_t;
    pub fn Astronomy_AngleFromSun(body: astro_body_t, time: astro_time_t) -> astro_angle_result_t;
    pub fn Astronomy_Elongation(body: astro_body_t, time: astro_time_t) -> astro_elongation_t;
    pub fn Astronomy_SearchMaxElongation(
        body: astro_body_t,
        startTime: astro_time_t,
    ) -> astro_elongation_t;
    pub fn Astronomy_PairLongitude(
        body1: astro_body_t,
        body2: astro_body_t,
        time: astro_time_t,
    ) -> astro_angle_result_t;
    pub fn Astronomy_SearchRelativeLongitude(
        body: astro_body_t,
        targetRelLon: f64,
        startTime: astro_time_t,
    ) -> astro_search_result_t;
    pub fn Astronomy_MoonPhase(time: astro_time_t) -> astro_angle_result_t;
    pub fn Astronomy_SearchMoonPhase(
        targetLon: f64,
        startTime: astro_time_t,
        limitDays: f64,
    ) -> astro_search_result_t;
    pub fn Astronomy_SearchMoonQuarter(startTime: astro_time_t) -> astro_moon_quarter_t;
    pub fn Astronomy_NextMoonQuarter(mq: astro_moon_quarter_t) -> astro_moon_quarter_t;
    pub fn Astronomy_SearchLunarEclipse(startTime: astro_time_t) -> astro_lunar_eclipse_t;
    pub fn Astronomy_NextLunarEclipse(prevEclipseTime: astro_time_t) -> astro_lunar_eclipse_t;
    pub fn Astronomy_SearchGlobalSolarEclipse(
        startTime: astro_time_t,
    ) -> astro_global_solar_eclipse_t;
    pub fn Astronomy_NextGlobalSolarEclipse(
        prevEclipseTime: astro_time_t,
    ) -> astro_global_solar_eclipse_t;
    pub fn Astronomy_SearchLocalSolarEclipse(
        startTime: astro_time_t,
        observer: astro_observer_t,
    ) -> astro_local_solar_eclipse_t;
    pub fn Astronomy_NextLocalSolarEclipse(
        prevEclipseTime: astro_time_t,
        observer: astro_observer_t,
    ) -> astro_local_solar_eclipse_t;
    pub fn Astronomy_SearchTransit(body: astro_body_t, startTime: astro_time_t) -> astro_transit_t;
    pub fn Astronomy_NextTransit(
        body: astro_body_t,
        prevTransitTime: astro_time_t,
    ) -> astro_transit_t;
    pub fn Astronomy_SearchMoonNode(startTime: astro_time_t) -> astro_node_event_t;
    pub fn Astronomy_NextMoonNode(prevNode: astro_node_event_t) -> astro_node_event_t;
    pub fn Astronomy_Search(
        func: astro_search_func_t,
        context: *mut c_void,
        t1: astro_time_t,
        t2: astro_time_t,
        dt_tolerance_seconds: f64,
    ) -> astro_search_result_t;
    pub fn Astronomy_SearchSunLongitude(
        targetLon: f64,
        startTime: astro_time_t,
        limitDays: f64,
    ) -> astro_search_result_t;
    pub fn Astronomy_SearchHourAngle(
        body: astro_body_t,
        observer: astro_observer_t,
        hourAngle: f64,
        startTime: astro_time_t,
    ) -> astro_hour_angle_t;
    pub fn Astronomy_SearchRiseSet(
        body: astro_body_t,
        observer: astro_observer_t,
        direction: astro_direction_t,
        startTime: astro_time_t,
        limitDays: f64,
    ) -> astro_search_result_t;
    pub fn Astronomy_SearchAltitude(
        body: astro_body_t,
        observer: astro_observer_t,
        direction: astro_direction_t,
        startTime: astro_time_t,
        limitDays: f64,
        altitude: f64,
    ) -> astro_search_result_t;
    pub fn Astronomy_RotationAxis(body: astro_body_t, time: *mut astro_time_t) -> astro_axis_t;
    pub fn Astronomy_Seasons(year: c_int) -> astro_seasons_t;
    pub fn Astronomy_Illumination(body: astro_body_t, time: astro_time_t) -> astro_illum_t;
    pub fn Astronomy_SearchPeakMagnitude(
        body: astro_body_t,
        startTime: astro_time_t,
    ) -> astro_illum_t;
    pub fn Astronomy_SearchLunarApsis(startTime: astro_time_t) -> astro_apsis_t;
    pub fn Astronomy_NextLunarApsis(apsis: astro_apsis_t) -> astro_apsis_t;
    pub fn Astronomy_SearchPlanetApsis(
        body: astro_body_t,
        startTime: astro_time_t,
    ) -> astro_apsis_t;
    pub fn Astronomy_NextPlanetApsis(body: astro_body_t, apsis: astro_apsis_t) -> astro_apsis_t;
    pub fn Astronomy_IdentityMatrix() -> astro_rotation_t;
    pub fn Astronomy_InverseRotation(rotation: astro_rotation_t) -> astro_rotation_t;
    pub fn Astronomy_CombineRotation(a: astro_rotation_t, b: astro_rotation_t) -> astro_rotation_t;
    pub fn Astronomy_Pivot(rotation: astro_rotation_t, axis: c_int, angle: f64)
        -> astro_rotation_t;
    pub fn Astronomy_VectorFromSphere(
        sphere: astro_spherical_t,
        time: astro_time_t,
    ) -> astro_vector_t;
    pub fn Astronomy_SphereFromVector(vector: astro_vector_t) -> astro_spherical_t;
    pub fn Astronomy_EquatorFromVector(vector: astro_vector_t) -> astro_equatorial_t;
    pub fn Astronomy_VectorFromHorizon(
        sphere: astro_spherical_t,
        time: astro_time_t,
        refraction: astro_refraction_t,
    ) -> astro_vector_t;
    pub fn Astronomy_HorizonFromVector(
        vector: astro_vector_t,
        refraction: astro_refraction_t,
    ) -> astro_spherical_t;
    pub fn Astronomy_RotateVector(
        rotation: astro_rotation_t,
        vector: astro_vector_t,
    ) -> astro_vector_t;
    pub fn Astronomy_RotateState(
        rotation: astro_rotation_t,
        state: astro_state_vector_t,
    ) -> astro_state_vector_t;
    pub fn Astronomy_Rotation_EQD_EQJ(time: *mut astro_time_t) -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQD_ECL(time: *mut astro_time_t) -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQD_HOR(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQJ_EQD(time: *mut astro_time_t) -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQJ_ECL() -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQJ_HOR(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_ECL_EQD(time: *mut astro_time_t) -> astro_rotation_t;
    pub fn Astronomy_Rotation_ECL_EQJ() -> astro_rotation_t;
    pub fn Astronomy_Rotation_ECL_HOR(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_HOR_EQD(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_HOR_EQJ(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_HOR_ECL(
        time: *mut astro_time_t,
        observer: astro_observer_t,
    ) -> astro_rotation_t;
    pub fn Astronomy_Rotation_EQJ_GAL() -> astro_rotation_t;
    pub fn Astronomy_Rotation_GAL_EQJ() -> astro_rotation_t;
    pub fn Astronomy_Refraction(refraction: astro_refraction_t, altitude: f64) -> f64;
    pub fn Astronomy_InverseRefraction(refraction: astro_refraction_t, bent_altitude: f64) -> f64;
    pub fn Astronomy_Constellation(ra: f64, dec: f64) -> astro_constellation_t;
    pub fn Astronomy_GravSimInit(
        simOut: *mut *mut astro_grav_sim_t,
        originBody: astro_body_t,
        time: astro_time_t,
        numBodies: c_int,
        bodyStateArray: *const astro_state_vector_t,
    ) -> astro_status_t;
    pub fn Astronomy_GravSimUpdate(
        sim: *mut astro_grav_sim_t,
        time: astro_time_t,
        numBodies: c_int,
        bodyStateArray: *mut astro_state_vector_t,
    ) -> astro_status_t;
    pub fn Astronomy_GravSimBodyState(
        sim: *mut astro_grav_sim_t,
        body: astro_body_t,
    ) -> astro_state_vector_t;
    pub fn Astronomy_GravSimTime(sim: *mut astro_grav_sim_t) -> astro_time_t;
    pub fn Astronomy_GravSimNumBodies(sim: *mut astro_grav_sim_t) -> c_int;
    pub fn Astronomy_GravSimOrigin(sim: *mut astro_grav_sim_t) -> astro_body_t;
    pub fn Astronomy_GravSimSwap(sim: *mut astro_grav_sim_t);
    pub fn Astronomy_GravSimFree(sim: *mut astro_grav_sim_t);
}

pub type astro_position_func_t = core::option::Option<
    unsafe extern "C" fn(context: *mut c_void, time: astro_time_t) -> astro_vector_t,
>;
extern "C" {
    pub fn Astronomy_CorrectLightTravel(
        context: *mut c_void,
        func: astro_position_func_t,
        time: astro_time_t,
    ) -> astro_vector_t;
}
extern "C" {
    pub fn Astronomy_BackdatePosition(
        time: astro_time_t,
        observerBody: astro_body_t,
        targetBody: astro_body_t,
        aberration: astro_aberration_t,
    ) -> astro_vector_t;
}
