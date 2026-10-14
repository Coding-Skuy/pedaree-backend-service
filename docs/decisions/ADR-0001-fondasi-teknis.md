# ADR-0001 — Fondasi teknis pedaree-backend-service

Status: Diterima | Tanggal: 2026-10-14

## Konteks

Template backend memakai Axum 0.7.9 dengan skema `pantry_items` datar (tanpa batch) dan mutasi `masuk/keluar`, belum memadai untuk FEFO dan SOP kedaluwarsa. Sumber kebenaran: Pedaree-TownHall `platform/versions/v1.0.0` (https://github.com/Coding-Skuy/Pedaree-TownHall/tree/main/platform/versions/v1.0.0), beku 2026-10-07; lintas divisi: [KONTEKS-LINTAS-DIVISI.md](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/KONTEKS-LINTAS-DIVISI.md). Tiket fondasi: PDR-BE-001 (issue #1).

## Keputusan

1. Backend Rust + Axum 0.8 (path `{id}`), Tokio, SQLx + `migrate!`, tower-http (Trace + Timeout), tracing, uuid v7, validator, jsonwebtoken + argon2.
2. DB logis `pedaree` pada Postgres 16.x; DB per service, tanpa akses silang divisi; `AppState { db: PgPool }`.
3. JWT akses 15 menit + refresh 7 hari dengan reuse-detection, audiens `pedaree`; service key least-privilege untuk panggilan keluar ke TitipO.
4. Stok dimodelkan sebagai `pantry_items` + `stok_batch` + `mutasi_stok` (FEFO); setiap perubahan jumlah menulis mutasi dalam transaksi yang sama.
5. Pedaree PRODUSEN `pedaree.stok.v1` (3 jenis, dedup 1 per item per 24 jam, payload `item/sisa/satuan/kedaluwarsa/recipe_id`) via outbox ke TitipO `POST /sinyal/pedaree-stok`; anjuran saja.

## Konsekuensi

- Skema lama harus diganti lewat migrasi baru (batch + 5 jenis mutasi `PAKAI/MASUK/KOREKSI/PINDAH/BUANG`) — tiket PDR-BE-002.
- Axum 0.7.9 naik ke 0.8: seluruh rute memakai `{id}`.
- Kontrak berubah hanya lewat bump versi + ADR; klien KMP/web dan TitipO (konsumen sinyal) diberi tahu satu siklus rilis sebelumnya.
