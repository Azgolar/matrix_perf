#!/bin/bash
set -e

ordner=(	"Matrix_1d/basis_1d"
				"Matrix_1d/basis_laenge_1d"
				"Matrix_1d/iterator_1d"
				"Matrix_1d/slice_1d"
				"Matrix_1d/split_at_1d"
				"Matrix_1d/unsicher_1d"
				"Matrix_2d/basis_2d"
				"Matrix_2d/basis_laenge_2d"
				"Matrix_2d/iterator_2d"
				"Matrix_2d/slice_2d"
				"Matrix_2d/split_at_2d"
				"Matrix_2d/unsicher_2d")
				
echo ""
echo "Kompiliere aller Programme"
sleep 2
for ((i=0; i < 12; i++)); do
	aktuell="${ordner[$i]}"
	echo "kompiliere $aktuell ($((i+1))/12)"
	(cd "$aktuell" && cargo build --release)
done 

echo ""
echo "Teste alle Programme auf Korrektheit"
sleep 2
for ((i=0; i < 12; i++)); do
	aktuell="${ordner[$i]}"
	bin="$aktuell/target/release/benchmark"
	echo "Test $aktuell ($((i+1))/12)"
	"$bin" test
done 

echo ""
echo "Benchmarking aller Programme"
sleep 2
for ((i=0; i < 12; i++)); do
	aktuell="${ordner[$i]}"
	bin="$aktuell/target/release/benchmark"
	echo "Benchmark $aktuell ($((i+1))/12)"
	perf stat \
	-e cycles,instructions,cache-references,cache-misses,branches,branch-misses,br_inst_retired.cond,task-clock \
	-x, \
	-o "${aktuell//\//_}.csv" \
	-- "$bin" benchmark
done 

echo ""
echo "fertig"
