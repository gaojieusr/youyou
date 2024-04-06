use crate::materials::display_unit::DisplayUnit;

use self::{
    control_card::{ControlCard, ControlCards},
    display_unit::DisplayUnits,
};

///此模块用于构造各种物料
///
///
///@2024-3-9

///特征：物料的操作CRUD
pub trait MaterialsCRUD<M> {
    type Output;
    //创建物料
    fn create_materials() -> Self::Output;
    // //查询物料
    // fn query_materials() -> Self::Output;
    //添加物料
    fn add_material(&mut self, m: &M);
    // //修改物料
    // fn update_material(&mut self, m: M);
    //删除物料
    fn delete_material(&mut self, m: &M);
}

//###########################
// ##########################
// 控制卡部分
// ##########################
//##########################

pub(crate) mod control_card {
    use std::ops::Div;

    use super::{
        display_unit::{DisplayUnit, Resolution},
        MaterialsCRUD,
    };

    #[derive(Debug, Clone, PartialEq)]

    //控制卡接口类型
    pub enum ControlCardInterfaceType {
        I50Pin,
        HUB75,
        HUB320,
        Netwrkport,
        OtherCardInterfaceType,
    }

    impl ControlCardInterfaceType {
        pub fn from_str(s: String) -> ControlCardInterfaceType {
            let s = s.as_str();
            match s {
                "50PIN" => ControlCardInterfaceType::I50Pin,
                "HUB75" => ControlCardInterfaceType::HUB75,
                "HUB320" => ControlCardInterfaceType::HUB320,
                "Netwrkport" => ControlCardInterfaceType::Netwrkport,
                _ => ControlCardInterfaceType::OtherCardInterfaceType,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    ///控制卡集合：
    pub(crate) struct ControlCards {
        control_cards: Vec<ControlCard>,
    }
    //控制卡集合的CUD
    impl MaterialsCRUD<ControlCard> for ControlCards {
        type Output = ControlCards;

        //生成控制卡集合
        fn create_materials() -> Self::Output {
            ControlCards {
                control_cards: Vec::new(),
            }
        }

        //在控制卡集合中添加控制卡
        fn add_material(&mut self, m: &ControlCard) {
            self.control_cards.push(m.clone());
            println!("添加控制卡成功！");
        }

        //在控制卡集合中删除控制卡
        fn delete_material(&mut self, m: &ControlCard) {
            self.control_cards
                .iter()
                .position(|x| x == &m.clone())
                .map(|i| self.control_cards.remove(i))
                .unwrap();
            println!("删除控制卡成功！");
        }
    }

    #[derive(Debug, Clone, PartialEq)]

    ///控制卡：
    pub struct ControlCard {
        //产品名称：默认是“控制卡”
        pub(crate) name: String,
        //型号名称：根据函数参数确定
        pub(crate) type_name: String,
        //技术参数：根据函数参数确定
        parameter: String,
        //价格：根据输入确定
        cost: u32,
        //输入接口：根据函数参数确定
        pub(crate) input_interface: (ControlCardInterfaceType, u32),
        //输出接口：根据函数参数确定
        output_interface: (ControlCardInterfaceType, u32),
        //负载能力：根据函数参数确定
        load_capacity: (u32, u32),
    }
    //计算结果：卡的类型和数量
    pub struct CardsTypesAndNums {
        type_name: String,
        card_nums: u32,
        const_total: u32,
    }

    ///Method：控制卡的方法
    impl ControlCard {
        pub fn new(
            type_name: String,
            parameter: String,
            cost: u32,
            input_interface: (String, u32),
            output_interface: (String, u32),
            load_capacity: (u32, u32),
        ) -> Self {
            ControlCard {
                name: "控制卡".to_string(),
                type_name,
                parameter,
                cost,
                input_interface: (
                    ControlCardInterfaceType::from_str(input_interface.0),
                    input_interface.1,
                ),
                output_interface: (
                    ControlCardInterfaceType::from_str(output_interface.0),
                    output_interface.1,
                ),
                load_capacity,
            }
        }

        ///计算用某种卡需要的数量
        pub fn cal_cards_num(
            //模组排列：
            units_matrix: (u32, u32),
            //模组分辨率：
            units_resolution: (u32, u32),
            //卡的带载分辨率
            cards_load_capacity: (u32, u32),
            //卡的接口数量
            cards_interface_nums: u32,
        ) -> u32 {
            //单张卡的最大带载模组数量
            let mut max_load_nums = (cards_load_capacity.0 * cards_load_capacity.1)
                / (units_resolution.0 * units_resolution.1);
            if max_load_nums > cards_interface_nums {
                max_load_nums = cards_interface_nums;
            }
            //计算每列需要多少张卡，取整
            let mut card_nums_row = (units_matrix.0 as f32 / max_load_nums as f32).ceil() as u32;

            //计算出总共需要多少张卡

            let cards_nums = card_nums_row * units_matrix.1;
            cards_nums
        }

        ///计算：根据显示单元种类、其排列矩阵，屏幕接口类型、产品库中的卡的种类,计算卡的数量及价格最少的方案
        pub fn cal_cards_type_nums_consts(
            //显示单元种类
            unit_type: DisplayUnit,
            matrix: (u32, u32),
            interface_type: ControlCardInterfaceType,
            vecs_cards: ControlCards,
        ) -> CardsTypesAndNums {
            //先对卡的接口和显示单元的接口进行匹配
            let vecs_cards_type: Vec<ControlCard> = vecs_cards
                .control_cards
                .iter()
                .filter(|x| x.output_interface.0 == interface_type)
                .map(|x| x.clone())
                .collect();

            //遍历控制卡的集合，对每种卡需要的数量和价格进行计算，并给出价格最低的组合
            let mut vecs_cards_type_nums: Vec<(ControlCard, u32)> = vecs_cards_type
                .iter()
                .map(|x| {
                    (
                        x.clone(),
                        ControlCard::cal_cards_num(
                            matrix,
                            (unit_type.resolution.length, unit_type.resolution.width),
                            x.load_capacity,
                            x.output_interface.1,
                        ),
                    )
                })
                .collect();
            vecs_cards_type_nums.sort_by(|a, b| a.1.cmp(&b.1));

            //计算出每种卡所需要的价格
            let vecs_cards_type_nums_price: Vec<(ControlCard, u32, u32)> = vecs_cards_type_nums
                .iter()
                .map(|x| (x.0.clone(), x.1, x.0.cost * x.1))
                .collect();

            //计算出价格最低的组合
            let (cards_type, cards_nums, cards_cost) = vecs_cards_type_nums_price
                .iter()
                .min_by(|a, b| a.2.cmp(&b.2))
                .unwrap();
            CardsTypesAndNums {
                type_name: cards_type.type_name.clone(),
                card_nums: cards_nums.clone(),
                const_total: cards_cost.clone(),
            }
        }
    }
}
//###########################
// ##########################
// 显示单元部分
// ##########################
//##########################
mod display_unit {
    use super::{control_card::ControlCardInterfaceType, MaterialsCRUD};

    #[derive(Debug, Clone, PartialEq)]
    //分辨率
    pub struct Resolution {
        //分辨率
        pub length: u32,
        //宽度
        pub width: u32,
    }
    impl Resolution {
        pub fn new(length: u32, width: u32) -> Self {
            Resolution { length, width }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    //显示单元套件类型：
    enum DisplayUnitSuitType {
        //塑料
        PlasticSuit,
        //铝合金套件
        AlumimetalSuit,
        //压铸铝箱体
        CastAlumimetalBox,
        //其他
        OtherDisplayUnitSuitType,
    }
    impl DisplayUnitSuitType {
        fn from_str(s: String) -> DisplayUnitSuitType {
            let s = s.as_str();
            match s {
                "塑料套件" => DisplayUnitSuitType::PlasticSuit,
                "铝合金套件" => DisplayUnitSuitType::AlumimetalSuit,
                "压铸铝箱体" => DisplayUnitSuitType::CastAlumimetalBox,
                _ => DisplayUnitSuitType::OtherDisplayUnitSuitType,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    ///显示单元集合：
    pub struct DisplayUnits {
        pub display_units: Vec<DisplayUnit>,
    }

    ///显示单元集合的CUD实现
    impl MaterialsCRUD<DisplayUnit> for DisplayUnits {
        type Output = DisplayUnits;
        fn create_materials() -> Self::Output {
            DisplayUnits {
                display_units: Vec::new(),
            }
        }
        fn add_material(&mut self, m: &DisplayUnit) {
            self.display_units.push(m.clone());
            println!("添加显示单元成功！");
        }
        fn delete_material(&mut self, m: &DisplayUnit) {
            self.display_units
                .iter()
                .position(|x| x == &m.clone())
                .map(|i| self.display_units.remove(i))
                .unwrap();
            println!("删除显示单元成功！");
        }
    }

    ///struct：显示单元
    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct DisplayUnit {
        //产品名称：默认是“显示单元”
        name: String,
        //型号名称：根据函数参数确定
        type_name: String,
        //技术参数：根据函数参数确定
        parameter: String,
        //价格：根据输入确定
        cost: u32,
        //输入接口：根据函数参数确定
        input_interface: ControlCardInterfaceType,
        //点间距：根据函数参数确定
        pub point_spacing: f32,
        //分辨率：根据函数参数确定
        pub resolution: Resolution,
        //套件类型
        pub suite_type: DisplayUnitSuitType,
        //尺寸:长宽厚
        pub size: (u32, u32, u32),
        //峰值功耗:千瓦
        peak_power: f32,
    }

    ///显示单元的方法
    impl DisplayUnit {
        pub fn new(
            type_name: String,
            parameter: String,
            cost: u32,
            input_interface: String,
            point_spacing: f32,
            resolution: (u32, u32),
            suite_type: String,
            size: (u32, u32, u32),
            peak_power: f32,
        ) -> Self {
            DisplayUnit {
                name: "显示单元".to_string(),
                type_name,
                parameter,
                cost,
                input_interface: ControlCardInterfaceType::from_str(input_interface),
                point_spacing,
                resolution: Resolution::new(resolution.0, resolution.1),
                suite_type: DisplayUnitSuitType::from_str(suite_type),
                size,
                peak_power,
            }
        }
    }
}

// ################################
///以下为测试用例
// ################################
//测试添加卡的型号
fn add_control_cards() -> ControlCards {
    // //创建一个供应商
    // let kalaite = Supplier::new("Kalaite".to_string());
    // //创建供应商集合
    // let suppliers = vec![kalaite];
    //创建一个控制卡集合
    let mut control_cards = ControlCards::create_materials();
    //创建一种卡e120和e80
    let mut e120 = ControlCard::new(
        "E120".to_string(),
        "技术参数1".to_string(),
        89,
        ("Netwrkport".to_string(), 1),
        ("HUB75".to_string(), 12),
        (192, 1024),
    );
    let mut e80 = ControlCard::new(
        "E80".to_string(),
        "技术参数2".to_string(),
        110,
        ("Netwrkport".to_string(), 1),
        ("HUB75".to_string(), 12),
        (128, 1024),
    );
    let mut i5a_75e = ControlCard::new(
        "5A-75E".to_string(),
        "技术参数3".to_string(),
        130,
        ("Netwrkport".to_string(), 16),
        ("HUB75".to_string(), 12),
        (256, 1024),
    );
    //为控制卡集合添加卡
    control_cards.add_material(&e120);
    control_cards.add_material(&e80);
    control_cards.add_material(&i5a_75e);
    control_cards.delete_material(&e120);

    control_cards
}

//添加显示单元
pub fn add_display_units() -> DisplayUnits {
    let mut display_units = DisplayUnits::create_materials();
    let ys_p2_0_sih = DisplayUnit::new(
        "YS-P2.0SIH".to_string(),
        "YS-P2.0塑料技术参数".to_string(),
        100,
        "HUB75".to_string(),
        2.0,
        (160, 80),
        "塑料套件".to_string(),
        (320, 160, 27),
        0.033,
    );
    let ys_p2_0_lih = DisplayUnit::new(
        "YS-P2.0LIH".to_string(),
        "YS-P2.0铝合金技术参数".to_string(),
        200,
        "HUB75".to_string(),
        2.0,
        (160, 80),
        "铝合金套件".to_string(),
        (320, 160, 27),
        0.033,
    );
    let ys_p1_5_sih = DisplayUnit::new(
        "YS-P1.5SIH".to_string(),
        "YS-P1.5塑料技术参数".to_string(),
        100,
        "HUB75".to_string(),
        1.538,
        (208, 104),
        "塑料套件".to_string(),
        (320, 160, 27),
        0.033,
    );

    println!("{:?}", ys_p2_0_sih);
    display_units.add_material(&ys_p2_0_sih);
    display_units.add_material(&ys_p2_0_lih);
    display_units.add_material(&ys_p1_5_sih);
    display_units
}

#[derive(Debug, Clone, PartialEq)]
///计算结果：显示单元排列矩阵及其他信息
pub struct LedUnitsMatrixInfo {
    led_unit_matrix: (u32, u32),
    other_info: String, //如果根据用户输入求得的结果是整数，则完全符合预期，否则提示尺寸不匹配
}

///根据输入的点间距、长宽、套件尺寸、来精确计算led显示屏精确尺寸长宽、模组排列
pub fn cal_led_screen_info(
    point_spacing: f32,
    screen_size: (u32, u32),
    suite_size: (u32, u32, u32),
) -> LedUnitsMatrixInfo {
    let disoplay_units = add_display_units();
    //获取所有符合点间距要求的显示单元类型
    let led_types: Vec<DisplayUnit> = disoplay_units
        .display_units
        .iter()
        .filter(|x| x.point_spacing == point_spacing)
        .map(|x| x.clone())
        .collect::<Vec<display_unit::DisplayUnit>>();
    println!("led_types第一次匹配: {:#?}", led_types);

    //根据输入的屏幕尺寸来精确计算所需要的模组排列
    //地板除法
    let x = std::ops::Div::div(screen_size.0, suite_size.0);
    let y = std::ops::Div::div(screen_size.1, suite_size.1);

    let led_unit_matrix = (x, y);
    let mut led_units_matrix_info = LedUnitsMatrixInfo {
        led_unit_matrix: led_unit_matrix,
        other_info: "其他信息".to_string(),
    };
    match (
        led_unit_matrix.0 * suite_size.0,
        led_unit_matrix.1 * suite_size.1,
    ) {
        screen_size => led_units_matrix_info.other_info = "尺寸完全相符".to_string(),
        _ => led_units_matrix_info.other_info = "尺寸不匹配，以下为建议尺寸".to_string(),
    }

    led_units_matrix_info
}

#[cfg(test)]
mod tests {

    use super::DisplayUnit;
    use super::*;
    use crate::materials::add_control_cards;
    use crate::materials::add_display_units;

    #[test]
    fn test1() {
        assert_eq!(
            cal_led_screen_info(2.0, (5120, 2880), (320, 160, 27)),
            LedUnitsMatrixInfo {
                led_unit_matrix: (16, 18),
                other_info: "尺寸完全相符".to_string()
            }
        );
    }
}



