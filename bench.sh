#!/bin/sh

input=./sample.d/seq.16G.txt

maxi=1677721600

gendata_seq(){
	seq 1 $maxi
}

gendata_rs_seq(){
	rs-seq $maxi
}

gendata_progress(){
	dd \
		if=/dev/stdin \
		of="${input}" \
		bs=1048576 \
		status=progress
}

gendata(){
	echo generating data...
	mkdir -p sample.d

	which rs-seq | fgrep -q rs-seq
	fasterseq=$?
	if test 0 -eq ${fasterseq}; then
		gendata_rs_seq | gendata_progress
	else
		gendata_seq | gendata_progress
	fi
}

bench_native(){
	dd \
		if="${input}" \
		of=/dev/stdout \
		bs=1048576 \
		status=progress |
		\time -l env ENV_CHAR_TO_FIND_NEEDLE='	' ./rs-find-char-ascii -
}

bench_wasmtime(){
	dd \
		if="${input}" \
		of=/dev/stdout \
		bs=1048576 \
		status=progress |
		\time -l \
			wasmtime \
				run \
				--env ENV_CHAR_TO_FIND_NEEDLE='	' \
				./rs-find-char-ascii.wasm -
}

bench_wazero(){
	dd \
		if="${input}" \
		of=/dev/stdout \
		bs=1048576 \
		status=progress |
		\time -l \
			wazero \
				run \
				--env ENV_CHAR_TO_FIND_NEEDLE='	' \
				./rs-find-char-ascii.wasm -
}

bench_fgrep(){
	dd \
		if="${input}" \
		of=/dev/stdout \
		bs=1048576 \
		status=progress |
		\time -l fgrep -l '	' -
}

test -f "${input}" || gendata

bench_native
#bench_wasmtime
#bench_wazero
#bench_fgrep
