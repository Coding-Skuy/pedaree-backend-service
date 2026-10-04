# Arsitektur pedaree-backend-service

TownHall: https://github.com/Coding-Skuy/Pedaree-TownHall

## Lapisan

1. `api` (axum): rute `/v1/stok`, `/v1/mutasi`, `/v1/kecocokan-resep`.
2. `domain`: validasi jumlah, satuan, dan aturan mutasi.
3. `db` (sqlx + PostgreSQL `pedaree`): tabel `pantry_items`, `mutasi_stok`,
   `kecocokan_resep`.

## Alur utama

Klien menambah item -> baris `pantry_items` dibuat dan `mutasi_stok` jenis
`masuk` dicatat. Klien memakai bahan -> mutasi `keluar` dicatat dan jumlah
berkurang. Setiap mutasi menerbitkan event ke topik `pedaree.stok.v1` untuk
pipeline data.

## Keputusan

- Rust + axum 0.7.9 untuk keandalan konkurensi API stok.
- Migrasi SQL berversi di `migrations/`; tidak ada perubahan skema manual.
- Pencocokan resep bersifat tanpa status: definisi diambil dari Pawonee,
  hasilnya dihitung per permintaan.
