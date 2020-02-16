extern crate sobol;

use crate::sobol::*;
use crate::sobol::params::*;


/** Loads parameter data and verifies a few values */
#[test] fn test_load_params() {

    /* Ensure at least 1000 parameter records loaded */
    let params = JoeKuoD6::load();
    assert!(params.max_dims > 999);

    /* Ensure some parameter data loaded correctly */
    let dim_30 = params.get_dim(30);
    assert!(dim_30.d() == 30);
    assert!(dim_30.s() == 7);
}

/** Initializes direction values and compares them to an external reference */
#[test] fn test_direction_vals() {

    /* Compute direction values for first 6 dimensions */
    let params = JoeKuoD6::load();
    let dir_vals = Sobol::<u32>::init_direction_vals(6, 32, &params);

    /* Reference direction values for a 32-bit sequence with 6 dimensions */
    let ref_dir_vals = vec![
        vec![ 2147483648, 1073741824,  536870912,  268435456,  134217728,   67108864,   33554432,   16777216,
                 8388608,    4194304,    2097152,    1048576,     524288,     262144,     131072,      65536,
                   32768,      16384,       8192,       4096,       2048,       1024,        512,        256,
                     128,         64,         32,         16,          8,          4,          2,          1 ],
        vec![ 2147483648, 3221225472, 2684354560, 4026531840, 2281701376, 3422552064, 2852126720, 4278190080,
              2155872256, 3233808384, 2694840320, 4042260480, 2290614272, 3435921408, 2863267840, 4294901760,
              2147516416, 3221274624, 2684395520, 4026593280, 2281736192, 3422604288, 2852170240, 4278255360,
              2155905152, 3233857728, 2694881440, 4042322160, 2290649224, 3435973836, 2863311530, 4294967295 ],
        vec![ 2147483648, 3221225472, 1610612736, 2415919104, 3892314112, 1543503872, 2382364672, 3305111552,
              1753219072, 2629828608, 3999268864, 1435500544, 2154299392, 3231449088, 1626210304, 2421489664,
              3900735488, 1556135936, 2388680704, 3314585600, 1751705600, 2627492864, 4008611328, 1431684352,
              2147543168, 3221249216, 1610649184, 2415969680, 3892340840, 1543543964, 2382425838, 3305133397 ],
        vec![ 2147483648, 3221225472,  536870912, 1342177280, 4160749568, 1946157056, 2717908992, 2466250752,
              3632267264,  624951296, 1507852288, 3872391168, 2013790208, 3020685312, 2181169152, 3271884800,
               546275328, 1363623936, 4226424832, 1977167872, 2693105664, 2437829632, 3689389568,  635137280,
              1484783744, 3846176960, 2044723232, 3067084880, 2148008184, 3222012020,  537002146, 1342505107 ],
        vec![ 2147483648, 1073741824,  536870912, 2952790016, 4160749568, 3690987520, 2046820352, 2634022912,
              1518338048,  801112064, 2707423232, 4038066176, 3666345984, 1875116032, 2170683392, 1085997056,
               579305472, 3016343552, 4217741312, 3719483392, 2013407232, 2617981952, 1510979072,  755882752,
              2726789248, 4090085440, 3680870432, 1840435376, 2147625208, 1074478300,  537900666, 2953698205 ],
        vec![ 2147483648, 1073741824, 1610612736,  805306368, 3355443200,  603979776, 1442840576, 4211081216,
              3766484992, 1883242496, 2824863744,  338690048, 2663907328, 3743678464, 3067478016, 2344288256,
              1207992320, 1677737984,  905994240, 3405787136,  679528448, 1413489664, 4267726336, 4012964608,
              2118705280, 2942595136,  515287136, 2676692016, 3603439304, 3139739428, 2161563350, 1086045115 ] ];

    /* Compare computed values with reference values */
    assert!(dir_vals[0] == ref_dir_vals[0]);
    assert!(dir_vals[1] == ref_dir_vals[1]);
    assert!(dir_vals[2] == ref_dir_vals[2]);
    assert!(dir_vals[3] == ref_dir_vals[3]);
    assert!(dir_vals[4] == ref_dir_vals[4]);
    assert!(dir_vals[5] == ref_dir_vals[5]);
}