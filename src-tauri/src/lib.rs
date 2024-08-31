mod strcuts;
mod util;
use std::sync::{Arc, Mutex};

// mod do_print;
// use do_print::print_btw;
use strcuts::{Config, DataInfo};
use tiberius::numeric::Numeric;
use util::client;
use chrono::Local;
use rand::Rng;
use serde_json::json;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .manage(MyState::default())
        .invoke_handler(tauri::generate_handler![get_printers, get_config, print_btws])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Default)]
struct MyState {
    library_id: std::sync::Mutex<String>,
    config: std::sync::Mutex<Config>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn get_id(state: tauri::State<'_, MyState>) -> Result<(), String> {
    let url = "http://127.0.0.1/BarTender/api/v1/libraries";
    let r = ureq::get(&url).call();
    match r {
        Ok(res) => {
            let value = res.into_json::<serde_json::Value>().unwrap();
            let ids = &value.get(0).unwrap()["id"];
            let s: String = ids
                .to_string()
                .chars()
                .map(|x| match x {
                    '"' => ' ',
                    '\\' => ' ',
                    _ => x,
                })
                .collect();
            let len = s.len();
            let id = &s[1..len - 1];
            *state.library_id.lock().unwrap() = id.into();
            Ok(())
        }
        Err(_e) => Err("id获取错误".to_string()),
    }
}

#[tauri::command]
async fn get_printers(state: tauri::State<'_, MyState>) -> Result<Vec<String>, String> {
    let _ = get_id(state).await;
    let mut list = vec![];
    let url = "http://127.0.0.1/BarTender/api/v1/printers";
    let res = ureq::get(url).call();
    match res {
        Ok(res) => {
            let value = res.into_json::<serde_json::Value>().unwrap();
            let x = value.get("serverPrinters").unwrap().as_array().unwrap();
            for i in x {
                let x = i.to_string();
                let s: String = x
                    .chars()
                    .map(|x| match x {
                        '"' => ' ',
                        '\\' => ' ',
                        _ => x,
                    })
                    .collect();
                let len = s.len();
                let f = &s[1..len - 1];

                list.push(f.to_string())
            }
            Ok(list)
        }
        Err(_) => Err("未找到打印机".to_string()),
    }
}
#[tauri::command]
async fn get_config(cus_number: String,
  pn: String, state: tauri::State<'_, MyState>) -> Result<Config, String> {
    let mut client = client().await?;
    let queryvalues = "NEW_PN,cus_number,data_bt_1,cus_pn,btw_name,data_pnbt_1
        ,data_sj_2min,data_sj_2max,data_sj_3min,data_sj_3max,data_sj_4min,data_sj_4max
        ,data_sj_5min,data_sj_5max,data_inname,data_outname,data_xswsi,data_xswsq";
    let query_code = format!(
        "select {} from pro_bqlist where cus_number = '{}' and new_pn = '{}'",
        queryvalues, cus_number, pn
    );
    let stream = client.simple_query(query_code).await.unwrap();
    let result = stream.into_row().await.unwrap();
    match result {
        Some(r) => {
            let new_pn = r.get::<&str, _>(0).unwrap();
            let cus_number = r.get::<&str, _>(1).unwrap();
            let data_bt_1 = r.get::<&str, _>(2).unwrap();
            let cus_pn = r.get::<&str, _>(3).unwrap_or("");
            let btw_name = r.get::<&str, _>(4).unwrap();
            let data_pnbt_1 = r.get::<&str, _>(5).unwrap();
            let data_sj_2min = r.get::<Numeric, _>(6).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_2max = r.get::<Numeric, _>(7).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_3min = r.get::<Numeric, _>(8).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_3max = r.get::<Numeric, _>(9).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_4min = r.get::<Numeric, _>(10).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_4max = r.get::<Numeric, _>(11).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_5min = r.get::<Numeric, _>(12).unwrap().to_string().parse::<f64>().unwrap();
            let data_sj_5max = r.get::<Numeric, _>(13).unwrap().to_string().parse::<f64>().unwrap();
            let data_inname = r.get::<&str, _>(14).unwrap();
            let data_outname = r.get::<&str, _>(15).unwrap();
            let data_xswsi = r.get::<i32, _>(16).unwrap();
            let data_xswsq = r.get::<i32, _>(17).unwrap();
            let config = Config {
                pn: new_pn.to_string(),
                cus_number: cus_number.to_string(),
                data_bt_1: data_bt_1.to_string(),
                cus_pn: cus_pn.to_string(),
                btw_name: btw_name.to_string(),
                data_pnbt_1: data_pnbt_1.to_string(),
                data_sj_2min,
                data_sj_2max,
                data_sj_3min,
                data_sj_3max,
                data_sj_4min,
                data_sj_4max,
                data_sj_5min,
                data_sj_5max,
                data_inname: data_inname.to_string(),
                data_outname: data_outname.to_string(),
                data_xswsi: data_xswsi,
                data_xswsq: data_xswsq,
            };
            let c = config.clone();
            *state.config.lock().unwrap() = c.clone();
            Ok(c)
        }
        None => {
            Err("配置获取失败".to_string())
        }
    }
}

#[tauri::command]
async fn print_btws(sn: String, order: String, printer: String, state: tauri::State<'_, MyState>) -> Result<DataInfo, String> {
    let mut rng = Arc::new(Mutex::new(rand::thread_rng()));
    let config = state.config.lock().unwrap();
    let inloss = if config.data_xswsi == 1 {
        let inloss1_c = format!(
            "{:.1}",
            rng.lock().unwrap().gen_range(config.data_sj_2min..config.data_sj_2max)
        );
        let inloss2_c = format!(
            "{:.1}",
            rng.lock().unwrap().gen_range(config.data_sj_4min..config.data_sj_4max)
        );
        (inloss1_c, inloss2_c)
    } else if config.data_xswsi == 2 {
        let inloss1_c = format!(
            "{:.2}",
            rng.lock().unwrap().gen_range(config.data_sj_2min..config.data_sj_2max)
        );
        let inloss2_c = format!(
            "{:.2}",
            rng.lock().unwrap().gen_range(config.data_sj_4min..config.data_sj_4max)
        );
        (inloss1_c, inloss2_c)
    } else {
        let inloss1_c = format!(
            "{:.3}",
            rng.lock().unwrap().gen_range(config.data_sj_2min..config.data_sj_2max)
        );
        let inloss2_c = format!(
            "{:.3}",
            rng.lock().unwrap().gen_range(config.data_sj_4min..config.data_sj_4max)
        );
        (inloss1_c, inloss2_c)
    };
    let reloss = if config.data_xswsq == 1 {
        let reloss1_c = format!(
            "{:.1}",
            rng.lock().unwrap().gen_range(config.data_sj_3min..config.data_sj_3max)
        );
        let reloss2_c = format!(
            "{:.1}",
            rng.lock().unwrap().gen_range(config.data_sj_5min..config.data_sj_5max)
        );
        (reloss1_c, reloss2_c)
    } else if config.data_xswsq == 2 {
        let reloss1_c = format!(
            "{:.2}",
            rng.lock().unwrap().gen_range(config.data_sj_3min..config.data_sj_3max)
        );
        let reloss2_c = format!(
            "{:.2}",
            rng.lock().unwrap().gen_range(config.data_sj_5min..config.data_sj_5min)
        );
        (reloss1_c, reloss2_c)
    } else {
        let reloss1_c = format!(
            "{:.3}",
            rng.lock().unwrap().gen_range(config.data_sj_3min..config.data_sj_3max)
        );
        let reloss2_c = format!(
            "{:.3}",
            rng.lock().unwrap().gen_range(config.data_sj_5min..config.data_sj_5max)
        );
        (reloss1_c, reloss2_c)
    };
    let pn = config.cus_pn.to_string();
    let sntitle = config.data_bt_1.to_string();
    let in_name = config.data_inname.to_string();
    let out_name = config.data_outname.to_string();
    let data_pnbt_1 = config.data_pnbt_1.clone().to_string();
    let bt = String::from("test/");
    let btw_name = bt + &config.btw_name.to_string();
    let printer = printer;
    let data = DataInfo {
        sn: sn.clone(),
        pn: config.pn.to_string(),
        cus_pn: pn.clone(),
        sntitle: sntitle.clone(),
        in_name: in_name.clone(),
        inloss1: inloss.0.clone(),
        reloss1: reloss.0.clone(),
        out_name: out_name.clone(),
        inloss2: inloss.1.clone(),
        reloss2: reloss.1.clone(),
        order: order.to_string(),
        data_pnbt_1: data_pnbt_1.clone(),
        ..Default::default()
    };
    // let btw_check = load_btws(id.clone()).await;
    // let datas = if btws.contains(&config.btw_name.to_string()) {
    let datas = json!({
        // 模版库的ID
        "LibraryID": format!("{}",state.library_id.lock().unwrap()),
            // 模版的绝对路径,与相对路径二者选其一
        // "AbsolutePath": "global_test.btw",
            // 模版的相对路径，例如：Automotive/AIAG/B-10/BMW.btw
        "relativePath": format!("{}",btw_name),

            // 打印机名称
        "printer": format!("{}",printer),
            // 起始位置（一般不传，从参数中拿掉）
        // "StartingPosition": 1,
            // /打印份数
        "Copies": 1,
            // 自增序列
        "SerialNumbers": format!("{}",0),
            // 老版软件设置参数接口
        // "DataEntryControls": {
            // 新版软件设置参数接口
        "namedDataSources": {
            "SN":format!("{}",sn.clone()),
            "PN":format!("{}",pn.clone()),
            "SNTITLE":format!("{}",data_pnbt_1.clone()),
            "TITLE":format!("{}",sntitle.clone()),
            "INLOSS1":format!("≤{}dB",inloss.0.clone()),
            "INLOSS2":format!("≤{}dB",inloss.1.clone()),
            "JK1":format!("{}",in_name.clone()),
            "JK2":format!("{}",out_name.clone()),
            "RELOSS1":format!("≥{}dB",reloss.0).clone(),
            "RELOSS2":format!("≥{}dB",reloss.1.clone()),
        }
    });

    // let worker_thread = tokio::task::spawn(async move{
    let url = "http://127.0.0.1/BarTender/api/v1/print".to_string();
    let res = ureq::post(&url).send_json(&datas);
    match res {
        Ok(res) => {
            let value = res.status_text();
            if value == "OK" {
                Ok(data)
                // insert(data).await
            } else {
                Err("打印错误".to_string())
            }
        }
        Err(_) => Err("打印错误".to_string()),
    }
    // });
    //  match worker_thread.await {
    //     Ok(b) => b,

    //     Err(_e) => Err(MyError::BtwPrintErr),
    // }
}

pub async fn insert(data: DataInfo) -> Result<usize, String> {
    // println!("sn: {}开始写入", data.sn);
    let mut client = client().await?;
    let stream = client
        .simple_query(format!(
            "select print_num,Intime from Cabledata_cy where sn = '{}' order by Intime",
            data.sn.clone()
        ))
        .await
        .unwrap();
    let row = stream.into_row().await.unwrap();
    match row {
        Some(r) => {
            let num = r.get::<i32, _>(0).unwrap();
            let p_n = num + 1;
            let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let stream = client
                .execute(
                    format!(
                        "UPDATE Cabledata_cy set order1 = '{}', In_name = '{}', out_name = '{}', Inloss1 = '{}', Inloss2 = '{}',
                        ReLoss1 = '{}', ReLoss2 = '{}', SNTitle = '{}', cus_info = '{}', print_num = '{}', print_date = '{}', order_num = '{}' WHERE sn = '{}'",
                        data.pn, data.in_name, data.out_name, data.inloss1, data.inloss2, data.reloss1, data.reloss2, data.data_pnbt_1, data.cus_pn, p_n, date, data.order, data.sn
                    ),
                    &[&1i32],
                )
                .await;
            match stream {
                Ok(_) => {
                    // Ok((num.try_into().unwrap(),data.sn.clone()))
                    // println!("sn: {}写入结束", data.sn);
                    Ok(num.try_into().unwrap())
                }
                Err(_) => Err("数据更新错误".to_string()),
            }
        }
        None => {
            let p_n = 1;
            let sql_text = "(sn,order1,In_name,out_name,Inloss1,Inloss2,ReLoss1,ReLoss2,SNTitle,cus_info,print_num,order_num)";
            let stream = client
                .execute(
                    format!(
                        "insert into  Cabledata_cy {} values ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}')",
                        sql_text,data.sn, data.pn, data.in_name, data.out_name, data.inloss1, data.inloss2, data.reloss1, data.reloss2, data.data_pnbt_1, data.cus_pn, p_n, data.order
                    ),
                    &[&1i32],
                )
                .await;
            match stream {
                // Ok(_) => Ok((0,data.sn.clone())),
                Ok(_) => Ok(0),
                Err(_) => Err("数据写入错误".to_string()),
            }
        }
    }
}
