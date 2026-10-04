# pedaree-backend-service — Layanan Stok dan Mutasi (Rust)

Divisi **Pedaree (Smart Pantry)**, org **Coding-Skuy**. Opsi A.

TownHall: https://github.com/Coding-Skuy/Pedaree-TownHall

## Ringkasan

Layanan backend Rust untuk stok pantry dan mutasi keluar-masuk bahan.
Basis data utama bernama `pedaree`. Menyediakan API untuk aplikasi seluler,
web, pipeline data, dan model AI.

## Modul pantry-resep

Kontrak lintas divisi (detail: `docs/MODUL-PANTRY-RESEP.md`):

- **Pemilik modul:** Pawonee.
- **Konsumen modul:** Pedaree (backend memanggil API resep Pawonee untuk
  mencocokkan stok dengan kebutuhan resep, lalu mengembalikan status
  ketersediaan bahan).
- Skema resep tidak disimpan permanen di basis data `pedaree`; yang disimpan
  adalah hasil pencocokan per permintaan beserta rujukan `recipe_id` Pawonee.

## Teknologi (versi dikunci)

- Rust 1.84.1
- axum 0.7.9
- tokio 1.42.0
- sqlx 0.8.3 (PostgreSQL)
- serde 1.0.219, serde_json 1.0.140
- PostgreSQL 16.6
- Redis 7.4.1 (antar-muka pengingat kedaluwarsa)

Lihat `Cargo.toml` sebagai sumber kebenaran versi Rust.

## Skema basis data `pedaree`

- `pantry_items(id, nama, satuan, jumlah, kedaluwarsa, dibuat_pada)`
- `mutasi_stok(id, item_id, jenis, jumlah, alasan, dibuat_pada)`
- `kecocokan_resep(id, recipe_id_pawonee, status, dibuat_pada)`

Migrasi ada di `migrations/`.

## Cara jalan

```bash
cargo build
cargo test
DATABASE_URL=postgres://pedaree:pedaree@localhost:5432/pedaree cargo run
```

## CI

Workflow `.github/workflows/ci.yml` menjalankan `cargo fmt --check`,
`cargo clippy`, dan `cargo test`.
