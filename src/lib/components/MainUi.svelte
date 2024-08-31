<script lang="ts">
	import { cpn, pn, order, options, selectedOption, configs, sn, } from '../stores/store';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	$: isButtonDisabled = !($cpn.trim() && $pn.trim());
	$: isCleanButtonDisabled = !($cpn.trim() || $pn.trim() || $order.trim());
	$: isLoadedConfig = $configs == null;
	$: issnloaded = !$order.trim() || $configs == null;
	$: isprintaction= !($sn.trim()) || $configs==null;
	// 模拟从服务器拉取数据
	async function fetchData() {
		// 这里可以替换成真实的 API 调用
		invoke('get_printers')
			.then((value: any) => {
				options.set(value);
			})
			.catch((value) => options.set([value]));
	}

	onMount(() => {
		// 组件加载后拉取数据
		fetchData();
	});
	function handleSelect(event: Event) {
		const target = event.target as HTMLSelectElement;
		selectedOption.set(target.value);
	}
	function clean() {
		cpn.set('');
		pn.set('');
		order.set('');
		configs.set(null);
		selectedOption.set(null);
	}
	function getConfig() {
		invoke('get_config', {
			cpn: $cpn,
			pn: $pn
		})
			.then((value: any) => {
				$configs = value;
			})
			.catch((value) => options.set(['未找到打印机']));
	}
	function printAction() {
		invoke('print_btw', {
			sn:$sn, order:$order, printer:$selectedOption
		})
			.then((value: any) => {
				invoke('insert_sql',{
					value
				}).then((num)=>{
					console.log(num)
				}).catch((value)=>{
					console.log(value)
				})
			})
			.catch((value) => options.set(['未找到打印机']));
	}
	
</script>

<div class="container">
	<span class="custom-input">客户号:</span>
	<input
		type="text"
		class="custom-input h-30 rounded-lg"
		placeholder="输入客户号...."
		bind:value={$cpn}
	/>
	<span class="custom-input">料号:</span>
	<input
		type="text"
		class="custom-input h-30 w-[100px] rounded-md bg-white px-3 py-2 text-magnum-700"
		placeholder="输入料号...."
		bind:value={$pn}
	/>
	<button class="custom-input" on:click={getConfig} disabled={isButtonDisabled}>配置拉取</button>
</div>

<div class="container">
	<span class="custom-input">工单号:</span>
	<input
		type="text"
		class="custom-input h-30"
		placeholder="输入工单号...."
		disabled={isLoadedConfig}
		bind:value={$order}
	/>
	<span class="custom-input">打印机:</span>

	<select class="select" on:change={handleSelect} bind:value={$selectedOption}>
		<option value="" disabled selected>选择打印机</option>
		{#each $options as option}
			<option value={option}>{option}</option>
		{/each}
	</select>
	<button on:click={clean} class="custom-button" disabled={isCleanButtonDisabled}>界面清空</button>
</div>
<div class="infos">
	<span>料号名称:{$configs?.data_pnbt_1}</span>
	<span>模板名称:{$configs?.btw_name}</span>
	<span>接口一:{$configs?.data_inname}</span>
	<span>接口二:{$configs?.data_outname}</span>
</div>

<div><input on:submit={printAction} type="text" disabled={issnloaded} class="sn" placeholder="扫描Sn...." bind:value={$sn} />
	<button on:click={printAction} class="print-btn" disabled={isprintaction}>打印</button></div>

<style>
	.print-btn {
		width: 120px;
		font-size: 40px !important;
		border-radius: 15px;
	}
	.sn {
		width: 80%;
		font-size: 30px !important;
		text-align: center;
		border-radius: 10px;
		justify-content: space-between;
		padding: 15px;
	}
	.infos {
		display: flex;
		justify-content: space-between;
		padding: 10px;
	}
	.custom-input {
		font-size: 20px !important;
		text-align: left;
		border-radius: 10px;
		/* border: 1px solid #eee; */
	}
	.custom-button {
		font-size: 20px !important;
		border-radius: 10px;
		/* border: 1px solid #eee; */
	}
	.container {
		border-radius: 30px;
		display: flex;
		padding: 5px;
		/* text-align: center; */
		justify-content: space-between;
		align-items: center;
		/* justify-items: center; */
	}
	.select {
		height: 40px;
		font-size: 30px;
	}
</style>
