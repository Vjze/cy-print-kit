use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Config {
    pub pn: String,
    pub cus_number: String,
    pub data_bt_1: String,
    pub cus_pn: String,
    pub btw_name: String,
    pub data_pnbt_1: String,
    pub data_sj_2min: f64,
    pub data_sj_2max: f64,
    pub data_sj_3min: f64,
    pub data_sj_3max: f64,
    pub data_sj_4min: f64,
    pub data_sj_4max: f64,
    pub data_sj_5min: f64,
    pub data_sj_5max: f64,
    pub data_inname: String,
    pub data_outname: String,
    pub data_xswsi: i32,
    pub data_xswsq: i32,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DataInfo {
    pub sn: String,
    pub pn: String,
    pub cus_pn: String,
    pub sntitle: String,
    pub in_name: String,
    pub inloss1: String,
    pub reloss1: String,
    pub out_name: String,
    pub inloss2: String,
    pub reloss2: String,
    pub print_num: i32,
    pub order: String,
    pub data_pnbt_1: String,
    pub btw_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct PrintData {
    pub sn: String,
    pub order: String,
    pub printer: String,
}