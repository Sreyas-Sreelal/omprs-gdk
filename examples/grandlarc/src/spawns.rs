use omp::types::vector::Vector3;
use rand::seq::SliceRandom;

type SpawnLocationCoords = (Vector3, f32);
pub struct SpawnLocations {
    los_santos_locations: Vec<SpawnLocationCoords>,
    las_venturas_locations: Vec<SpawnLocationCoords>,
    san_fierro_locations: Vec<SpawnLocationCoords>,
}

impl SpawnLocations {
    pub fn new() -> Self {
        let los_santos_locations = vec![
            (Vector3::new(1751.1097, -2_106.453, 13.5469), 183.1979), // El-Corona - Outside random house
            (Vector3::new(2652.6418, -1989.9175, 13.9988), 182.7107), // Random house in willowfield - near playa de seville and stadium
            (Vector3::new(2489.5225, -1957.9258, 13.5881), 2.3440), // Hotel in willowfield - near cluckin bell
            (Vector3::new(2689.5203, -1695.9354, 10.0517), 39.5312), // Outside stadium - lots of cars
            (Vector3::new(2770.5393, -1628.3069, 12.1775), 4.9637), // South in east beach - north of stadium - carparks nearby
            (Vector3::new(2807.9282, -1176.8883, 25.3805), 173.6018), // North in east beach - near apartments
            (Vector3::new(2552.5417, -958.085, 82.6345), 280.2542), // Random house north of Las Colinas
            (Vector3::new(2232.1309, -1159.5679, 25.8906), 103.2939), // Jefferson motel
            (Vector3::new(2388.1003, -1279.8933, 25.1291), 94.3321), // House south of pig pen
            (Vector3::new(2481.1885, -1536.7186, 24.1467), 273.4944), // East LS - near clucking bell and car wash
            (Vector3::new(2_495.072, -1687.5278, 13.5150), 359.6696), // Outside CJ's house - lots of cars nearby
            (Vector3::new(2306.8252, -1_675.434, 13.9221), 2.6271), // House in ganton - lots of cars nearby
            (Vector3::new(2191.8403, -1455.8251, 25.5391), 267.9925), // House in south jefferson - lots of cars nearby
            (Vector3::new(1830.1359, -1092.1849, 23.8656), 94.0113), // Mulholland intersection carpark
            (Vector3::new(2_015.363, -1717.2535, 13.5547), 93.3655), // Idlewood house
            (Vector3::new(1654.7091, -1656.8516, 22.5156), 177.9729), // Right next to PD
            (Vector3::new(1219.0851, -1812.8058, 16.5938), 190.0045), // Conference Center
            (Vector3::new(1508.6849, -1059.0846, 25.0625), 1.8058), // Across the street of BANK - lots of cars in intersection carpark
            (Vector3::new(1421.0819, -885.3383, 50.6531), 3.6516),  // Outside house in vinewood
            (Vector3::new(1133.8237, -1272.1558, 13.5469), 192.4113), // Near hospital
            (Vector3::new(1235.2196, -1608.6111, 13.5469), 181.2655), // Backalley west of mainstreet
            (Vector3::new(590.4648, -1252.2269, 18.2116), 25.0473), // Outside "BAnk of San Andreas"
            (Vector3::new(842.526, -1007.7679, 28.4185), 213.9953), // North of Graveyard
            (Vector3::new(911.9332, -1_232.649, 16.9766), 5.2999),  // LS Film Studio
            (Vector3::new(477.6021, -1496.6207, 20.4345), 266.9252), // Rodeo Place
            (Vector3::new(255.4621, -1366.3256, 53.1094), 312.0852), // Outside propery in richman
            (Vector3::new(281.5446, -1261.4562, 73.9319), 305.0017), // Another richman property
            (Vector3::new(790.1918, -839.8533, 60.6328), 191.9514), // Mulholland house
            (Vector3::new(1299.1859, -801.4249, 84.1406), 269.5274), // Maddoggs
            (Vector3::new(1_240.317, -2036.6886, 59.9575), 276.4659), // Verdant Bluffs
            (Vector3::new(2_215.518, -2627.8174, 13.5469), 273.7786), // Ocean docks 1
            (Vector3::new(2509.4346, -2637.6543, 13.6453), 358.3565), // Ocean Docks spawn 2
        ];

        let las_venturas_locations = vec![
            (Vector3::new(1435.8024, 2662.3647, 11.3926), 1.1650), //  Northern train station
            (Vector3::new(1457.4762, 2773.4868, 10.8203), 272.2754), //  Northern golf club
            (Vector3::new(1_739.639, 2_803.057, 14.2735), 285.3929), //  Northern housing estate 1
            (Vector3::new(1870.3096, 2_785.247, 14.2734), 42.3102), //  Northern housing estate 2
            (Vector3::new(1959.7142, 2754.6863, 10.8203), 181.4731), //  Northern house 1
            (Vector3::new(2314.2556, 2759.4504, 10.8203), 93.2711), //  Northern industrial estate 1
            (Vector3::new(2216.5674, 2715.0334, 10.8130), 267.654), //  Northern industrial estate 2
            (Vector3::new(2101.4192, 2678.7874, 10.8130), 92.0607), //  Northern near railway line
            (Vector3::new(1_951.109, 2660.3877, 10.8203), 180.8461), //  Northern house 2
            (Vector3::new(1_666.695, 2_604.986, 10.8203), 179.8495), //  Northern house 3
            (Vector3::new(2808.3367, 2421.5107, 11.0625), 136.206), //  Northern shopping centre
            (Vector3::new(2633.3203, 2_349.706, 10.6719), 178.7175), //  V-Rock
            (Vector3::new(2606.6348, 2_161.749, 10.8203), 88.7508), //  South V-Rock
            (Vector3::new(2616.5286, 2100.6226, 10.8158), 177.7834), //  North Ammunation 1
            (Vector3::new(2491.8816, 2_397.937, 10.8203), 266.6003), //  North carpark 1
            (Vector3::new(2_531.789, 2530.3223, 21.8750), 91.6686), //  North carpark 2
            (Vector3::new(2340.6677, 2530.4324, 10.8203), 177.863), //  North Pizza Stack
            (Vector3::new(2097.6855, 2491.3313, 14.8390), 181.8117), //  Emerald Isle
            (Vector3::new(1_893.1, 2423.2412, 11.1782), 269.4385), //  Souvenir shop
            (Vector3::new(1_698.933, 2_241.832, 10.8203), 357.8584), //  Northern casino
            (Vector3::new(1479.4559, 2_249.077, 11.0234), 306.379), //  Baseball stadium 1
            (Vector3::new(1298.1548, 2083.4016, 10.8127), 256.7034), //  Baseball stadium 2
            (Vector3::new(1117.8785, 2304.1514, 10.8203), 81.5490), //  North carparks
            (Vector3::new(1108.9878, 1705.8639, 10.8203), 0.6785), //  Dirtring racing 1
            (Vector3::new(1_423.978, 1034.4188, 10.8203), 90.9590), //  Sumo
            (Vector3::new(1537.4377, 752.0641, 11.0234), 271.6893), //  Church
            (Vector3::new(1_917.959, 702.6984, 11.1328), 359.2682), //  Southern housing estate
            (Vector3::new(2089.4785, 658.0414, 11.2707), 357.3572), //  Southern house 1
            (Vector3::new(2489.8286, 928.3251, 10.8280), 67.2245), //  Wedding chapel
            (Vector3::new(2697.4717, 856.4916, 9.8360), 267.0983), //  Southern construction site
            (Vector3::new(2845.6104, 1288.1444, 11.3906), 3.6506), //  Southern train station
            (Vector3::new(2_437.937, 1293.1442, 10.8203), 86.3830), //  Wedding chapel (near Pyramid)
            (Vector3::new(2_299.543, 1451.4177, 10.8203), 269.1287), //  Carpark (near Pyramid)
            (Vector3::new(2214.3008, 2041.9165, 10.8203), 268.7626), //  Central parking lot
            (Vector3::new(2005.9174, 2152.0835, 10.8203), 270.1372), //  Central motel
            (Vector3::new(2222.1042, 1_837.422, 10.8203), 88.6461), //  Clowns Pocket
            (Vector3::new(2025.6753, 1916.4363, 12.3382), 272.5852), //  The Visage
            (Vector3::new(2087.9902, 1516.5336, 10.8203), 48.9300), //  Royal Casino
            (Vector3::new(2172.1624, 1398.7496, 11.0625), 91.3783), //  Auto Bahn
            (Vector3::new(2_139.184, 987.7975, 10.8203), 0.2315),   //  Come-a-lot
            (Vector3::new(1860.9672, 1_030.291, 10.8203), 271.6988), //  Behind 4 Dragons
            (Vector3::new(1673.2345, 1316.1067, 10.8203), 177.7294), //  Airport carpark
            (Vector3::new(1412.6187, 2000.0596, 14.7396), 271.3568), //  South baseball stadium houses
        ];

        let san_fierro_locations = vec![
            (Vector3::new(-2723.4639, -314.8138, 7.1839), 43.5562), // golf course spawn
            (Vector3::new(-2694.5344, 64.5550, 4.3359), 95.0190),   // in front of a house
            (Vector3::new(-2_458.2, 134.5419, 35.1719), 303.9446),  // hotel
            (Vector3::new(-2_796.659, 219.5733, 7.1875), 88.8288),  // house
            (Vector3::new(-2706.5261, 397.7129, 4.3672), 179.8611), // park
            (Vector3::new(-2866.7683, 691.9363, 23.4989), 286.306), // house
            (Vector3::new(-2764.9543, 785.6434, 52.7813), 357.6817), // donut shop
            (Vector3::new(-2660.9402, 883.2115, 79.7738), 357.444), // house
            (Vector3::new(-2861.0796, 1047.7109, 33.6068), 188.275), //  parking lot
            (Vector3::new(-2_629.201, 1383.1367, 7.1833), 179.7006), // parking lot at the bridge
            (Vector3::new(-2079.6802, 1430.0189, 7.1016), 177.6486), // pier
            (Vector3::new(-1660.2294, 1382.6698, 9.8047), 136.2952), //  pier 69
            (Vector3::new(-1674.1964, 430.3246, 7.1797), 226.1357), // gas station]
            (Vector3::new(-1954.9982, 141.808, 27.1747), 277.7342), // train station
            (Vector3::new(-1956.1447, 287.1091, 35.4688), 90.4465), // car shop
            (Vector3::new(-1888.1117, 615.7245, 35.1719), 128.4498), // random
            (Vector3::new(-1922.5566, 886.8939, 35.3359), 272.1293), // random
            (Vector3::new(-1983.3458, 1117.0645, 53.1243), 271.239), // church
            (Vector3::new(-2417.6458, 970.1491, 45.2969), 269.3676), // gas station
            (Vector3::new(-2_108.017, 902.803, 76.5792), 5.7139),   // house
            (Vector3::new(-2097.5664, 658.0771, 52.3672), 270.4487), // random
            (Vector3::new(-2_263.665, 393.7423, 34.7708), 136.4152), // random
            (Vector3::new(-2287.5027, 149.1875, 35.3125), 266.3989), // baseball parking lot
            (Vector3::new(-2_039.357, -97.7205, 35.1641), 7.4744),  // driving school
            (Vector3::new(-1867.5022, -141.9203, 11.8984), 22.4499), // factory
            (Vector3::new(-1537.8992, 116.0441, 17.3226), 120.8537), // docks ship
            (Vector3::new(-1708.4763, 7.0187, 3.5489), 319.326),    // docks hangar
            (Vector3::new(-1427.0858, -288.943, 14.1484), 137.0812), // airport
            (Vector3::new(-2173.0654, -392.7444, 35.3359), 237.0159), // stadium
            (Vector3::new(-2320.5286, -180.387, 35.3135), 179.698), // burger shot
            (Vector3::new(-2_930.005, 487.2518, 4.9141), 3.8258),   // harbor
        ];

        Self {
            los_santos_locations,
            las_venturas_locations,
            san_fierro_locations,
        }
    }

    pub fn get_random_ls(&self) -> &SpawnLocationCoords {
        self.los_santos_locations
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
    pub fn get_random_sf(&self) -> &SpawnLocationCoords {
        self.san_fierro_locations
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
    pub fn get_random_lv(&self) -> &SpawnLocationCoords {
        self.las_venturas_locations
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}
