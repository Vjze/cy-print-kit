import { writable, type Writable } from 'svelte/store';
export const cpn = writable('');
export const pn = writable('');
export const sn = writable<string>('');
export const order = writable('');
export const options = writable<string[]>([]);
export const selectedOption = writable<string | null>(null);
export const configs = writable<config | null> (null);

interface config {
    pn: string,
    cus_number: string,
    data_bt_1: string,
    cus_pn: string,
    btw_name: string,
    data_pnbt_1: string,
    data_sj_2min: number,
    data_sj_2max: number,
    data_sj_3min: number,
    data_sj_3max: number,
    data_sj_4min: number,
    data_sj_4max: number,
    data_sj_5min: number,
    data_sj_5max: number,
    data_inname: string,
    data_outname: string,
    data_xswsi: number,
    data_xswsq: number,
}
