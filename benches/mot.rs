use criterion::{criterion_group, criterion_main, Criterion};
use ioutrack::{ByteTrack, SORTTracker};
use ndarray::prelude::*;
use ndarray_npy::read_npy;

fn run_sort_on_dets(dets: &Array2<f32>, frame_borders: &Array1<usize>) {
    let mut tracker = SORTTracker::new(25, 2, 0.3, 0.5);
    let mut first_i: usize = 0;
    for &last_i in frame_borders.iter() {
        tracker
            .update(dets.slice(s![first_i..last_i, ..]).into(), false)
            .unwrap();
        first_i = last_i;
    }
}

pub fn criterion_mot_sort_benchmark(c: &mut Criterion) {
    let dets: Array2<f32> = read_npy("benches/data/mot_20-03_500_dets.npy").unwrap();
    let frame_borders: Array1<u64> =
        read_npy("benches/data/mot_20-03_500_frame_borders.npy").unwrap();
    let frame_borders = frame_borders.mapv(|x| x as usize);

    c.bench_function("mot_20-03_sort", |b| {
        b.iter(|| run_sort_on_dets(&dets, &frame_borders))
    });
}

fn run_bytetrack_on_dets(dets: &Array2<f32>, frame_borders: &Array1<usize>) {
    let mut tracker = ByteTrack::new(25, 2, 0.3, 0.5, 0.5, 0.1);
    let mut first_i: usize = 0;
    for &last_i in frame_borders.iter() {
        tracker
            .update(dets.slice(s![first_i..last_i, ..]).into(), false)
            .unwrap();
        first_i = last_i;
    }
}

pub fn criterion_mot_bytetrack_benchmark(c: &mut Criterion) {
    let dets: Array2<f32> = read_npy("benches/data/mot_20-03_yolox_500_dets.npy").unwrap();
    let frame_borders: Array1<u64> =
        read_npy("benches/data/mot_20-03_yolox_500_frame_borders.npy").unwrap();
    let frame_borders = frame_borders.mapv(|x| x as usize);

    c.bench_function("mot_20-03_bytetrack", |b| {
        b.iter(|| run_bytetrack_on_dets(&dets, &frame_borders))
    });
}

criterion_group!(
    benches,
    criterion_mot_sort_benchmark,
    criterion_mot_bytetrack_benchmark
);
criterion_main!(benches);
